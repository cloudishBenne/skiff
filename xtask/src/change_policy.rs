use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const SUBJECT_TYPES: &[&str] = &[
    "feat", "fix", "docs", "refactor", "perf", "test", "build", "ci", "chore", "revert",
];

const TARGETS: &[&str] = &[
    "clasp",
    "clasp-zellij",
    "distribution",
    "lla-patch",
    "repository",
    "skiff",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ChangeClass {
    Internal,
    Patch,
    Feature,
    Breaking,
}

impl ChangeClass {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "internal" => Ok(Self::Internal),
            "patch" => Ok(Self::Patch),
            "feature" => Ok(Self::Feature),
            "breaking" => Ok(Self::Breaking),
            _ => Err(format!(
                "invalid change class {value:?}; expected internal, patch, feature, or breaking"
            )),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Patch => "patch",
            Self::Feature => "feature",
            Self::Breaking => "breaking",
        }
    }
}

#[derive(Clone, Debug)]
struct Workstream {
    roadmap: u64,
    planned_max: ChangeClass,
}

#[derive(Clone, Debug)]
struct TargetImpact {
    name: String,
    observed: ChangeClass,
}

#[derive(Clone, Debug)]
struct SliceRecord {
    roadmap: u64,
    workstream: u64,
    slice: u64,
    pr: u64,
    subject: String,
    targets: Vec<TargetImpact>,
    changelog: String,
    breaking_evidence: Vec<String>,
}

struct Policy {
    slices: BTreeMap<u64, SliceRecord>,
}

fn read_utf8(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn parse_table(text: &str, source: &str) -> Result<toml::Table, String> {
    text.parse::<toml::Table>()
        .map_err(|error| format!("invalid TOML in {source}: {error}"))
}

fn take_value(table: &mut toml::Table, key: &str, context: &str) -> Result<toml::Value, String> {
    table
        .remove(key)
        .ok_or_else(|| format!("{context}: missing required field {key:?}"))
}

fn take_u64(table: &mut toml::Table, key: &str, context: &str) -> Result<u64, String> {
    let value = take_value(table, key, context)?;
    let integer = value
        .as_integer()
        .ok_or_else(|| format!("{context}: {key:?} must be an integer"))?;
    let integer =
        u64::try_from(integer).map_err(|_| format!("{context}: {key:?} must be positive"))?;
    if integer == 0 {
        return Err(format!("{context}: {key:?} must be positive"));
    }
    Ok(integer)
}

fn take_string(table: &mut toml::Table, key: &str, context: &str) -> Result<String, String> {
    match take_value(table, key, context)? {
        toml::Value::String(value) if !value.trim().is_empty() => Ok(value),
        toml::Value::String(_) => Err(format!("{context}: {key:?} must not be empty")),
        _ => Err(format!("{context}: {key:?} must be a string")),
    }
}

fn take_string_array(
    table: &mut toml::Table,
    key: &str,
    context: &str,
) -> Result<Vec<String>, String> {
    let toml::Value::Array(values) = take_value(table, key, context)? else {
        return Err(format!("{context}: {key:?} must be an array of strings"));
    };

    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| match value {
            toml::Value::String(value) if !value.trim().is_empty() => Ok(value),
            toml::Value::String(_) => Err(format!("{context}: {key:?}[{index}] must not be empty")),
            _ => Err(format!("{context}: {key:?}[{index}] must be a string")),
        })
        .collect()
}

fn take_targets(table: &mut toml::Table, context: &str) -> Result<Vec<TargetImpact>, String> {
    let toml::Value::Array(rows) = take_value(table, "target", context)? else {
        return Err(format!("{context}: \"target\" must be an array of tables"));
    };

    rows.into_iter()
        .enumerate()
        .map(|(index, row)| {
            let target_context = format!("{context}: target[{index}]");
            let toml::Value::Table(mut row) = row else {
                return Err(format!("{target_context} must be a table"));
            };
            let target = TargetImpact {
                name: take_string(&mut row, "name", &target_context)?,
                observed: ChangeClass::parse(&take_string(&mut row, "observed", &target_context)?)?,
            };
            reject_unknown_fields(&row, &target_context)?;
            Ok(target)
        })
        .collect()
}

