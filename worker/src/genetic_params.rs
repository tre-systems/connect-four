use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub struct GeneticParams {
    // Unique identifier for tracking individuals across generations
    pub id: String,

    // Ancestor tracking for lineage analysis
    pub parent_ids: Vec<String>,
    pub generation: usize,

    // Win/loss scores
    pub win_score: i32,
    pub loss_score: i32,

    // Position evaluation weights
    pub center_column_value: i32,
    pub adjacent_center_value: i32,
    pub outer_column_value: i32,
    pub edge_column_value: i32,
    pub row_height_weight: f64,

    // Feature weights
    pub center_control_weight: f64,
    pub piece_count_weight: f64,
    pub threat_weight: f64,
    pub mobility_weight: f64,
    pub vertical_control_weight: f64,
    pub horizontal_control_weight: f64,
    pub defensive_weight: f64,
}

impl Default for GeneticParams {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_ids: vec![],
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
        }
    }
}

impl GeneticParams {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            id: Uuid::new_v4().to_string(),
            parent_ids: vec![],
            generation: 0,
            win_score: rng.gen_range(5000..15000),
            loss_score: rng.gen_range(-15000..-5000),
            center_column_value: rng.gen_range(50..200),
            adjacent_center_value: rng.gen_range(25..100),
            outer_column_value: rng.gen_range(5..25),
            edge_column_value: rng.gen_range(1..10),
            row_height_weight: rng.gen_range(0.5..2.0),
            center_control_weight: rng.gen_range(0.0..3.0),
            piece_count_weight: rng.gen_range(0.0..2.0),
            threat_weight: rng.gen_range(0.5..5.0),
            mobility_weight: rng.gen_range(0.0..2.0),
            vertical_control_weight: rng.gen_range(0.5..3.0),
            horizontal_control_weight: rng.gen_range(0.5..3.0),
            defensive_weight: rng.gen_range(0.5..3.0),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let params: GeneticParams = serde_json::from_str(&content)?;
        Ok(params)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn random_mutation(&self, mutation_rate: f64, mutation_strength: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            id: Uuid::new_v4().to_string(),
            parent_ids: vec![self.id.clone()],
            generation: self.generation + 1,
            win_score: if rng.gen_bool(mutation_rate) {
                (self.win_score as f64 + rng.gen_range(-500.0..500.0) * mutation_strength) as i32
            } else {
                self.win_score
            },
            loss_score: if rng.gen_bool(mutation_rate) {
                (self.loss_score as f64 + rng.gen_range(-500.0..500.0) * mutation_strength) as i32
            } else {
                self.loss_score
            },
            center_column_value: if rng.gen_bool(mutation_rate) {
                (self.center_column_value as f64 + rng.gen_range(-20.0..20.0) * mutation_strength)
                    as i32
            } else {
                self.center_column_value
            },
            adjacent_center_value: if rng.gen_bool(mutation_rate) {
                (self.adjacent_center_value as f64 + rng.gen_range(-10.0..10.0) * mutation_strength)
                    as i32
            } else {
                self.adjacent_center_value
            },
            outer_column_value: if rng.gen_bool(mutation_rate) {
                (self.outer_column_value as f64 + rng.gen_range(-5.0..5.0) * mutation_strength)
                    as i32
            } else {
                self.outer_column_value
            },
            edge_column_value: if rng.gen_bool(mutation_rate) {
                (self.edge_column_value as f64 + rng.gen_range(-2.0..2.0) * mutation_strength)
                    as i32
            } else {
                self.edge_column_value
            },
            row_height_weight: if rng.gen_bool(mutation_rate) {
                self.row_height_weight + rng.gen_range(-0.2..0.2) * mutation_strength
            } else {
                self.row_height_weight
            },
            center_control_weight: if rng.gen_bool(mutation_rate) {
                self.center_control_weight + rng.gen_range(-1.0..1.0) * mutation_strength
            } else {
                self.center_control_weight
            },
            piece_count_weight: if rng.gen_bool(mutation_rate) {
                self.piece_count_weight + rng.gen_range(-0.5..0.5) * mutation_strength
            } else {
                self.piece_count_weight
            },
            threat_weight: if rng.gen_bool(mutation_rate) {
                self.threat_weight + rng.gen_range(-1.0..1.0) * mutation_strength
            } else {
                self.threat_weight
            },
            mobility_weight: if rng.gen_bool(mutation_rate) {
                self.mobility_weight + rng.gen_range(-0.5..0.5) * mutation_strength
            } else {
                self.mobility_weight
            },
            vertical_control_weight: if rng.gen_bool(mutation_rate) {
                self.vertical_control_weight + rng.gen_range(-0.5..0.5) * mutation_strength
            } else {
                self.vertical_control_weight
            },
            horizontal_control_weight: if rng.gen_bool(mutation_rate) {
                self.horizontal_control_weight + rng.gen_range(-0.5..0.5) * mutation_strength
            } else {
                self.horizontal_control_weight
            },
            defensive_weight: if rng.gen_bool(mutation_rate) {
                self.defensive_weight + rng.gen_range(-0.5..0.5) * mutation_strength
            } else {
                self.defensive_weight
            },
        }
    }

    pub fn crossover(&self, other: &Self, crossover_rate: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            id: Uuid::new_v4().to_string(),
            parent_ids: vec![self.id.clone(), other.id.clone()],
            generation: std::cmp::max(self.generation, other.generation) + 1,
            win_score: if rng.gen_bool(crossover_rate) {
                other.win_score
            } else {
                self.win_score
            },
            loss_score: if rng.gen_bool(crossover_rate) {
                other.loss_score
            } else {
                self.loss_score
            },
            center_column_value: if rng.gen_bool(crossover_rate) {
                other.center_column_value
            } else {
                self.center_column_value
            },
            adjacent_center_value: if rng.gen_bool(crossover_rate) {
                other.adjacent_center_value
            } else {
                self.adjacent_center_value
            },
            outer_column_value: if rng.gen_bool(crossover_rate) {
                other.outer_column_value
            } else {
                self.outer_column_value
            },
            edge_column_value: if rng.gen_bool(crossover_rate) {
                other.edge_column_value
            } else {
                self.edge_column_value
            },
            row_height_weight: if rng.gen_bool(crossover_rate) {
                other.row_height_weight
            } else {
                self.row_height_weight
            },
            center_control_weight: if rng.gen_bool(crossover_rate) {
                other.center_control_weight
            } else {
                self.center_control_weight
            },
            piece_count_weight: if rng.gen_bool(crossover_rate) {
                other.piece_count_weight
            } else {
                self.piece_count_weight
            },
            threat_weight: if rng.gen_bool(crossover_rate) {
                other.threat_weight
            } else {
                self.threat_weight
            },
            mobility_weight: if rng.gen_bool(crossover_rate) {
                other.mobility_weight
            } else {
                self.mobility_weight
            },
            vertical_control_weight: if rng.gen_bool(crossover_rate) {
                other.vertical_control_weight
            } else {
                self.vertical_control_weight
            },
            horizontal_control_weight: if rng.gen_bool(crossover_rate) {
                other.horizontal_control_weight
            } else {
                self.horizontal_control_weight
            },
            defensive_weight: if rng.gen_bool(crossover_rate) {
                other.defensive_weight
            } else {
                self.defensive_weight
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = GeneticParams::default();
        assert_eq!(params.win_score, 10000);
        assert_eq!(params.loss_score, -10000);
        assert_eq!(params.center_column_value, 165);
        assert_eq!(params.center_control_weight, 2.022);
        assert_eq!(params.threat_weight, 1.588);
    }

    #[test]
    fn test_random_params() {
        let params = GeneticParams::random();
        assert!(params.win_score >= 5000 && params.win_score < 15000);
        assert!(params.loss_score >= -15000 && params.loss_score < -5000);
        assert!(params.center_column_value >= 50 && params.center_column_value < 200);
        assert!(params.center_control_weight >= 0.0 && params.center_control_weight < 3.0);
        assert!(params.threat_weight >= 0.5 && params.threat_weight < 5.0);

        // Test that ID is a valid UUID format (36 characters with hyphens)
        assert_eq!(params.id.len(), 36);
        assert!(params.id.contains('-'));
        assert!(params.id.matches('-').count() == 4);
    }

    #[test]
    fn test_uuid_ids_are_short() {
        let params1 = GeneticParams::random();
        let params2 = GeneticParams::random();

        // UUIDs should be 36 characters (much shorter than the old format)
        assert_eq!(params1.id.len(), 36);
        assert_eq!(params2.id.len(), 36);

        // IDs should be unique
        assert_ne!(params1.id, params2.id);

        // Test mutation creates new UUID
        let mutated = params1.random_mutation(0.5, 0.3);
        assert_ne!(params1.id, mutated.id);
        assert_eq!(mutated.id.len(), 36);

        // Test crossover creates new UUID
        let crossed = params1.crossover(&params2, 0.5);
        assert_ne!(params1.id, crossed.id);
        assert_ne!(params2.id, crossed.id);
        assert_eq!(crossed.id.len(), 36);
    }

    #[test]
    fn test_save_and_load() {
        let params = GeneticParams::default();
        let temp_path = "test_params.json";

        params.save_to_file(temp_path).unwrap();
        let loaded_params = GeneticParams::load_from_file(temp_path).unwrap();

        assert_eq!(params, loaded_params);

        // Clean up
        std::fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_mutation() {
        let original = GeneticParams::default();
        let mutated = original.random_mutation(1.0, 0.1);

        // Mutation should change values
        assert_ne!(original.win_score, mutated.win_score);
        assert_ne!(
            original.center_control_weight,
            mutated.center_control_weight
        );
    }

    #[test]
    fn test_crossover() {
        let parent1 = GeneticParams::default();
        let parent2 = GeneticParams {
            id: "test_parent2".to_string(),
            parent_ids: vec![],
            generation: 0,
            win_score: 15000,
            loss_score: -15000,
            center_column_value: 200,
            adjacent_center_value: 100,
            outer_column_value: 20,
            edge_column_value: 5,
            row_height_weight: 2.0,
            center_control_weight: 5.0,
            piece_count_weight: 3.0,
            threat_weight: 7.0,
            mobility_weight: 4.0,
            vertical_control_weight: 6.0,
            horizontal_control_weight: 8.0,
            defensive_weight: 9.0,
        };

        let child = parent1.crossover(&parent2, 1.0);

        // With 100% crossover rate, child should inherit from parent2
        assert_eq!(child.win_score, parent2.win_score);
        assert_eq!(child.center_control_weight, parent2.center_control_weight);
    }
}
