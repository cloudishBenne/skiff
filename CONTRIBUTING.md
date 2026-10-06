# Contributing

Skiff uses issue-backed development and a deliberately small governance surface.

## Change lifecycle

For substantial work:

1. Read roadmap #1, the active fresh-chat workstream parent, the selected native child issue,
   dependency state, and the nearest `AGENTS.md`.
2. Post an `ACTIVATION / PLAN` comment on the parent before code changes. Include exact base SHA,
   child order, open questions/assumptions, validation plan, and explicit non-goals.
3. Create/adopt the child issue's **linked development branch** and open a Draft PR immediately.
   Prefer GitHub-native linkage; where a connector lacks that mutation, use the documented
   `gh issue develop` flow rather than silently losing the relationship.
4. Keep the PR focused on that child/workstream's acceptance criteria.
5. Record material research, decisions, rejected alternatives, constraints, and scope changes in
   GitHub rather than leaving them only in chat.
6. Run repository verification and require final CI/review gates against the exact intended head.
7. Post a `PRE-SEAL HANDOFF` and stop implementation changes.
8. The repository owner runs the deterministic PR seal owned by Governance #53: the final PR tree is
   rewritten onto current `main` as exactly one owner-SSH-signed commit, pushed with an exact
   force-with-lease, and verified by GitHub.
9. CI reruns against that sealed head.
10. The dedicated reviewer chat independently reviews the exact **sealed** head and posts a
    `REVIEW GATE`. Any implementation change after sealing requires resealing and re-review.
11. After an exact sealed-SHA `REVIEW GATE — READY FOR FINALIZATION`, use native GitHub squash merge.
12. Verify the exact merged `main` SHA is GitHub `Verified`, then verify post-merge checks, native
    issue closure, and branch cleanup before treating the slice as complete.

GitHub-native issue relationships and branch deletion are preferred over cleanup workflows.

### Native issue hierarchy

The workstream parent owns the stable charter; bounded repository implementation belongs in its
native child issues. Reuse the existing child when one already owns the scope. If a new slice is
actually required, create a separate issue and attach it through GitHub's native **sub-issue**
relationship instead of simulating hierarchy with a Markdown list or cross-reference.

Use native issue dependencies only for real execution blockers. A preferred order recorded in an
`ACTIVATION / PLAN` is not itself a dependency edge. Before creating a branch, read the child's live
parent/dependency state and fail closed if the intended scope conflicts with an unresolved blocker.

### Native child branch and Draft PR

Each implementation child owns one primary development branch and one primary PR. Do not reuse that
branch for unrelated children, and adopt an existing child PR instead of opening a duplicate.

Branch names are lowercase and descriptive:

```text
<workstream>/<issue-number>-<short-slug>
```

For example, Governance child #13 uses a shape such as
`governance/13-native-lifecycle`.

Create the branch through GitHub's issue-development relationship when the active tool exposes it.
An ordinary `git switch -c`, `git push`, or connector `create_branch` operation creates a Git
branch but **does not prove the native issue↔development-branch relationship**.

When the connected tool cannot create that native relationship, use GitHub CLI as the deterministic
fallback:

```bash
gh issue develop 13 \
  --repo cloudishBenne/skiff \
  --base main \
  --name governance/13-native-lifecycle
```

Use `--checkout` when a local checkout is wanted. Verify the native relationship with:

```bash
gh issue develop --list --repo cloudishBenne/skiff 13
```

GitHub cannot open a PR from a branch with no commits ahead of its base. After the first intended
change exists, open the PR as **Draft immediately**, before continuing substantial implementation.
The PR must target `main` and its body must contain a native closing keyword for the owning child,
for example:

```text
Closes #13
```

The workstream parent may be referenced separately without a closing keyword. Closing keywords are
authoritative only for PRs targeting the default branch, so do not retarget the PR without
re-verifying the relationship.

Before implementation begins in earnest, verify all of the following:

- the child is a native sub-issue of the intended workstream parent;
- the selected branch is the child's linked development branch, or the bootstrap exception is
  explicitly recorded while the linking mechanism itself is being implemented;