fn reject_unknown_fields(table: &toml::Table, context: &str) -> Result<(), String> {
    if table.is_empty() {
        return Ok(());
    }

    let mut keys: Vec<_> = table.keys().cloned().collect();
    keys.sort();
    Err(format!("{context}: unknown field(s): {}", keys.join(", ")))
}

fn parse_workstreams(text: &str, source: &str) -> Result<BTreeMap<u64, Workstream>, String> {
    let mut root = parse_table(text, source)?;
    let schema = take_u64(&mut root, "schema", source)?;
    if schema != 1 {
        return Err(format!("{source}: unsupported schema {schema}; expected 1"));
    }

    let toml::Value::Array(rows) = take_value(&mut root, "workstream", source)? else {
        return Err(format!(
            "{source}: \"workstream\" must be an array of tables"
        ));
    };
    reject_unknown_fields(&root, source)?;

    let mut workstreams = BTreeMap::new();
    for (index, row) in rows.into_iter().enumerate() {
        let context = format!("{source}: workstream[{index}]");
        let toml::Value::Table(mut row) = row else {
            return Err(format!("{context} must be a table"));
        };

        let issue = take_u64(&mut row, "issue", &context)?;
        let roadmap = take_u64(&mut row, "roadmap", &context)?;
        let planned_max = ChangeClass::parse(&take_string(&mut row, "planned_max", &context)?)?;
        let _decision = take_string(&mut row, "decision", &context)?;
        reject_unknown_fields(&row, &context)?;

        if workstreams
            .insert(
                issue,
                Workstream {
                    roadmap,
                    planned_max,
                },
            )
            .is_some()
        {
            return Err(format!("{source}: duplicate Workstream issue #{issue}"));
        }
    }

    if workstreams.is_empty() {
        return Err(format!("{source}: at least one Workstream is required"));
    }

    Ok(workstreams)
}

fn parse_slice(text: &str, source: &str) -> Result<SliceRecord, String> {
    let mut table = parse_table(text, source)?;
    let schema = take_u64(&mut table, "schema", source)?;
    if schema != 1 {
        return Err(format!("{source}: unsupported schema {schema}; expected 1"));
    }

    let record = SliceRecord {
        roadmap: take_u64(&mut table, "roadmap", source)?,
        workstream: take_u64(&mut table, "workstream", source)?,
        slice: take_u64(&mut table, "slice", source)?,
        pr: take_u64(&mut table, "pr", source)?,
        subject: take_string(&mut table, "subject", source)?,
        changelog: take_string(&mut table, "changelog", source)?,
        breaking_evidence: take_string_array(&mut table, "breaking_evidence", source)?,
        targets: take_targets(&mut table, source)?,
    };
    reject_unknown_fields(&table, source)?;
    Ok(record)
}

fn has_pr_number_suffix(description: &str) -> bool {
    if !description.ends_with(')') {
        return false;
    }

    let Some(start) = description.rfind(" (#") else {
        return false;
    };
    let digits = &description[start + 3..description.len() - 1];
    !digits.is_empty() && digits.chars().all(|character| character.is_ascii_digit())
}

fn subject_class(subject: &str) -> Result<ChangeClass, String> {
    if subject.trim() != subject || subject.contains(['\n', '\r']) {
        return Err("subject must be one trimmed line".to_owned());
    }

    let (prefix, description) = subject.split_once(": ").ok_or_else(|| {
        "subject must use the form type(scope): description or type(scope)!: description".to_owned()
    })?;
    if description.is_empty() {
        return Err("subject description must not be empty".to_owned());
    }
    if has_pr_number_suffix(description) {
        return Err("subject must not end with a GitHub PR-number suffix such as (#56)".to_owned());
    }

    let (prefix, breaking) = match prefix.strip_suffix('!') {
        Some(prefix) => (prefix, true),
        None => (prefix, false),
    };
    let open = prefix
        .find('(')
        .ok_or_else(|| "subject scope is required".to_owned())?;
    if !prefix.ends_with(')') || open == 0 {
        return Err("subject must contain exactly one required scope".to_owned());
    }

    let kind = &prefix[..open];
    let scope = &prefix[open + 1..prefix.len() - 1];
    if prefix[open + 1..].contains('(') || scope.contains(')') {
        return Err("subject scope syntax is invalid".to_owned());
    }
    if !SUBJECT_TYPES.contains(&kind) {
        return Err(format!("unsupported Conventional Commit type {kind:?}"));
    }
    if scope.is_empty()
        || !scope.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(format!("invalid subject scope {scope:?}"));
    }

    if breaking {
        Ok(ChangeClass::Breaking)
    } else {
        match kind {
            "feat" => Ok(ChangeClass::Feature),
            "fix" => Ok(ChangeClass::Patch),
            _ => Ok(ChangeClass::Internal),
        }
    }
}

