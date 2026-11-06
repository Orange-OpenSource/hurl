//! `Parser::is_well_formed_html`: libxml2's verdict, except that unknown-tag
//! reports from libxml2's pre-2.14 HTML4-era parser (HTML5, SVG and MathML
//! elements) are not counted.
//!
//! It watches libxml2's errors through the thread's structured error handler,
//! which must be restored afterwards, and what it records must belong to the
//! calling parse only.
use libxml::bindings::__xmlStructuredError;
use libxml::parser::Parser;

const WELL_FORMED_UNKNOWN_TAG: &str =
  "<!DOCTYPE html>\n<html><head><title>T</title></head><body><math><mn>2</mn></math></body></html>";
const ILL_FORMED: &str = "<broken <markup>> </boom>";

#[test]
/// The thread's structured error handler is left as it was found.
fn handler_is_restored() {
  let parser = Parser::default_html();
  let before = unsafe { *__xmlStructuredError() }.map(|f| f as usize);
  assert!(parser.is_well_formed_html(WELL_FORMED_UNKNOWN_TAG));
  assert!(!parser.is_well_formed_html(ILL_FORMED));
  let after = unsafe { *__xmlStructuredError() }.map(|f| f as usize);
  assert_eq!(
    before, after,
    "is_well_formed_html left its error handler installed"
  );
}

#[test]
/// One thread's tolerated unknown tag must not make another thread's malformed
/// input pass.
fn results_do_not_leak_across_threads() {
  let threads: Vec<_> = (0..8)
    .map(|i| {
      std::thread::spawn(move || {
        let parser = Parser::default_html();
        for _ in 0..500 {
          if i % 2 == 0 {
            assert!(parser.is_well_formed_html(WELL_FORMED_UNKNOWN_TAG));
          } else {
            assert!(
              !parser.is_well_formed_html(ILL_FORMED),
              "malformed input passed: another thread's unknown tag leaked in"
            );
          }
        }
      })
    })
    .collect();
  for thread in threads {
    thread.join().unwrap();
  }
}

fn page(body: &str) -> String {
  format!("<!DOCTYPE html>\n<html><head><title>t</title></head><body>{body}</body></html>")
}

#[test]
/// HTML5, SVG and MathML content is well-formed on every libxml2 version: 2.14+
/// does not report it, and older versions' unknown-tag reports are not counted.
fn html5_svg_and_mathml_are_well_formed() {
  let parser = Parser::default_html();
  for body in [
    "<main><section><article><figure><figcaption>c</figcaption></figure></article></section></main>",
    "<video src=a></video><template><p>x</p></template><my-widget>w</my-widget>",
    r#"<svg viewBox="0 0 10 10"><linearGradient id=g/><circle cx=5 cy=5 r=4/><foreignObject><p>x</p></foreignObject></svg>"#,
    r#"<math display=block><semantics><mrow><mi>x</mi><mo>+</mo><mn>2</mn></mrow><annotation-xml encoding="MathML-Content"><apply><plus/><ci>x</ci><cn>2</cn></apply></annotation-xml></semantics></math>"#,
  ] {
    assert!(parser.is_well_formed_html(page(body)), "{body}");
  }
}

#[test]
/// A tolerated unknown tag must not change libxml2's verdict on the rest of the
/// document. (libxml2 2.9 judges the mismatched tags ill-formed; 2.14+ does not.)
fn unknown_tags_do_not_mask_other_errors() {
  let parser = Parser::default_html();
  let mismatched = "<p><b>x</p></i>";
  let without = parser.is_well_formed_html(page(mismatched));
  let with = parser.is_well_formed_html(page(&format!("<math><mi>x</mi></math>{mismatched}")));
  assert_eq!(
    with, without,
    "an unknown tag changed the verdict on unrelated errors"
  );
}
