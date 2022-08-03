/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use hurl::cli::Logger;
use std::default::Default;
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

fn default_get_request(url: String) -> RequestSpec {
    RequestSpec {
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
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://localhost:8000/hello".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/hello'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/hello".to_string());
    assert_eq!(request.headers.len(), 3);
    assert!(request.headers.contains(&Header {
        name: "Host".to_string(),
        value: "localhost:8000".to_string(),
    }));
    assert!(request.headers.contains(&Header {
        name: "Accept".to_string(),
        value: "*/*".to_string(),
    }));

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
    assert_eq!(response.get_header_values("Date").len(), 1);
}

// endregion

// region http method

#[test]
fn test_put() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/put' -X PUT".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "PUT".to_string());
    assert_eq!(request.url, "http://localhost:8000/put".to_string());
    assert!(request.headers.contains(&Header {
        name: "Host".to_string(),
        value: "localhost:8000".to_string(),
    }));
    assert!(request.headers.contains(&Header {
        name: "Accept".to_string(),
        value: "*/*".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_patch() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/patch/file.txt' -X PATCH -H 'Host: www.example.com' -H 'Content-Type: application/example' -H 'If-Match: \"e0023aa4e\"'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "PATCH".to_string());
    assert_eq!(
        request.url,
        "http://localhost:8000/patch/file.txt".to_string()
    );
    assert!(request.headers.contains(&Header {
        name: "Host".to_string(),
        value: "www.example.com".to_string(),
    }));
    assert!(request.headers.contains(&Header {
        name: "Content-Type".to_string(),
        value: "application/example".to_string(),
    }));

    assert_eq!(response.status, 204);
    assert!(response.body.is_empty());
}

// endregion

// region headers

#[test]
fn test_custom_headers() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/custom-headers' -H 'Fruit: Raspberry' -H 'Fruit: Apple' -H 'Fruit: Banana' -H 'Fruit: Grape' -H 'Color: Green'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(
        request.url,
        "http://localhost:8000/custom-headers".to_string()
    );
    assert!(request.headers.contains(&Header {
        name: "Fruit".to_string(),
        value: "Raspberry".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region querystrings

#[test]
fn test_querystring_params() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3'".to_string()
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3".to_string());
    assert_eq!(request.headers.len(), 3);

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region form params

#[test]
fn test_form_params() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
            Param {
                name: "values[0]".to_string(),
                value: "0".to_string(),
            },
            Param {
                name: "values[1]".to_string(),
                value: "1".to_string(),
            },
        ],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: Some("application/x-www-form-urlencoded".to_string()),
    };
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/form-params' --data 'param1=value1' --data 'param2=' --data 'param3=a%3Db' --data 'param4=a%253db' --data 'values[0]=0' --data 'values[1]=1'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "POST".to_string());
    assert_eq!(request.url, "http://localhost:8000/form-params".to_string());
    assert!(request.headers.contains(&Header {
        name: "Content-Type".to_string(),
        value: "application/x-www-form-urlencoded".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let (request, response) = client.execute(&request, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/hello".to_string());
    assert_eq!(request.headers.len(), 3);
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

// endregion

// region redirect

#[test]
fn test_redirect() {
    let request_spec = default_get_request("http://localhost:8000/redirect".to_string());
    let logger = Logger::new(false, false, "", "");
    let mut client = default_client();
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/redirect".to_string());
    assert_eq!(request.headers.len(), 3);

    assert_eq!(response.status, 302);
    assert_eq!(
        response.get_header_values("Location").get(0).unwrap(),
        "http://localhost:8000/redirected"
    );
    assert_eq!(client.redirect_count, 0);
}

#[test]
fn test_follow_location() {
    let request_spec = default_get_request("http://localhost:8000/redirect".to_string());
    let logger = Logger::new(false, false, "", "");
    let options = ClientOptions {
        follow_location: true,
        ..Default::default()
    };
    let mut client = Client::init(options);
    assert_eq!(client.options.curl_args(), vec!["-L".to_string()]);
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/redirect' -L".to_string()
    );

    let calls = client
        .execute_with_redirect(&request_spec, &logger)
        .unwrap();
    assert_eq!(calls.len(), 2);

    let (request1, response1) = calls.get(0).unwrap();
    assert_eq!(request1.method, "GET".to_string());
    assert_eq!(request1.url, "http://localhost:8000/redirect".to_string());
    assert_eq!(request1.headers.len(), 3);
    assert_eq!(response1.status, 302);
    assert!(response1.headers.contains(&Header {
        name: "Location".to_string(),
        value: "http://localhost:8000/redirected".to_string(),
    }));

    let (request2, response2) = calls.get(1).unwrap();
    assert_eq!(request2.method, "GET".to_string());
    assert_eq!(request2.url, "http://localhost:8000/redirected".to_string());
    assert_eq!(request2.headers.len(), 3);
    assert_eq!(response2.status, 200);

    assert_eq!(client.redirect_count, 1);

    // make sure that the redirect count is reset to 0
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let calls = client.execute_with_redirect(&request, &logger).unwrap();
    let (_, response) = calls.get(0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
    assert_eq!(client.redirect_count, 0);
}

#[test]
fn test_max_redirect() {
    let options = ClientOptions {
        follow_location: true,
        max_redirect: Some(10),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");

    let request_spec = default_get_request("http://localhost:8000/redirect/15".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/redirect/15' -L --max-redirs 10".to_string()
    );
    let error = client
        .execute_with_redirect(&request_spec, &logger)
        .err()
        .unwrap();
    assert_eq!(error, HttpError::TooManyRedirect);

    let request_spec = default_get_request("http://localhost:8000/redirect/8".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/redirect/8' -L --max-redirs 10".to_string()
    );
    let calls = client
        .execute_with_redirect(&request_spec, &logger)
        .unwrap();
    let (request, response) = calls.last().unwrap();
    assert_eq!(request.url, "http://localhost:8000/redirect/0".to_string());
    assert_eq!(response.status, 200);
    assert_eq!(client.redirect_count, 8);
}

// endregion

// region multipart

#[test]
fn test_multipart_form_data() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
                data: b"<div>Hello <b>World</b>!</div>".to_vec(),
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/multipart-form-data' -F 'key1=value1' -F 'upload1=@data.txt;type=text/plain' -F 'upload2=@data.html;type=text/html' -F 'upload3=@data.txt;type=text/html'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "627".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request_spec = default_get_request("http://localhost:8000/hello".to_string());
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

// endregion

// region http body

#[test]
fn test_post_bytes() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/post-base64' -H 'Content-Type: application/octet-stream' --data $'\\x48\\x65\\x6c\\x6c\\x6f\\x20\\x57\\x6f\\x72\\x6c\\x64\\x21'".to_string()
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "12".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

#[test]
fn test_expect() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/expect' -H 'Expect: 100-continue' -H 'Content-Type:' --data 'data'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Expect".to_string(),
        value: "100-continue".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert!(response.body.is_empty());
}

#[test]
fn test_basic_authentication() {
    let options = ClientOptions {
        user: Some("bob@email.com:secret".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/basic-authentication' --user 'bob@email.com:secret'"
            .to_string()
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Authorization".to_string(),
        value: "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert_eq!(response.body, b"You are authenticated".to_vec());

    let mut client = default_client();
    let request_spec = RequestSpec {
        method: Method::Get,
        url: "http://bob%40email.com:secret@localhost:8000/basic-authentication".to_string(),
        headers: vec![],
        querystring: vec![],
        form: vec![],
        multipart: vec![],
        cookies: vec![],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        request_spec.curl_args(&ContextDir::default()),
        vec!["'http://bob%40email.com:secret@localhost:8000/basic-authentication'".to_string()]
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Authorization".to_string(),
        value: "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http10);
    assert_eq!(response.body, b"You are authenticated".to_vec());
}

#[test]
fn test_cacert() {
    let options = ClientOptions {
        cacert_file: Some("tests/cert.pem".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("https://localhost:8001/hello".to_string());
    let (_, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(response.status, 200);
}

// region error

#[test]
fn test_error_could_not_resolve_host() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request = default_get_request("http://unknown".to_string());
    let error = client.execute(&request, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 6);
        assert_eq!(description, "Could not resolve host: unknown");
        assert_eq!(url, "http://unknown");
    }
}

#[test]
fn test_error_fail_to_connect() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://localhost:9999".to_string());
    let error = client.execute(&request_spec, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 7);
        assert!(description.starts_with("Failed to connect to localhost port 9999"));
        assert_eq!(url, "http://localhost:9999");
    }

    let options = ClientOptions {
        proxy: Some("localhost:9999".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let request = default_get_request("http://localhost:8000/hello".to_string());
    let error = client.execute(&request, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 7);
        eprintln!("description={}", description);
        assert!(description.starts_with("Failed to connect to localhost port 9999"));
        assert_eq!(url, "http://localhost:8000/hello");
    }
}

#[test]
fn test_error_could_not_resolve_proxy_name() {
    let options = ClientOptions {
        proxy: Some("unknown".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://localhost:8000/hello".to_string());
    let error = client.execute(&request_spec, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 5);
        assert_eq!(description, "Could not resolve proxy: unknown");
        assert_eq!(url, "http://localhost:8000/hello");
    }
}

#[test]
fn test_error_ssl() {
    let options = ClientOptions::default();
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("https://localhost:8001/hello".to_string());
    let error = client.execute(&request_spec, &logger).err().unwrap();
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 60);
        let descriptions = [
            // Windows messages:
            "schannel: SEC_E_UNTRUSTED_ROOT (0x80090325) - The certificate chain was issued by an authority that is not trusted.".to_string(),
            // Unix-like, before OpenSSL 3.0.0
            "SSL certificate problem: self signed certificate".to_string(),
            // Unix-like, after OpenSSL 3.0.0
            "SSL certificate problem: self-signed certificate".to_string(),
        ];
        assert!(descriptions.contains(&description));
        assert_eq!(url, "https://localhost:8001/hello");
    }
}

#[test]
fn test_timeout() {
    let options = ClientOptions {
        timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://localhost:8000/timeout".to_string());
    let error = client.execute(&request_spec, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 28);
        assert!(description.starts_with("Operation timed out after "));
        assert_eq!(url, "http://localhost:8000/timeout");
    }
}

#[test]
fn test_accept_encoding() {
    let options = ClientOptions {
        compressed: true,
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");

    let request_spec = RequestSpec {
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
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Accept-Encoding".to_string(),
        value: "gzip, deflate, br".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "32".to_string(),
    }));
}

#[test]
fn test_connect_timeout() {
    let options = ClientOptions {
        connect_timeout: Duration::from_secs(1),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://10.0.0.0".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://10.0.0.0' --connect-timeout 1".to_string()
    );
    let error = client.execute(&request_spec, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        eprintln!("description={}", description);
        // TODO: remove the 7 / "Couldn't connect to server" case
        // On Linux/Windows libcurl version, the correct error message
        // is 28 / "Connection timeout" | "Connection timed out"
        // On macOS <= 11.6.4, the built-in libcurl use
        // 7 / "Couldn't connect to server" errors. On the GitHub CI, macOS images are 11.6.4.
        // So we keep this code until a newer macOS image is used in the GitHub actions.
        assert!(code == 7 || code == 28);
        assert!(
            description.starts_with("Couldn't connect to server")
                || description.starts_with("Connection timed out")
                || description.starts_with("Connection timeout")
        );
        assert_eq!(url, "http://10.0.0.0");
    }
}
// endregion

// region cookie

#[test]
fn test_cookie() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/cookies/set-request-cookie1-valueA' --cookie 'cookie1=valueA'"
            .to_string()
    );

    //assert_eq!(request.cookies(), vec!["cookie1=valueA".to_string(),]);

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "cookie1=valueA".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    let request_spec = RequestSpec {
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
    let (_request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn test_multiple_request_cookies() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec = RequestSpec {
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
            RequestCookie {
                name: "user3".to_string(),
                value: "Bruce".to_string(),
            },
        ],
        body: Body::Binary(vec![]),
        content_type: None,
    };
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/cookies/set-multiple-request-cookies' --cookie 'user1=Bob; user2=Bill; user3=Bruce'".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "user1=Bob; user2=Bill; user3=Bruce".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_cookie_storage() {
    let mut client = default_client();
    let logger = Logger::new(false, false, "", "");
    let request_spec =
        default_get_request("http://localhost:8000/cookies/set-session-cookie2-valueA".to_string());
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(
        request.url,
        "http://localhost:8000/cookies/set-session-cookie2-valueA".to_string()
    );
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
            http_only: false,
        }
    );

    let request_spec = default_get_request(
        "http://localhost:8000/cookies/assert-that-cookie2-is-valueA".to_string(),
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "cookie2=valueA".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_cookie_file() {
    let options = ClientOptions {
        cookie_input_file: Some("tests/cookies.txt".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request(
        "http://localhost:8000/cookies/assert-that-cookie2-is-valueA".to_string(),
    );
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/cookies/assert-that-cookie2-is-valueA' --cookie tests/cookies.txt".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(
        request.url,
        "http://localhost:8000/cookies/assert-that-cookie2-is-valueA"
    );
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "cookie2=valueA".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

// endregion

// region proxy

#[test]
fn test_proxy() {
    // mitmproxy listening on port 8888
    let options = ClientOptions {
        proxy: Some("localhost:8888".to_string()),
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    let request_spec = default_get_request("http://localhost:8000/proxy".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'http://localhost:8000/proxy' --proxy 'localhost:8888'".to_string()
    );
    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.url, "http://localhost:8000/proxy");
    assert_eq!(response.status, 200);
}

// endregion

#[test]
fn test_insecure() {
    let options = ClientOptions {
        insecure: true,
        ..Default::default()
    };
    let mut client = Client::init(options);
    let logger = Logger::new(false, false, "", "");
    assert_eq!(client.options.curl_args(), vec!["--insecure".to_string()]);
    let request_spec = default_get_request("https://localhost:8001/hello".to_string());
    assert_eq!(
        client.curl_command_line(&request_spec),
        "curl 'https://localhost:8001/hello' --insecure".to_string()
    );

    let (request, response) = client.execute(&request_spec, &logger).unwrap();
    assert_eq!(request.url, "https://localhost:8001/hello");
    assert_eq!(response.status, 200);
}

#[test]
fn test_version() {
    // This test if only informative for the time-being

    let output = std::process::Command::new("curl")
        .args(["--version"])
        .output()
        .expect("failed to execute process");
    let curl_version = std::str::from_utf8(&output.stdout).unwrap();
    let index = curl_version.find("libcurl").expect("libcurl substring");
    let expected_version = &curl_version[index..];
    eprintln!("{:?}", expected_version);
    let versions = libcurl_version_info();
    eprintln!("{:?}", versions);
}

// This test function can be used to reproduce bug
#[test]
fn test_libcurl_directly() {
    use curl;
    use std::io::{stdout, Write};

    let mut easy = curl::easy::Easy::new();
    easy.url("http://localhost:8000/hello").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