fn no_changelog_path(slice: u64, path: &str) -> Result<bool, String> {
    let prefix = format!("changes/{slice}.");
    let Some(suffix) = path.strip_prefix(&prefix) else {
        return Err(format!(
            "Slice #{slice}: changelog path must start with {prefix:?}"
        ));
    };

    match suffix {
        "added.md" | "changed.md" | "deprecated.md" | "removed.md" | "fixed.md" | "security.md" => {
            Ok(false)
        }
        "no-changelog.md" => Ok(true),
        _ => Err(format!(
            "Slice #{slice}: unsupported changelog path {path:?}"
        )),
    }
}

fn validate_slice_semantics(slice: &SliceRecord, workstream: &Workstream) -> Result<(), String> {
    if slice.roadmap != workstream.roadmap {
        return Err(format!(
            "Slice #{}: roadmap #{} does not match Workstream #{} roadmap #{}",
            slice.slice, slice.roadmap, slice.workstream, workstream.roadmap
        ));
    }

    let derived = subject_class(&slice.subject)
        .map_err(|error| format!("Slice #{}: invalid subject: {error}", slice.slice))?;

    if slice.targets.is_empty() {
        return Err(format!(
            "Slice #{}: at least one target is required",
            slice.slice
        ));
    }

    let mut seen = BTreeSet::new();
    for target in &slice.targets {
        if !TARGETS.contains(&target.name.as_str()) {
            return Err(format!(
                "Slice #{}: unsupported target {:?}",
                slice.slice, target.name
            ));
        }
        if !seen.insert(target.name.as_str()) {
            return Err(format!(
                "Slice #{}: duplicate target {:?}",
                slice.slice, target.name
            ));
        }
    }
    if slice
        .targets
        .windows(2)
        .any(|pair| pair[0].name.as_str() >= pair[1].name.as_str())
    {
        return Err(format!(
            "Slice #{}: targets must be sorted and unique",
            slice.slice
        ));
    }

    let Some(max_observed) = slice.targets.iter().map(|target| target.observed).max() else {
        return Err(format!(
            "Slice #{}: at least one target is required",
            slice.slice
        ));
    };
    if max_observed != derived {
        return Err(format!(
            "Slice #{}: maximum per-target observed class {} contradicts subject-derived {}",
            slice.slice,
            max_observed.as_str(),
            derived.as_str()
        ));
    }
    if max_observed > workstream.planned_max {
        return Err(format!(
            "Slice #{}: maximum observed {} exceeds Workstream #{} planned maximum {}",
            slice.slice,
            max_observed.as_str(),
            slice.workstream,
            workstream.planned_max.as_str()
        ));
    }

    let _ = no_changelog_path(slice.slice, &slice.changelog)?;

    match derived {
        ChangeClass::Breaking if slice.breaking_evidence.is_empty() => {
            return Err(format!(
                "Slice #{}: breaking changes require durable breaking evidence",
                slice.slice
            ));
        }
        ChangeClass::Breaking => {}
        _ if !slice.breaking_evidence.is_empty() => {
            return Err(format!(
                "Slice #{}: non-breaking changes must not carry breaking evidence",
                slice.slice
            ));
        }
        _ => {}
    }

    Ok(())
}

