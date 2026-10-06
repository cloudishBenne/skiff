# CLI UX and terminal presentation

## Normative baseline

Use [clig.dev](https://clig.dev/) as the default CLI interaction-design reference. Also follow
[NO_COLOR](https://no-color.org/) and normal stdout/stderr separation. This policy is adapted from the
public PipeWire Headroom Lab CLI policy but intentionally reduced to Skiff's needs.

## Defaults

The default invocation must be the right thing. `skiff` should converge toward the configured
persistent session without requiring routine mode flags.

Use `clap` for parsing, usage, built-in help, argument relationships, and suggestions. Do not build a
parallel custom help parser. Use `anstream` and `anstyle` for project-owned semantic terminal output
unless a later measured need justifies a richer UI layer.

## Human output

Human output should use:

- one blank line before the compact command/product header and one after final output;
- stable semantic sections instead of raw subprocess streams;
- two-space indentation for events within a section;
- blank lines only between semantic groups;
- concise evidence and one explicit next action when action is required.

Default mode should not expose command traces. A later `--verbose` mode may add diagnostics while
preserving the same hierarchy.

## Semantic presentation

- success `✓`: green;
- next action `→`: cyan;
- warning `!`: yellow;
- error `✗`: red;
- info `·`: neutral;
- headings: bold;
- secondary verbose command text: dim.

Meaning must remain understandable without color.

ANSI behavior is per-stream and TTY-aware. Emit no styling to non-interactive streams or when
`TERM=dumb`. Suppress semantic colors when `NO_COLOR` is non-empty or the user passes `--no-color`.
Non-color emphasis such as bold may remain on a capable TTY.

Do not add progress spinners unless a measured UX need justifies them.

## Stream contract

- primary requested data: stdout;
- human status and errors: stderr;
- machine-readable output, when introduced, must be a stable undecorated protocol and never contain
  ANSI escapes.

## Testing

Presentation changes should cover the deterministic behavior they claim, including non-TTY output,
TTY semantic color, `NO_COLOR`, `TERM=dumb`, explicit `--no-color`, stderr routing, and stable blank-line
hierarchy.
