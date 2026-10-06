# Clasp crate agent guide

Issue #5 owns substantial implementation in this crate.

- Clipboard reads must be explicit; do not create ambient remote clipboard-read authority.
- Broker transport is local/Unix-socket scoped and forwarded through the existing SSH connection.
- Do not expose a LAN TCP clipboard service.
- Bound request/frame/payload sizes and never log clipboard content.
- `clasp get` writes clipboard data to stdout; `clasp set` reads data from stdin.
- Zellij UX belongs in `plugins/clasp-zellij`, not in this transport crate.
