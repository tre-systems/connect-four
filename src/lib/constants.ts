export const DEFAULT_GENETIC_PARAMS = {
  id: 'default-fallback',
  parent_ids: [],
  generation: 0,
  win_score: 10000,
  loss_score: -10000,
  center_column_value: 165,
  adjacent_center_value: 97,
  outer_column_value: 17,
  edge_column_value: 6,
  row_height_weight: 1.798,
  center_control_weight: 2.022,
  piece_count_weight: 0.965,
  threat_weight: 1.588,
  mobility_weight: 1.453,
  vertical_control_weight: 2.862,
  horizontal_control_weight: 1.344,
  defensive_weight: 1.372,
  horizontal_connection_weight: 1.344,
};

// Search depth for the Classic AI (bitboard solver) in human-vs-AI play. Shared
// by the AI engine (ai-logic) and the selection-card label so they can't drift.
export const CLASSIC_AI_DEPTH = 5;
