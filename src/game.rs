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
    answer_combination: Vec<usize>,
}

impl Game {
    /// Create a new game
    pub fn new_parameterized(
        total_slots: usize,
        total_colors: usize,
        max_tries: usize,
        colors_combination: Vec<usize>,
    ) -> Self {
        let game = Self {
            current_try: 0,
            max_tries: max_tries,
            total_slots: total_slots,
            total_colors: total_colors,
            answer_combination: colors_combination,
        };

        game
    }

    pub fn generate_random_colors_combination(
        total_slots: usize,
        total_colors: usize,
    ) -> Vec<usize> {
        let mut rng = rand::rng();

        (0..total_slots)
            .map(|_| rng.random_range(0..total_colors))
            .collect()
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
        self.answer_combination.clone()
    }

    pub fn inc_current_try(&mut self) {
        self.current_try += 1;
    }

    /// Compute the correct/misplaced slots and check for victory
    pub fn check_combination(&self, combination: &Vec<usize>) -> CombinationCheckResult {
        let mut well_placed: usize = 0;
        let mut misplaced_slots: usize = 0;
        let mut exact_matches = vec![false; self.answer_combination.len()];
        let mut misplaced_matches = vec![false; combination.len()];
        if combination.len() > 0 {
            // Search for exact matches
            for i in 0..combination.len() {
                if combination[i] == self.answer_combination[i] {
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

                for j in 0..self.answer_combination.len() {
                    if !exact_matches[j] && combination[i] == self.answer_combination[j] {
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
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1,2,3,4], vec![1,0,0,0], 1)]
    #[case([1,2,3,4], vec![0,2,0,0], 1)]
    #[case([1,2,3,4], vec![0,0,3,0], 1)]
    #[case([1,2,3,4], vec![0,0,0,4], 1)]
    #[case([1,2,3,4], vec![1,2,0,0], 2)]
    #[case([1,2,3,4], vec![0,2,3,0], 2)]
    #[case([1,2,3,4], vec![0,0,3,4], 2)]
    #[case([1,2,3,4], vec![1,0,0,4], 2)]
    #[case([1,2,3,4], vec![1,2,3,0], 3)]
    #[case([1,2,3,4], vec![0,2,3,4], 3)]
    #[case([1,2,3,4], vec![1,0,3,4], 3)]
    #[case([1,2,3,4], vec![1,2,3,4], 4)]
    fn test_check_combination_handles_well_placed(
        #[case] combination: [usize; 4],
        #[case] combination_try: Vec<usize>,
        #[case] expected: usize,
    ) {
        // arrange
        let game = Game::new_parameterized(4, 4, 1, combination.to_vec());

        // act
        let result = game.check_combination(&combination_try);

        // assert
        assert_eq!(result.well_placed(), expected);
    }

    #[rstest]
    #[case([1,1,2,2], vec![1,0,0,0], 1)]
    #[case([1,1,2,2], vec![0,1,0,0], 1)]
    #[case([1,1,2,2], vec![1,1,0,0], 2)]
    #[case([1,1,2,2], vec![0,0,2,0], 1)]
    #[case([1,1,2,2], vec![0,0,0,2], 1)]
    #[case([1,1,2,2], vec![0,0,2,2], 2)]
    #[case([1,1,1,2], vec![1,0,0,0], 1)]
    #[case([1,1,1,2], vec![0,1,0,0], 1)]
    #[case([1,1,1,2], vec![0,0,1,0], 1)]
    #[case([1,1,1,2], vec![1,1,1,0], 3)]
    #[case([1,2,2,2], vec![0,0,0,2], 1)]
    #[case([1,2,2,2], vec![0,0,2,2], 2)]
    #[case([1,2,2,2], vec![0,2,2,2], 3)]
    #[case([1,1,1,1], vec![1,1,1,1], 4)]
    fn test_check_combination_handles_well_placed_duplicates(
        #[case] combination: [usize; 4],
        #[case] combination_try: Vec<usize>,
        #[case] expected: usize,
    ) {
        // arrange
        let game = Game::new_parameterized(4, 4, 1, combination.to_vec());

        // act
        let result = game.check_combination(&combination_try);

        // assert
        assert_eq!(result.well_placed(), expected);
    }

    #[rstest]
    #[case([1,2,3,4], vec![4,0,0,0], 1)]
    #[case([1,2,3,4], vec![0,1,0,0], 1)]
    #[case([1,2,3,4], vec![0,0,2,0], 1)]
    #[case([1,2,3,4], vec![0,0,0,3], 1)]
    #[case([1,2,3,4], vec![4,1,0,0], 2)]
    #[case([1,2,3,4], vec![0,1,2,0], 2)]
    #[case([1,2,3,4], vec![0,0,2,3], 2)]
    #[case([1,2,3,4], vec![4,0,0,3], 2)]
    #[case([1,2,3,4], vec![4,1,2,0], 3)]
    #[case([1,2,3,4], vec![0,1,2,3], 3)]
    #[case([1,2,3,4], vec![4,0,2,3], 3)]
    #[case([1,2,3,4], vec![1,2,3,4], 0)]
    fn test_check_combination_handles_misplaced(
        #[case] combination: [usize; 4],
        #[case] combination_try: Vec<usize>,
        #[case] expected: usize,
    ) {
        // arrange
        let game = Game::new_parameterized(4, 4, 1, combination.to_vec());

        // act
        let result = game.check_combination(&combination_try);

        // assert
        assert_eq!(result.misplaced(), expected);
    }

    #[rstest]
    #[case([1,1,2,2], vec![2,0,0,0], 1)]
    #[case([1,1,2,2], vec![0,2,0,0], 1)]
    #[case([1,1,2,2], vec![2,2,0,0], 2)]
    #[case([1,1,2,2], vec![0,0,1,0], 1)]
    #[case([1,1,2,2], vec![0,0,0,1], 1)]
    #[case([1,1,2,2], vec![0,0,1,1], 2)]
    fn test_check_combination_handles_misplaced_duplicates(
        #[case] combination: [usize; 4],
        #[case] combination_try: Vec<usize>,
        #[case] expected: usize,
    ) {
        // arrange
        let game = Game::new_parameterized(4, 4, 1, combination.to_vec());

        // act
        let result = game.check_combination(&combination_try);

        // assert
        assert_eq!(result.misplaced(), expected);
    }

    #[rstest]
    #[case([1,2,3,4], vec![4,4,0,0], 1)]
    #[case([1,2,3,4], vec![0,1,1,0], 1)]
    #[case([1,2,3,4], vec![0,0,2,2], 1)]
    #[case([1,2,3,4], vec![3,0,0,3], 1)]
    #[case([1,2,3,4], vec![4,1,4,1], 2)]
    #[case([1,2,3,4], vec![2,1,2,1], 2)]
    #[case([1,2,3,4], vec![2,3,2,3], 2)]
    #[case([1,2,3,4], vec![4,3,4,3], 2)]
    #[case([1,2,3,4], vec![4,1,2,1], 3)]
    #[case([1,2,3,4], vec![3,1,2,3], 3)]
    #[case([1,2,3,4], vec![4,3,2,3], 3)]
    fn test_check_combination_handles_misplaced_proposed_duplicates(
        #[case] combination: [usize; 4],
        #[case] combination_try: Vec<usize>,
        #[case] expected: usize,
    ) {
        // arrange
        let game = Game::new_parameterized(4, 4, 1, combination.to_vec());

        // act
        let result = game.check_combination(&combination_try);

        // assert
        assert_eq!(result.misplaced(), expected);
    }
}
