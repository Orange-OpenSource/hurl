extern crate curl;

use std::cell::{Cell, RefCell};
use std::io::Read;
use std::rc::Rc;
use std::str;
use std::time::Duration;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
        }
    };
}

use curl::easy::{Easy, List, ReadError, Transfer, WriteError};

use server::Server;
mod server;

fn handle() -> Easy {
    let mut e = Easy::new();
    t!(e.timeout(Duration::new(20, 0)));
    return e;
}

fn sink(data: &[u8]) -> Result<usize, WriteError> {
    Ok(data.len())
}

#[test]
fn get_smoke() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send("HTTP/1.1 200 OK\r\n\r\n");

    let mut handle = handle();
    t!(handle.url(&s.url("/")));
    t!(handle.perform());
}

#[test]
fn get_path() {
    let s = Server::new();
    s.receive(
        "\
         GET /foo HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send("HTTP/1.1 200 OK\r\n\r\n");

    let mut handle = handle();
    t!(handle.url(&s.url("/foo")));
    t!(handle.perform());
}

#[test]
fn write_callback() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send("HTTP/1.1 200 OK\r\n\r\nhello!");

    let mut all = Vec::<u8>::new();
    {
        let mut handle = handle();
        t!(handle.url(&s.url("/")));
        let mut handle = handle.transfer();
        t!(handle.write_function(|data| {
            all.extend(data);
            Ok(data.len())
        }));
        t!(handle.perform());
    }
    assert_eq!(all, b"hello!");
}

//#[test]
//fn resolve() {
//    let s = Server::new();
//    s.receive(
//        "\
//         GET / HTTP/1.1\r\n\
//         Host: example.com:$PORT\r\n\
//         Accept: */*\r\n\
//         \r\n",
//    );
//    s.send("HTTP/1.1 200 OK\r\n\r\n");
//
//    let mut list = List::new();
//    t!(list.append(&format!("example.com:{}:127.0.0.1", s.addr().port())));
//    let mut handle = handle();
//    t!(handle.url(&format!("http://example.com:{}/", s.addr().port())));
//    t!(handle.resolve(list));
//    t!(handle.perform());
//}

#[test]
fn progress() {
    let s = Server::new();
    s.receive(
        "\
         GET /foo HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send("HTTP/1.1 200 OK\r\n\r\nHello!");

    let mut hits = 0;
    let mut dl = 0.0;
    {
        let mut handle = handle();
        t!(handle.url(&s.url("/foo")));
        t!(handle.progress(true));
        t!(handle.write_function(sink));

        let mut handle = handle.transfer();
        t!(handle.progress_function(|_, a, _, _| {
            hits += 1;
            dl = a;
            true
        }));
        t!(handle.perform());
    }
    assert!(hits > 0);
    assert_eq!(dl, 6.0);
}

#[test]
fn headers() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
HTTP/1.1 200 OK\r\n\
Foo: bar\r\n\
Bar: baz\r\n\
\r\n
Hello!",
    );

    let mut headers = Vec::new();
    {
        let mut handle = handle();
        t!(handle.url(&s.url("/")));

        let mut handle = handle.transfer();
        t!(handle.header_function(|h| {
            headers.push(str::from_utf8(h).unwrap().to_string());
            true
        }));
        t!(handle.write_function(sink));
        t!(handle.perform());
    }
    assert_eq!(
        headers,
        vec![
            "HTTP/1.1 200 OK\r\n".to_string(),
            "Foo: bar\r\n".to_string(),
            "Bar: baz\r\n".to_string(),
            "\r\n".to_string(),
        ]
    );
}

#[test]
fn fail_on_error() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 401 Not so good\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.fail_on_error(true));
    assert!(h.perform().is_err());

    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 401 Not so good\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.fail_on_error(false));
    t!(h.perform());
}

#[test]
fn port() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: localhost:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url("http://localhost/"));
    t!(h.port(s.addr().port()));
    t!(h.perform());
}

