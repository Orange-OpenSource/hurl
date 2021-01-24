extern crate termion;

use std::io::{stderr, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn pre_entry() -> bool {
    let stdin = stdin();
    let mut stderr = stderr().into_raw_mode().unwrap();

    eprintln!("\n\rinteractive mode:");
    write!(
        stderr,
        "\rPress Q (Quit) or C (Continue)\n\n\r{}",
        termion::cursor::Hide
    )
    .unwrap();

    stderr.flush().unwrap();
    let mut exit = false;

    for c in stdin.keys() {
        print!("\r");
        match c.unwrap() {
            Key::Char('q') => {
                exit = true;
                break;
            }
            Key::Char('c') => {
                break;
            }
            _ => {}
        }
    }
    print!("{}\r{}", termion::clear::CurrentLine, termion::cursor::Show);
    exit
}

pub fn post_entry() -> bool {
    false
}
