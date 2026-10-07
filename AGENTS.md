# Repository agent guide

This is the root operating entry point for coding/repository agents. Keep it compact; implementation
rules live in the nearest scoped `AGENTS.md`.

## Mandatory cold boot

Before substantial planning, review, or mutation:

1. Establish the exact current branch and commit SHA. Do not silently mix repository states.
2. Read [`CONTRIBUTING.md`](CONTRIBUTING.md).
3. Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).
4. Read [`README.md`](README.md) for current product status.
5. Read roadmap issue #1, the active **fresh-chat workstream parent**, its native implementation
   sub-issue(s), dependency state, and the active Draft PR.
6. Load the nearest scoped `AGENTS.md` for every path you will change.
7. Read additional files only to resolve an active design, implementation, verification, or review
   question.

Repository files, live GitHub issue/PR state, and exact Git state outrank conversational memory.
[`docs/START_PROMPTS.md`](docs/START_PROMPTS.md) is a convenience launcher; it does not override
live issue/repository state.

## Change discipline

- A substantial fresh implementation chat owns one workstream parent issue and works through its
  native child issues; do not silently absorb a sibling workstream.
- Start by posting an `ACTIVATION / PLAN` comment on the parent with the exact base SHA, intended
  child order, open questions, explicit non-goals, and verification plan.
- Work issue-first. Each implementation child owns one primary development branch and one primary
  Draft PR; adopt an existing child PR instead of opening a duplicate.
- Create the branch through the issue's native development relationship. If the active connector
  cannot create that relationship, use the documented `gh issue develop` fallback in
  `CONTRIBUTING.md`; an ordinary branch creation is not equivalent to native linkage.
- Open the Draft PR as soon as the branch has its first intended commit. Target `main` and use a
  native closing keyword such as `Closes #123` for the owning implementation child.
- Prefer bounded slices: `read -> decide -> change -> verify -> checkpoint`.
- Record research, architectural decisions, rejected alternatives, surprising constraints, and
  durable evidence in GitHub; do not leave them only in chat.
- Route out-of-scope findings/debt to linked issues instead of silently expanding the current PR.
- Treat native issue dependencies as execution constraints. If an active issue references a downstream
  workstream that is still blocked on the current one, implement/test only the stable interface or
  capability boundary with synthetic stubs; do not create a dependency cycle by pulling downstream
  implementation forward.
- Use scoped Conventional Commits (`type(scope): description`); scope is mandatory. The exact PR title is the squash-merge commit subject; never append a `(#PR)` suffix.
- Every repository-changing Slice carries `change-accounting/slices/<slice>.toml` with explicit per-target semantic impacts plus exactly one curated `changes/<slice>.<category>.md` fragment or explicit `changes/<slice>.no-changelog.md` reason. Changelog relevance is independent from semantic class; planned Workstream scope guards live in `change-accounting/workstreams.toml`.
- Run repository-owned verification (`cargo xtask check`) before declaring a slice ready.
- Implementation work stops at a `PRE-SEAL HANDOFF`. The owner then seals the branch into exactly
  one owner-signed commit with an unchanged tree; Governance #53 owns the deterministic procedure.
- CI reruns against the sealed head. The dedicated reviewer chat independently cold-boots and posts
  an exact-SHA `REVIEW GATE` on that **sealed** PR head.
- Any implementation change after sealing invalidates both the seal and review: reseal, rerun CI,
  and re-review the new exact head before merge.
- Do not bypass required CI or merge directly into protected `main`.

## Public-repository safety

Never commit or publish real runtime topology, credentials, keys, tunnel configuration, MAC
addresses, hostnames/addresses, SSH identities, user-specific filesystem paths, clipboard contents,
or other private deployment state. Use synthetic/documentation-reserved examples only.

Do not log clipboard contents. Do not weaken a trust boundary merely to simplify local setup.

## Architecture boundaries

- Skiff: network/readiness/SSH/Zellij orchestration.
- Clasp: explicit clipboard bridge.
- clasp-zellij: Zellij WASM paste UX only.
- lla patch: two-line file listing and per-entry Git status for normal Termux/Linux-host use as well
  as the Skiff remote shell.
- Starship: shell and repository-wide Git context.
- Build/CI: reproducible artifacts and provenance.
- Install: target bootstrap/update/uninstall from verified release artifacts.
- Release: SemVer, owner-signed version tags, attestations, and immutable publication.
- `upstream.lock.toml`: sole owner of exact third-party source revisions.

Durable architecture changes that cross these boundaries require explicit rationale in the owning
issue/PR and an update to `docs/ARCHITECTURE.md` when the stable model changes.
