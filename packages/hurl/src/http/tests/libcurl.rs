/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use std::default::Default;
use std::time::Duration;

use regex::Regex;

use crate::http::*;
use crate::util::logger::LoggerBuilder;
use crate::util::path::ContextDir;

fn default_get_request(url: &str) -> RequestSpec {
    RequestSpec {
        url: url.to_string(),
        ..Default::default()
    }
}

fn redirect_count(calls: &[Call]) -> usize {
    calls
        .iter()
        .filter(|call| call.response.status >= 300 && call.response.status < 399)
        .count()
}

#[test]
fn test_hello() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:8000/hello");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl 'http://localhost:8000/hello'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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

    assert_eq!(response.version, Version::Http11);
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());

    assert_eq!(response.headers.len(), 6);
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

#[test]
fn test_put() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();
    let request_spec = RequestSpec {
        method: Method::Put,
        url: "http://localhost:8000/put".to_string(),
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --request PUT 'http://localhost:8000/put'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();
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
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --request PATCH --header 'Host: www.example.com' --header 'Content-Type: application/example' --header 'If-Match: \"e0023aa4e\"' 'http://localhost:8000/patch/file.txt'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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

#[test]
fn test_custom_headers() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Get,
        url: "http://localhost:8000/custom-headers".to_string(),
        headers: vec![
            Header::new("Fruit", "Raspberry"),
            Header::new("Fruit", "Apple"),
            Header::new("Fruit", "Banana"),
            Header::new("Fruit", "Grape"),
            Header::new("Color", "Green"),
        ],
        ..Default::default()
    };
    assert!(options.curl_args().is_empty());
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --header 'Fruit: Raspberry' --header 'Fruit: Apple' --header 'Fruit: Banana' --header 'Fruit: Grape' --header 'Color: Green' 'http://localhost:8000/custom-headers'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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

