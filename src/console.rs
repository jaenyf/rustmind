use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{app::App, game::Game};

use crate::combination_check_result::CombinationCheckResult;

pub struct Console<'a> {
    app: &'a App,
}

impl<'a> Console<'a> {
    pub fn new(app: &'a App) -> Self {
        let console = Self { app: app };

        console
    }

    pub fn print_game_config(&self, game: &Game) {
        println!("Colors: {:?}", game.total_colors());
        println!("Slots: {:?}", game.total_slots());
        println!("Tries: {:?}", game.max_tries());
        //println!("Duplicates:  {:?}", cli.duplicates);

        println!(
            "Combination is {}",
            game.colors_combination()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }

    pub fn read_combination(&mut self, game: &mut Game, input: &mut Vec<usize>) -> io::Result<()> {
        let mut buffer = String::new();

        println!(
            "Enter a combination (try #{}/{}):",
            game.current_try() + 1,
            game.max_tries()
        );

        game.inc_current_try();

        //slots read
        io::stdout().flush()?;
        enable_raw_mode()?;
        while !self.app.is_interrupted() && buffer.len() < game.total_slots() {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.app.interrupt();
                        continue;
                    }
                    KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                        buffer.push(c);
                        print!("{c}");
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace if !buffer.is_empty() => {
                        buffer.pop();
                        print!("\x08 \x08");
                        io::stdout().flush()?;
                    }
                    KeyCode::Esc => break,
                    _ => {} // Ignore invalid keys
                }
            }
        }
        disable_raw_mode()?;
        println!();
        //end slots read

        let color_numbers: Vec<usize> = buffer
            .chars()
            .map(|c| c.to_digit(16).unwrap() as usize)
            .collect();

        input.resize(color_numbers.len(), 0);
        input.copy_from_slice(&color_numbers);

        Ok(())
    }

    pub fn print_combination_result(&self, combination_result: &CombinationCheckResult) {
        println!(
            "Corrects:{}, misplaced:{}",
            combination_result.well_placed(),
            combination_result.misplaced()
        );
    }

    pub fn print_game_won(&self) {
        println!("You win!");
    }

    pub fn print_game_lost(&self, game: &Game) {
        println!(
            "Solution was {}",
            game.colors_combination()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("")
        );

        println!("You lose!");
    }
}
