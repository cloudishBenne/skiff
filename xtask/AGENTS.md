# xtask agent guide

`xtask` owns deterministic repository-local automation that would otherwise become copy/paste shell
folklore.

- Keep commands small, composable, and fail-closed.
- Prefer Rust stdlib unless an external dependency materially improves correctness.
- Do not reimplement GitHub-native issue/PR/branch behavior here.
- Foundation #2 owns the bounded public-repository boundary check in `cargo xtask check`; keep it deterministic and avoid heuristic secret-scanner creep.
- Governance #8 owns per-PR change/policy validation.
- Build/CI #7 owns build provenance automation.
- Release #12 owns SemVer calculation, changelog materialization, and release-state automation.
- A command that mutates release state must validate exact inputs before mutation.
