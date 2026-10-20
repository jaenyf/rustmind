pub struct ExitCodes {}

impl ExitCodes {
    pub fn game_won() -> i32 {
        0
    }
    pub fn game_lost() -> i32 {
        1
    }
    pub fn general_error() -> i32 {
        2
    }
}