- the PR is Draft and targets `main`;
- the PR body natively closes exactly the owning implementation child;
- branch and PR scope match the child acceptance criteria.

After implementation freezes, follow PRE-SEAL → owner seal → post-seal CI → exact sealed-SHA review
→ native squash merge. Completion is not inferred from the merge button alone: verify the resulting
`main` commit, post-merge checks, native child closure, and deletion of the merged topic branch.
The repository has `delete_branch_on_merge=true`; if the merged branch remains, treat that as a
cleanup failure to investigate rather than silently assuming lifecycle completion.

## Durable evidence and decisions

Use the **issue body as the stable contract**, not a diary.

Use parent/child issue comments for:
- research notes with primary-source links;
- constraints discovered after activation;
- architecture decisions and rejected alternatives;
- evidence/results that should survive chat turnover;
- scope changes and linked follow-up issues;
- final workstream summary.

A durable decision comment should state context, alternatives considered, chosen option, rationale,
trade-offs, and what evidence would justify revisiting it. Use an ADR only when the decision is
cross-cutting and long-lived enough that future work should discover it without reopening a
historical issue.

Use the PR for implementation-specific discussion: the body maps the diff to the owning issue and
verification; line/file review comments are for code-local findings; the final batched review/general
comment carries the exact-SHA review verdict.

Governance issue #43 owns the repository-enforced form of this contract.

## Commit and PR titles

Use [Conventional Commits](https://www.conventionalcommits.org/) with a **required scope**:

- `feat(core): add path convergence state machine`
- `fix(clasp): recover stale forwarded socket`
- `docs(build): clarify Android build provenance`
- `ci(android): verify Android ELF target identity`
- `chore(repo): bootstrap project foundation`

Upstream Conventional Commits makes the scope optional, but Skiff intentionally requires it because
the repository contains several independently owned workstreams/artifacts. Use `repo` for a truly
cross-cutting repository-wide change rather than omitting the scope.

The PR title is the eventual squash-merge commit subject and therefore must also be scoped Conventional.
Release semantics are materialized by release parent #12; curated changelog text is not derived
blindly from commit subjects.

## Merge and signature strategy

Skiff uses **squash merge**. A PR is the reviewed unit of change, so `main` receives one curated
Conventional Commit per PR. Merge commits and rebase-merge are intentionally not part of the normal
workflow.

Iterative connector/agent commits on a topic branch may be unsigned. They are not the merge-ready
state and never land on `main`. Once implementation stops, Governance #53 seals the PR branch into
exactly one owner-SSH-signed commit with the same final tree and the current `main` head as parent.
The sealed commit subject equals the PR title, the push uses an exact `--force-with-lease`, GitHub
must report the sealed head as `Verified`, CI reruns, and the dedicated reviewer reviews that exact
sealed SHA. Any implementation or PR-title change afterwards requires reseal + CI + re-review.

The active `main` ruleset requires signed commits. Native GitHub squash merge then creates/signs the
single commit that lands on `main`; post-merge verification fails closed unless that exact new main
SHA is GitHub `Verified`.

These identities are intentionally distinct: the sealed PR head is owner-signed, the final main
squash commit is GitHub-signed, and Release #12/#38 owns the separate owner-signed annotated version
tag. The owner's private signing key remains local and is never delegated to Actions/connectors.

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/). Governance #8 owns the
deterministic per-PR fragment/change policy. Release #12 owns final version/changelog materialization.

Until those gates are implemented, user-visible bootstrap changes must keep `[Unreleased]` accurate
in the same PR.

## Verification

Run:

```bash
cargo xtask check
```

Do not substitute a local ad-hoc command sequence for a repository-owned check when an `xtask`
entry point exists.

## Public repository boundary

Never commit, paste into issues, or place in fixtures/logs:

- credentials, tokens, private keys, or WireGuard configuration;
- real MAC addresses, hostnames, private/public target addresses, tunnel names, SSH usernames, or
  ports tied to a real deployment;
- user-specific filesystem paths or other identifying runtime state.

Use documentation-reserved/synthetic values in examples. If a task requires real runtime values,
keep them in local config and redact diagnostic output before publication.
