# GitHub integration agent guide

Applies to `.github/`.

- GitHub-native relationships and repository settings are preferred over custom automation.
- Workflow permissions must be least-privilege and explicit.
- Public pull-request workflows must not expose secrets or gain write authority by default.
- PR templates and workflows must reinforce the issue -> Draft PR -> CI/review -> squash merge lifecycle.
