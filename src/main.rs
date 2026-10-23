mod app;
mod cli;
mod combination_check_result;
mod console;
mod exit_codes;
mod game;
mod messages;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::app::App;
use crate::cli::Cli;
use crate::console::Console;
use crate::exit_codes::ExitCodes;
use crate::game::Game;
use crate::messages::Messages;

fn main() -> io::Result<()> {
    static INTERRUPTED: AtomicBool = AtomicBool::new(false);

    ctrlc::set_handler(|| {
        INTERRUPTED.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    Console::print_title()?;

    let mut cli = Cli::initialize_and_parse();

    let app = App::new(cli.verbose, cli.debug);

    // Check for configuration errors :
    if (cli.slots > cli.colors) && !cli.allow_duplicates {
        Console::print_error(
            Messages::combination_generation_slots_exceed_colors_without_duplicates(),
        )?;
        std::process::exit(ExitCodes::general_error());
    }

    let combination_answer: Vec<usize>;
    let forced_combination_answer = cli.combination_answer;
    if forced_combination_answer.is_none() {
        combination_answer = Game::generate_random_colors_combination(
            cli.allow_duplicates,
            cli.slots as usize,
            cli.colors as usize,
        );
    } else {
        let combination_answer_chars: Vec<char> =
            forced_combination_answer.unwrap().chars().collect();

        if combination_answer_chars.len() > 255 {
            Console::print_error(Messages::forced_combination_length_exceeded_())?;
            std::process::exit(ExitCodes::general_error());
        }
        cli.slots = combination_answer_chars.len() as u8;

        if cli.slots <= 0 {
            Console::print_error(Messages::slots_should_be_greater_than_zero())?;
            std::process::exit(ExitCodes::general_error());
        }

        let mut max_digit_value: u32 = 0;
        let mut seen_chars: Vec<char> = Vec::new();
        for i in 0..combination_answer_chars.len() {
            let char = combination_answer_chars[i].to_ascii_uppercase();
            if seen_chars.contains(&char) {
                cli.allow_duplicates = true;
            } else {
                seen_chars.push(char);
            }
            let digit = char.to_digit(16);
            if digit.is_none() {
                Console::print_error(
                    Messages::forced_combination_contains_non_hexadecimal_characters(),
                )?;
                std::process::exit(ExitCodes::general_error());
            }
            let unwrapped_digit_value = digit.unwrap();
            if max_digit_value < unwrapped_digit_value {
                max_digit_value = unwrapped_digit_value
            }
        }
        cli.colors = (max_digit_value + 1) as u8;

        combination_answer = combination_answer_chars
            .into_iter()
            .map(|c| c.to_digit(16).unwrap() as usize)
            .collect()
    }

    let mut game = Game::new_parameterized(
        cli.allow_duplicates,
        cli.slots as usize,
        cli.colors as usize,
        cli.tries as usize,
        combination_answer,
    );

    if app.is_verbose_flag() {
        Console::print_game_config(&game)?;
    }

    if app.is_debug_flag() {
        Console::print_answer(&game)?;
    }

    Console::print_empty_block()?;

    let mut is_game_won: bool = false;
    let mut is_game_lost: bool = false;
    let mut input: Vec<usize> = Vec::new();
    input.resize(game.total_slots() as usize, 0);
    for _i in 0..game.max_tries() {
        if INTERRUPTED.load(Ordering::SeqCst) {
            app.interrupt();
        }

        if app.is_interrupted() {
            break;
        }

        Console::read_combination(&app, &mut game, &mut input)?;
        let combination_result = game.check_combination(&input);

        if app.is_interrupted() {
            break;
        }

        Console::print_combination_result(&combination_result)?;

        if combination_result.is_game_won() {
            is_game_won = true;
            break;
        }
        if combination_result.is_game_lost() {
            is_game_lost = true;
        }
        input.clear();
        input.resize(game.total_slots() as usize, 0);
    }

    if app.is_interrupted() {
        Console::print_game_quit()?;
    }

    if !app.is_interrupted() {
        if is_game_won {
            Console::print_game_won()?;
            std::process::exit(ExitCodes::game_won());
        } else if is_game_lost {
            Console::print_game_lost(&game)?;
        }
    }
    std::process::exit(ExitCodes::game_lost());
}
