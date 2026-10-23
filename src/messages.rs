pub struct Messages {}

impl Messages {
    pub fn combination_generation_slots_exceed_colors_without_duplicates() -> String {
        "Cannot generate a combination without duplicates when slots exceed colors".to_string()
    }

    pub fn slots_exceed_colors_without_duplicates() -> String {
        "Combination slots exceed colors while duplicated are prevented".to_string()
    }

    pub fn forced_combination_length_exceeded_() -> String {
        "Forced combination combination length exceeds the maximum length".to_string()
    }

    pub fn forced_combination_contains_non_hexadecimal_characters() -> String {
        "Forced combination answer contains non hexadecimal characters".to_string()
    }

    pub fn combination_answer_contains_duplicates_while_prevented() -> String {
        "Combination answer contains duplicates while prevented".to_string()
    }

    pub fn combination_answer_contains_values_higher_than_permitted() -> String {
        "Combination answer contains values higher than permitted".to_string()
    }
}
