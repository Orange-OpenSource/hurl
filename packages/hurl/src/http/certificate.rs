/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use super::easy_ext::CertInfo;

/// Represents an SSL/TLS certificate.
///
/// Each attribute `subject`, `issuer` etc... is optional, so we can test invalid certificates,
/// (i.e. a certificate without serial number). For the moment, we parse attributes values coming
/// from libcurl, whose format depends on the SSL/TLS backend and is very weak.
/// TODO: parse the X.509 certificate value ourselves to have string guarantee on the format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate {
    subject: Option<String>,
    issuer: Option<String>,
    start_date: Option<DateTime<Utc>>,
    expire_date: Option<DateTime<Utc>>,
    serial_number: Option<String>,
    subject_alt_name: Option<String>,
}

impl Certificate {
    /// Creates a new instance.
    pub fn new(
        subject: Option<String>,
        issuer: Option<String>,
        start_date: Option<DateTime<Utc>>,
        expire_date: Option<DateTime<Utc>>,
        serial_number: Option<String>,
        subject_alt_name: Option<String>,
    ) -> Self {
        Self {
            subject,
            issuer,
            start_date,
            expire_date,
            serial_number,
            subject_alt_name,
        }
    }

    /// Returns the subject attribute.
    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    /// Returns the issuer attribute.
    pub fn issuer(&self) -> Option<&String> {
        self.issuer.as_ref()
    }

    /// Returns the start date attribute.
    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }

    /// Returns the expire date attribute.
    pub fn expire_date(&self) -> Option<DateTime<Utc>> {
        self.expire_date
    }

    /// Returns the serial number attribute.
    pub fn serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Returns the subject alternative name attribute.
    pub fn subject_alt_name(&self) -> Option<&String> {
        self.subject_alt_name.as_ref()
    }
}

impl TryFrom<CertInfo> for Certificate {
    type Error = String;

    /// Parses `cert_info`.
    ///
    /// Support different "formats" in cert info
    /// - attribute name: "Start date" vs "Start Date"
    /// - date format: "Jan 10 08:29:52 2023 GMT" vs "2023-01-10 08:29:52 GMT"
    fn try_from(cert_info: CertInfo) -> Result<Self, Self::Error> {
        let attributes = parse_attributes(&cert_info.data);
        let subject = parse_subject(&attributes);
        let issuer = parse_issuer(&attributes);
        let start_date = parse_start_date(&attributes);
        let expire_date = parse_expire_date(&attributes);
        let serial_number = parse_serial_number(&attributes);
        let subject_alt_name = parse_subject_alt_name(&attributes);
        Ok(Certificate {
            subject,
            issuer,
            start_date,
            expire_date,
            serial_number,
            subject_alt_name,
        })
    }
}

const SUBJECT_ATTRIBUTE: &str = "subject";
const ISSUER_ATTRIBUTE: &str = "issuer";
const START_DATE_ATTRIBUTE: &str = "start date";
const EXPIRE_DATE_ATTRIBUTE: &str = "expire date";
const SERIAL_NUMBER_ATTRIBUTE: &str = "serial number";
const SUBJECT_ALT_NAME_ATTRIBUTE: &str = "x509v3 subject alternative name";
const ATTRIBUTES: &[&str] = &[
    SUBJECT_ATTRIBUTE,
    ISSUER_ATTRIBUTE,
    START_DATE_ATTRIBUTE,
    EXPIRE_DATE_ATTRIBUTE,
    SERIAL_NUMBER_ATTRIBUTE,
    SUBJECT_ALT_NAME_ATTRIBUTE,
];

/// Parses certificate's subject attribute.
///
/// TODO: we're exposing the subject and issuer directly from libcurl. In the certificate, these
/// properties are list of pair of key-value.
/// Through libcurl, these lists are serialized to a string:
///
/// Example:
/// vec![("C","US"),("O","Google Trust Services LLC"),("CN","GTS Root R1"))] =>
/// "C = US, O = Google Trust Services LLC, CN = GTS Root R1"
///
/// We should normalize the serialization (use 'A = B' or 'A=B') to always have the same issuer/
/// subject given a certain certificate. Actually the value can differ on different platforms, for
/// a given certificate.
///
/// See:
/// - <integration/hurl/ssl/cacert_to_json.out.pattern>
/// - <https://curl.se/mail/lib-2024-06/0013.html>
fn parse_subject(attributes: &HashMap<&str, &str>) -> Option<String> {
    attributes.get(SUBJECT_ATTRIBUTE).map(|s| s.to_string())
}

/// Parses certificate's issuer attribute.
fn parse_issuer(attributes: &HashMap<&str, &str>) -> Option<String> {
    attributes.get(ISSUER_ATTRIBUTE).map(|s| s.to_string())
}

fn parse_start_date(attributes: &HashMap<&str, &str>) -> Option<DateTime<Utc>> {
    attributes
        .get(START_DATE_ATTRIBUTE)
        .and_then(|date| parse_date(date).ok())
}

