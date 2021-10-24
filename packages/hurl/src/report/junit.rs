use crate::{cli::CliError, runner::HurlResult};
use hurl_core::error::Error;
use libxml::{
    parser::Parser,
    tree::{Document, Node},
};
use std::path::PathBuf;

/// Generate a JUnix XML report and write it to the specified `PathBuf`
pub fn write_junit_report(
    file_path: PathBuf,
    hurl_results: Vec<HurlResult>,
) -> Result<(), CliError> {
    let mut doc = if file_path.exists() {
        let parser = Parser::default();
        parser
            .parse_string(
                std::fs::read_to_string(file_path.clone()).map_err(|e| CliError {
                    message: format!("Failed to read file {:?}: {:?}", file_path, e),
                })?,
            )
            .map_err(|e| CliError {
                message: format!("Failed to parse file {:?}: {:?}", file_path, e),
            })?
    } else {
        initialise_junit_report()?
    };
    let mut testsuites = doc
        .get_root_element()
        .ok_or_else(|| CliError::from("Missing testsuites element"))?;

    create_junit_report(&mut doc, &mut testsuites, hurl_results)?;

    if !file_path.exists() {
        let _ = match std::fs::File::create(&file_path) {
            Err(why) => {
                return Err(CliError {
                    message: format!("Issue writing to {}: {:?}", file_path.display(), why),
                });
            }
            Ok(file) => file,
        };
    } else {
    }

    write_junit_report_with(file_path, &doc)?;
    Ok(())
}

pub fn write_junit_report_with(file_path: PathBuf, doc: &Document) -> Result<(), CliError> {
    doc.save_file(&file_path.to_string_lossy())
        .map_err(|_| format!("Failed to save to {:?}", file_path))?;
    Ok(())
}

/// Write a JUnit XML report to the specified `libxml::Document`, appending to the `testsuites`
/// node
pub fn create_junit_report(
    doc: &mut Document,
    testsuites: &mut Node,
    reports: Vec<HurlResult>,
) -> Result<(), CliError> {
    let test_count: usize = testsuites
        .get_attribute("tests")
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap_or(0);
    let count = test_count + reports.len();
    testsuites.set_attribute("tests", &count.to_string())?;
    testsuites.set_attribute("name", "Hurl")?;

    let mut failures: usize = 0;
    let mut time: u128 = 0;

    for report in reports {
        time += report.time_in_ms;
        failures += if report.success { 0 } else { 1 };

        let mut testsuite = create_test_suite(doc, &report)?;

        testsuites.add_child(&mut testsuite)?;

        append_report_to(doc, &mut testsuite, &report)?;
    }

    testsuites.set_attribute("time", &(time / 1000).to_string())?;
    testsuites.set_attribute("failures", &failures.to_string())?;
    Ok(())
}

fn create_test_suite(doc: &mut Document, report: &HurlResult) -> Result<Node, CliError> {
    let mut testsuite = Node::new("testsuite", None, doc).unwrap();
    testsuite.set_attribute("name", &report.filename)?;
    testsuite.set_attribute("tests", &report.entries.len().to_string())?;
    Ok(testsuite)
}

fn append_report_to(
    doc: &Document,
    testsuite: &mut Node,
    report: &HurlResult,
) -> Result<(), CliError> {
    for er in &report.entries.clone() {
        let mut testcase = Node::new("testcase", None, doc).expect("Creating testcase");
        let req = er
            .request
            .as_ref()
            .map(|r| format!("{} {}", r.method, r.url))
            .unwrap_or_else(|| "(No request details)".to_string());
        testcase.set_attribute("name", &req)?;
        if !report.success {
            for ass in &er.asserts {
                let mut failure = Node::new("failure", None, doc).unwrap();
                if let Some(err) = ass.clone().error() {
                    failure.set_attribute("message", &err.fixme().to_string())?;
                }
                testcase.add_child(&mut failure)?;
            }

            for er in &er.errors {
                let mut failure = Node::new("failure", None, doc).unwrap();
                failure.set_attribute("message", &er.description())?;
                failure.set_content(&er.fixme().to_string())?;
                testcase.add_child(&mut failure)?;
            }
        }
        testsuite.add_child(&mut testcase)?;
    }
    Ok(())
}

