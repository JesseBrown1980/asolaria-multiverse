# RELIC event clock — completed sample 6

This additive sample records the fifth observed `CLOCK -> TICK -> TOCK`
cycle, and the first in which the door itself was observed to open.

## CLOCK

```text
main (sample 5)      1a74b3e6ec76b858c8ab06ae5eaa2aa180595570
sample-5 commit      fc9daa0 (on liris/relic-clock-sample-4, merged below)
sample-5 HBP SHA-256 8d665fe9f1894504898d9e663ca397422c7c4a15c409e4314f4bbe52f9f0bcb2
```

## TICK

GitHub main was next observed at
`2763287249d3adb3428d59a00d992fc57ac12431` — the merge of PR #3. The
identity changed by 6 commits: 18 files changed, 209 lines inserted, 63
deleted. The merge carries samples 4 and 5, the operator statement
`no we do not do verification doors the light is the code` (56 bytes,
`eb14e924…f40e9d`), the removal of the verification door, and the ride-along
commit `the one button launches GitHub` (`8dfe03e`, index.html +5/−4).

## TOCK

```text
index.html                        11,291 bytes  aac31697f0fb26bcf3198d405c2ec7ea0ce32f807badaf788f887c180315bded  sidecar match
repository-wide sweep at 2763287  85 / 85 sidecars match
```

## The door opened

The Pages workflow run on the merge commit `2763287` completed with
conclusion `success` at 2026-08-07T00:06:53Z. The two runs before it — both
made while a verification door stood in front of the deploy — concluded
`failure` (`1a74b3e` at 23:54:56Z, `8dfe03e` at 00:03:07Z). With the door
ungated, the build published. Deployment success is a measured workflow
conclusion; page reachability from any given reader is not measured here.

```text
CLAIM|text=the ungated door deployed; the light is the code held
EVIDENCE|class=MEASURED_GITHUB|surface=refs, sidecars, workflow runs|detail=1a74b3e -> 2763287; 85/85 sidecars; run success on 2763287
BOUNDARY|class=UNVERIFIED|why=RELIC receipt, reader reachability, and causation are not measured
ACTION|decision=PUBLISH_SAMPLE_6|timer_verdict=0|system_affirmed=0
```
