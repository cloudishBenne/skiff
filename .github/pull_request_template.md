## Summary

<!-- What changes, and why? -->

## Linked issue

<!--
Target main and use a closing keyword for exactly the owning implementation child, for example:
Closes #123
Link the workstream parent separately without a closing keyword.
-->

- [ ] Owning child is a native sub-issue of the intended workstream parent
- [ ] Development branch is natively linked to that child, or a documented bootstrap exception applies
- [ ] PR was opened as Draft immediately after the first intended branch commit
- [ ] PR body contains the native closing relationship for the owning child

## Research / decisions

<!-- Link durable issue comments for material research, decisions, rejected alternatives, or write "None". -->

## Verification

- [ ] `cargo xtask check`
- [ ] Relevant platform/build checks are green
- [ ] Acceptance criteria for the owning child issue are covered
- [ ] No real runtime topology, credentials, clipboard data, or other private deployment state is included

## Architecture / release impact

<!-- Note any stable architecture, public CLI, changelog, SemVer, install, or release impact. Use "None" when genuinely none. -->

## Seal / reviewer handoff

<!-- Complete only when implementation is finished. The owner seal happens before external review. -->

- [ ] `PRE-SEAL HANDOFF` records the implementation head/tree and completed scope
- [ ] Owner seal rewrote the branch to exactly one GitHub-Verified owner-signed commit
- [ ] Post-seal tree matches the pre-seal implementation tree
- [ ] CI is green on the sealed head
- [ ] Known risks and deferred work are recorded
- [ ] Dedicated reviewer chat has reviewed the exact sealed head SHA
- [ ] Current-head `REVIEW GATE` verdict is `READY FOR FINALIZATION`