fn initialise_junit_report() -> Result<Document, CliError> {
    let mut doc = Document::new().map_err(|e| CliError {
        message: format!("Failed to produce junit report: {:?}", e),
    })?;

    let testsuites = Node::new("testsuites", None, &doc).expect("Could not create testsuites node");
    doc.set_root_element(&testsuites);
    Ok(doc)
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, time::Duration};

    use hurl_core::ast::Pos;

    use crate::{
        http::{Request, Response, Version},
        report::junit::{create_junit_report, initialise_junit_report, write_junit_report_with},
        runner::{EntryResult, HurlResult},
    };

    use libxml::parser::Parser;

    #[test]
    fn test_create_jnit_report_empty() {
        let mut doc = initialise_junit_report().unwrap();
        let mut testsuites = doc.get_root_element().expect("No root element");
        create_junit_report(&mut doc, &mut testsuites, vec![]).unwrap();

        assert_eq!(0, testsuites.get_child_nodes().len());
    }

    #[test]
    fn test_append_jnit_report() {
        let mut doc = initialise_junit_report().unwrap();
        let mut testsuites = doc.get_root_element().expect("No root element");
        create_junit_report(&mut doc, &mut testsuites, vec![]).unwrap();

        let random_chars = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let random_filename = &format!(
            "{}/{}.xml",
            std::env::temp_dir().to_str().expect("No temp dir?"),
            random_chars
        );

        write_junit_report_with(PathBuf::from(random_filename), &doc).expect("Saving report");

        assert_eq!(
            "0".to_string(),
            testsuites.get_attribute("tests").expect("tests attribute")
        );

        assert_eq!(0, testsuites.get_child_nodes().len());

        let parser = Parser::default();
        let mut doc2 = parser
            .parse_string(std::fs::read_to_string(PathBuf::from(random_filename)).unwrap())
            .unwrap();

        let reports = make_reports();
        let mut testsuites = doc2
            .get_root_element()
            .expect("Failed to find root element for existing junit report");

        create_junit_report(&mut doc2, &mut testsuites, reports.clone())
            .expect("Could not create junit report for append test");
        doc2.save_file(random_filename).unwrap();
        let new_ts = doc2.get_root_element().unwrap();

        let test_count = new_ts.get_attribute("tests").expect("tests attribute");
        assert_eq!(reports.len().to_string(), test_count);

        let new_tc = new_ts.get_child_elements();
        assert_eq!(reports.len(), new_tc.len());
    }

    #[test]
    fn test_junit_report() {
        let reports = make_reports();

        let mut doc = initialise_junit_report().unwrap();
        let mut testsuites = doc.get_root_element().expect("No root element");

        create_junit_report(&mut doc, &mut testsuites, reports.clone()).unwrap();
        assert_eq!(
            testsuites.get_name(),
            "testsuites".to_string(),
            "Root element must be testsuites"
        );

        assert_eq!(
            "Hurl",
            testsuites.get_attribute("name").expect("tests attribute")
        );

        assert_eq!(
            "2",
            testsuites.get_attribute("tests").expect("tests attribute")
        );

        assert_eq!(
            "1",
            testsuites
                .get_attribute("failures")
                .expect("failures attribute"),
            "failure count"
        );

        let time_attr: u128 = testsuites
            .get_attribute("time")
            .expect("time attribute")
            .parse()
            .expect("Time doesn't parse as f32");
        assert_eq!(0, time_attr, "Time attribute in seconds"); // 100 + 200 ms is < 1 second -_-

        let testsuites = testsuites.get_child_nodes();
        assert_eq!(2, testsuites.len());
        {
            let first_ts = testsuites.get(0).expect("First testsuite");

            assert_eq!(
                reports.get(0).as_ref().unwrap().filename,
                first_ts
                    .get_attribute("name")
                    .expect("First testsuite name attribute")
            );

            let first_testcases = first_ts.get_child_nodes();
            assert_eq!(
                1,
                first_testcases.len(),
                "First testsuite should have 1 test case"
            );
            let first_tec = first_testcases
                .get(0)
                .expect("First testsuite should have 1 test case");
            assert_eq!(
                "GET https://www.google.com/",
                &first_tec.get_attribute("name").unwrap()
            );
            assert!(
                first_tec.get_first_child().is_none(),
                "No child expected for first successful result"
            );
        }

        let snd_ts = testsuites.get(1).expect("Second testsuite");
        {
            assert_eq!(
                reports.get(1).as_ref().unwrap().filename,
                snd_ts
                    .get_attribute("name")
                    .expect("2nd testsuite name attribute")
            );

            assert_eq!(
                reports.get(1).as_ref().unwrap().entries.len().to_string(),
                snd_ts
                    .get_attribute("tests")
                    .expect("2nd testsuite tests attribute")
            );

            // <testcase> under <testsuite> under <testsuites>
            let tc = snd_ts.get_child_nodes();
            assert_eq!(1, tc.len(), "2nd testsuite should have 1 test case");

            let first_tec = tc.get(0).expect("2nd testsuite should have 1 test case");
            assert_eq!(
                "GET https://www.legiggle.com/",
                &first_tec.get_attribute("name").unwrap()
            );
            let failure = first_tec
                .get_first_child()
                .expect("child expected for first failing result");
            assert_eq!(
                "actual value is <405>",
                failure.get_attribute("message").expect("message attribute")
            );
        }
    }

    fn make_reports() -> Vec<HurlResult> {
        let reports = vec![
            HurlResult {
                filename: "tests/hello.hurl".to_string(),
                entries: vec![EntryResult {
                    request: Some(Request {
                        url: "https://www.google.com/".to_string(),
                        method: "GET".to_string(),
                        headers: vec![],
                    }),
                    response: Some(Response {
                        body: vec![],
                        version: Version::Http11,
                        status: 200,
                        headers: vec![],
                        duration: Duration::from_millis(100),
                    }),
                    asserts: vec![],
                    captures: vec![],
                    errors: vec![],
                    time_in_ms: 100,
                }],
                time_in_ms: 100,
                success: true,
                cookies: vec![],
            },
            HurlResult {
                filename: "tests/failure.hurl".to_string(),
                entries: vec![EntryResult {
                    request: Some(Request {
                        url: "https://www.legiggle.com/".to_string(),
                        method: "GET".to_string(),
                        headers: vec![],
                    }),
                    response: Some(Response {
                        body: vec![],
                        version: Version::Http11,
                        status: 500,
                        headers: vec![],
                        duration: Duration::from_millis(200),
                    }),
                    asserts: vec![crate::runner::AssertResult::Status {
                        actual: 405,
                        expected: 200,
                        source_info: hurl_core::ast::SourceInfo {
                            end: Pos {
                                line: 0,
                                column: 15,
                            },
                            start: Pos {
                                line: 0,
                                column: 15,
                            },
                        },
                    }],
                    captures: vec![],
                    errors: vec![],
                    time_in_ms: 100,
                }],
                time_in_ms: 100,
                success: false,
                cookies: vec![],
            },
        ];
        return reports;
    }
}
