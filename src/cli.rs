use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Ammount of allowed colors
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..16))]
    colors: u8,

    /// Ammount of slots to find
    #[arg(short, long, default_value_t = 4)]
    slots: u8,

    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 12)]
    tries: u8,

    /// Prevent duplicates
    #[arg(short, long, default_value_t = true)]
    prevent_duplicates: bool,

    /// Display diagnostic information
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Display verbose information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

impl Cli {
    pub fn initialize_and_parse() -> Self {
        let cli = Cli::parse();

        cli
    }

    pub fn prevent_duplicates(&self) -> bool {
        self.prevent_duplicates
    }

    pub fn colors(&self) -> u8 {
        self.colors
    }

    pub fn slots(&self) -> u8 {
        self.slots
    }

    pub fn tries(&self) -> u8 {
        self.tries
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
