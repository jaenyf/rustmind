mod app;
mod cli;
mod combination_check_result;
mod console;
mod game;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::app::App;
use crate::cli::Cli;
use crate::console::Console;
use crate::game::Game;

fn main() -> io::Result<()> {
    let app = App::new();

    static INTERRUPTED: AtomicBool = AtomicBool::new(false);

    ctrlc::set_handler(|| {
        INTERRUPTED.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    println!("Welcome to rustmind!");

    let cli = Cli::initialize_and_parse();

    println!("Colors: {:?}", cli.colors);
    println!("Slots: {:?}", cli.slots);
    println!("Tries: {:?}", cli.tries);
    println!("Duplicates:  {:?}", cli.duplicates);

    let game = Game::new_parameterized(cli.slots as usize, cli.colors as usize, cli.tries as usize);

    // for i in 0..cli.slots {
    //     println!("Pin {:?} is {:?}", i + 1, colors_vector[i as usize]);
    // }

    let mut console = Console::new(&app, cli.slots as usize);

    let mut is_victory: bool = false;
    let mut input: Vec<usize> = Vec::new();
    input.resize(cli.slots as usize, 0);
    for _i in 0..cli.tries {
        if INTERRUPTED.load(Ordering::SeqCst) {
            app.interrupt();
        }

        if app.is_interrupted() {
            break;
        }

        console.read_combination(&mut input)?;
        let combination_result = game.check_combination(&input);
        console.print_combination_result(&combination_result);

        if combination_result.is_game_won() {
            is_victory = true;
            break;
        }
        input.clear();
        input.resize(cli.slots as usize, 0);
    }

    if !app.is_interrupted() {
        if is_victory {
            console.print_game_won();
            std::process::exit(0);
        } else {
            console.print_game_lost();
        }
    }
    std::process::exit(1);
}
