# Phase 2 AI Training: Resumption Plan

## 🛑 Current Status

**Paused before Training Execution.**

We have successfully upgraded the Machine Learning pipeline to "Phase 2" standards, aimed at achieving tactical parity with the Bitboard Solver. All code changes are implemented and compiled.

## ✅ Completed Changes

### 1. Training Pipeline (`train.rs`)

- **Symmetry Augmentation**: Now doubles the effective dataset size (25k raw &rarr; 50k total) by mirroring every board state.
- **Deep Teacher Labels**: Increased Solver evaluation depth to **18** (previously 12) for near-perfect training targets.
- **LR Scheduling**: Implemented Learning Rate decay (standard &rarr; 0.1x &rarr; 0.01x) over 100 epochs.

### 2. Perspective Alignment (`features.rs` & `ml_ai.rs`)

- **Fixed Perspective Bug**: Features are now **Perspective-Invariant** (Current Player is always `1.0`, Opponent `-1.0`).
- **Logic Simplification**: Removed legacy code that manually negated values for Player 2, enabling consistent inference.

### 3. Frontend Integration (`wasm-ai-service.ts`)

- Updated the frontend to look for the new `ml_ai_weights_best.json` model file that will be generated.

---

## 🚀 Next Steps (How to Continue)

### 1. Run the Training

This will generate the 50k dataset and train the model. This process is computationally intensive and may take **15-30 minutes**.

```bash
cd worker/rust_ai_core
caffeinate -i cargo run --release --bin train
```

### 2. Verify Performance

After training completes, run the AI Matrix benchmark in **Release Mode**.

```bash
# In worker/rust_ai_core
# Set games to 20 for a solid statistical sample
NUM_GAMES=20 cargo test test_ai_matrix --release -- --nocapture
```

**Success Criteria:**

- **vs Random**: > 95% Win Rate
- **vs Bitboard Solver**: > 40% Win/Draw Rate (Targeting 50%+)

### 3. Commit Results

Once verified, commit the new weights and the updated documentation.

```bash
git add public/ml/data/weights/ml_ai_weights_best.json
git commit -m "Train Phase 2 ML AI: Symmetry augmented, Depth 18 teacher"
git push
```
