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
use clap::ArgAction;

pub fn headers() -> clap::Arg {
    clap::Arg::new("headers")
        .long("header")
        .short('H')
        .value_name("NAME:VALUE")
        .action(ArgAction::Append)
        .num_args(1)
}

pub fn method() -> clap::Arg {
    clap::Arg::new("method")
        .long("request")
        .short('X')
        .value_name("METHOD")
        .num_args(1)
}

pub fn url() -> clap::Arg {
    clap::Arg::new("url")
        .help("Sets the url to use")
        .required(false)
        .num_args(1)
}
