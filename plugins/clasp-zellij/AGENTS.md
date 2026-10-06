# clasp-zellij agent guide

Issue #5 owns substantial implementation.

- This is a Zellij WASM UX adapter for deliberate paste into the focused pane.
- It invokes the stable Clasp CLI/broker boundary; it does not reimplement clipboard transport.
- Never enable global ambient clipboard reads as a shortcut.
- Paste must be caused by an explicit user action/key binding.
- Keep WASM permissions minimal and test failure behavior when Clasp is unavailable.
