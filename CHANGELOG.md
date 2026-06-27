# Changelog

## Unreleased

### Fixed
- **CSA time tracking**: server echoes moves as `+9796FU,T18`; `parse_time_line` only matched standalone `T18`, so `time_left_ms` was never decremented and the engine bled time on every move. Fixed `parse_time_from_echo` to handle both formats.
- Read `Total_Time`/`Byoyomi`/`Increment` from `Game_Summary` header instead of parsing the game_id string.

## [0.2.0] – 2026-06-28

### Added
- Match runner: Elo rating, CI, LOS, illegal-move detection, repetition draw, SFEN openings.
- `SpeculativeSearcher` enabled in USI; king-capture panics fixed.
- NNUE training pipeline improvements.
- GitHub Actions CI; all clippy warnings fixed.

### Fixed
- Mate score direction in `spec_alpha_beta`.
- NMP fail-soft + depth-scaled LMR formula.

## [0.1.0] – Initial

- NNUE-based shogi engine with alpha-beta search.
- CSA v2.2 TCP client for floodgate.
- USI protocol support.
