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

use super::core::{HttpError, Request, Response, Method};
use curl::easy::Easy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Client {}


impl Client {
    pub fn execute(&self, request: &Request) -> Result<Response, HttpError> {
        let mut handle = Easy::new();
        let mut body = Vec::<u8>::new();

        match request.method {
            Method::Get => handle.get(true).unwrap(),
            Method::Post => handle.post(true).unwrap(),
            Method::Put => handle.put(true).unwrap(),
            _ => { todo!()}
        }

        handle.url(request.url.as_str()).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                body.extend(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let status= handle.response_code().unwrap();

        Ok(Response {
            status,
            body
        })
    }
}
