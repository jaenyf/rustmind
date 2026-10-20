pub struct Messages {}

impl Messages {
    pub fn slots_exceed_colors_without_duplicates() -> String {
        "Cannot generate a combination without duplicates when slots exceed colors".to_string()
    }
}
