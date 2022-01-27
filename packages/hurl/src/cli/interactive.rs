use hurl_core::ast::Entry;
#[cfg(target_family = "unix")]
use hurl_core::ast::{MultipartParam, Request, SectionValue};

#[cfg(target_family = "unix")]
use {
    std::io::{stderr, stdin, Write},
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
};

#[cfg(target_family = "unix")]
pub fn pre_entry(entry: Entry) -> bool {
    let stdin = stdin();
    let mut stderr = stderr().into_raw_mode().unwrap();

    eprintln!("\n\rinteractive mode");
    eprintln!("\n\rnext request:");
    log_request(entry.request);

    write!(
        stderr,
        "\r\nPress Q (Quit) or C (Continue)\n\n\r{}",
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

#[cfg(target_family = "unix")]
fn log_request(request: Request) {
    eprintln!("\r\n{} {}", request.method, request.url);
    for header in request.headers {
        eprintln!("\r{}: {}", header.key.value, header.value);
    }
    for section in request.sections {
        eprintln!("\r[{}]", section.name());
        match section.value {
            SectionValue::QueryParams(key_values) => {
                for value in key_values {
                    eprintln!("\r{}: {}", value.key.value, value.value);
                }
            }
            SectionValue::BasicAuth(key_value) => {
                eprintln!("\r{}: {}", key_value.key.value, key_value.value);
            }
            SectionValue::FormParams(key_values) => {
                for value in key_values {
                    eprintln!("\r{}: {}", value.key.value, value.value);
                }
            }
            SectionValue::MultipartFormData(multipart_params) => {
                for param in multipart_params {
                    match param {
                        MultipartParam::Param(value) => {
                            eprintln!("\r{}: {}", value.key.value, value.value)
                        }
                        MultipartParam::FileParam(file_param) => {
                            let content_type =
                                if let Some(content_type) = file_param.value.content_type {
                                    format!("; {}", content_type)
                                } else {
                                    "".to_string()
                                };
                            eprintln!(
                                "\r{}: {}{}",
                                file_param.key.value, file_param.value.filename.value, content_type
                            );
                        }
                    }
                }
            }
            SectionValue::Cookies(cookies) => {
                for cookie in cookies {
                    eprintln!("\r{}: {}", cookie.name.value, cookie.value.value);
                }
            }
            _ => {}
        }
    }
}

#[cfg(target_family = "windows")]
pub fn pre_entry(_: Entry) -> bool {
    eprintln!("interactive not supported yet in windows!");
    true
}

pub fn post_entry() -> bool {
    false
}
