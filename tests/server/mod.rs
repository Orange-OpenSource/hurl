#![allow(dead_code)]

use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct Server {
    messages: Option<Sender<Message>>,
    addr: Addr,
    thread: Option<thread::JoinHandle<()>>,
}

enum Message {
    Read(String),
    Write(String),
}

enum Addr {
    Tcp(SocketAddr),
    Unix(PathBuf),
}

fn run(stream: impl Read + Write, rx: &Receiver<Message>) {
    let mut socket = BufReader::new(stream);
    for msg in rx.iter() {
        match msg {
            Message::Read(ref expected) => {
                let mut expected = &expected[..];
                let mut expected_headers = HashSet::new();
                while let Some(i) = expected.find("\n") {
                    let line = &expected[..i + 1];
                    expected = &expected[i + 1..];
                    expected_headers.insert(line);
                    if line == "\r\n" {
                        break;
                    }
                }

                let mut expected_len = None;
                while expected_headers.len() > 0 {
                    let mut actual = String::new();
                    t!(socket.read_line(&mut actual));
                    if actual.starts_with("Content-Length") {
                        let len = actual.split(": ").skip(1).next().unwrap();
                        expected_len = len.trim().parse().ok();
                    }
                    // various versions of libcurl do different things here
                    if actual == "Proxy-Connection: Keep-Alive\r\n" {
                        continue;
                    }
                    if expected_headers.remove(&actual[..]) {
                        continue;
                    }

                    let mut found = None;
                    for header in expected_headers.iter() {
                        if lines_match(header, &actual) {
                            found = Some(header.clone());
                            break;
                        }
                    }
                    if let Some(found) = found {
                        expected_headers.remove(&found);
                        continue;
                    }
                    panic!(
                        "unexpected header: {:?} (remaining headers {:?})",
                        actual, expected_headers
                    );
                }
                for header in expected_headers {
                    panic!("expected header but not found: {:?}", header);
                }

                let mut line = String::new();
                let mut socket = match expected_len {
                    Some(amt) => socket.by_ref().take(amt),
                    None => socket.by_ref().take(expected.len() as u64),
                };
                while socket.limit() > 0 {
                    line.truncate(0);
                    t!(socket.read_line(&mut line));
                    if line.len() == 0 {
                        break;
                    }
                    if expected.len() == 0 {
                        panic!("unexpected line: {:?}", line);
                    }
                    let i = expected.find("\n").unwrap_or(expected.len() - 1);
                    let expected_line = &expected[..i + 1];
                    expected = &expected[i + 1..];
                    if lines_match(expected_line, &line) {
                        continue;
                    }
                    panic!(
                        "lines didn't match:\n\
                         expected: {:?}\n\
                         actual:   {:?}\n",
                        expected_line, line
                    )
                }
                if expected.len() != 0 {
                    println!("didn't get expected data: {:?}", expected);
                }
            }
            Message::Write(ref to_write) => {
                t!(socket.get_mut().write_all(to_write.as_bytes()));
                return;
            }
        }
    }

    let mut dst = Vec::new();
    t!(socket.read_to_end(&mut dst));
    assert_eq!(dst.len(), 0);
}

fn lines_match(expected: &str, mut actual: &str) -> bool {
    for (i, part) in expected.split("[..]").enumerate() {
        match actual.find(part) {
            Some(j) => {
                if i == 0 && j != 0 {
                    return false;
                }
                actual = &actual[j + part.len()..];
            }
            None => return false,
        }
    }
    actual.is_empty() || expected.ends_with("[..]")
}

impl Server {
    pub fn new() -> Server {
        let listener = t!(TcpListener::bind("127.0.0.1:0"));
        let addr = t!(listener.local_addr());
        let (tx, rx) = channel();
        let thread = thread::spawn(move || run(listener.accept().unwrap().0, &rx));
        Server {
            messages: Some(tx),
            addr: Addr::Tcp(addr),
            thread: Some(thread),
        }
    }

    #[cfg(not(windows))]
    pub fn new_unix() -> Server {
        use std::os::unix::net::UnixListener;

        let path = "/tmp/easy_server.sock";
        std::fs::remove_file(path).ok();
        let listener = t!(UnixListener::bind(path));
        let (tx, rx) = channel();
        let thread = thread::spawn(move || run(listener.incoming().next().unwrap().unwrap(), &rx));
        Server {
            messages: Some(tx),
            addr: Addr::Unix(path.into()),
            thread: Some(thread),
        }
    }

    pub fn receive(&self, msg: &str) {
        self.msg(Message::Read(self.replace_port(msg)));
    }

    fn replace_port(&self, msg: &str) -> String {
        match &self.addr {
            Addr::Tcp(addr) => msg.replace("$PORT", &addr.port().to_string()),
            Addr::Unix(_) => msg.to_string(),
        }
    }

    pub fn send(&self, msg: &str) {
        self.msg(Message::Write(self.replace_port(msg)));
    }

    fn msg(&self, msg: Message) {
        t!(self.messages.as_ref().unwrap().send(msg));
    }

    pub fn addr(&self) -> &SocketAddr {
        match &self.addr {
            Addr::Tcp(addr) => addr,
            Addr::Unix(_) => panic!("server is a UnixListener"),
        }
    }

    #[cfg(not(windows))]
    pub fn path(&self) -> &str {
        match &self.addr {
            Addr::Tcp(_) => panic!("server is a TcpListener"),
            Addr::Unix(p) => p.as_os_str().to_str().unwrap(),
        }
    }

    pub fn url(&self, path: &str) -> String {
        match &self.addr {
            Addr::Tcp(addr) => format!("http://{}{}", addr, path),
            Addr::Unix(_) => format!("http://localhost{}", path),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        match &self.addr {
            Addr::Tcp(addr) => drop(TcpStream::connect(addr)),
            Addr::Unix(p) => t!(std::fs::remove_file(p)),
        }

        drop(self.messages.take());
        let res = self.thread.take().unwrap().join();
        if !thread::panicking() {
            t!(res);
        } else if let Err(e) = res {
            println!("child server thread also failed: {:?}", e);
        }
    }
}
