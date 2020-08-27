/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
extern crate hurl;

use hurl::http;

fn default_client_options() -> http::client::ClientOptions {
    http::client::ClientOptions {
        noproxy_hosts: vec![],
        insecure: true,
        redirect: http::client::Redirect::None,
        http_proxy: None,
        https_proxy: None,
        all_proxy: None,
    }
}

#[test]
fn test_hello() {
    let client = http::client::Client::init(default_client_options());

    let request = http::request::Request {
        method: http::request::Method::Get,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/hello".to_string(),
            query_string: "".to_string(), //   querystring: None
        }, //"http://localhost:8000/hello".to_string(),
        //querystring_params: vec![],
        querystring: vec![],
        headers: vec![
            http::core::Header {
                name: String::from("User-Agent"),
                value: String::from("hurl/0.1.1"),
            },
            http::core::Header {
                name: String::from("Host"),
                value: String::from("TBD"),
            },
        ],
        cookies: vec![],
        body: vec![],
        multipart: vec![],
    };

    let result = client.execute(&request);
    println!("{:?}", result);
    let response = result.unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body.len(), 12);
    assert_eq!(
        String::from_utf8(response.body).unwrap(),
        "Hello World!".to_string()
    );
}

//#[test]
//fn test_text_utf8() {
//    let client = Client::init(ClientOptions {});
//
//    let request = Request {
//        method: Method::Get,
//        url: "http://localhost:5000/cafe".to_string(),
//        headers: vec![],
//        body: vec![],
//    };
//    let response = client.execute(&request).unwrap();
//    assert_eq!(response.status, 200);
//    assert_eq!(response.body.len(), 5);
//    assert_eq!(
//        String::from_utf8(response.body).unwrap(),
//        "café".to_string()
//    );
//}

#[cfg(test)]
fn hello_request() -> http::request::Request {
    http::request::Request {
        method: http::request::Method::Get,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/hello".to_string(),
            query_string: "".to_string(), //   querystring: None
        }, //"http://localhost:8000/hello".to_string(),
        querystring: vec![],
        headers: vec![],
        cookies: vec![],
        body: vec![],
        multipart: vec![],
    }
}

#[test]
fn test_multiple_calls() {
    let client = http::client::Client::init(default_client_options());
    let response = client.execute(&hello_request()).unwrap();
    assert_eq!(response.status, 200);
    let response = client.execute(&hello_request()).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn test_response_headers() {
    let client = http::client::Client::init(default_client_options());
    let response = client.execute(&hello_request()).unwrap();
    println!("{:?}", response);
    assert_eq!(response.status, 200);
    assert_eq!(
        response
            .get_header("Content-Length", false)
            .first()
            .unwrap(),
        "12"
    );
}

#[test]
fn test_send_cookie() {
    let client = http::client::Client::init(default_client_options());
    let request = http::request::Request {
        method: http::request::Method::Get,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/cookies/set-request-cookie1-valueA".to_string(),
            query_string: "".to_string(), //      querystring: None
        }, //"http://localhost:8000/send-cookie".to_string(),
        querystring: vec![],
        headers: vec![http::core::Header {
            name: "Cookie".to_string(),
            value: "cookie1=valueA;".to_string(),
        }],
        cookies: vec![],
        body: vec![],
        multipart: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);

    let _client = http::client::Client::init(default_client_options());
    //    let _cookie_header = http::cookie::Cookie {
    //        name: "Cookie1".to_string(),
    //        value: "valueA;".to_string(),
    //        max_age: None,
    //        domain: None,
    //        path: None,
    //    }.to_header();
    /*
    let request = Request {
        method: Method::Get,
        url: "http://localhost:5000/send-cookie1-value1".to_string(),
        headers: vec![cookie_header],
        body: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);
    */
}

#[test]
fn test_redirect() {
    let client = http::client::Client::init(default_client_options());

    let request = http::request::Request {
        method: http::request::Method::Get,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/redirect".to_string(),
            query_string: "".to_string(), //   querystring: None
        }, // "http://localhost:8000/redirect".to_string(),
        querystring: vec![],
        headers: vec![],
        cookies: vec![],
        body: vec![],
        multipart: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 302);
    assert_eq!(
        response.get_header("location", true).first().unwrap(),
        "http://localhost:8000/redirected"
    );
}

#[test]
fn test_querystring_param() {
    let client = http::client::Client::init(default_client_options());

    let request = http::request::Request {
        method: http::request::Method::Get,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/querystring-params".to_string(),
            query_string: "".to_string(), //  querystring: Some(String::from("param1=value1&param2&param3=a%3db"))
        },
        querystring: vec![
            http::core::Param {
                name: String::from("param1"),
                value: String::from("value1"),
            },
            http::core::Param {
                name: String::from("param2"),
                value: String::from(""),
            },
            http::core::Param {
                name: String::from("param3"),
                value: String::from("a=b"),
            },
            http::core::Param {
                name: String::from("param4"),
                value: String::from("1,2,3"),
            },
        ],
        headers: vec![],
        cookies: vec![],
        body: vec![],
        multipart: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
// curl -H 'Host:localhost:5000' -H 'content-type:application/x-www-form-urlencoded' -X POST 'http://localhost:5000/form-params' --data-binary 'param1=value1&param2='
fn test_form_param() {
    let client = http::client::Client::init(default_client_options());

    let request = http::request::Request {
        method: http::request::Method::Post,
        url: http::core::Url {
            scheme: "http".to_string(),
            host: "localhost".to_string(),
            port: Some(8000),
            path: "/form-params".to_string(),
            query_string: "".to_string(),
        }, // "http://localhost:8000/form-params".to_string(),
        querystring: vec![],
        headers: vec![http::core::Header {
            name: "Content-Type".to_string(),
            value: "application/x-www-form-urlencoded".to_string(),
        }],
        cookies: vec![],
        body: "param1=value1&param2=&param3=a%3db&param4=a%253db"
            .to_string()
            .into_bytes(),
        multipart: vec![],
    };
    let response = client.execute(&request).unwrap();
    assert_eq!(response.status, 200);

    /*
        let client = Client::init(ClientOptions {}); // TO BE FIXED connection ended before message read => sync wait??
        let request = Request {
            method: Method::Post,
            url: "http://localhost:5000/form-params".to_string(),
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "application/x-www-form-urlencoded".to_string(),
            }],
            body: encode_form_params(vec![
                Param {
                    name: "param1".to_string(),
                    value: "value1".to_string(),
                },
                Param {
                    name: "param2".to_string(),
                    value: "".to_string(),
                },
            ]),
        };
        let response = client.execute(&request).unwrap();
        assert_eq!(response.status, 200);
    */
}
