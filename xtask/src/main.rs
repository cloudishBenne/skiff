mod change_policy;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn run(mut command: Command) -> Result<(), String> {
    let display = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| format!("failed to start {display}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed ({status}): {display}"))
    }
}

fn command_output(mut command: Command) -> Result<Vec<u8>, String> {
    let display = format!("{command:?}");
    let output = command
        .output()
        .map_err(|error| format!("failed to start {display}: {error}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!("command failed ({}): {display}", output.status))
    }
}

fn repository_root() -> Result<PathBuf, String> {
    let mut command = Command::new("git");
    command.args(["rev-parse", "--show-toplevel"]);
    let stdout = command_output(command)?;
    let root = String::from_utf8(stdout)
        .map_err(|error| format!("git returned a non-UTF-8 repository path: {error}"))?;
    Ok(PathBuf::from(root.trim()))
}

fn tracked_files(root: &Path) -> Result<Vec<String>, String> {
    let mut command = Command::new("git");
    command.current_dir(root).args(["ls-files", "-z"]);
    let stdout = command_output(command)?;
    let paths = String::from_utf8(stdout)
        .map_err(|error| format!("git returned a non-UTF-8 tracked path: {error}"))?;

    Ok(paths
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect())
}

fn forbidden_tracked_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let lower = normalized.to_ascii_lowercase();
    let name = lower.rsplit('/').next().unwrap_or(lower.as_str());
    let extension = Path::new(&lower)
        .extension()
        .and_then(|value| value.to_str());

    lower == "config.toml"
        || lower.starts_with(".skiff/")
        || lower.ends_with(".local.toml")
        || name == ".env"
        || (name.starts_with(".env.") && name != ".env.example")
        || lower.starts_with(".ssh/")
        || lower.contains("/.ssh/")
        || lower.starts_with(".gnupg/")
        || lower.contains("/.gnupg/")
        || matches!(name, "id_rsa" | "id_dsa" | "id_ecdsa" | "id_ed25519")
        || matches!(extension, Some("pem" | "p12" | "pfx" | "key"))
        || ((lower.contains("wireguard") || name.starts_with("wg")) && extension == Some("conf"))
}

fn contains_private_key_material(bytes: &[u8]) -> bool {
    // Keep the forbidden markers split in source so this checker does not flag its own implementation.
    let markers = [
        ["-----BEGIN OPENSSH ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN RSA ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN DSA ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN EC ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN ENCRYPTED ", "PRIVATE KEY-----"].concat(),
        ["-----BEGIN PGP ", "PRIVATE KEY BLOCK-----"].concat(),
    ];

    markers.iter().any(|marker| {
        bytes
            .windows(marker.len())
            .any(|window| window == marker.as_bytes())
    })
}

fn check_public_repository_boundary(root: &Path) -> Result<(), String> {
    let mut violations = Vec::new();

    for path in tracked_files(root)? {
        if forbidden_tracked_path(&path) {
            violations.push(format!("forbidden tracked path: {path}"));
            continue;
        }

        let bytes = fs::read(root.join(&path))
            .map_err(|error| format!("failed to inspect tracked file {path}: {error}"))?;
        if contains_private_key_material(&bytes) {
            violations.push(format!(
                "private-key material marker in tracked file: {path}"
            ));
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "public repository boundary violation(s):\n  {}",
            violations.join("\n  ")
        ))
    }
}

fn check() -> Result<(), String> {
    let root = repository_root()?;
    check_public_repository_boundary(&root)?;
    change_policy::check(&root)?;

    let mut fmt = Command::new("cargo");
    fmt.args(["fmt", "--all", "--", "--check"]);
    run(fmt)?;

    let mut clippy = Command::new("cargo");
    clippy.args([
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
    run(clippy)?;

    let mut test = Command::new("cargo");
    test.args(["test", "--workspace"]);
    run(test)
}

fn parse_pr_number(value: &str) -> Result<u64, String> {
    let pr = value
        .parse::<u64>()
        .map_err(|_| format!("invalid PR number {value:?}"))?;
    if pr == 0 {
        return Err("PR number must be positive".to_owned());
    }
    Ok(pr)
}

fn pr_policy(pr: &str, title: &str) -> Result<(), String> {
    let root = repository_root()?;
    change_policy::check_pr(&root, parse_pr_number(pr)?, title)
}

fn change_summary() -> Result<(), String> {
    let root = repository_root()?;
    let summary = change_policy::summary(&root)?;
    print!("{summary}");
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:\n  cargo xtask check\n  cargo xtask pr-policy <pr-number> <exact-pr-title>\n  cargo xtask change-summary"
    );
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("check") if args.next().is_none() => check(),
        Some("pr-policy") => {
            let pr = args.next();
            let title = args.next();
            let extra = args.next();
            if let (Some(pr), Some(title), None) = (pr, title, extra) {
                pr_policy(&pr, &title)
            } else {
                usage();
                return ExitCode::from(2);
            }
        }
        Some("change-summary") if args.next().is_none() => change_summary(),
        _ => {
            usage();
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{contains_private_key_material, forbidden_tracked_path};

    #[test]
    fn rejects_runtime_and_secret_paths() {
        for path in [
            "config.toml",
            ".skiff/state.json",
            "dev.local.toml",
            ".env",
            ".env.production",
            ".ssh/id_ed25519",
            "fixtures/private.pem",
            "wireguard/wg0.conf",
        ] {
            assert!(forbidden_tracked_path(path), "{path} should be rejected");
        }
    }

    #[test]
    fn permits_public_examples_and_normal_source_paths() {
        for path in [
            "config.example.toml",
            ".env.example",
            "crates/skiff/src/main.rs",
            "docs/ARCHITECTURE.md",
            "tests/key.rs",
        ] {
            assert!(!forbidden_tracked_path(path), "{path} should be allowed");
        }
    }

    #[test]
    fn detects_private_key_material_markers() {
        for private_key_fixture in [
            ["prefix\n-----BEGIN OPENSSH ", "PRIVATE KEY-----\nredacted"].concat(),
            [
                "prefix\n-----BEGIN ENCRYPTED ",
                "PRIVATE KEY-----\nredacted",
            ]
            .concat(),
            ["prefix\n-----BEGIN DSA ", "PRIVATE KEY-----\nredacted"].concat(),
            [
                "prefix\n-----BEGIN PGP ",
                "PRIVATE KEY BLOCK-----\nredacted",
            ]
            .concat(),
        ] {
            assert!(contains_private_key_material(
                private_key_fixture.as_bytes()
            ));
        }
        assert!(!contains_private_key_material(
            b"synthetic documentation without private key material"
        ));
    }
}