fn slice_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let directory = root.join("change-accounting/slices");
    let entries = fs::read_dir(&directory)
        .map_err(|error| format!("failed to read {}: {error}", directory.display()))?;
    let mut paths = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read an entry from {}: {error}",
                directory.display()
            )
        })?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == "toml")
        {
            paths.push(path);
        }
    }
    paths.sort();

    if paths.is_empty() {
        return Err(format!(
            "{} must contain at least one Slice record",
            directory.display()
        ));
    }

    Ok(paths)
}

fn validate_change_files(root: &Path, slices: &BTreeMap<u64, SliceRecord>) -> Result<(), String> {
    let directory = root.join("changes");

    for slice in slices.values() {
        let path = root.join(&slice.changelog);
        let content = read_utf8(&path)?;
        if content.trim().is_empty() {
            return Err(format!(
                "Slice #{}: {} must not be empty",
                slice.slice,
                path.display()
            ));
        }
    }

    let entries = fs::read_dir(&directory)
        .map_err(|error| format!("failed to read {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read an entry from {}: {error}",
                directory.display()
            )
        })?;
        let path = entry.path();
        if !path.is_file() {
            return Err(format!("{} may contain files only", directory.display()));
        }

        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| format!("non-UTF-8 filename in {}", directory.display()))?;
        if name == "README.md" {
            continue;
        }

        let Some((slice_text, _)) = name.split_once('.') else {
            return Err(format!("unrecognized change fragment filename {name:?}"));
        };
        let slice = slice_text
            .parse::<u64>()
            .map_err(|_| format!("unrecognized change fragment filename {name:?}"))?;
        let record = slices.get(&slice).ok_or_else(|| {
            format!("orphan change fragment {name:?} has no Slice accounting record")
        })?;
        let relative = format!("changes/{name}");
        let _ = no_changelog_path(slice, &relative)?;
        if record.changelog != relative {
            return Err(format!(
                "Slice #{slice}: extra or contradictory change fragment {relative:?}; accounting points to {:?}",
                record.changelog
            ));
        }
    }

    Ok(())
}

fn load_policy(root: &Path) -> Result<Policy, String> {
    let workstreams_path = root.join("change-accounting/workstreams.toml");
    let workstreams = parse_workstreams(
        &read_utf8(&workstreams_path)?,
        &workstreams_path.display().to_string(),
    )?;

    let mut slices = BTreeMap::new();
    let mut prs = BTreeSet::new();

    for path in slice_paths(root)? {
        let source = path.display().to_string();
        let slice = parse_slice(&read_utf8(&path)?, &source)?;
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("invalid Slice filename {}", path.display()))?;
        let file_slice = stem
            .parse::<u64>()
            .map_err(|_| format!("Slice filename must be numeric: {}", path.display()))?;
        if file_slice != slice.slice {
            return Err(format!(
                "{} names Slice #{file_slice} but contains Slice #{}",
                path.display(),
                slice.slice
            ));
        }

        let workstream = workstreams.get(&slice.workstream).ok_or_else(|| {
            format!(
                "Slice #{} references unknown Workstream #{}",
                slice.slice, slice.workstream
            )
        })?;
        validate_slice_semantics(&slice, workstream)?;

        if !prs.insert(slice.pr) {
            return Err(format!("duplicate PR #{} in Slice accounting", slice.pr));
        }
        if slices.insert(slice.slice, slice).is_some() {
            return Err(format!("duplicate Slice #{file_slice} accounting record"));
        }
    }

    validate_change_files(root, &slices)?;
    Ok(Policy { slices })
}

pub fn check(root: &Path) -> Result<(), String> {
    let _ = load_policy(root)?;
    Ok(())
}

pub fn check_pr(root: &Path, pr: u64, title: &str) -> Result<(), String> {
    let _ = subject_class(title).map_err(|error| format!("invalid PR title: {error}"))?;
    let policy = load_policy(root)?;
    let mut matches = policy.slices.values().filter(|slice| slice.pr == pr);
    let slice = matches
        .next()
        .ok_or_else(|| format!("PR #{pr} has no Slice accounting record"))?;
    if matches.next().is_some() {
        return Err(format!("PR #{pr} has multiple Slice accounting records"));
    }
    if slice.subject != title {
        return Err(format!(
            "PR #{pr} title {title:?} does not exactly match Slice #{} subject {:?}",
            slice.slice, slice.subject
        ));
    }
    Ok(())
}

