# Fresh-chat start prompts

These prompts are convenience launchers. Live repository state, issue bodies/comments, dependencies,
PRs, and the nearest `AGENTS.md` are canonical. If a prompt and live state disagree, follow live
state and record the discrepancy.

Every merge-ready implementation **PR** must stop at a PRE-SEAL HANDOFF. The owner seal runs next,
CI reruns, and the dedicated reviewer chat reviews only the exact sealed PR head. If the
implementation or PR title changes afterwards, reseal and repeat review before merge. One fresh chat
may own a workstream parent across multiple child slices; after Governance #13 is complete, follow
its native child→branch→PR mapping for each slice rather than bundling unrelated children for
convenience. After the parent's children are complete/closed, leave the required final parent summary
and close the parent according to its workstream-closure contract.

Treat native dependency edges as execution constraints, not suggestions. If a current workstream
references functionality owned by a downstream workstream that is blocked on it, implement only the
stable interface/capability boundary and test it with synthetic stubs; do not pull downstream
implementation forward. If a relevant Draft PR already exists, adopt it instead of opening a
duplicate. Research-only workstreams do not fabricate a PR when no repository diff is needed.

## Governance — parent #8

```text
Work on cloudishBenne/skiff parent issue #8 (Governance) as one focused implementation chat.

Cold-boot from the live repository: establish exact main SHA, read root AGENTS.md,
CONTRIBUTING.md, docs/ARCHITECTURE.md, the relevant scoped AGENTS.md files, roadmap #1,
parent #8, all native sub-issues/dependencies, and any active PR. Treat repo/GitHub state as
canonical; do not rely on prior chat memory.

First post an ACTIVATION / PLAN comment on #8 with exact base SHA, child order, open questions,
non-goals, and verification plan. Reconcile the live child contracts and repository docs before
coding if bootstrap-era wording conflicts with the now-proven seal/signature lifecycle. Start with
#13: for that bootstrap-governance slice, adopt any relevant existing Draft PR or use the currently
documented development-branch fallback. Once #13 has materialized the canonical native
child→branch→PR mapping, use that mapping for the remaining #8 children. Record research,
primary-source evidence, decisions and rejected alternatives as durable issue comments. Keep
GitHub-native relationships native.

Goal: make the issue→development-branch→Draft-PR lifecycle, Conventional change metadata,
exact-head review/post-merge gates, signature-verification policy, evidence/review handoff, and
public security policy deterministic for all later chats.

For each merge-ready Governance PR slice, do NOT merge. Post a PRE-SEAL HANDOFF with the exact
implementation head/tree, completed acceptance criteria, checks, evidence links, risks and
deferrals. Return control to the user so the owner-signing seal from Governance #53 can rewrite the
branch to one verified commit without changing its tree. CI must rerun; only then does the dedicated
reviewer chat review the exact sealed SHA. Any later implementation or PR-title change requires
reseal + re-review. After all #8 children are complete/closed, post the final parent summary and
close #8.
```

## Core — parent #3

```text
Work on cloudishBenne/skiff parent issue #3 (Core) as one focused implementation chat.

Cold-boot from live GitHub/repo state using root AGENTS.md and CONTRIBUTING.md. Read roadmap #1,
parent #3, its sub-issues #17–#20, dependencies, docs/ARCHITECTURE.md, docs/CLI_STYLE.md, and
crates/skiff/AGENTS.md. Confirm Governance #8 is complete before implementation.

Post ACTIVATION / PLAN on #3 before code. Preserve the canonical prerequisite order:
functional private/home-path proof → optional path activation → concurrent host/SSH probes →
WoL only when appropriate → distinct host-ready and SSH-ready waits → OpenSSH → existing/new
Zellij session → configured remote-entrypoint contract → transport-loss reconvergence. Core #19
must test that entrypoint boundary with a synthetic/stub command; do not require downstream Shell #4
to be implemented before Core can close. DNS-answer assertions must support split/local DNS without
forcing literal IPs. Keep all topology in runtime config.

Record research/decisions in issues and implementation/review evidence in PRs. For each Core PR
slice, when the selected child acceptance criteria are complete and checks are green, stop that PR's
implementation and post a PRE-SEAL HANDOFF. The owner seal must run, CI must rerun, and only the
exact sealed SHA may be reviewed. Any later implementation or PR-title change requires reseal +
re-review. After all Core children are complete/closed, post the final parent summary and close #3.
```

## lla patch — parent #6

