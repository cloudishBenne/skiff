# Documentation agent guide

Applies to `docs/`.

- Keep stable architecture/policy here; task-local exploration belongs in issues/PRs.
- Do not duplicate exact upstream SHAs from `upstream.lock.toml`.
- Keep diagrams consistent with the text and avoid private deployment examples.
- `ARCHITECTURE.md` owns stable component boundaries and flows.
- `CLI_STYLE.md` owns project-specific CLI presentation policy and links outward for broad guidance.
- `START_PROMPTS.md` is a convenience launcher for fresh chats; prompts must route to live issues
  rather than duplicate detailed task contracts that can drift.
- Prefer concise canonical rules plus links over copied prose that can drift.
