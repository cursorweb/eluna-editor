use crossterm::{
    cursor::{self, CursorShape},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{fs, io::stdout};

fn main() {
    let name = "./test/main.eo";
    let file: Vec<String> = fs::read_to_string(name)
        .unwrap()
        .split("\n")
        .map(|a| a.to_string())
        .collect();

    let mut stdout = stdout();

    enable_raw_mode().unwrap();

    let mut y = 0;
    let mut x = 0;

    execute!(stdout, cursor::SetCursorShape(CursorShape::Line)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();

    loop {
        match read().unwrap() {
            Event::Key(event) => {
                let KeyEvent {
                    code: c,
                    modifiers: _,
                } = event;

                if let KeyCode::Char(k) = c {
                    execute!(stdout, cursor::MoveTo(x, y), Print(k)).unwrap();
                    x += 1;
                } else {
                    match c {
                        KeyCode::Enter => {
                            y += 1;
                            x = 0;
                            execute!(stdout, cursor::MoveTo(x, y)).unwrap();
                        }
                        _ => break,
                    }
                }
            }
            _ => (),
        }
    }

    // cleanup
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::SetCursorShape(CursorShape::Block)
    )
    .unwrap();
    disable_raw_mode().unwrap();
}