```text
Work on cloudishBenne/skiff parent issue #6 (lla patch) as one focused implementation chat.

Cold-boot from live repository/GitHub state. Read root/scoped AGENTS.md, roadmap #1, parent #6,
children #21–#23, dependencies, upstream.lock.toml, and current lla primary sources. Confirm
Governance #8 is complete.

Post ACTIVATION / PLAN on #6. Maintain a minimal deterministic patch against the exact pinned
upstream revision; do not create a permanent fork unless evidence requires it. Implement compact
strict two-line output, long-mode filename wrapping with correct hanging indentation/icon
placement, and batch Rust-native per-entry Git XY status. The patched lla is for normal Termux
and normal Linux-host use as well as the Skiff remote shell.

Record upstream research, formatter/plugin constraints, performance evidence, decisions and
rejected alternatives in issue comments. For each merge-ready PR slice, stop at green checks and
post a PRE-SEAL HANDOFF. The owner must seal the branch through Governance #53, CI must rerun, and
only the exact sealed SHA may receive the dedicated reviewer chat's REVIEW GATE before merge. After
all #6 children are complete/closed, post the final parent summary and close #6.
```

## Clasp — parent #5

```text
Work on cloudishBenne/skiff parent issue #5 (Clasp) as one focused implementation chat.

Cold-boot from live repo/GitHub state; read root/scoped AGENTS.md, roadmap #1, parent #5,
children #24–#27, dependencies, Core interfaces, and relevant OpenSSH/Zellij/Termux primary
sources. Confirm #8 and #3 are complete before implementation.

Post ACTIVATION / PLAN on #5. Preserve the trust model: remote→Android copy uses OSC 52;
Android→remote reads are explicit through the Clasp Unix-socket broker forwarded inside the
existing SSH connection; clasp get/set remain composable CLI operations; clasp-zellij is only the
explicit one-keystroke paste UX and must not create ambient clipboard-read authority.

Record protocol/security research and alternatives in issue comments. Test payload bounds, stale
socket/reconnect behavior and no-content logging. For each merge-ready PR slice, stop implementation,
post PRE-SEAL HANDOFF, then wait for owner seal + CI + the dedicated reviewer chat's exact sealed-head
REVIEW GATE before merge. After all #5 children are complete/closed, post the final parent summary
and close #5.
```

## Shell integration — parent #4

```text
Work on cloudishBenne/skiff parent issue #4 (Shell integration) as one focused implementation chat.

Cold-boot from live repository/GitHub state; read root/scoped AGENTS.md, roadmap #1, parent #4,
children #28–#30 and dependencies. Confirm #8, #3, #5 and #6 are complete before implementation.
Core supplies the remote-entrypoint/session contract; this workstream supplies the real dedicated
Bash/Zellij/Starship implementation plugged into that boundary.

Post ACTIVATION / PLAN on #4. Implement a dedicated Skiff Bash rcfile/Zellij shell while also
making patched lla/ll available in the user's ordinary Linux-host Bash and normal Termux shell.
Do not duplicate alias/completion definitions: use a Skiff-owned shared Bash fragment sourced by
both normal .bashrc and the dedicated rcfile, and a managed Fish conf.d fragment on Termux.
Install/source useful shell completions from exact CLI versions and reuse upstream completions
where available. Starship owns repository-wide Git context.

Record shell-integration decisions and compatibility findings in issues. For each merge-ready PR
slice, stop at green checks, post PRE-SEAL HANDOFF, then wait for owner seal + CI and the exact
sealed-head REVIEW GATE before merge. After all #4 children are complete/closed, post the final
parent summary and close #4.
```

## Build / CI — parent #7

```text
Work on cloudishBenne/skiff parent issue #7 (Build/CI) as one focused implementation chat.

Cold-boot from live state; read root/.github/workflows scoped AGENTS.md, roadmap #1, parent #7,
children #31–#33, upstream.lock.toml, and the referenced cloudishBenne/codex Termux-port evidence.
Confirm #8, #3, #5 and #6 are complete before implementation.

Post ACTIVATION / PLAN on #7. GitHub Actions is the build authority. Do NOT use the
termux-packages build environment. Use direct pinned Android NDK cross-building for
aarch64-linux-android with explicit linker/sysroot/native deps and fail-closed ELF target
verification. Produce independent skiff, clasp, patched-lla Linux/Android artifacts plus
clasp-zellij WASM, checksums and provenance/attestation handoff. Keep workflow permissions
least-privilege and third-party Actions pinned immutably where practical.

Record build/toolchain research, failures and decisions in issues. For each merge-ready PR slice,
stop with exact head/check evidence and post PRE-SEAL HANDOFF. Require owner seal + CI and the
dedicated reviewer chat's exact sealed-head REVIEW GATE before merge. After all #7 children are
complete/closed, post the final parent summary and close #7.
```

## Install — parent #11

