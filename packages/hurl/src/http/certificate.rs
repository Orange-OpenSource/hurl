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
use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use super::easy_ext::CertInfo;

/// Represents an SSL/TLS certificate.
///
/// Each attribute `subject`, `issuer` etc... is optional, so we can test invalid certificate,
/// (i.e. a certificate without serial number).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate {
    subject: Option<String>,
    issuer: Option<String>,
    pub start_date: DateTime<Utc>,
    pub expire_date: DateTime<Utc>,
    pub serial_number: String,
    pub subject_alt_name: Option<String>,
}

impl Certificate {
    /// Creates a new instance.
    pub fn new(
        subject: Option<String>,
        issuer: Option<String>,
        start_date: DateTime<Utc>,
        expire_date: DateTime<Utc>,
        serial_number: String,
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

    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    pub fn issuer(&self) -> Option<&String> {
        self.issuer.as_ref()
    }
}

impl TryFrom<CertInfo> for Certificate {
    type Error = String;

    /// parse `cert_info`
    /// support different "formats" in cert info
    /// - attribute name: "Start date" vs "Start Date"
    /// - date format: "Jan 10 08:29:52 2023 GMT" vs "2023-01-10 08:29:52 GMT"
    fn try_from(cert_info: CertInfo) -> Result<Self, Self::Error> {
        let attributes = parse_attributes(&cert_info.data);
        let subject = parse_subject(&attributes).ok();
        let issuer = parse_issuer(&attributes).ok();
        let start_date = parse_start_date(&attributes)?;
        let expire_date = parse_expire_date(&attributes)?;
        let serial_number = parse_serial_number(&attributes)?;
        let subject_alt_name = parse_subject_alt_name(&attributes).ok();
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
/// - https://curl.se/mail/lib-2024-06/0013.html
fn parse_subject(attributes: &HashMap<String, String>) -> Result<String, String> {
    match attributes.get("subject") {
        None => Err(format!("missing Subject attribute in {attributes:?}")),
        Some(value) => Ok(value.clone()),
    }
}

/// Parses certificate's issuer attribute.
fn parse_issuer(attributes: &HashMap<String, String>) -> Result<String, String> {
    match attributes.get("issuer") {
        None => Err(format!("missing Issuer attribute in {attributes:?}")),
        Some(value) => Ok(value.clone()),
    }
}

fn parse_start_date(attributes: &HashMap<String, String>) -> Result<DateTime<Utc>, String> {
    match attributes.get("start date") {
        None => Err(format!("missing start date attribute in {attributes:?}")),
        Some(value) => Ok(parse_date(value)?),
    }
}

fn parse_expire_date(attributes: &HashMap<String, String>) -> Result<DateTime<Utc>, String> {
    match attributes.get("expire date") {
        None => Err("missing expire date attribute".to_string()),
        Some(value) => Ok(parse_date(value)?),
    }
}

fn parse_date(value: &str) -> Result<DateTime<Utc>, String> {
    let naive_date_time = match NaiveDateTime::parse_from_str(value, "%b %d %H:%M:%S %Y GMT") {
        Ok(d) => d,
        Err(_) => NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S GMT")
            .map_err(|_| format!("can not parse date <{value}>"))?,
    };
    Ok(naive_date_time.and_local_timezone(Utc).unwrap())
}

fn parse_serial_number(attributes: &HashMap<String, String>) -> Result<String, String> {
    let value = attributes
        .get("serial number")
        .cloned()
        .ok_or(format!("Missing serial number attribute in {attributes:?}"))?;
    let normalized_value = if value.contains(':') {
        value
            .split(':')
            .filter(|e| !e.is_empty())
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
    };

    Ok(normalized_value)
}

fn parse_subject_alt_name(attributes: &HashMap<String, String>) -> Result<String, String> {
    match attributes.get("x509v3 subject alternative name") {
        None => Err(format!(
            "missing x509v3 subject alternative name attribute in {attributes:?}"
        )),
        Some(value) => Ok(value.clone()),
    }
}

fn parse_attributes(data: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for s in data {
        if let Some((name, value)) = parse_attribute(s) {
            map.insert(name.to_lowercase(), value);
        }
    }
    map
}

fn parse_attribute(s: &str) -> Option<(String, String)> {
    if let Some(index) = s.find(':') {
        let (name, value) = s.split_at(index);
        Some((name.to_string(), value[1..].to_string()))
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
            "subject".to_string(),
            "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost".to_string(),
        );
        assert_eq!(
            parse_subject(&attributes).unwrap(),
            "C=US, ST=Denial, L=Springfield, O=Dis, CN=localhost".to_string()
        );
    }

    #[test]
    fn test_parse_start_date() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "start date".to_string(),
            "Jan 10 08:29:52 2023 GMT".to_string(),
        );
        assert_eq!(
            parse_start_date(&attributes).unwrap(),
            chrono::DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                .unwrap()
                .with_timezone(&Utc)
        );

        let mut attributes = HashMap::new();
        attributes.insert(
            "start date".to_string(),
            "2023-01-10 08:29:52 GMT".to_string(),
        );
        assert_eq!(
            parse_start_date(&attributes).unwrap(),
            chrono::DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                .unwrap()
                .with_timezone(&Utc)
        );
    }

    #[test]
    fn test_parse_serial_number() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "serial number".to_string(),
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0:".to_string(),
        );
        assert_eq!(
            parse_serial_number(&attributes).unwrap(),
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0".to_string()
        );

        let mut attributes = HashMap::new();
        attributes.insert(
            "serial number".to_string(),
            "1ee8b17f1b64d8d6b3de870103d2a4f533535ab0".to_string(),
        );
        assert_eq!(
            parse_serial_number(&attributes).unwrap(),
            "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0".to_string()
        );
    }

    #[test]
    fn test_parse_subject_alt_name() {
        let mut attributes = HashMap::new();
        attributes.insert(
            "x509v3 subject alternative name".to_string(),
            "DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1".to_string(),
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
                start_date: chrono::DateTime::parse_from_rfc2822("Tue, 10 Jan 2023 08:29:52 GMT")
                    .unwrap()
                    .with_timezone(&Utc),
                expire_date: chrono::DateTime::parse_from_rfc2822("Thu, 30 Oct 2025 08:29:52 GMT")
                    .unwrap()
                    .with_timezone(&Utc),
                serial_number: "1e:e8:b1:7f:1b:64:d8:d6:b3:de:87:01:03:d2:a4:f5:33:53:5a:b0"
                    .to_string(),
                subject_alt_name: Some("DNS:localhost, IP address:127.0.0.1, IP address:0:0:0:0:0:0:0:1".to_string())
            }
        );
        assert_eq!(
            Certificate::try_from(CertInfo { data: vec![] })
                .err()
                .unwrap(),
            "missing start date attribute in {}".to_string()
        );
    }
}
