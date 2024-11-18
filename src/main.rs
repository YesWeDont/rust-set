mod card;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};
mod util;
use crossterm::execute;
use rand::seq::SliceRandom;
use util::*;
mod tutorial;
use tutorial::*;
mod game;
use game::Game;

static S: crossterm::style::Print<&str> = crossterm::style::Print(" ");
fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        exit(|| println!("An error occured: {panic_info}"), 1)
    }));
    main_fn().unwrap();
}

// ensure question mark syntax panics so panic handler handles it
fn main_fn() -> Result<(), Box<dyn std::error::Error>> {
    use crossterm::{
        cursor::*, event::*, queue, style::Print as P, style::PrintStyledContent as PS,
        style::Stylize, terminal::*,
    };
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;

    queue!(
        stdout,
        EnableMouseCapture,
        EnterAlternateScreen,
        Clear(ClearType::Purge),
        Hide,
        SavePosition,
        MoveTo(0, 0),
        P("rust-set v0.0.0 - Rust implementation of Set"),
        MoveToNextLine(1)
    )?;
    loop {
        queue!(
            stdout,
            P("Press h for tutorial, s to start or Q to quit (case sensitive)"),
            MoveToNextLine(1)
        )?;
        stdout.flush()?;
        let ev = read_char(false)?;
        if let (KeyCode::Char(char), ..) = ev {
            if char == 'h' {
                tutorial()?;
            } else if char == 's' {
                break;
            } else if char == 'Q' {
                exit(|| println!("Q pressed, exiting..."), 0)
            }
        }
    }

    let mut card_ids = (0u8..81).collect::<Vec<_>>();
    card_ids.shuffle(&mut rand::thread_rng());
    let state_mutex = Arc::new(Mutex::new(Game::new(card_ids.into_iter())));
    let clone_state_mutex = Arc::clone(&state_mutex);
    // increment timer
    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        PS("Time elapsed: ".bold()),
    )?;
    let game = state_mutex.lock().unwrap();
    game.print(&mut stdout)?;
    std::mem::drop(game);
    queue!(
        stdout,
        MoveTo(0, 5),
        P("Keybind table".bold()),
        MoveToNextLine(1),
        P(KEYBINDS),
        MoveToNextLine(1),
        PS("Last input: ".bold()),
        P("<nothing>")
    )?;
    std::thread::spawn(|| {
        let mut stdout = std::io::stdout();
        let started = std::time::Instant::now();
        let state_mutex = clone_state_mutex;
        loop {
            std::thread::yield_now();
            let game = state_mutex.lock().unwrap();
            let now = game.ended.unwrap_or_else(|| std::time::Instant::now());
            let elapsed = (now - started).as_millis();
            let elapsed_millis = elapsed % 1000;
            let elapsed_secs = (elapsed % 60000 - elapsed_millis) / 1000;
            let elapsed_mins = (elapsed % 3600000 - elapsed_millis - elapsed_secs * 1000) / 60000;
            let elapsed_hs =
                (elapsed - elapsed_millis - elapsed_secs * 1000 - elapsed_mins * 60000) / 3600000;
            if game.ended.is_some() {
                exit(
                    || {
                        execute!(stdout,
                    P("Set completed in "),
                    PS(format!("{elapsed_hs:02}:{elapsed_mins:02}:{elapsed_secs:02}.{elapsed_millis:03}s").bold())
                ).unwrap()
                    },
                    0,
                )
            } else {
                queue!(
                    stdout,
                    MoveTo(14, 0),
                    P(format!(
                        "{elapsed_hs:02}:{elapsed_mins:02}:{elapsed_secs:02}.{elapsed_millis:03}s"
                    ))
                )
                .unwrap();
            }
            stdout.flush().unwrap();
            std::mem::drop(game);
        }
    });
    loop {
        let read = read_char(true)?;
        let mut game = state_mutex.lock().unwrap();
        queue!(
            stdout,
            MoveTo(0, 1),
            MoveTo(12, 10),
            Clear(ClearType::UntilNewLine),
            P(if read.1 {
                read.0.to_string()
            } else {
                "<mouse click>".to_string()
            }),
            P(" => ")
        )?;
        if let (crossterm::event::KeyCode::Char(char), ..) = read {
            if char == 'Q' {
                exit(|| println!("Requested exit (Q pressed), exiting..."), 0);
            }
            if let Some(index) = corresponding_index(char) {
                match game.select_card(index) {
                    Err(()) => queue!(stdout, P("Selction out of bounds"))?,
                    Ok((card, false)) => queue!(stdout, P("Deselected "), card.stylise(false))?,
                    Ok((card, true)) => {
                        let styled_card = card.stylise(false);
                        match game.check_selected_set() {
                            Some(set_result) => match set_result {
                                Ok((card1, card2, card3)) => queue!(
                                    stdout,
                                    card1.stylise(false),
                                    S,
                                    card2.stylise(false),
                                    S,
                                    card3.stylise(false),
                                    S,
                                    P("form a set!")
                                )?,
                                Err((card1, card2, card3)) => queue!(
                                    stdout,
                                    card1.stylise(false),
                                    S,
                                    card2.stylise(false),
                                    S,
                                    card3.stylise(false),
                                    S,
                                    P("do not form a set!")
                                )?,
                            },
                            None => queue!(std::io::stdout(), P("Selected "), styled_card)?,
                        }
                    }
                }
            } else if read.0 == KeyCode::Backspace {
                match game.pop_last() {
                    Some(card) => queue!(stdout, P("Deselected "), card.stylise(false))?,
                    None => queue!(stdout, P("Nothing selected"))?,
                }
            } else {
                queue!(stdout, P(format!("Key has no known binding")))?;
            }
        }
        queue!(stdout, MoveTo(0, 1))?;
        game.print(&mut stdout)?;

        std::mem::drop(game); // unlock thread for others to use
    }
}