#[test]
fn test_querystring_params() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Get,
        url: "http://localhost:8000/querystring-params".to_string(),
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
            Param {
                name: "$top".to_string(),
                value: "5".to_string(),
            },
        ],
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl 'http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3&$top=5'".to_string()
    );
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3&$top=5".to_string());
    assert_eq!(request.headers.len(), 3);

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_form_params() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Post,
        url: "http://localhost:8000/form-params".to_string(),
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
        content_type: Some("application/x-www-form-urlencoded".to_string()),
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --data 'param1=value1' --data 'param2=' --data 'param3=a%3Db' --data 'param4=a%253db' --data 'values[0]=0' --data 'values[1]=1' 'http://localhost:8000/form-params'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.method, "POST".to_string());
    assert_eq!(request.url, "http://localhost:8000/form-params".to_string());
    assert!(request.headers.contains(&Header {
        name: "Content-Type".to_string(),
        value: "application/x-www-form-urlencoded".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request = default_get_request("http://localhost:8000/hello");
    let call = client.execute(&request, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(request.url, "http://localhost:8000/hello".to_string());
    assert_eq!(request.headers.len(), 3);
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

#[test]
fn test_redirect() {
    let request_spec = default_get_request("http://localhost:8000/redirect-absolute");
    let logger = LoggerBuilder::new().build();

    let options = ClientOptions::default();
    let mut client = Client::new(None);
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(
        request.url,
        "http://localhost:8000/redirect-absolute".to_string()
    );
    assert_eq!(request.headers.len(), 3);

    assert_eq!(response.status, 302);
    assert_eq!(
        response.get_header_values("Location").get(0).unwrap(),
        "http://localhost:8000/redirected"
    );
    assert_eq!(
        response.url,
        "http://localhost:8000/redirect-absolute".to_string()
    );
}

#[test]
fn test_follow_location() {
    let request_spec = default_get_request("http://localhost:8000/redirect-absolute");
    let logger = LoggerBuilder::new().build();

    let options = ClientOptions {
        follow_location: true,
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    assert_eq!(options.curl_args(), vec!["--location".to_string()]);
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --location 'http://localhost:8000/redirect-absolute'".to_string()
    );

    let calls = client
        .execute_with_redirect(&request_spec, &options, &logger)
        .unwrap();
    assert_eq!(calls.len(), 2);

    let call1 = calls.get(0).unwrap();
    let request1 = &call1.request;
    let response1 = &call1.response;
    assert_eq!(request1.method, "GET".to_string());
    assert_eq!(
        request1.url,
        "http://localhost:8000/redirect-absolute".to_string()
    );
    assert_eq!(request1.headers.len(), 3);
    assert_eq!(response1.status, 302);
    assert!(response1.headers.contains(&Header {
        name: "Location".to_string(),
        value: "http://localhost:8000/redirected".to_string(),
    }));
    assert_eq!(
        response1.url,
        "http://localhost:8000/redirect-absolute".to_string()
    );

    let call2 = calls.get(1).unwrap();
    let request2 = &call2.request;
    let response2 = &call2.response;
    assert_eq!(request2.method, "GET".to_string());
    assert_eq!(request2.url, "http://localhost:8000/redirected".to_string());
    assert_eq!(request2.headers.len(), 3);
    assert_eq!(response2.status, 200);
    assert_eq!(
        response2.url,
        "http://localhost:8000/redirected".to_string()
    );

    let request = default_get_request("http://localhost:8000/hello");
    let calls = client
        .execute_with_redirect(&request, &options, &logger)
        .unwrap();
    assert_eq!(calls.len(), 1);
    let Call { response, .. } = calls.get(0).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

#[test]
fn test_max_redirect() {
    let options = ClientOptions {
        follow_location: true,
        max_redirect: Some(10),
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:8000/redirect/15");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --location --max-redirs 10 'http://localhost:8000/redirect/15'".to_string()
    );
    let error = client
        .execute_with_redirect(&request_spec, &options, &logger)
        .err()
        .unwrap();
    assert_eq!(error, HttpError::TooManyRedirect);

    let request_spec = default_get_request("http://localhost:8000/redirect/8");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --location --max-redirs 10 'http://localhost:8000/redirect/8'".to_string()
    );
    let calls = client
        .execute_with_redirect(&request_spec, &options, &logger)
        .unwrap();
    let call = calls.last().unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.url, "http://localhost:8000/redirect/0".to_string());
    assert_eq!(response.status, 200);
    assert_eq!(redirect_count(&calls), 8);
}

#[test]
fn test_multipart_form_data() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Post,
        url: "http://localhost:8000/multipart-form-data".to_string(),
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
        content_type: Some("multipart/form-data".to_string()),
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --form 'key1=value1' --form 'upload1=@data.txt;type=text/plain' --form 'upload2=@data.html;type=text/html' --form 'upload3=@data.txt;type=text/html' 'http://localhost:8000/multipart-form-data'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "627".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    // make sure you can reuse client for other request
    let request_spec = default_get_request("http://localhost:8000/hello");
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.method, "GET".to_string());
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"Hello World!".to_vec());
}

#[test]
fn test_post_bytes() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

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
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --header 'Content-Type: application/octet-stream' --data $'\\x48\\x65\\x6c\\x6c\\x6f\\x20\\x57\\x6f\\x72\\x6c\\x64\\x21' 'http://localhost:8000/post-base64'".to_string()
    );
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "12".to_string(),
    }));

    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_expect() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

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
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --header 'Expect: 100-continue' --header 'Content-Type:' --data 'data' 'http://localhost:8000/expect'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Expect".to_string(),
        value: "100-continue".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http11);
    assert!(response.body.is_empty());
}

#[test]
fn test_basic_authentication() {
    let options = ClientOptions {
        user: Some("bob@email.com:secret".to_string()),
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:8000/basic-authentication");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --user 'bob@email.com:secret' 'http://localhost:8000/basic-authentication'"
            .to_string()
    );
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Authorization".to_string(),
        value: "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http11);
    assert_eq!(response.body, b"You are authenticated".to_vec());

    let options = ClientOptions::default();
    let mut client = Client::new(None);
    let request_spec =
        default_get_request("http://bob%40email.com:secret@localhost:8000/basic-authentication");
    assert_eq!(
        request_spec.curl_args(&ContextDir::default()),
        vec!["'http://bob%40email.com:secret@localhost:8000/basic-authentication'".to_string()]
    );
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Authorization".to_string(),
        value: "Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert_eq!(response.version, Version::Http11);
    assert_eq!(response.body, b"You are authenticated".to_vec());
}

