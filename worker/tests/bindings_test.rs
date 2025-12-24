#[cfg(test)]
mod tests {
    use connect_four_ai_core::wasm_api::{WasmBestMoveResponse, WasmHeuristicResponse, WasmMLResponse, MoveEvaluationWasm};
    use connect_four_ai_core::{GameState, Player, Cell};
    use connect_four_ai_core::genetic_params::GeneticParams;
    use connect_four_ai_core::ml_ai::{MLDiagnostics, MLMoveEvaluation};
    use ts_rs::TS;

    #[test]
    fn test_bindings() {
        assert!(GeneticParams::export().is_ok());
        assert!(GameState::export().is_ok());
        assert!(Player::export().is_ok());
        assert!(Cell::export().is_ok());
        assert!(WasmBestMoveResponse::export().is_ok());
        assert!(WasmHeuristicResponse::export().is_ok());
        assert!(WasmMLResponse::export().is_ok());
        assert!(MoveEvaluationWasm::export().is_ok());
        assert!(MLDiagnostics::export().is_ok());
        assert!(MLMoveEvaluation::export().is_ok());
    }
}
