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
use std::ffi::CStr;
use std::ptr;
use std::time::Duration;

use curl::easy::Easy;
use curl::Error;
use curl_sys::{curl_certinfo, curl_off_t, curl_slist, CURLINFO};

/// Some definitions not present in curl-sys
const CURLINFO_OFF_T: CURLINFO = 0x600000;

const CURLINFO_TOTAL_TIME_T: CURLINFO = CURLINFO_OFF_T + 50;
const CURLINFO_NAMELOOKUP_TIME_T: CURLINFO = CURLINFO_OFF_T + 51;
const CURLINFO_CONNECT_TIME_T: CURLINFO = CURLINFO_OFF_T + 52;
const CURLINFO_PRETRANSFER_TIME_T: CURLINFO = CURLINFO_OFF_T + 53;
const CURLINFO_STARTTRANSFER_TIME_T: CURLINFO = CURLINFO_OFF_T + 54;
const CURLINFO_APPCONNECT_TIME_T: CURLINFO = CURLINFO_OFF_T + 56;

/// Represents certificate information.
/// `data` has format "name:content";
#[derive(Clone)]
pub struct CertInfo {
    pub data: Vec<String>,
}

/// Returns the information of the first certificate in the certificates chain.
pub fn get_certinfo(easy: &Easy) -> Result<Option<CertInfo>, Error> {
    unsafe {
        let mut certinfo = ptr::null_mut::<curl_certinfo>();
        let rc =
            curl_sys::curl_easy_getinfo(easy.raw(), curl_sys::CURLINFO_CERTINFO, &mut certinfo);
        cvt(easy, rc)?;
        if certinfo.is_null() {
            return Ok(None);
        }
        let count = (*certinfo).num_of_certs;
        if count <= 0 {
            return Ok(None);
        }
        let slist = *((*certinfo).certinfo.offset(0));
        let data = to_list(slist);
        Ok(Some(CertInfo { data }))
    }
}

// Timing of a typical HTTP exchange (over TLS 1.2 connection) from libcurl
// (courtesy of <https://blog.cloudflare.com/a-question-of-timing/>
// =========================================================================
//
//                                     ┌───────────┐            ┌──────────────┐   ┌──────────────┐
//                                     │  Client   │            │  DNS Server  │   │  Web Server  │
//                                     └─────┬─────┘            └──────┬───────┘   └──────┬───────┘
//                                           │                         │                  │
//            ┌                           0s ├────── DNS Request ─────►│                  │
//    DNS     │                              │                         │ DNS Resolver     │
//  Lookup   <                               │                         │ e.g. 1.1.1.1     │
//            │                              │◄───── DNS Response ─────┘                  │
//            └     time_namelookup   1.510s │                                            │
//            ┌                              ├────────────────── SYN ────────────────────►│
//    TCP    <                               │                                            │
// Handshake  └        time_connect   1.757s │◄──────────────  SYN/ACK ───────────────────┤
//            ┌                              │                                            │
//            │                              │                                            │
//            │                              ├────────────────── ACK ────────────────────►│
//            │                              ├────────────── ClientHello ────────────────►│
//            │                              │                                            │
//            │                              │◄───────────── ServerHello ─────────────────┤
//    SSL    <                               │               Certificate                  │
// Handshake  │                              │                                            │
//            │                              ├───────────── ClientKeyExch, ──────────────►│
//            │                              │            ChangeCipherSpec                │
//            │                              │                                            │
//            │                              │◄────────── ChangeCipherSpec ───────────────┤
//            └     time_appconnect   2.256s │                Finished                    │
//            ┌   time_pretransfert   2.259s ├─────────────── HTTP GET ──────────────────►│
//            │                              │                                            │
//    Wait   <                               │                                            │
//            │                              │                                            │
//            └ time_starttransfert   2.506s │                                            │
//            ┌                              │◄───────────────────────────────────────────┤
//    Data    │                              │◄─────────────── Response ──────────────────┤
//  Transfer <                               │                   ...                      │
//            │                              │◄───────────────────────────────────────────┤
//            └         time_total    3.001s │                                            │
//                                           ▼                                            ▼

/// Get the name lookup time.
///
/// Returns the total time in microseconds from the start until the name resolving was completed.
///
/// Corresponds to [`CURLINFO_NAMELOOKUP_TIME_T`] and may return an error if the
/// option isn't supported.
pub fn namelookup_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_NAMELOOKUP_TIME_T).map(microseconds_to_duration)
}

/// Get the time until connect.
///
/// Returns the total time in microseconds from the start until the connection to the remote host (or proxy) was completed.
///
/// Corresponds to [`CURLINFO_CONNECT_TIME_T`] and may return an error if the
/// option isn't supported.
pub fn connect_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_CONNECT_TIME_T).map(microseconds_to_duration)
}

/// Get the time until the SSL/SSH handshake is completed.
///
/// Returns the total time in microseconds it took from the start until the SSL/SSH
/// connect/handshake to the remote host was completed. This time is most often
/// very near to the [`pretransfer_time_t`] time, except for cases such as
/// HTTP pipelining where the pretransfer time can be delayed due to waits in
/// line for the pipeline and more.
///
/// Corresponds to [`CURLINFO_APPCONNECT_TIME_T`] and may return an error if the
/// option isn't supported.
pub fn appconnect_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_APPCONNECT_TIME_T).map(microseconds_to_duration)
}

