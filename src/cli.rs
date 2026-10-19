use clap::{CommandFactory, Parser};
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..16))]
    pub(crate) colors: u8,

    /// Ammount of slots to find
    #[arg(short, long, default_value_t = 4)]
    pub(crate) slots: u8,

    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 12)]
    pub(crate) tries: u8,

    /// Ammount of allowed duplicate colors
    #[arg(short, long, default_value_t = 0)]
    pub(crate) duplicates: u8,
}

impl Cli {
    pub fn initialize_and_parse() -> Self {
        let cli = Cli::parse();

        if cli.duplicates > cli.slots {
            Cli::command()
                .error(
                    clap::error::ErrorKind::ValueValidation,
                    format!("Duplicates value must be between 0 and {}", cli.slots),
                )
                .exit();
        }

        cli
    }
}
