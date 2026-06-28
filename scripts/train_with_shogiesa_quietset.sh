#!/usr/bin/env bash
# Full shogiesa + quietset + sekirei training pipeline.
#
# Usage:
#   bash scripts/train_with_shogiesa_quietset.sh [CSA_DIR] [OUTPUT_WEIGHTS] [BASELINE_WEIGHTS]
#
# Environment overrides:
#   DEPTHS=2,4,6   search depths for shogiesa label (default: 2,4)
#   GAMES=400      games for Elo comparison (default: 400)
#   MIN_PLY=20     minimum ply to extract (default: 20)
#   MAX_PLY=160    maximum ply to extract (default: 160)
#
# Examples:
#   bash scripts/train_with_shogiesa_quietset.sh
#   DEPTHS=2,4,6 bash scripts/train_with_shogiesa_quietset.sh data/csa weights_new.bin data/weights_v7.bin
#
# Exit code: forwarded from 'sekirei-match gate' (0=PASS, 1=FAIL, 2=INCONCLUSIVE)
set -e

CSA_DIR=${1:-./data/csa}
OUTPUT=${2:-data/weights_new.bin}
BASELINE=${3:-data/weights_v7.bin}
DEPTHS=${DEPTHS:-2,4}
GAMES=${GAMES:-400}
MIN_PLY=${MIN_PLY:-20}
MAX_PLY=${MAX_PLY:-160}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "=== shogiesa + quietset + sekirei pipeline ==="
echo "  CSA dir   : $CSA_DIR"
echo "  output    : $OUTPUT"
echo "  baseline  : $BASELINE"
echo "  depths    : $DEPTHS"
echo "  games     : $GAMES"
echo ""

mkdir -p data/stage1 data/stage2 data/stage3 data/checkpoints results

# ---- Stage 1: extract positions ----------------------------------------
echo "[1/5] shogiesa extract  (min-ply=$MIN_PLY max-ply=$MAX_PLY every-n-plies=4)"
shogiesa extract \
  --input "$CSA_DIR" \
  --out data/stage1/positions.jsonl \
  --min-ply "$MIN_PLY" \
  --max-ply "$MAX_PLY" \
  --every-n-plies 4 \
  --dedup
echo "  -> data/stage1/positions.jsonl ($(wc -l < data/stage1/positions.jsonl) positions)"

# ---- Stage 2: label with sekirei ----------------------------------------
echo "[2/5] shogiesa label  (engine=sekirei depths=$DEPTHS)"
cargo build --release -q -p sekirei
shogiesa label \
  --input data/stage1/positions.jsonl \
  --engine "./target/release/sekirei" \
  --depths "$DEPTHS" \
  --timeout-ms 10000 \
  --out data/stage2/observations.jsonl
echo "  -> data/stage2/observations.jsonl ($(wc -l < data/stage2/observations.jsonl) observations)"

# ---- Stage 3: score with quietset ----------------------------------------
echo "[3/5] quietset score  (profile=game-ai)"
quietset score data/stage2/observations.jsonl \
  --profile game-ai \
  > data/stage3/scored.jsonl
echo "  -> data/stage3/scored.jsonl ($(wc -l < data/stage3/scored.jsonl) scored positions)"

# ---- Train ---------------------------------------------------------------
echo "[4/5] sekirei-train  (stability-weighted validation-ratio=0.1)"
cargo run --release -q -p sekirei-train -- \
  --positions data/stage1/positions.jsonl \
  --scored data/stage3/scored.jsonl \
  --stability-weighted \
  --validation-ratio 0.1 \
  --seed 42 \
  --checkpoint-dir data/checkpoints \
  --output "$OUTPUT"
echo "  -> $OUTPUT"

# ---- Elo comparison -------------------------------------------------------
echo "[5/5] strength regression  ($GAMES games)"
OUT_JSON="results/${TIMESTAMP}.json"
cargo run --release -q -p sekirei-match-runner -- \
  --engine1 "./target/release/sekirei $OUTPUT" \
  --engine2 "./target/release/sekirei $BASELINE" \
  --games "$GAMES" \
  --byoyomi 1000 \
  --json "$OUT_JSON"

echo ""
cargo run --release -q -p sekirei-match-runner -- gate "$OUT_JSON" \
  --pass-elo 20 --pass-los 0.95 --fail-elo -10
