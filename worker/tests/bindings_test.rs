#[cfg(test)]
mod tests {
    use connect_four_ai_core::wasm_api::{WasmBestMoveResponse, WasmHeuristicResponse, WasmMLResponse, MoveEvaluationWasm};
    use connect_four_ai_core::{GameState, Player, Cell};
    use connect_four_ai_core::genetic_params::GeneticParams;
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
    }
}
