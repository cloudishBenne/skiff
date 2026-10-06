# Rust crates agent guide

Applies to first-party executable/library crates under `crates/`.

- Preserve product boundaries from `docs/ARCHITECTURE.md`.
- Prefer shared portable logic plus explicit target-specific modules over broad platform conditionals.
- Do not hide unavailable platform capabilities behind silent no-ops.
- Keep CLI parsing separate from orchestration/domain logic so state transitions are unit-testable.
- Unsafe Rust is forbidden by workspace policy unless a later reviewed architecture change revises it.