#[test]
fn test_cacert() {
    let options = ClientOptions {
        cacert_file: Some("tests/server_cert_selfsigned.pem".to_string()),
        ..Default::default()
    };
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("https://localhost:8001/hello");
    let Call { response, .. } = client.execute(&request_spec, &options, &logger).unwrap();
    assert_eq!(response.status, 200);

    let certificate = response.certificate.unwrap();

    assert_eq!(
        certificate.issuer,
        "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost".to_string()
    );
    assert_eq!(
        certificate.subject,
        "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost".to_string()
    );
    assert_eq!(
        certificate.start_date,
        chrono::DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
            .unwrap()
            .with_timezone(&chrono::Utc)
    );
    assert_eq!(
        certificate.expire_date,
        chrono::DateTime::parse_from_rfc2822("Thu, 30 Oct 2025 08:29:52 GMT")
            .unwrap()
            .with_timezone(&chrono::Utc)
    );
    assert_eq!(
        certificate.serial_number,
        "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0".to_string()
    );
}

#[test]
fn test_error_could_not_resolve_host() {
    let options = ClientOptions::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://unknown");
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
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
    let options = ClientOptions::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:9999");
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
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
    let mut client = Client::new(None);
    let request = default_get_request("http://localhost:8000/hello");
    let error = client.execute(&request, &options, &logger).err().unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        assert_eq!(code, 7);
        eprintln!("description={description}");
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
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:8000/hello");
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
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
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("https://localhost:8001/hello");
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        // libcurl on linux and mac exists with 60
        // libcurl with openssl3 feature built by vcpkg on x64-windows exists with 35
        assert_eq!(code, 60);
        let descriptions = [

            // Windows 2000 github runner messages:
            "schannel: SEC_E_UNTRUSTED_ROOT (0x80090325) - The certificate chain was issued by an authority that is not trusted.".to_string(),
            // Windows 10 Enterprise 2009 10.0.19041.1806
            "schannel: SEC_E_UNTRUSTED_ROOT (0x80090325)".to_string(),
            // Unix-like, before OpenSSL 3.0.0
            "SSL certificate problem: self signed certificate in certificate chain".to_string(),
            // Unix-like, after OpenSSL 3.0.0
            "SSL certificate problem: self-signed certificate".to_string(),
            "SSL certificate problem: self signed certificate".to_string(),
        ];
        assert!(
            descriptions.contains(&description),
            "actual description is {description}"
        );
        assert_eq!(url, "https://localhost:8001/hello");
    }
}

#[test]
fn test_timeout() {
    let options = ClientOptions {
        timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://localhost:8000/timeout");
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
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
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();
    let request_spec = default_get_request("http://localhost:8000/compressed/gzip");
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://10.0.0.0");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --connect-timeout 1 'http://10.0.0.0'".to_string()
    );
    let error = client
        .execute(&request_spec, &options, &logger)
        .err()
        .unwrap();
    assert!(matches!(error, HttpError::Libcurl { .. }));
    if let HttpError::Libcurl {
        code,
        description,
        url,
    } = error
    {
        eprintln!("description={description}");
        // TODO: remove the 7 / "Couldn't connect to server" case
        // On Linux/Windows libcurl version, the correct error message is:
        // `CURLE_OPERATION_TIMEDOUT` (28) / "Connection timeout" | "Connection timed out"
        // On macOS <= 11.6.4, the built-in libcurl is:
        // `CURLE_COULDNT_CONNECT` (7) / "Couldn't connect to server" errors.
        // On the GitHub CI, macOS images are 11.6.4.
        // So we keep this code until a newer macOS image is used in the GitHub actions.
        assert!(code == 7 || code == 28);
        let re = Regex::new(r"^Failed to connect to.*: Timeout was reached$").unwrap();
        assert!(
            re.is_match(&description)
                || description.starts_with("Couldn't connect to server")
                || description.starts_with("Connection timed out")
                || description.starts_with("Connection timeout")
        );
        assert_eq!(url, "http://10.0.0.0");
    }
}

