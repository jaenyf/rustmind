use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Ammount of allowed colors
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=16))]
    pub(crate) colors: u8,

    /// Ammount of slots to find
    #[arg(short, long, default_value_t = 4)]
    pub(crate) slots: u8,

    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 12)]
    pub(crate) tries: u8,

    /// Prevent duplicates
    #[arg(short, long, default_value_t = true)]
    pub(crate) prevent_duplicates: bool,

    /// Display diagnostic information
    #[arg(short, long, default_value_t = false)]
    pub(crate) debug: bool,

    /// Display verbose information
    #[arg(short, long, default_value_t = false)]
    pub(crate) verbose: bool,

    /// Force the combination answer to the given value ([0-9a-fA-F]) -
    /// When this option is set, it overrides any other set options with values derived from the given combination
    #[arg(short('a'), long("answer"))]
    pub(crate) combination_answer: Option<String>,
}

impl Cli {
    pub fn initialize_and_parse() -> Self {
        let cli = Cli::parse();

        cli
    }
}