#[test]
fn proxy() {
    let s = Server::new();
    s.receive(
        "\
         GET http://example.com/ HTTP/1.1\r\n\
         Host: example.com\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url("http://example.com/"));
    t!(h.proxy(&s.url("/")));
    t!(h.perform());
}

#[test]
#[ignore] // fails on newer curl versions? seems benign
fn noproxy() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.proxy(&s.url("/")));
    t!(h.noproxy("127.0.0.1"));
    t!(h.perform());
}

#[test]
fn misc() {
    let mut h = handle();
    t!(h.tcp_nodelay(true));
    // t!(h.tcp_keepalive(true));
    // t!(h.tcp_keepidle(Duration::new(3, 0)));
    // t!(h.tcp_keepintvl(Duration::new(3, 0)));
    t!(h.buffer_size(10));
    t!(h.dns_cache_timeout(Duration::new(1, 0)));
}

#[test]
fn dns_servers() {
    let mut h = handle();
    // Tests are not using a libcurl with c-ares, so this
    // always fails. Test anyway to make sure it returns
    // an error instead of panicing.
    assert!(h.dns_servers("").is_err());
    assert!(h.dns_servers("nonsense").is_err());
    assert!(h.dns_servers("8.8.8.8,8.8.4.4").is_err());
}

#[test]
fn userpass() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Authorization: Basic YmFyOg==\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.username("foo"));
    t!(h.username("bar"));
    t!(h.perform());
}

#[test]
fn accept_encoding() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Accept-Encoding: gzip\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.accept_encoding("gzip"));
    t!(h.perform());
}

#[test]
fn follow_location() {
    let s1 = Server::new();
    let s2 = Server::new();
    s1.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s1.send(&format!(
        "\
         HTTP/1.1 301 Moved Permanently\r\n\
         Location: http://{}/foo\r\n\
         \r\n",
        s2.addr()
    ));

    s2.receive(
        "\
         GET /foo HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s2.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s1.url("/")));
    t!(h.follow_location(true));
    t!(h.perform());
}

#[test]
fn put() {
    let s = Server::new();
    s.receive(
        "\
         PUT / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Content-Length: 5\r\n\
         \r\n\
         data\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut data = "data\n".as_bytes();
    let mut list = List::new();
    t!(list.append("Expect:"));
    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.put(true));
    t!(h.in_filesize(5));
    t!(h.upload(true));
    t!(h.http_headers(list));
    let mut h = h.transfer();
    t!(h.read_function(|buf| Ok(data.read(buf).unwrap())));
    t!(h.perform());
}

#[test]
fn post1() {
    let s = Server::new();
    s.receive(
        "\
         POST / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Content-Length: 5\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         \r\n\
         data\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.post(true));
    t!(h.post_fields_copy(b"data\n"));
    t!(h.perform());
}

#[test]
fn post2() {
    let s = Server::new();
    s.receive(
        "\
         POST / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Content-Length: 5\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         \r\n\
         data\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.post(true));
    t!(h.post_fields_copy(b"data\n"));
    t!(h.write_function(sink));
    t!(h.perform());
}

#[test]
fn post3() {
    let s = Server::new();
    s.receive(
        "\
         POST / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Content-Length: 5\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         \r\n\
         data\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut data = "data\n".as_bytes();
    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.post(true));
    t!(h.post_field_size(5));
    let mut h = h.transfer();
    t!(h.read_function(|buf| Ok(data.read(buf).unwrap())));
    t!(h.perform());
}

#[test]
fn referer() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Referer: foo\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.referer("foo"));
    t!(h.perform());
}

#[test]
fn useragent() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         User-Agent: foo\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.useragent("foo"));
    t!(h.perform());
}

#[test]
fn custom_headers() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Foo: bar\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut custom = List::new();
    t!(custom.append("Foo: bar"));
    t!(custom.append("Accept:"));
    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.http_headers(custom));
    t!(h.perform());
}

#[test]
fn cookie() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Cookie: foo\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.cookie("foo"));
    t!(h.perform());
}

