# lib-ip

`ip` — IP address, CIDR, and socket primitives (Handl stdlib port).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the interface — `op` declarations for `ip.*`, `ip.cidr.*`, `ip.socket.*` |
| `src/ipv4.hnl`, `src/cidr.hnl`, `src/socket.hnl` | per-domain modules |
| `src/internal/num.hnl` | numeric helpers |
| `src/ip.test.hnl` | tests |
| `src/docs/{ip,cidr,socket}.yml` | docs specs |
| `pkg.hnl`, `scripts/hnl-gate.sh` | manifest + `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Interface lives in `.hnl`; the native backend is `lib-ip-native`
  (`ip-native` module) — this repo declares the `ip.*`/`ip.cidr.*`/
  `ip.socket.*` surface.
- Pure `.hnl`, no Rust crate in this repo.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
