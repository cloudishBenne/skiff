# Changelog fragments

These files are curated human-readable release-note inputs. They follow the Keep a Changelog
categories and are deliberately separate from machine-readable planning/accounting under
`change-accounting/`.

Each repository-changing Slice owns exactly one changelog-state file.

For a change that warrants a user/operator-facing release note, use one curated category:

```text
changes/<slice>.added.md
changes/<slice>.changed.md
changes/<slice>.deprecated.md
changes/<slice>.removed.md
changes/<slice>.fixed.md
changes/<slice>.security.md
```

When no release-note entry is warranted, use:

```text
changes/<slice>.no-changelog.md
```

The no-changelog file contains a short human-readable reason. A Slice may not carry both a curated
fragment and a no-changelog marker, and the selected file must not be empty.

This choice is independent from Conventional-Commit semantic impact. For example, an internally
classified documentation/refactor change may still deserve a curated note, while a patch-classified
repository/tooling change may have an explicit no-changelog reason.

Release #12/#37 later materializes validated fragments into the release changelog; Governance #14
does not assign release or component versions.