#[test]
fn test_cookie() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Get,
        url: "http://localhost:8000/cookies/set-request-cookie1-valueA".to_string(),
        cookies: vec![RequestCookie {
            name: "cookie1".to_string(),
            value: "valueA".to_string(),
        }],
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --cookie 'cookie1=valueA' 'http://localhost:8000/cookies/set-request-cookie1-valueA'"
            .to_string()
    );

    //assert_eq!(request.cookies(), vec!["cookie1=valueA".to_string(),]);

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "cookie1=valueA".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());

    let request_spec =
        default_get_request("http://localhost:8000/cookies/assert-that-cookie1-is-not-in-session");
    let Call { response, .. } = client.execute(&request_spec, &options, &logger).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn test_multiple_request_cookies() {
    let options = ClientOptions::default();
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Get,
        url: "http://localhost:8000/cookies/set-multiple-request-cookies".to_string(),
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
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --cookie 'user1=Bob; user2=Bill; user3=Bruce' 'http://localhost:8000/cookies/set-multiple-request-cookies'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert!(request.headers.contains(&Header {
        name: "Cookie".to_string(),
        value: "user1=Bob; user2=Bill; user3=Bruce".to_string(),
    }));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn test_cookie_storage() {
    let options = ClientOptions::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec =
        default_get_request("http://localhost:8000/cookies/set-session-cookie2-valueA");
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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

    let request_spec =
        default_get_request("http://localhost:8000/cookies/assert-that-cookie2-is-valueA");
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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
    let context_dir = ContextDir::default();
    let mut client = Client::new(Some("tests/cookies.txt".to_string()));
    let logger = LoggerBuilder::new().build();

    let request_spec =
        default_get_request("http://localhost:8000/cookies/assert-that-cookie2-is-valueA");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --cookie tests/cookies.txt 'http://localhost:8000/cookies/assert-that-cookie2-is-valueA'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
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

#[test]
fn test_proxy() {
    // roxy listening on port 3128
    let options = ClientOptions {
        proxy: Some("localhost:3128".to_string()),
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = default_get_request("http://127.0.0.1:8000/proxy");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --proxy 'localhost:3128' 'http://127.0.0.1:8000/proxy'".to_string()
    );
    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.url, "http://127.0.0.1:8000/proxy");
    assert_eq!(response.status, 200);
}

#[test]
fn test_insecure() {
    let options = ClientOptions {
        insecure: true,
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();
    assert_eq!(options.curl_args(), vec!["--insecure".to_string()]);
    let request_spec = default_get_request("https://localhost:8001/hello");
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --insecure 'https://localhost:8001/hello'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.url, "https://localhost:8001/hello");
    assert_eq!(response.status, 200);
}

#[test]
fn test_head() {
    let options = ClientOptions {
        ..Default::default()
    };
    let context_dir = ContextDir::default();
    let mut client = Client::new(None);
    let logger = LoggerBuilder::new().build();

    let request_spec = RequestSpec {
        method: Method::Head,
        url: "http://localhost:8000/head".to_string(),
        ..Default::default()
    };
    assert_eq!(
        client.curl_command_line(&request_spec, &context_dir, &options),
        "curl --head 'http://localhost:8000/head'".to_string()
    );

    let call = client.execute(&request_spec, &options, &logger).unwrap();
    let request = &call.request;
    let response = &call.response;
    assert_eq!(request.url, "http://localhost:8000/head");
    assert_eq!(response.status, 200);
    assert!(response.headers.contains(&Header {
        name: "Content-Length".to_string(),
        value: "10".to_string(),
    }));
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
    eprintln!("{expected_version:?}");
    let versions = libcurl_version_info();
    eprintln!("{versions:?}");
}

// This test function can be used to reproduce bug
#[test]
fn test_libcurl_directly() {
    use std::io::{stdout, Write};

    use curl;

    let mut easy = curl::easy::Easy::new();
    easy.url("http://localhost:8000/hello").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
