use crossterm::event::*;
static KEYMAP: &str = "qwertyuasdfghjzxcvbnm";
pub fn read_char(capture_clicks: bool) -> Result<(KeyCode, bool), std::io::Error> {
    loop {
        match read()? {
            Event::Key(ev) => {
                if ev.kind == KeyEventKind::Press {
                    return Ok((ev.code, true));
                }
            }
            Event::Mouse(ev) => {
                if ev.kind == MouseEventKind::Down(MouseButton::Left) {
                    if capture_clicks {
                        if ev.row == 0 || ev.row > 3 {
                            continue;
                        }
                        let col = (ev.column + 2) / 8;
                        if col % 8 == 5 {
                            continue;
                        } // ignore pressing on pipe
                        let row = ev.row - 1;
                        let index = row * 7 + col;
                        if index > 20 {
                            continue;
                        }
                        return Ok((
                            KeyCode::Char(KEYMAP.chars().nth(index.into()).unwrap()),
                            false,
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn exit(final_remarks: impl FnOnce(), code: i32) -> ! {
    use crossterm::{cursor, execute, terminal::*};
    disable_raw_mode().unwrap();
    let _ = execute!(
        std::io::stdout(),
        Clear(ClearType::Purge),
        LeaveAlternateScreen,
        cursor::Show,
        cursor::RestorePosition,
        DisableMouseCapture
    );
    final_remarks();
    std::process::exit(code);
}

pub fn corresponding_index(c: char) -> Option<usize> {
    match c {
        'q' => Some(0),
        'w' => Some(3),
        'e' => Some(6),
        'r' => Some(9),
        't' => Some(12),
        'y' => Some(15),
        'u' => Some(18),
        'a' => Some(1),
        's' => Some(4),
        'd' => Some(7),
        'f' => Some(10),
        'g' => Some(13),
        'h' => Some(16),
        'j' => Some(19),
        'z' => Some(2),
        'x' => Some(5),
        'c' => Some(8),
        'v' => Some(11),
        'b' => Some(14),
        'n' => Some(17),
        'm' => Some(20),
        _ => None,
    }
}
