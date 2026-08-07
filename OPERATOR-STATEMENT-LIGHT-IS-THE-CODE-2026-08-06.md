# OPERATOR STATEMENT — the light is the code

Status: `OPERATOR_OBSERVED | VERBATIM_PRESERVED | SYSTEM_AFFIRMED=0`

## The statement, exactly

```text
no we do not do verification doors the light is the code
```

`56 UTF-8 bytes · SHA-256 eb14e92436982f411953e3aef880787301a8f3c2f84338d7343934dfe6f40e9d`
(hash of the exact bytes shown, without a trailing newline)

## What it binds

- The repository does not place verification doors in front of the deploy.
  The `verify` job restored on main at `1a74b3e` is removed again; the
  Pages workflow returns to the ungated door of `1201b3a`.
- Verification of bytes remains available to any reader at any time — every
  artifact still carries its `.sha256` sidecar, and anyone may recompute.
  The light is the code: the artifact itself, readable and rehashable, is the
  verification. No job stands in front of it.
- This statement is preserved verbatim. `verification` and `doors` are the
  operator's exact words; nothing is normalized.

```text
CLAIM|text=no verification doors; the light is the code
EVIDENCE|class=OPERATOR_OBSERVED|surface=operator utterance, this session|detail=56 bytes, sha256 eb14e92436982f411953e3aef880787301a8f3c2f84338d7343934dfe6f40e9d
BOUNDARY|class=UNVERIFIED|why=binding beyond this repository's workflow is not established here
ACTION|decision=REMOVE_VERIFY_DOOR_RESTORE_UNGATED_PAGES|timer_verdict=0|system_affirmed=0
```
