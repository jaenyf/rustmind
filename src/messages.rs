pub struct Messages {}

impl Messages {
    pub fn slots_exceed_colors_without_duplicates() -> String {
        "Cannot generate a combination without duplicates when slots exceed colors".to_string()
    }

    pub fn forced_combination_length_exceeded_() -> String {
        "Forced combination combination length exceeds the maximum length".to_string()
    }

    pub fn forced_combination_contains_non_hexadecimal_characters() -> String {
        "Forced combination answer contains non hexadecimal characters".to_string()
    }
}
