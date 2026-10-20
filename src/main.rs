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

    let cli = Cli::initialize_and_parse();

    let app = App::new(cli.verbose(), cli.debug());

    // Check for configuration errors :
    if (cli.slots() > cli.colors()) && cli.prevent_duplicates() {
        Console::print_error(Messages::slots_exceed_colors_without_duplicates())?;
        std::process::exit(ExitCodes::general_error());
    }

    let mut game = Game::new_parameterized(
        cli.prevent_duplicates(),
        cli.slots() as usize,
        cli.colors() as usize,
        cli.tries() as usize,
        Game::generate_random_colors_combination(
            cli.prevent_duplicates(),
            cli.slots() as usize,
            cli.colors() as usize,
        ),
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
