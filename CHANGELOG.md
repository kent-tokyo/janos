# Changelog

## Unreleased

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
