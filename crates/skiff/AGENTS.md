# Skiff crate agent guide

Issue #3 owns substantial implementation in this crate.

- Default behavior is prerequisite-driven convergence to the persistent Zellij session.
- Home-path readiness precedes target/WoL checks.
- A successful SSH probe proves the target is online.
- Keep host-ready and SSH-ready waits distinct for diagnostics.
- On transport loss, reconverge from the home-path prerequisite; do not reset remote Zellij state.
- All topology/endpoints/timeouts come from runtime config. No deployment-specific constants.
- OpenSSH is the transport; do not grow a custom SSH implementation without a reviewed reason.
