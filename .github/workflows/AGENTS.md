# GitHub Actions agent guide

Applies to workflow YAML.

- Pin third-party Actions by immutable commit SHA where practical.
- Default to `permissions: contents: read` or less; add write scopes only for a reviewed workflow that
  genuinely requires them.
- CI is read-only. Release/promotion workflows must be separated from untrusted PR execution.
- Linux and Android/Termux are distinct targets. Android release outputs must be proven Android/Bionic
  artifacts, not generic Linux ARM64 binaries.
- Android/Termux builds follow issue #7 and the direct NDK cross-build pattern, not termux-packages.
- Workflows consume `upstream.lock.toml`; they do not own duplicated upstream pins.