```text
Work on cloudishBenne/skiff parent issue #11 (Install) as one focused implementation chat.

Cold-boot from live state; read root/scoped AGENTS.md, roadmap #1, parent #11, children #34–#36,
release/build contracts and dependencies. Confirm #8, #4 and #7 are complete before implementation.

Post ACTIVATION / PLAN on #11. Implement idempotent install/update/uninstall flows for Termux and
the Linux host around the verified GitHub Release contract rather than rebuilding locally. Because
Release #12 is downstream of Install #11, do not require a previously published production release:
exercise resolver/integrity/target/atomic-update behavior with deterministic fixtures or controlled
draft/test-release evidence, and leave first production publication to #12. Install project binaries
independently. Patched lla must be available for normal Termux use and globally for the Linux user's
ordinary shell as well as Skiff/Zellij. Manage shell fragments/completions without rewriting
unrelated user config; preserve rollback/update safety and target verification.

Record distribution/installer decisions and failure evidence in issues. For each merge-ready PR
slice, stop at green checks and post PRE-SEAL HANDOFF. Require owner seal + CI and the dedicated
reviewer chat's exact sealed-head REVIEW GATE before merge. After all #11 children are complete/closed,
post the final parent summary and close #11.
```

## Release — parent #12

```text
Work on cloudishBenne/skiff parent issue #12 (Release) as one focused implementation chat.

Cold-boot from live repo/GitHub state; read root/scoped AGENTS.md, roadmap #1, parent #12,
children #37–#39 and completed Governance/Build/Install contracts. Confirm #8, #7 and #11 are
complete, then confirm the exact intended main SHA.

Post ACTIVATION / PLAN on #12. Treat release preparation as a two-SHA process: verify the exact
preparation-base main SHA, let #37 materialize version/CHANGELOG changes through the normal
seal/review/squash lifecycle, then capture and verify the resulting new main SHA as the only final
release SHA. Build artifacts/provenance and the owner-controlled SSH-signed annotated version tag
must all bind to that final release SHA, never the pre-preparation base. The private signing key must
remain local to the owner (Termux is an acceptable signing workstation when tool versions support
it) and must never enter Actions/connectors/repository secrets. Publish only complete draft releases
and verify immutable-release and asset/attestation integrity.

Any step requiring the owner's local signing key must stop and provide an exact command plus
pre/post verification rather than attempting to obtain the key. Stop before publication/merge
where owner action or dedicated review is required. For each repository-change PR, post PRE-SEAL
HANDOFF, require owner seal + CI, and wait for the dedicated reviewer chat's exact sealed-head REVIEW
GATE. After all #12 children and the publication acceptance criteria are complete, post the final
parent summary and close #12.
```

## Deferred WireGuard companion — parent #9

```text
Only start this chat if roadmap #1 explicitly activates deferred parent #9 and its blockers #3/#8
are complete.

Cold-boot from live state and read parent #9, children #40–#41, Core's path-activator contract and
Android/WireGuard/Termux primary sources. Post ACTIVATION / PLAN first.

Implement the least-privilege CONTROL_TUNNELS companion and Rust activator without broad root/
Shizuku authority. Support tunnel-state query for ownership/diagnostics, but keep the functional
home-path probe authoritative. Never tear down a tunnel that predated Skiff.

Use the same durable issue/PR evidence and exact-SHA handoff/review protocol as active workstreams.
```

## Deferred Kata/T3 Code sandbox research — parent #44

```text
Research cloudishBenne/skiff parent issue #44 only; do not implement a production sandbox yet.

Cold-boot from live issue/repo state and read children #45–#49. Post ACTIVATION / PLAN with the
questions to resolve. Use current primary Kata Containers and T3 Code sources, plus measured host
experiments where appropriate, to evaluate isolation, VMM/resource elasticity, T3 Code headless/
mobile topology, Codex/optional Claude credentials, project/worktree/container models, and optional
Termux SSH/Zellij access.

Record RESEARCH NOTE and DECISION comments with sources, alternatives, caveats and measurements.
The required output is a recommended architecture/threat/resource/credential model and an explicit
decision whether implementation belongs in Skiff or a separate repository. No production
implementation in this research chat. Do not invent a PR merely to satisfy the normal implementation
lifecycle; use an exact-SHA REVIEW GATE only if this research actually changes repository files or
lands a durable repository artifact.
```

## Deferred distribution/upstream research — parent #50

```text
Research cloudishBenne/skiff parent issue #50 only after the relevant APIs and lla patch are mature.

Cold-boot from live state; read children #51–#52 and current crates.io/Cargo/lla contribution
requirements. Post ACTIVATION / PLAN first. Decide which first-party packages should be published
to crates.io, package naming/version/feature/MSRV policy and cargo-install UX, and prepare a
minimal upstream lla contribution/migration strategy.

Record research, rejected alternatives and upstream constraints in issue comments. Do not publish
crates or open an upstream PR until the research decision and dedicated review are complete. A local
Skiff PR is needed only if the research changes repository files or lands a durable repository
artifact; otherwise the issue/decision record is the research output.
```