fn parse_expire_date(attributes: &HashMap<&str, &str>) -> Option<DateTime<Utc>> {
    attributes
        .get(EXPIRE_DATE_ATTRIBUTE)
        .and_then(|date| parse_date(date).ok())
}

fn parse_date(value: &str) -> Result<DateTime<Utc>, String> {
    let naive_date_time = match NaiveDateTime::parse_from_str(value, "%b %d %H:%M:%S %Y GMT") {
        Ok(d) => d,
        Err(_) => NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S GMT")
            .map_err(|_| format!("can not parse date <{value}>"))?,
    };
    Ok(naive_date_time.and_local_timezone(Utc).unwrap())
}

fn parse_serial_number(attributes: &HashMap<&str, &str>) -> Option<String> {
    attributes.get(SERIAL_NUMBER_ATTRIBUTE).map(|value| {
        // Serial numbers can come through libcurl in various format.
        // Either `AA:BB:CC` or `AABBCC`.
        if value.contains(':') {
            value
                .split(':')
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .join(":")
        } else {
            value
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(":")
        }
    })
}

fn parse_subject_alt_name(attributes: &HashMap<&str, &str>) -> Option<String> {
    attributes
        .get(SUBJECT_ALT_NAME_ATTRIBUTE)
        .map(|it| it.to_string())
}

fn parse_attributes(data: &Vec<String>) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for s in data {
        if let Some((name, value)) = parse_attribute(s) {
            // We're only interested in attributes declared in `ATTRIBUTES`.
            // We work with indices to use a `HashMap<&str, &str>` instead of `HashMap<String, &str>`
            ATTRIBUTES
                .iter()
                .position(|&att| att == name.to_lowercase())
                .map(|index| map.insert(ATTRIBUTES[index], value));
        }
    }
    map
}

fn parse_attribute(s: &str) -> Option<(&str, &str)> {
    if let Some(index) = s.find(':') {
        let (name, value) = s.split_at(index);
        Some((name, &value[1..]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::certificate::Certificate;
    use crate::http::easy_ext::CertInfo;

    #[test]
    fn test_parse_subject() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "subject",
            "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost",
        );
        assert_eq!(
            parse_subject(&attributes).unwrap(),
            "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost".to_string()
        );
    }

    #[test]
    fn test_parse_start_date() {
        let mut attributes = HashMap::new();
        attributes.insert("start date", "Jan 10 08:29:52 2023 GMT");
        assert_eq!(
            parse_start_date(&attributes).unwrap(),
            DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                .unwrap()
                .with_timezone(&Utc)
        );

        let mut attributes = HashMap::new();
        attributes.insert("start date", "2023-01-10 08:29:52 GMT");
        assert_eq!(
            parse_start_date(&attributes).unwrap(),
            DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                .unwrap()
                .with_timezone(&Utc)
        );
    }

    #[test]
    fn test_parse_serial_number() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "serial number",
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0:",
        );
        assert_eq!(
            parse_serial_number(&attributes).unwrap(),
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0".to_string()
        );

        let mut attributes = HashMap::new();
        attributes.insert("serial number", "1ee8b17f1b64d8d6b3de870103d2a4f533535ab0");
        assert_eq!(
            parse_serial_number(&attributes).unwrap(),
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0".to_string()
        );
    }

    #[test]
    fn test_parse_subject_alt_name() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "x509v3 subject alternative name",
            "DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1",
        );
        assert_eq!(
            parse_subject_alt_name(&attributes).unwrap(),
            "DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1".to_string()
        );
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            Certificate::try_from(CertInfo {
                data: vec![
                    "Subject:C = US, ST = Denial, L = Springfield, O = Dis, CN = localhost"
                        .to_string(),
                    "Issuer:C = US, ST = Denial, L = Springfield, O = Dis, CN = localhost"
                        .to_string(),
                    "Serial Number:1ee8b17f1b64d8d6b3de870103d2a4f533535ab0".to_string(),
                    "Start date:Jan 10 08:29:52 2023 GMT".to_string(),
                    "Expire date:Oct 30 08:29:52 2025 GMT".to_string(),
                    "x509v3 subject alternative name:DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1"
                        .to_string(),
                ]
            })
            .unwrap(),
            Certificate {
                subject: Some("C = US, ST = Denial, L = Springfield, O = Dis, CN = localhost"
                    .to_string()),
                issuer: Some("C = US, ST = Denial, L = Springfield, O = Dis, CN = localhost".to_string()),
                start_date: Some(DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                    .unwrap()
                    .with_timezone(&Utc)),
                expire_date: Some(DateTime::parse_from_rfc2822("Thu, 30 Oct 2025 08:29:52 GMT")
                    .unwrap()
                    .with_timezone(&Utc)),
                serial_number: Some("1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0"
                    .to_string()),
                subject_alt_name: Some("DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1".to_string())
            }
        );
    }
}
