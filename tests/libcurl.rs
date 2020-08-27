

#[test]
fn test_easy() {
    use curl::easy::Easy;
    let url ="http://localhost:8000/hello";

    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    assert_eq!(data, b"Hello World!");

}

#[test]
fn test_hello() {
    use hurl::http::libcurl;
    use hurl::http::libcurl::core::*;

    let client = libcurl::client::Client {};
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/hello".to_string()
    };
    assert_eq!(
        client.execute(&request),
        Ok(Response {
            status: 200,
            body: b"Hello World!".to_vec(),
        })
    );
}

