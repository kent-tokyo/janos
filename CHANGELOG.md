# Changelog

## Unreleased

## [0.2.4] – 2026-06-28

### Added
- `sekirei-train --positions <jsonl>` — accept a [shogiesa](https://github.com/kent-tokyo/shogiesa) `positions.jsonl` file as an alternative to `--games`; skips CSA parsing and trains from pre-extracted SFENs with phase/side/source metadata.
- `PositionSample`: carries `phase`, `side_to_move`, `ply`, `source` from shogiesa tags for training control.
- `--phase-weights <spec>` — per-phase loss multipliers (e.g. `opening=0.5,middlegame=1.0,endgame=1.2`).
- `--side-balance` — equalise black/white sample weights based on training-split distribution.
- `--source-cap <N>` — deterministic hash-based per-source sample cap (seed-reproducible, order-independent).
- `--validation-ratio <f>` / `--seed <n>` — hold-out split via SFEN hash; logs `loss_raw` and `loss_weighted` per epoch.
- `--checkpoint-dir <dir>` — save epoch checkpoints to a custom directory with `.meta.json` (training params + sample counts).
- `--teacher-cache <path>` / `--reuse-teacher-cache` — cache teacher scores (sfen → score_cp) to JSONL; epoch 2+ skips search entirely on cache hits.

## [0.2.3] – 2026-06-28

### Added
- `sekirei-train --label-threshold-cp <n>` — configurable adv/equal/disadv boundary (default: 120 cp).
- Epoch stats log: `missing_rate`, `avg_weight`, `matched` counts printed per epoch when `--scored` is active; `missing_rate > 50%` triggers a SFEN-mismatch warning.
- `Trainer::reset_epoch_stats()` — resets `total_loss / total_count / total_weight / dropped_missing` between epochs so per-epoch stats are correct.

### Fixed
- `avg_loss` now divides by `total_weight` (sum of stability weights) instead of `total_count`; previously under-reported loss when `--stability-weighted` was active.
- `scored.rs`: duplicate SFENs in the scored JSONL are now averaged (previously last-wins, which made results dependent on file ordering); switched JSON parsing from manual string scan to `serde_json`.

## [0.2.2] – 2026-06-28

### Added
- `setoption MoveOverhead` (default 50 ms) — subtracts network latency from time budget.
- `setoption Ponder` option declaration; `go ponder` treated as infinite search.
- `ponderhit` command — aborts ponder search; GUI follows with a real `go`.
- `sekirei-train --export <path>` — exports observation JSONL for quietset stability filtering.
- `sekirei-train --depths <list>` — comma-separated search depths for export (default: `4,6,8`).
- `sekirei-match-runner --games-per-position <n>` — cover-all mode: play N games per SFEN entry.
- `sekirei-train --quiet`, `--min-ply`, `--label-depth` — quiet position filtering based on "Study of the Proper NNUE Dataset".
- `setoption Threads` — configure rayon global thread pool from GUI.

### Fixed
- **`go depth N` time cap**: pure depth search (no clock args) no longer capped at 50 ms.
- **TT size**: `Tt::new` now uses floor-power-of-two; previously halved capacity for power-of-2 inputs (e.g. 64 MB → 32 MB).
- **Root TT bound**: stores `Bound::Lower` on fail-high instead of always `Bound::Exact`.
- **USI search thread race**: `JoinHandle` now stored and joined on `stop`/`usinewgame`/`go`/`quit`; prevents stale `bestmove` output.
- Time control: tighter divisor (÷15) when < 30 s remain; panic mode when < 5 s and byoyomi exists.
- CSA client: `dotenvy` loads `.env`; env vars renamed `FLOODGATE_ACCOUNT` / `FLOODGATE_TRIP`.

## [0.2.0] – 2026-06-28

### Added
- Match runner: Elo rating, CI, LOS, illegal-move detection, repetition draw, SFEN openings.
- `SpeculativeSearcher` enabled in USI; king-capture panics fixed.
- NNUE training pipeline improvements.
- GitHub Actions CI + smoke job; all clippy warnings fixed.
- `setoption EvalFile` support in USI engine.
- CI pre-commit hook (`.githooks/pre-commit`).

### Fixed
- Mate score direction in `spec_alpha_beta`.
- NMP fail-soft + depth-scaled LMR formula.
- **CSA time tracking**: `parse_time_from_echo` now handles `+9796FU,T18` server echo format; `time_left_ms` was never decremented before.
- Read `Total_Time`/`Byoyomi`/`Increment` from `Game_Summary` header instead of parsing the game_id string.

## [0.1.0] – Initial

- NNUE-based shogi engine with alpha-beta search.
- CSA v2.2 TCP client for floodgate.
- USI protocol support.
