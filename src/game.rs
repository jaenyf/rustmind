use rand::RngExt;

use crate::combination_check_result::CombinationCheckResult;

/// Describe a single game state
pub struct Game {
    /// The user current try
    current_try: usize,
    /// The maximum allowed tries
    max_tries: usize,
    /// The number of slots to discover
    total_slots: usize,
    /// The number of available colors
    total_colors: usize,
    /// The correct colors combination
    colors_combination: Vec<usize>,
}

impl Game {
    /// Create a new game
    pub fn new_parameterized(total_slots: usize, total_colors: usize, max_tries: usize) -> Self {
        let mut game = Self {
            current_try: 0,
            max_tries: max_tries,
            total_slots: total_slots,
            total_colors: total_colors,
            colors_combination: vec![0; total_slots],
        };

        let mut rng = rand::rng();
        for i in 0..total_slots {
            game.colors_combination[i] = rng.random_range(0..total_colors);
        }

        game
    }

    pub fn current_try(&self) -> usize {
        self.current_try
    }

    pub fn max_tries(&self) -> usize {
        self.max_tries
    }

    pub fn total_slots(&self) -> usize {
        self.total_slots
    }

    pub fn total_colors(&self) -> usize {
        self.total_colors
    }

    pub fn colors_combination(&self) -> Vec<usize> {
        self.colors_combination.clone()
    }

    /// Compute the correct/misplaced slots and check for victory
    pub fn check_combination(&self, combination: &Vec<usize>) -> CombinationCheckResult {
        let mut well_placed: usize = 0;
        let mut misplaced_slots: usize = 0;
        if combination.len() > 0 {
            for slot_index in 0..combination.len() {
                if self.colors_combination[slot_index] == combination[slot_index] {
                    well_placed += 1;
                    continue;
                } else {
                    for other_slot_index in 0..combination.len() {
                        if other_slot_index == slot_index {
                            continue;
                        } else {
                            if self.colors_combination[slot_index] == combination[other_slot_index]
                            {
                                misplaced_slots += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        let is_victory = well_placed == self.total_slots;
        let result: CombinationCheckResult = CombinationCheckResult::new_parameterized(
            is_victory,
            !is_victory && self.current_try == self.max_tries,
            well_placed,
            misplaced_slots,
        );
        result
    }
}