#[test]
fn url_encoding() {
    let mut h = handle();
    assert_eq!(h.url_encode(b"foo"), "foo");
    assert_eq!(h.url_encode(b"foo bar"), "foo%20bar");
    assert_eq!(h.url_encode(b"foo bar\xff"), "foo%20bar%FF");
    assert_eq!(h.url_encode(b""), "");
    assert_eq!(h.url_decode("foo"), b"foo");
    assert_eq!(h.url_decode("foo%20bar"), b"foo bar");
    assert_eq!(h.url_decode("foo%2"), b"foo%2");
    assert_eq!(h.url_decode("foo%xx"), b"foo%xx");
    assert_eq!(h.url_decode("foo%ff"), b"foo\xff");
    assert_eq!(h.url_decode(""), b"");
}

#[test]
fn getters() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.cookie_file("/dev/null"));
    t!(h.perform());
    assert_eq!(t!(h.response_code()), 200);
    assert_eq!(t!(h.redirect_count()), 0);
    assert_eq!(t!(h.redirect_url()), None);
    assert_eq!(t!(h.content_type()), None);

    let addr = format!("http://{}/", s.addr());
    assert_eq!(t!(h.effective_url()), Some(&addr[..]));

    // TODO: test this
    // let cookies = t!(h.cookies()).iter()
    //                              .map(|s| s.to_vec())
    //                              .collect::<Vec<_>>();
    // assert_eq!(cookies.len(), 1);
}

#[test]
#[should_panic]
fn panic_in_callback() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.header_function(|_| panic!()));
    t!(h.perform());
}

#[test]
fn abort_read() {
    let s = Server::new();
    s.receive(
        "\
         PUT / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         Content-Length: 2\r\n\
         \r\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.read_function(|_| Err(ReadError::Abort)));
    t!(h.put(true));
    t!(h.in_filesize(2));
    let mut list = List::new();
    t!(list.append("Expect:"));
    t!(h.http_headers(list));
    let err = h.perform().unwrap_err();
    assert!(err.is_aborted_by_callback());
}

#[test]
fn pause_write_then_resume() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
HTTP/1.1 200 OK\r\n\
\r\n
a\n
b",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.progress(true));

    struct State<'a, 'b> {
        paused: Cell<bool>,
        unpaused: Cell<bool>,
        transfer: RefCell<Transfer<'a, 'b>>,
    }

    let h = Rc::new(State {
        paused: Cell::new(false),
        unpaused: Cell::new(false),
        transfer: RefCell::new(h.transfer()),
    });

    let h2 = h.clone();
    t!(h.transfer
        .borrow_mut()
        .write_function(move |data| if h2.unpaused.get() {
            h2.unpaused.set(false);
            Ok(data.len())
        } else {
            h2.paused.set(true);
            Err(WriteError::Pause)
        }));
    let h2 = h.clone();
    t!(h.transfer
        .borrow_mut()
        .progress_function(move |_, _, _, _| {
            if h2.paused.get() {
                h2.paused.set(false);
                h2.unpaused.set(true);
                t!(h2.transfer.borrow().unpause_write());
            }
            true
        }));
    t!(h.transfer.borrow().perform());
}

#[test]
fn perform_in_perform_is_bad() {
    let s = Server::new();
    s.receive(
        "\
         GET / HTTP/1.1\r\n\
         Host: 127.0.0.1:$PORT\r\n\
         Accept: */*\r\n\
         \r\n",
    );
    s.send(
        "\
HTTP/1.1 200 OK\r\n\
\r\n
a\n
b",
    );

    let mut h = handle();
    t!(h.url(&s.url("/")));
    t!(h.progress(true));

    let h = Rc::new(RefCell::new(h.transfer()));

    let h2 = h.clone();
    t!(h.borrow_mut().write_function(move |data| {
        assert!(h2.borrow().perform().is_err());
        Ok(data.len())
    }));
    t!(h.borrow().perform());
}

#[cfg(not(windows))]
#[test]
fn check_unix_socket() {
    let s = Server::new_unix();
    s.receive(
        "\
         POST / HTTP/1.1\r\n\
         Host: localhost\r\n\
         Accept: */*\r\n\
         Content-Length: 5\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         \r\n\
         data\n",
    );
    s.send(
        "\
         HTTP/1.1 200 OK\r\n\
         \r\n",
    );

    let mut h = handle();
    t!(h.unix_socket(s.path()));
    t!(h.url(&s.url("/")));
    t!(h.post(true));
    t!(h.post_fields_copy(b"data\n"));
    t!(h.perform());
}
