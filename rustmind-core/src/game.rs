#[cfg(test)]
#[path = "game.test.rs"]
mod tests;

use crate::{combination_check_result::CombinationCheckResult, messages::Messages};
use rand::{RngExt, seq::SliceRandom};

/// Describe a single game state
pub struct Game {
    /// Whether or not to prevent duplicates in combination
    allow_duplicates: bool,
    /// The user current try
    current_try: usize,
    /// The maximum allowed tries
    max_tries: usize,
    /// The number of slots to discover
    total_slots: usize,
    /// The number of available colors
    total_colors: usize,
    /// The correct colors combination
    combination_answer: Vec<usize>,
}

impl Game {
    /// Create a new game
    pub fn new_parameterized(
        allow_duplicates: bool,
        total_slots: usize,
        total_colors: usize,
        max_tries: usize,
        combination_answer: Vec<usize>,
    ) -> Self {
        let game = Self {
            allow_duplicates,
            current_try: 0,
            max_tries: max_tries,
            total_slots: total_slots,
            total_colors: total_colors,
            combination_answer: combination_answer.clone(),
        };

        // Check for configuration errors :

        // Ensure the answer length and slots number does match
        if combination_answer.len() != total_slots {
            panic!(
                "{}",
                Messages::combination_answer_slots_number_does_not_match_declared_one()
            );
        }

        // Ensure slots number is greater than 0
        if total_slots <= 0 {
            panic!("{}", Messages::slots_should_be_greater_than_zero());
        }

        // Ensure tries number is greater than 0
        if max_tries <= 0 {
            panic!("{}", Messages::tries_should_be_greater_than_zero());
        }

        // Ensure slots length greater than colors length forces use of duplicates
        if (total_slots > total_colors) && !allow_duplicates {
            panic!("{}", Messages::slots_exceed_colors_without_duplicates());
        }

        // Ensure validty of the given answer :
        let mut seen_colors: Vec<usize> = Vec::new();
        for i in 0..combination_answer.len() {
            let color = combination_answer[i];
            if seen_colors.contains(&color) {
                if !allow_duplicates {
                    panic!(
                        "{}",
                        Messages::combination_answer_contains_duplicates_while_prevented()
                    )
                }
            } else {
                seen_colors.push(color);
            }

            if color >= total_colors {
                panic!(
                    "{}",
                    Messages::combination_answer_contains_values_higher_than_permitted()
                )
            }
        }

        game
    }

    pub fn generate_random_colors_combination(
        allow_duplicates: bool,
        total_slots: usize,
        total_colors: usize,
    ) -> Result<Vec<usize>, String> {
        let mut rng = rand::rng();

        if !allow_duplicates {
            if total_slots > total_colors {
                return Err(
                    Messages::combination_generation_slots_exceed_colors_without_duplicates(),
                );
            }

            let mut colors: Vec<usize> = (0..total_colors).collect();
            colors.shuffle(&mut rng);
            colors.truncate(total_slots);
            Ok(colors)
        } else {
            if total_colors <= 0 {
                return Err(Messages::no_allowed_colors());
            }

            Ok((0..total_slots)
                .map(|_| rng.random_range(0..total_colors))
                .collect())
        }
    }

    pub fn allow_duplicates(&self) -> bool {
        self.allow_duplicates
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
        self.combination_answer.clone()
    }

    pub fn inc_current_try(&mut self) {
        self.current_try += 1;
    }

    /// Compute the correct/misplaced slots and check for victory
    pub fn check_combination(
        &self,
        combination: &Vec<usize>,
    ) -> Result<CombinationCheckResult, String> {
        if combination.len() != self.total_slots() {
            return Err(Messages::combination_answer_slots_number_does_not_match_declared_one());
        }

        let mut well_placed: usize = 0;
        let mut misplaced_slots: usize = 0;
        let mut exact_matches = vec![false; self.combination_answer.len()];
        let mut misplaced_matches = vec![false; combination.len()];
        if combination.len() > 0 {
            // Search for exact matches
            for i in 0..combination.len() {
                if i < self.combination_answer.len() && combination[i] == self.combination_answer[i]
                {
                    well_placed += 1;
                    exact_matches[i] = true;
                    misplaced_matches[i] = true;
                }
            }

            // Search for misplaced matches
            for i in 0..combination.len() {
                if misplaced_matches[i] {
                    continue;
                }

                for j in 0..self.combination_answer.len() {
                    if !exact_matches[j]
                        && i < self.combination_answer.len()
                        && combination[i] == self.combination_answer[j]
                    {
                        misplaced_slots += 1;
                        exact_matches[j] = true;
                        misplaced_matches[i] = true;
                        break;
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
        Ok(result)
    }
}
