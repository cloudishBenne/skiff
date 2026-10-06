# lla patch agent guide

Issue #6 owns this patch set.

- Implement layout in lla core; the plugin API cannot register the required host-level formatter.
- Compact two-line mode is exactly two physical lines per entry.
- Long mode may wrap only the filename portion, with the Nerd Font icon on the filename's first line
  and continuation hanging-indent aligned to the filename start.
- `--git` places Git XY status before the icon on the filename line.
- Prefer one Rust-native `gix` status map per repository listing; never spawn `git` per entry.
- Do not duplicate repository-wide Git summary already shown by Starship.
