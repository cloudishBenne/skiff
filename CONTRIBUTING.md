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
11. After a current-head `READY FOR FINALIZATION` gate, use native GitHub squash merge.
12. Verify the exact merged `main` SHA is GitHub `Verified`, then verify post-merge checks, native
    issue closure, and branch cleanup before treating the slice as complete.

GitHub-native issue relationships and branch deletion are preferred over cleanup workflows.

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

GitHub's native server-side squash merge creates/signs the final squash commit. That verification is
not the same thing as a signature made with the repository owner's local private key. Governance #8
owns the compatible commit-verification policy; release #12 owns owner-controlled signed version
tags and immutable release publication.

Git signatures cannot be retrofitted onto an existing commit object without creating a new commit
SHA. Therefore the policy is to keep unsigned iterative topic-branch commits out of `main`, verify
the GitHub-signed squash commit after merge, and sign release tags deliberately before publication.

Governance #53 resolves the signed-commit compatibility problem by sealing each merge-ready PR into
one owner-signed commit before final review. The `main` ruleset requires signed commits. Unsigned
iterative connector commits remain allowed only before the seal and never land on `main`.

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
