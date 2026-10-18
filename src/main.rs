use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Ammount of colors
    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(u8).range(1..=16))]
    colors: u8,

    /// Ammount of allowed duplicate colors
    #[arg(short, long, default_value_t = 0)]
    duplicates: u8,
}

fn main() {
    println!("Welcome to rustmind!");

    let cli = Cli::parse();

    if cli.duplicates > cli.colors {
        Cli::command()
            .error(
                clap::error::ErrorKind::ValueValidation,
                format!("Duplicates value must be between 0 and {}", cli.colors),
            )
            .exit();
    }

    println!("Colors: {:?}", cli.colors);
    println!("Duplicates:  {:?}", cli.duplicates);
}