/// Get the time until the file transfer start.
///
/// Returns the total time in microseconds it took from the start until the file
/// transfer is just about to begin. This includes all pre-transfer commands
/// and negotiations that are specific to the particular protocol(s) involved.
/// It does not involve the sending of the protocol- specific request that
/// triggers a transfer.
///
/// Corresponds to [`CURLINFO_PRETRANSFER_TIME`] and may return an error if the
/// option isn't supported.
pub fn pretransfer_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_PRETRANSFER_TIME_T).map(microseconds_to_duration)
}

/// Get the time in microseconds until the first byte is received.
///
/// Returns the total time it took from the start until the first
/// byte is received by libcurl. This includes [`pretransfer_time_t`] and
/// also the time the server needs to calculate the result.
///
/// Corresponds to [`CURLINFO_STARTTRANSFER_TIME`] and may return an error if the
/// option isn't supported.
pub fn starttransfer_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_STARTTRANSFER_TIME_T).map(microseconds_to_duration)
}

/// Get total time of previous transfer
///
/// Returns the total time in microseconds for the previous transfer,
/// including name resolving, TCP connect etc.
///
/// Corresponds to [`CURLINFO_TOTAL_TIME_T`] and may return an error if the
/// option isn't supported.
pub fn total_time_t(easy: &mut Easy) -> Result<Duration, Error> {
    getopt_off_t(easy, CURLINFO_TOTAL_TIME_T).map(microseconds_to_duration)
}

/// Converts an instance of libcurl linked list [`curl_slist`] to a vec of [`String`].
fn to_list(slist: *mut curl_slist) -> Vec<String> {
    let mut data = vec![];
    let mut cur = slist;
    loop {
        if cur.is_null() {
            break;
        }
        unsafe {
            let ret = CStr::from_ptr((*cur).data).to_bytes();
            let value = String::from_utf8_lossy(ret);
            data.push(value.to_string());
            cur = (*cur).next
        }
    }
    data
}

/// Check if the return code `rc` is OK, and returns an error if not.
fn cvt(easy: &Easy, rc: curl_sys::CURLcode) -> Result<(), Error> {
    if rc == curl_sys::CURLE_OK {
        return Ok(());
    }
    let mut err = Error::new(rc);
    if let Some(msg) = easy.take_error_buf() {
        err.set_extra(msg);
    }
    Err(err)
}

fn getopt_off_t(easy: &mut Easy, opt: CURLINFO) -> Result<curl_off_t, Error> {
    unsafe {
        let mut p = 0 as curl_off_t;
        let rc = curl_sys::curl_easy_getinfo(easy.raw(), opt, &mut p);
        cvt(easy, rc)?;
        Ok(p)
    }
}

fn microseconds_to_duration(microseconds: i64) -> Duration {
    Duration::from_micros(microseconds as u64)
}

// // Iterator based implementation more similar to curl crates List implementation.
// // See <https://github.com/alexcrichton/curl-rust/blob/main/src/easy/list.rs>
// pub struct CertInfo2 {
//     raw: *mut curl_certinfo,
// }
//
// // An iterator over CertInfo2
// pub struct Iter<'a> {
//     me: &'a CertInfo2,
//     cur: u32,
// }
//
// pub unsafe fn from_raw(raw: *mut curl_certinfo) -> CertInfo2 {
//     CertInfo2 { raw }
// }
//
// impl CertInfo2 {
//     pub fn new() -> CertInfo2 {
//         CertInfo2 {
//             raw: ptr::null_mut(),
//         }
//     }
//
//     pub fn iter(&self) -> Iter {
//         Iter {
//             me: self,
//             cur: 0,
//         }
//     }
// }
//
// impl<'a> IntoIterator for &'a CertInfo2 {
//     type Item = *mut curl_slist;
//     type IntoIter = Iter<'a>;
//
//     fn into_iter(self) -> Iter<'a> {
//         self.iter()
//     }
// }
//
// impl<'a> Iterator for Iter<'a> {
//     type Item = *mut curl_slist;
//
//     fn next(&mut self) -> Option<*mut curl_slist> {
//         unsafe {
//             if self.cur >= (*self.me.raw).num_of_certs as u32 {
//                 return None
//             }
//             let slist = *((*self.me.raw).certinfo.offset(self.cur as isize));
//             self.cur += 1;
//             Some(slist)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::ptr;

    use super::to_list;

    #[test]
    fn convert_curl_slist_to_vec() {
        let mut slist = ptr::null_mut();

        unsafe {
            for value in ["foo", "bar", "baz"] {
                let str = CString::new(value).unwrap();
                slist = curl_sys::curl_slist_append(slist, str.as_ptr());
            }
        }

        assert_eq!(
            to_list(slist),
            vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
        );

        unsafe {
            curl_sys::curl_slist_free_all(slist);
        }
    }
}
