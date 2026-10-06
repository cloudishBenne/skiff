# Skiff

Skiff is a Rust workspace for resilient remote-shell access across changing network paths.

> **Status:** bootstrap only. The architecture and implementation are tracked in the native issue
> hierarchy under [roadmap issue #1](https://github.com/cloudishBenne/skiff/issues/1).

The intended default experience is a single `skiff` command that converges from an unknown network
state to a persistent Zellij session without losing remote work when the client changes networks.
The same repository also owns Clasp, an explicit Android/remote clipboard bridge, a small Zellij
plugin for deliberate paste, and a pinned patch set for the `lla` file lister.

## Design constraints

- Real topology, device identifiers, credentials, keys, tunnel names, and user-specific paths never
  belong in this public repository.
- User configuration lives outside the repository, normally under `~/.config/skiff/`.
- OpenSSH remains the transport and Zellij owns remote session persistence.
- Network convergence follows prerequisites: home path -> target host -> SSH -> Zellij.
- Third-party source identities used by derived builds are owned by [`upstream.lock.toml`](upstream.lock.toml).
- Linux, Android/Termux, WASM, and patched `lla` release artifacts are produced by GitHub Actions.
- The patched `lla` is intended for normal interactive use on both Termux and the Linux host, not
  only inside Skiff's Zellij session.
- Installation consumes verified GitHub Release artifacts; final releases use signed/attested,
  immutable publication policy.

## Workstream model

Roadmap #1 aggregates **fresh-chat workstream parents**. Each substantial implementation chat owns
one parent and works through its native child issues. Governance #8 is intentionally the first
workstream after foundation #2 so later chats inherit deterministic branch/PR/change gates.

## Repository map

- `crates/skiff/` — network/SSH/Zellij orchestrator (parent issue #3).
- `crates/clasp/` — explicit clipboard broker/client (parent issue #5).
- `plugins/clasp-zellij/` — Zellij WASM UX integration (parent issue #5).
- `patches/lla/` — minimal pinned `lla` patch set (parent issue #6).
- `xtask/` — repository-local deterministic automation.
- `docs/` — architecture and CLI policy.
- issue #7 — reproducible builds/artifact provenance.
- issue #11 — Termux/Linux install/update/uninstall flows.
- issue #12 — signed, attested, immutable releases.

Repository agents must start with [`AGENTS.md`](AGENTS.md). Human contributors should start with
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## Bootstrap verification

```bash
cargo xtask check
```

The placeholder binaries intentionally do not implement product behavior yet. Substantial feature
work belongs in the corresponding workstream parent, native child issue, and Draft PR.
