/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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
use std::fmt::{Display, Formatter};

/// An IP address, either IPv4 or IPv6.
///
/// The `raw` field of this structure comes from libcurl `as is`. We keep it as a
/// [`String`] instead of a [`std::net::IpAddr`] to not make any assumptions
/// of the address format. We don't want to invalidate an HTTP exchange and raise a
/// runtime error because of an unusual format coming from libcurl.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IpAddr {
    raw: String,
}

impl IpAddr {
    /// Creates a new IP address from a raw string (from libcurl).
    pub fn new(raw: String) -> IpAddr {
        IpAddr { raw }
    }
}

impl Display for IpAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