fn aggregate_impacts<'a>(
    slices: impl Iterator<Item = &'a SliceRecord>,
) -> BTreeMap<String, ChangeClass> {
    let mut aggregate = BTreeMap::new();

    for slice in slices {
        for target in &slice.targets {
            aggregate
                .entry(target.name.clone())
                .and_modify(|current| {
                    if target.observed > *current {
                        *current = target.observed;
                    }
                })
                .or_insert(target.observed);
        }
    }

    aggregate
}

pub fn summary(root: &Path) -> Result<String, String> {
    let policy = load_policy(root)?;
    let aggregate = aggregate_impacts(policy.slices.values());

    let mut output = String::from("target\tobserved\n");
    for (target, class) in aggregate {
        writeln!(&mut output, "{target}\t{}", class.as_str())
            .map_err(|error| format!("failed to build change summary: {error}"))?;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::{
        ChangeClass, SliceRecord, TargetImpact, Workstream, aggregate_impacts, parse_slice,
        subject_class, validate_slice_semantics,
    };

    fn target(name: &str, observed: ChangeClass) -> TargetImpact {
        TargetImpact {
            name: name.to_owned(),
            observed,
        }
    }

    fn slice(subject: &str, targets: Vec<TargetImpact>, changelog: &str) -> SliceRecord {
        SliceRecord {
            roadmap: 1,
            workstream: 8,
            slice: 14,
            pr: 56,
            subject: subject.to_owned(),
            targets,
            changelog: changelog.to_owned(),
            breaking_evidence: Vec::new(),
        }
    }

    #[test]
    fn classifies_scoped_conventional_subjects() {
        assert_eq!(
            subject_class("chore(governance): enforce metadata").unwrap(),
            ChangeClass::Internal
        );
        assert_eq!(
            subject_class("fix(skiff): recover session").unwrap(),
            ChangeClass::Patch
        );
        assert_eq!(
            subject_class("feat(clasp): add explicit paste").unwrap(),
            ChangeClass::Feature
        );
        assert_eq!(
            subject_class("feat(core)!: change config contract").unwrap(),
            ChangeClass::Breaking
        );
    }

    #[test]
    fn rejects_unscoped_or_pr_suffixed_subjects() {
        for subject in [
            "feat: missing scope",
            "feat(Core): uppercase scope",
            "chore(governance): metadata (#56)",
            "docs(governance): ",
        ] {
            assert!(subject_class(subject).is_err(), "{subject:?} should fail");
        }
    }

    #[test]
    fn validates_planned_scope_and_subject_maximum() {
        let internal_workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Internal,
        };
        let valid = slice(
            "chore(governance): enforce metadata",
            vec![target("repository", ChangeClass::Internal)],
            "changes/14.no-changelog.md",
        );
        validate_slice_semantics(&valid, &internal_workstream).unwrap();

        let feature_workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Feature,
        };
        let subject_overstates_targets = slice(
            "feat(governance): add capability",
            vec![target("repository", ChangeClass::Patch)],
            "changes/14.changed.md",
        );
        assert!(
            validate_slice_semantics(&subject_overstates_targets, &feature_workstream).is_err()
        );

        let target_exceeds_subject = slice(
            "fix(governance): repair policy",
            vec![target("repository", ChangeClass::Feature)],
            "changes/14.changed.md",
        );
        assert!(validate_slice_semantics(&target_exceeds_subject, &feature_workstream).is_err());

        let scope_creep = slice(
            "feat(governance): add public behavior",
            vec![target("repository", ChangeClass::Feature)],
            "changes/14.added.md",
        );
        assert!(validate_slice_semantics(&scope_creep, &internal_workstream).is_err());
    }

    #[test]
    fn changelog_state_is_independent_of_semantic_class() {
        let feature_workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Feature,
        };

        let internal_with_curated_fragment = slice(
            "docs(governance): explain operator-visible policy",
            vec![target("repository", ChangeClass::Internal)],
            "changes/14.changed.md",
        );
        validate_slice_semantics(&internal_with_curated_fragment, &feature_workstream).unwrap();

        let patch_with_no_changelog = slice(
            "fix(skiff): correct internal accounting",
            vec![target("skiff", ChangeClass::Patch)],
            "changes/14.no-changelog.md",
        );
        validate_slice_semantics(&patch_with_no_changelog, &feature_workstream).unwrap();
    }

    #[test]
    fn mixed_target_impacts_remain_independent() {
        let workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Feature,
        };
        let record = slice(
            "feat(governance): coordinate mixed component update",
            vec![
                target("clasp", ChangeClass::Feature),
                target("skiff", ChangeClass::Patch),
            ],
            "changes/14.changed.md",
        );
        validate_slice_semantics(&record, &workstream).unwrap();

        let aggregate = aggregate_impacts([&record].into_iter());
        assert_eq!(aggregate.get("clasp"), Some(&ChangeClass::Feature));
        assert_eq!(aggregate.get("skiff"), Some(&ChangeClass::Patch));
    }

    #[test]
    fn breaking_changes_require_evidence_without_forcing_changelog_state() {
        let workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Breaking,
        };
        let mut record = slice(
            "feat(core)!: replace protocol",
            vec![target("skiff", ChangeClass::Breaking)],
            "changes/14.no-changelog.md",
        );
        assert!(validate_slice_semantics(&record, &workstream).is_err());

        record.breaking_evidence = vec![
            "https://github.com/cloudishBenne/skiff/issues/14#issuecomment-example".to_owned(),
        ];
        validate_slice_semantics(&record, &workstream).unwrap();
    }

    #[test]
    fn rejects_unsorted_or_duplicate_targets() {
        let workstream = Workstream {
            roadmap: 1,
            planned_max: ChangeClass::Feature,
        };
        let unsorted = slice(
            "feat(governance): mixed impacts",
            vec![
                target("skiff", ChangeClass::Patch),
                target("clasp", ChangeClass::Feature),
            ],
            "changes/14.changed.md",
        );
        assert!(validate_slice_semantics(&unsorted, &workstream).is_err());

        let duplicate = slice(
            "feat(governance): duplicate target",
            vec![
                target("clasp", ChangeClass::Feature),
                target("clasp", ChangeClass::Patch),
            ],
            "changes/14.changed.md",
        );
        assert!(validate_slice_semantics(&duplicate, &workstream).is_err());
    }

    #[test]
    fn rejects_unknown_slice_fields() {
        let document = r#"
schema = 1
roadmap = 1
workstream = 8
slice = 14
pr = 56
subject = "chore(governance): enforce metadata"
changelog = "changes/14.no-changelog.md"
breaking_evidence = []
surprise = true

[[target]]
name = "repository"
observed = "internal"
"#;
        assert!(parse_slice(document, "fixture").is_err());
    }

    #[test]
    fn rejects_missing_slice_fields() {
        let document = r#"
schema = 1
roadmap = 1
workstream = 8
slice = 14
pr = 56
subject = "chore(governance): enforce metadata"
changelog = "changes/14.no-changelog.md"

[[target]]
name = "repository"
observed = "internal"
"#;
        assert!(parse_slice(document, "fixture").is_err());
    }

    #[test]
    fn parses_per_target_impact_tables() {
        let document = r#"
schema = 1
roadmap = 1
workstream = 8
slice = 14
pr = 56
subject = "feat(governance): mixed impacts"
changelog = "changes/14.changed.md"
breaking_evidence = []

[[target]]
name = "clasp"
observed = "feature"

[[target]]
name = "skiff"
observed = "patch"
"#;
        let record = parse_slice(document, "fixture").unwrap();
        assert_eq!(record.targets.len(), 2);
        assert_eq!(record.targets[0].name, "clasp");
        assert_eq!(record.targets[0].observed, ChangeClass::Feature);
        assert_eq!(record.targets[1].name, "skiff");
        assert_eq!(record.targets[1].observed, ChangeClass::Patch);
    }
}
