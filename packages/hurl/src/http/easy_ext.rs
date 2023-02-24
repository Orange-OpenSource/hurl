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
use curl::easy::Easy;
use curl::Error;
use curl_sys::{curl_certinfo, curl_slist};
use std::ffi::CStr;
use std::ptr;

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
        if rc != curl_sys::CURLE_OK {
            return Err(Error::new(rc));
        }
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
    use super::to_list;
    use std::ffi::CString;
    use std::ptr;

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
