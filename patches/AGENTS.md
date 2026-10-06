# Patch-set agent guide

Applies to maintained third-party patch sets.

- Exact upstream revisions come only from `upstream.lock.toml`.
- Keep patches minimal, reviewable, deterministic, and suitable for eventual upstreaming.
- Patch application must fail closed on drift.
- Do not vendor full upstream source trees into this repository.
