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
use chrono::{DateTime, Utc};
use curl::easy::Easy;
use std::time::Duration;

use crate::http::easy_ext;

/// Timing information for an HTTP transfer.
// See [`easy_ext::namelookup_time_t`], [`easy_ext::connect_time_t`], [`easy_ext::app_connect_time_t`],
// [`easy_ext::pre_transfer_time_t`], [`easy_ext::start_transfer_time_t`] and [`easy_ext::total_time_t`]
// for fields definition.
#[serde_with::serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize)]
pub struct Timings {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub begin_call: DateTime<Utc>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub end_call: DateTime<Utc>,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub name_lookup: Duration,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub connect: Duration,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub app_connect: Duration,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub pre_transfer: Duration,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub start_transfer: Duration,
    #[serde_as(as = "serde_with::DurationMicroSeconds")]
    pub total: Duration,
}

impl Timings {
    pub fn new(easy: &mut Easy, begin_call: DateTime<Utc>, end_call: DateTime<Utc>) -> Self {
        // We try the *_t timing function of libcurl (available for libcurl >= 7.61.0)
        // returning timing in nanoseconds, or fallback to timing function returning seconds
        // if *_t are not available.
        let name_lookup = easy_ext::namelookup_time_t(easy)
            .or(easy.namelookup_time())
            .unwrap_or(Duration::default());
        let connect = easy_ext::connect_time_t(easy)
            .or(easy.connect_time())
            .unwrap_or(Duration::default());
        let app_connect = easy_ext::appconnect_time_t(easy)
            .or(easy.appconnect_time())
            .unwrap_or(Duration::default());
        let pre_transfer = easy_ext::pretransfer_time_t(easy)
            .or(easy.pretransfer_time())
            .unwrap_or(Duration::default());
        let start_transfer = easy_ext::starttransfer_time_t(easy)
            .or(easy.starttransfer_time())
            .unwrap_or(Duration::default());
        let total = easy_ext::total_time_t(easy)
            .or(easy.total_time())
            .unwrap_or(Duration::default());
        Timings {
            begin_call,
            end_call,
            name_lookup,
            connect,
            app_connect,
            pre_transfer,
            start_transfer,
            total,
        }
    }
}
