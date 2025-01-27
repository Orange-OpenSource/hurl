/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use hurl_core::ast::Entry;
#[cfg(target_family = "unix")]
use hurl_core::ast::Request;
#[cfg(target_family = "unix")]
use {
    std::io::{stderr, stdin, Write},
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
};

/// Interactively asks user to execute `entry` or quit.
#[cfg(target_family = "unix")]
pub fn pre_entry(entry: &Entry) -> bool {
    eprintln!("\nInteractive mode");
    eprintln!("\nNext request:");
    eprintln!();

    log_request(&entry.request);

    // In raw mode, "\n" only means "go one line down", not "line break"
    // To effectively go do the next new line, we have to write "\r\n".
    let mut stderr = stderr().into_raw_mode().unwrap();

    write!(
        stderr,
        "\r\nPress Q (Quit) or C (Continue){}\r\n",
        termion::cursor::Hide
    )
    .unwrap();

    stderr.flush().unwrap();
    let mut exit = false;

    for c in stdin().keys() {
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

#[allow(dead_code)]
#[cfg(target_family = "unix")]
fn log_request(request: &Request) {
    let method = &request.method;
    let url = &request.url;
    eprintln!("{method} {url}");
}

#[cfg(target_family = "windows")]
pub fn pre_entry(_: &Entry) -> bool {
    eprintln!("Interactive not supported yet in windows!");
    true
}

pub fn post_entry() -> bool {
    false
}
