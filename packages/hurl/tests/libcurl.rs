use std::time::Duration;

use hurl::http::*;

pub fn new_header(name: &str, value: &str) -> Header {
    Header {
        name: name.to_string(),
        value: value.to_string(),
    }
}

fn default_client() -> Client {
    let options = ClientOptions::default();
    Client::init(options)
}

fn default_get_request(url: String) -> Request {
    Request {
        method: Method::Get,
        url,
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    }
}

// region basic

#[test]
fn test_hello() {
    let mut client = default_client();
    let request = default_get_request("http://localhost:8000/hello".to_string());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/hello'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.version, Version::Http10);
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());

    assert_eq!(response.headers.len(), 4);
    assert!(response.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "12".to_string(),
    }));
    assert!(response.headers.contains(&Header {
        name: "Content-Type".to_string(),
        value: "text/html; charset=utf-8".to_string(),
    }));
    assert_eq!(response.get_header_values("Date".to_string()).len(), 1);
}

// endregion

// region http method

#[test]
fn test_put() {
    let mut client = default_client();
    let request = Request {
        method: Method::Put,
        url: "http://localhost:8000/put".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/put' -X PUT".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_patch() {
    let mut client = default_client();
    let request = Request {
        method: Method::Patch,
        url: "http://localhost:8000/patch/file.txt".to_string(),
        headers: vec![
            Header {
                name: "Host".to_string(),
                value: "www.example.com".to_string(),
            },
            Header {
                name: "Content-Type".to_string(),
                value: "application/example".to_string(),
            },
            Header {
                name: "If-Match".to_string(),
                value: "\"e0023aa4e\"".to_string(),
            },
        ],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/patch/file.txt' -X PATCH -H 'Host: www.example.com' -H 'Content-Type: application/example' -H 'If-Match: \"e0023aa4e\"'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 204);
    assert!(response.body.is_empty());
}

// endregion

// region headers

#[test]
fn test_custom_headers() {
    let mut client = default_client();
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/custom-headers".to_string(),
        headers: vec![
            new_header("Fruit", "Raspberry"),
            new_header("Fruit", "Apple"),
            new_header("Fruit", "Banana"),
            new_header("Fruit", "Grape"),
            new_header("Color", "Green"),
        ],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert!(client.options.curl_args().is_empty());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/custom-headers' -H 'Fruit: Raspberry' -H 'Fruit: Apple' -H 'Fruit: Banana' -H 'Fruit: Grape' -H 'Color: Green'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region querystrings

#[test]
fn test_querystring_params() {
    let mut client = default_client();
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/querystring-params".to_string(),
        headers: vec![],
        querystring: vec![
            Param {
                name: "param1".to_string(),
                value: "value1".to_string(),
            },
            Param {
                name: "param2".to_string(),
                value: "".to_string(),
            },
            Param {
                name: "param3".to_string(),
                value: "a=b".to_string(),
            },
            Param {
                name: "param4".to_string(),
                value: "1,2,3".to_string(),
            },
        ],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3'".to_string()
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region form params

#[test]
fn test_form_params() {
    let mut client = default_client();
    let request = Request {
        method: Method::Post,
        url: "http://localhost:8000/form-params".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![
            Param {
                name: "param1".to_string(),
                value: "value1".to_string(),
            },
            Param {
                name: "param2".to_string(),
                value: "".to_string(),
            },
            Param {
                name: "param3".to_string(),
                value: "a=b".to_string(),
            },
            Param {
                name: "param4".to_string(),
                value: "a%3db".to_string(),
            },
        ],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: Some("application/x-www-form-urlencoded".to_string()),
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/form-params' --data 'param1=value1' --data 'param2=' --data 'param3=a%3Db' --data 'param4=a%253db'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

// endregion

// region redirect

#[test]
fn test_follow_location() {
    let request = default_get_request("http://localhost:8000/redirect".to_string());

    let mut client = default_client();
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 302);
    assert_eq!(
        response
            .get_header_values("Location".to_string())
            .get(0)
            .unwrap(),
        "http://localhost:8000/redirected"
    );
    assert_eq!(client.redirect_count, 0);

    let options = ClientOptions {
        follow_location: true,
        max_redirect: Some(50),
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::new(300, 0),
        connect_timeout: Duration::new(300, 0),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    assert_eq!(client.options.curl_args(), vec!["-L".to_string(),]);
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/redirect' -L".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(
        response
            .get_header_values("Content-Length".to_string())
            .get(0)
            .unwrap(),
        "0"
    );
    assert_eq!(client.redirect_count, 1);

    // make sure that the redirect count is reset to 0
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
    assert_eq!(client.redirect_count, 0);
}

#[test]
fn test_max_redirect() {
    let options = ClientOptions {
        follow_location: true,
        max_redirect: Some(10),
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::new(300, 0),
        connect_timeout: Duration::new(300, 0),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/redirect".to_string());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/redirect' -L --max-redirs 10".to_string()
    );

    let response = client.execute(&request, 5).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(client.redirect_count, 6);

    let error = client.execute(&request, 11).err().unwrap();
    assert_eq!(error, HttpError::TooManyRedirect);
}

// endregion

// region multipart

#[test]
fn test_multipart_form_data() {
    let mut client = default_client();
    let request = Request {
        method: Method::Post,
        url: "http://localhost:8000/multipart-form-data".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![
            MultipartParam::Param(Param {
                name: "key1".to_string(),
                value: "value1".to_string(),
            }),
            MultipartParam::FileParam(FileParam {
                name: "upload1".to_string(),
                filename: "data.txt".to_string(),
                data: b"Hello World!".to_vec(),
                content_type: "text/plain".to_string(),
            }),
            MultipartParam::FileParam(FileParam {
                name: "upload2".to_string(),
                filename: "data.html".to_string(),
                data: b"Hello <b>World</b>!".to_vec(),
                content_type: "text/html".to_string(),
            }),
            MultipartParam::FileParam(FileParam {
                name: "upload3".to_string(),
                filename: "data.txt".to_string(),
                data: b"Hello World!".to_vec(),
                content_type: "text/html".to_string(),
            }),
        ],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: Some("multipart/form-data".to_string()),
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/multipart-form-data' -F 'key1=value1' -F 'upload1=@data.txt;type=text/plain' -F 'upload2=@data.html;type=text/html' -F 'upload3=@data.txt;type=text/html'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

// endregion

// region http body

#[test]
fn test_post_bytes() {
    let mut client = default_client();
    let request = Request {
        method: Method::Post,
        url: "http://localhost:8000/post-base64".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(b"Hello World!".to_vec()),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/post-base64' -H 'Content-Type: application/octet-stream' --data $'\\x48\\x65\\x6c\\x6c\\x6f\\x20\\x57\\x6f\\x72\\x6c\\x64\\x21'".to_string()
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

#[test]
fn test_expect() {
    let mut client = default_client();
    let request = Request {
        method: Method::Post,
        url: "http://localhost:8000/expect".to_string(),
        headers: vec![Header {
            name: "Expect".to_string(),
            value: "100-continue".to_string(),
        }],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Text("data".to_string()),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/expect' -H 'Expect: 100-continue' -H 'Content-Type:' --data 'data'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert!(response.body.is_empty());
}

#[test]
fn test_basic_authentication() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: Some(50),
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::from_secs(300),
        connect_timeout: Duration::from_secs(300),
        user: Some("bob:secret".to_string()),
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/basic-authentication".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/basic-authentication' --user 'bob:secret'".to_string()
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert_eq!(response.body, b"You are authenticated".to_vec());

    let mut client = default_client();
    let request = Request {
        method: Method::Get,
        url: "http://bob:secret@localhost:8000/basic-authentication".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        request.curl_args(".".to_string()),
        vec!["'http://bob:secret@localhost:8000/basic-authentication'".to_string()]
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert_eq!(response.body, b"You are authenticated".to_vec());
}

// region error

#[test]
fn test_error_could_not_resolve_host() {
    let mut client = default_client();
    let request = default_get_request("http://unknown".to_string());
    let error = client.execute(&request, 0).err().unwrap();

    assert_eq!(error, HttpError::CouldNotResolveHost);
}

#[test]
fn test_error_fail_to_connect() {
    let mut client = default_client();
    let request = default_get_request("http://localhost:9999".to_string());
    let error = client.execute(&request, 0).err().unwrap();
    assert_eq!(error, HttpError::FailToConnect);

    let options = ClientOptions {
        follow_location: false,
        max_redirect: None,
        cookie_input_file: None,
        proxy: Some("localhost:9999".to_string()),
        no_proxy: None,
        verbose: true,
        insecure: false,
        timeout: Default::default(),
        connect_timeout: Default::default(),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let error = client.execute(&request, 0).err().unwrap();
    assert_eq!(error, HttpError::FailToConnect);
}

#[test]
fn test_error_could_not_resolve_proxy_name() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: None,
        cookie_input_file: None,
        proxy: Some("unknown".to_string()),
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Default::default(),
        connect_timeout: Default::default(),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let error = client.execute(&request, 0).err().unwrap();
    assert_eq!(error, HttpError::CouldNotResolveProxyName);
}

#[test]
fn test_error_ssl() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: None,
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Default::default(),
        connect_timeout: Default::default(),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("https://localhost:8001/hello".to_string());
    let error = client.execute(&request, 0).err().unwrap();
    let message = if cfg!(windows) {
        "schannel: SEC_E_UNTRUSTED_ROOT (0x80090325) - The certificate chain was issued by an authority that is not trusted.".to_string()
    } else {
        "SSL certificate problem: self signed certificate".to_string()
    };
    assert_eq!(error, HttpError::SslCertificate(Some(message)));
}

#[test]
fn test_timeout() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: None,
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::from_millis(100),
        connect_timeout: Default::default(),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/timeout".to_string());
    let error = client.execute(&request, 0).err().unwrap();
    assert_eq!(error, HttpError::Timeout);
}

#[test]
fn test_accept_encoding() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: None,
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: true,
        insecure: false,
        timeout: Default::default(),
        connect_timeout: Default::default(),
        user: None,
        compressed: true,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);

    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/compressed/gzip".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "32".to_string(),
    }));
}

#[test]
fn test_connect_timeout() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: Some(50),
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::from_secs(300),
        connect_timeout: Duration::from_secs(1),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://10.0.0.0".to_string());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://10.0.0.0' --connect-timeout 1".to_string()
    );
    let error = client.execute(&request, 0).err().unwrap();
    if cfg!(target_os = "macos") {
        assert_eq!(error, HttpError::FailToConnect);
    } else {
        assert_eq!(error, HttpError::Timeout);
    }
}
// endregion

// region cookie

#[test]
fn test_cookie() {
    let mut client = default_client();
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/cookies/set-request-cookie1-valueA".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![RequestCookie {
            name: "cookie1".to_string(),
            value: "valueA".to_string(),
        }],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/cookies/set-request-cookie1-valueA' --cookie 'cookie1=valueA'"
            .to_string()
    );

    //assert_eq!(request.cookies(), vec!["cookie1=valueA".to_string(),]);

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/cookies/assert-that-cookie1-is-not-in-session".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn test_multiple_request_cookies() {
    let mut client = default_client();
    let request = Request {
        method: Method::Get,
        url: "http://localhost:8000/cookies/set-multiple-request-cookies".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![
            RequestCookie {
                name: "user1".to_string(),
                value: "Bob".to_string(),
            },
            RequestCookie {
                name: "user2".to_string(),
                value: "Bill".to_string(),
            },
        ],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/cookies/set-multiple-request-cookies' --cookie 'user1=Bob; user2=Bill'".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_cookie_storage() {
    let mut client = default_client();
    let request =
        default_get_request("http://localhost:8000/cookies/set-session-cookie2-valueA".to_string());
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    let cookie_store = client.get_cookie_storage();
    assert_eq!(
        cookie_store.get(0).unwrap().clone(),
        Cookie {
            domain: "localhost".to_string(),
            include_subdomain: "FALSE".to_string(),
            path: "/".to_string(),
            https: "FALSE".to_string(),
            expires: "0".to_string(),
            name: "cookie2".to_string(),
            value: "valueA".to_string(),
            http_only: false
        }
    );

    let request = default_get_request(
        "http://localhost:8000/cookies/assert-that-cookie2-is-valueA".to_string(),
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_cookie_file() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: Some(50),
        cookie_input_file: Some("tests/cookies.txt".to_string()),
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::new(300, 0),
        connect_timeout: Duration::new(300, 0),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request(
        "http://localhost:8000/cookies/assert-that-cookie2-is-valueA".to_string(),
    );
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/cookies/assert-that-cookie2-is-valueA' --cookie tests/cookies.txt".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region proxy

#[test]
fn test_proxy() {
    // mitmproxy listening on port 8888
    let options = ClientOptions {
        follow_location: false,
        max_redirect: Some(50),
        cookie_input_file: None,
        proxy: Some("localhost:8888".to_string()),
        no_proxy: None,
        verbose: false,
        insecure: false,
        timeout: Duration::new(300, 0),
        connect_timeout: Duration::new(300, 0),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/proxy".to_string());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'http://localhost:8000/proxy' --proxy 'localhost:8888'".to_string()
    );
    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
}

// endregion

#[test]
fn test_insecure() {
    let options = ClientOptions {
        follow_location: false,
        max_redirect: Some(50),
        cookie_input_file: None,
        proxy: None,
        no_proxy: None,
        verbose: false,
        insecure: true,
        timeout: Duration::new(300, 0),
        connect_timeout: Duration::new(300, 0),
        user: None,
        compressed: false,
        context_dir: ".".to_string(),
    };
    let mut client = Client::init(options);
    assert_eq!(client.options.curl_args(), vec!["--insecure".to_string()]);
    let request = default_get_request("https://localhost:8001/hello".to_string());
    assert_eq!(
        client.curl_command_line(&request),
        "curl 'https://localhost:8001/hello' --insecure".to_string()
    );

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);

    let response = client.execute(&request, 0).unwrap();
    assert_eq!(response.status, 200);
}
