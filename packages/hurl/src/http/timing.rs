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
use std::time::Duration;

use crate::http::easy_ext;

/// Timing information for an HTTP transfer.
///
/// See [`easy_ext::namelookup_time_t`], [`easy_ext::connect_time_t`], [`easy_ext::app_connect_time_t`],
/// [`easy_ext::pre_transfert_time_t`], [`easy_ext::start_transfert_time_t`] and [`easy_ext::total_time_t`]
/// for [`TimingInfo`] fields definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimingInfo {
    pub name_lookup: Duration,
    pub connect_time: Duration,
    pub app_connect: Duration,
    pub pre_transfert: Duration,
    pub start_transfert: Duration,
    pub total: Duration,
}

impl TimingInfo {
    pub fn new(easy: &Easy) -> Self {
        // TODO: maybe implement fallback to  *_time function in case *_time_t are
        //  not implemented.
        let name_lookup = easy_ext::namelookup_time_t(easy).unwrap_or(Duration::default());
        let connect_time = easy_ext::connect_time_t(easy).unwrap_or(Duration::default());
        let app_connect = easy_ext::appconnect_time_t(easy).unwrap_or(Duration::default());
        let pre_transfert = easy_ext::pretransfer_time_t(easy).unwrap_or(Duration::default());
        let start_transfert = easy_ext::starttransfer_time_t(easy).unwrap_or(Duration::default());
        let total = easy_ext::total_time_t(easy).unwrap_or(Duration::default());
        TimingInfo {
            name_lookup,
            connect_time,
            app_connect,
            pre_transfert,
            start_transfert,
            total,
        }
    }
}
