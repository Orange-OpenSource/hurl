#[cfg(target_family = "unix")]
use {
    std::io::{stderr, stdin, Write},
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
};

#[cfg(target_family = "unix")]
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

#[cfg(target_family = "windows")]
pub fn pre_entry() -> bool {
    eprintln!("interactive not supported yet in windows!");
    true
}

pub fn post_entry() -> bool {
    false
}
