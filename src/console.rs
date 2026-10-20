use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
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

    pub fn print_title(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            Print("\nWelcome to "),
            SetAttribute(Attribute::Bold),
            Print("Rustmind"),
            SetAttribute(Attribute::NormalIntensity),
            Print("!\n\n")
        )?;
        stdout.flush()?;

        Ok(())
    }

    pub fn print_error(&self, error: String) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print("\nError: "),
            ResetColor,
            SetAttribute(Attribute::NormalIntensity),
            Print(error),
            Print("!\n\n")
        )?;
        stdout.flush()?;

        Ok(())
    }

    pub fn print_game_config(&self, game: &Game) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetAttribute(Attribute::Underlined),
            Print("Configuration:\n"),
            SetAttribute(Attribute::NormalIntensity),
            SetAttribute(Attribute::NoUnderline),
        )?;

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Colors"),
            SetAttribute(Attribute::NormalIntensity),
            Print("      : "),
            Print(game.total_colors()),
            Print("\n")
        )?;

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Slots"),
            SetAttribute(Attribute::NormalIntensity),
            Print("       : "),
            Print(game.total_slots()),
            Print("\n")
        )?;

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Tries"),
            SetAttribute(Attribute::NormalIntensity),
            Print("       : "),
            Print(game.max_tries()),
            Print("\n")
        )?;

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Duplicates"),
            SetAttribute(Attribute::NormalIntensity),
            Print("  : "),
            Print(!game.prevent_duplicates()),
            Print("\n")
        )?;

        stdout.flush()?;

        Ok(())
    }

    pub fn print_answer(&self, game: &Game) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Combination"),
            SetAttribute(Attribute::NormalIntensity),
            Print(" : "),
        )?;

        for f in game.colors_combination().iter() {
            let c = char::from_digit(*f as u32, 16).unwrap();

            queue!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Self::color_for(c)),
                Print(c),
                SetAttribute(Attribute::NormalIntensity),
                ResetColor,
            )?;
        }

        queue!(stdout, Print("\n"),)?;

        stdout.flush()?;

        Ok(())
    }

    pub fn print_empty_block(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        queue!(stdout, Print("\r\n"),)?;

        stdout.flush()?;

        Ok(())
    }

    fn color_for(c: char) -> Color {
        Color::AnsiValue(c.to_digit(16).unwrap() as u8)
    }

    pub fn read_combination(&mut self, game: &mut Game, input: &mut Vec<usize>) -> io::Result<()> {
        let mut buffer = String::new();

        print!(
            "[#{}/{}] Combination : ",
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
                        queue!(
                            io::stdout(),
                            SetForegroundColor(Self::color_for(c)),
                            SetAttribute(Attribute::Bold),
                            Print(c),
                            ResetColor,
                            SetAttribute(Attribute::NormalIntensity),
                        )?;
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace if !buffer.is_empty() => {
                        buffer.pop();
                        print!("\x08 \x08");
                        io::stdout().flush()?;
                    }
                    _ => {} // Ignore invalid keys
                }
            }
        }
        queue!(io::stdout(), Print("\r\n"))?;
        disable_raw_mode()?;
        //end slots read

        let color_numbers: Vec<usize> = buffer
            .chars()
            .map(|c| c.to_digit(16).unwrap() as usize)
            .collect();

        input.resize(color_numbers.len(), 0);
        input.copy_from_slice(&color_numbers);

        Ok(())
    }

    pub fn print_combination_result(
        &self,
        combination_result: &CombinationCheckResult,
    ) -> io::Result<()> {
        // println!(
        //     "  \u{2514}\u{2500}[Correct:{}, Misplaced:{}]",
        //     combination_result.well_placed(),
        //     combination_result.misplaced()
        // );

        let mut stdout = io::stdout();
        queue!(
            stdout,
            Print("  "),
            SetAttribute(Attribute::Bold),
            Print("\u{2514}\u{2500}["),
            SetAttribute(Attribute::NormalIntensity),
            Print("Correct:"),
            Print(combination_result.well_placed()),
            Print(", Misplaced:"),
            Print(combination_result.misplaced()),
            SetAttribute(Attribute::Bold),
            Print("]\n"),
            ResetColor
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn print_game_won(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green),
            Print("You win!\n"),
            SetAttribute(Attribute::NormalIntensity),
            ResetColor,
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn print_game_lost(&self, game: &Game) -> io::Result<()> {
        let mut stdout = io::stdout();
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Combination was"),
            SetAttribute(Attribute::NormalIntensity),
            Print(" : "),
        )?;

        for f in game.colors_combination().iter() {
            let c = char::from_digit(*f as u32, 16).unwrap();

            queue!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Self::color_for(c)),
                Print(c),
                SetAttribute(Attribute::NormalIntensity),
                ResetColor,
            )?;
        }

        queue!(stdout, Print("\n"))?;

        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print("You lose!\n"),
            SetAttribute(Attribute::NormalIntensity),
            ResetColor,
        )?;

        stdout.flush()?;

        Ok(())
    }
}
