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
use crate::output::Error;
#[cfg(target_family = "windows")]
use atty::Stream;
use std::io;
use std::io::Write;

#[cfg(target_family = "unix")]
pub(crate) fn write_stdout(buf: &[u8]) -> Result<(), Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buf).map_err(|_| Error {
        message: "Error writing output".to_string(),
    })
}

#[cfg(target_family = "windows")]
pub(crate) fn write_stdout(buf: &[u8]) -> Result<(), Error> {
    if atty::is(Stream::Stdout) {
        println!("{}", String::from_utf8_lossy(buf));
        Ok(())
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(buf).map_err(|_| Error {
            message: "Error writing output".to_string(),
        })
    }
}
