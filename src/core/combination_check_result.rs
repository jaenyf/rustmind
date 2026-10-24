pub struct CombinationCheckResult {
    /// Amount of well-placed slots
    well_placed: usize,
    /// Amount of misplaced slots
    misplaced: usize,
    /// Whether or not the game is won
    is_game_won: bool,
    /// Whether or not the game is lost
    is_game_lost: bool,
}

impl CombinationCheckResult {
    pub fn new_parameterized(
        is_game_won: bool,
        is_game_lost: bool,
        well_placed: usize,
        misplaced: usize,
    ) -> Self {
        Self {
            well_placed: well_placed,
            misplaced: misplaced,
            is_game_won: is_game_won,
            is_game_lost: is_game_lost,
        }
    }

    pub fn well_placed(&self) -> usize {
        self.well_placed
    }

    pub fn misplaced(&self) -> usize {
        self.misplaced
    }

    pub fn is_game_won(&self) -> bool {
        self.is_game_won
    }

    pub fn is_game_lost(&self) -> bool {
        self.is_game_lost
    }
}
