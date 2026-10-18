use std::io::Write;
use std::sync::atomic::Ordering;
use std::vec;
use std::{io, sync::atomic::AtomicBool};

use clap::{CommandFactory, Parser};
use crossterm::event::KeyModifiers;
use rand::RngExt;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..16))]
    colors: u8,

    /// Ammount of slots to find
    #[arg(short, long, default_value_t = 4)]
    slots: u8,

    /// Ammount of allowed tries
    #[arg(short, long, default_value_t = 12)]
    tries: u8,

    /// Ammount of allowed duplicate colors
    #[arg(short, long, default_value_t = 0)]
    duplicates: u8,
}

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

fn main() -> io::Result<()> {
    ctrlc::set_handler(|| {
        INTERRUPTED.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    println!("Welcome to rustmind!");

    let cli = Cli::parse();

    if cli.duplicates > cli.slots {
        Cli::command()
            .error(
                clap::error::ErrorKind::ValueValidation,
                format!("Duplicates value must be between 0 and {}", cli.slots),
            )
            .exit();
    }

    println!("Colors: {:?}", cli.colors);
    println!("Slots: {:?}", cli.slots);
    println!("Tries: {:?}", cli.tries);
    println!("Duplicates:  {:?}", cli.duplicates);

    let mut rng = rand::rng();
    let mut colors_vector: Vec<u8> = vec![0; cli.slots as usize];
    for i in 0..cli.slots {
        let color = rng.random_range(0..cli.colors).into();
        colors_vector[i as usize] = color;
    }

    for i in 0..cli.slots {
        println!("Pin {:?} is {:?}", i + 1, colors_vector[i as usize]);
    }

    let mut is_victory: bool = false;
    for _i in 0..cli.tries {
        if INTERRUPTED.load(Ordering::SeqCst) {
            break;
        }
        let mut input = String::new();
        println!("Combination:");

        //slots read
        io::stdout().flush()?;
        enable_raw_mode()?;
        while !INTERRUPTED.load(Ordering::SeqCst) && input.len() < cli.slots as usize {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        INTERRUPTED.store(true, Ordering::SeqCst);
                        continue;
                    }
                    KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                        input.push(c);
                        print!("{c}");
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace if !input.is_empty() => {
                        input.pop();
                        print!("\x08 \x08");
                        io::stdout().flush()?;
                    }
                    KeyCode::Esc => break,
                    _ => {} // Ignore invalid keys
                }
            }
        }
        disable_raw_mode()?;
        //end slots read

        //map the read chars to color numbers:
        let color_numbers: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect();

        //compute the correct/misplaced slots
        let mut correct_slots: u8 = 0;
        let mut misplaced_slots: u8 = 0;
        for slot_index in 0..cli.slots {
            if colors_vector[slot_index as usize] == color_numbers[slot_index as usize] {
                correct_slots += 1;
                continue;
            } else {
                for other_slot_index in 0..cli.slots {
                    if other_slot_index == slot_index {
                        continue;
                    } else {
                        if colors_vector[slot_index as usize]
                            == color_numbers[other_slot_index as usize]
                        {
                            misplaced_slots += 1;
                            continue;
                        }
                    }
                }
            }
        }

        println!();
        println!("Corrects:{}, misplaced:{}", correct_slots, misplaced_slots);

        if correct_slots == cli.slots {
            is_victory = true;
            break;
        }
    }

    if is_victory {
        println!("You win!");
        Ok(())
    } else {
        println!("You lose!");
        std::process::exit(1);
    }
}
