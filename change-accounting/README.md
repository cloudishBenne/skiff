# Change accounting

This directory is the machine-readable planning/accounting input owned by Governance #14. It is
separate from the human-readable Keep a Changelog fragments under `changes/`.

## Workstream scope guard

`workstreams.toml` declares each Workstream's planned maximum change class:

```text
internal < patch < feature < breaking
```

A Slice whose maximum per-target observed class exceeds that bound must not become merge-ready until
the Workstream has a durable scope-change decision and the registry is updated through the normal
reviewed lifecycle.

## Slice records

Every repository-changing Slice has exactly one `slices/<issue>.toml` record with schema version,
Roadmap/Workstream/Slice/PR identity, the exact PR subject, Slice-owned changelog/no-changelog path,
breaking evidence when applicable, and one or more explicit target-impact tables.

Example:

```toml
schema = 1
roadmap = 1
workstream = 8
slice = 14
pr = 56
subject = "feat(governance): coordinate mixed component update"
changelog = "changes/14.changed.md"
breaking_evidence = []

[[target]]
name = "clasp"
observed = "feature"

[[target]]
name = "skiff"
observed = "patch"
```

The exact scoped Conventional subject derives the Slice's overall maximum semantic class:

- `type(scope)!: ...` -> `breaking`;
- `feat(scope): ...` -> `feature`;
- `fix(scope): ...` -> `patch`;
- other accepted scoped types -> `internal`.

The validator requires:

```text
max(per-target observed impacts) == subject-derived overall impact
```

This keeps the PR title honest about the maximum impact without promoting lower-impact targets.
Targets must be sorted and unique.

Allowed target identities initially are `skiff`, `clasp`, `clasp-zellij`, `lla-patch`,
`distribution`, and `repository`. Attribution does not assign a version.

Changelog relevance is independent from semantic class. The `changelog` field references exactly
one non-empty curated fragment or explicit no-changelog reason; it is not inferred from the target
impacts or Conventional subject.

Breaking semantic impact requires durable `breaking_evidence`; non-breaking records must not carry
breaking evidence. That rule is independent from changelog state.

`cargo xtask change-summary` aggregates each target's own observed class across the retained Slice
corpus. Release #12/#37 remains responsible for selecting the applicable release interval and for
concrete distribution/component version materialization.
