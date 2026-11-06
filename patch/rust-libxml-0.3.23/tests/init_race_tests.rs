//! Every root entry point must call `crate::init_parser()` before using libxml2.
//!
//! libxml2 before 2.12 creates some of its mutexes lazily, so threads racing to
//! use it first can deadlock (seen as a hung `schema_tests` in CI, blocked in
//! `xmlRMutexLock` <- `__xmlRandom` <- `xmlDictCreate`). The race exists only
//! once per process, at libxml2's first use, so this test re-runs its own binary
//! as child processes that race the entry points from a fresh start, and fails
//! if a child does not finish in time. A deadlocked child is killed, so the test
//! itself cannot hang.
use std::process::{Command, Stdio};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

use libxml::parser::Parser;
use libxml::reader::TextReader;
use libxml::schemas::SchemaParserContext;
use libxml::tree::Document;
use libxml::xpath::is_well_formed_xpath;

const CHILD_ENV: &str = "RUST_LIBXML_INIT_RACE_CHILD";
const CHILDREN: usize = 20;
const CHILD_TIMEOUT: Duration = Duration::from_secs(10);
const SCHEMA: &str =
  r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"><xs:element name="a"/></xs:schema>"#;

#[test]
/// Runs only in a child process: races each root entry point's first use of libxml2.
fn race_child() {
  if std::env::var_os(CHILD_ENV).is_none() {
    return;
  }
  let threads = 16;
  let barrier = Arc::new(Barrier::new(threads));
  let handles: Vec<_> = (0..threads)
    .map(|i| {
      let barrier = barrier.clone();
      std::thread::spawn(move || {
        barrier.wait();
        match i % 5 {
          0 => drop(SchemaParserContext::from_buffer(SCHEMA)),
          1 => drop(Document::new().unwrap()),
          2 => drop(TextReader::from_file("tests/resources/file01.xml", 0).unwrap()),
          3 => assert!(is_well_formed_xpath("//a")),
          _ => drop(Parser::default().parse_string("<a/>").unwrap()),
        }
      })
    })
    .collect();
  for handle in handles {
    handle.join().unwrap();
  }
}

#[test]
fn entry_points_initialize_libxml2() {
  if std::env::var_os(CHILD_ENV).is_some() {
    return;
  }
  let exe = std::env::current_exe().unwrap();
  for i in 0..CHILDREN {
    let mut child = Command::new(&exe)
      .args(["--exact", "race_child", "--quiet"])
      .env(CHILD_ENV, "1")
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .unwrap();
    let started = Instant::now();
    loop {
      if let Some(status) = child.try_wait().unwrap() {
        assert!(status.success(), "child {i} failed: {status}");
        break;
      }
      if started.elapsed() > CHILD_TIMEOUT {
        let _ = child.kill();
        let _ = child.wait();
        panic!("child {i} deadlocked: an entry point used libxml2 before init_parser()");
      }
      std::thread::sleep(Duration::from_millis(10));
    }
  }
}
