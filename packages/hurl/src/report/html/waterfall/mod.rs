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
use hurl_core::ast::{Entry, HurlFile};
use std::time::Duration;

use crate::http::Call;
use crate::report::html::waterfall::nice::NiceScale;
use crate::report::html::waterfall::svg::Attribute::{
    Class, Fill, Filter, FloodOpacity, Height, Id, StdDeviation, Stroke, StrokeWidth, ViewBox,
    Width, DX, DY,
};
use crate::report::html::waterfall::svg::Element;
use crate::report::html::waterfall::unit::{
    Interval, Microsecond, Millisecond, Pixel, Scale, Second, TimeUnit,
};
use crate::report::html::Testcase;
use crate::runner::EntryResult;

mod nice;
mod svg;
mod unit;

impl Testcase {
    /// Returns the HTML waterfall of these `entries`.
    /// `hurl_file` AST is used to construct URL with line numbers to the correponding
    /// entry in the colored HTML source file.
    pub fn get_waterfall_html(&self, hurl_file: &HurlFile, entries: &[EntryResult]) -> String {
        let svg = get_waterfall_svg(hurl_file, &self.id, entries);
        format!(include_str!("../resources/waterfall.html"), svg = svg,)
    }
}

/// Returns the start and end date for these entries.
fn get_times_interval(entries: &[EntryResult]) -> Option<Interval<DateTime<Utc>>> {
    let calls = entries
        .iter()
        .flat_map(|entry| &entry.calls)
        .collect::<Vec<&Call>>();
    let begin = calls.first();
    let end = calls.last();
    match (begin, end) {
        (Some(start), Some(end)) => {
            let start = start.timings.begin_call;
            let end = end.timings.end_call;
            Some(Interval { start, end })
        }
        _ => None,
    }
}

/// Returns the SVG string of this list of `entries`.
/// `hurl_file` AST is used to construct URL with line numbers to the correponding
/// entry in the colored HTML source file.
fn get_waterfall_svg(hurl_file: &HurlFile, id: &str, entries: &[EntryResult]) -> String {
    let margin_top = Pixel(50.0);
    let margin_bottom = Pixel(250.0);
    let margin_right = Pixel(20.0);
    let entry_height = Pixel(20.0);
    let y = Pixel(0.0);
    let width = Pixel(2000.0);
    let height = f64::max(
        100.0,
        (entry_height.0 * entries.len() as f64) + margin_top.0 + margin_bottom.0,
    );
    let height = Pixel(height);

    let mut root = svg::svg();
    root.add_attr(ViewBox(0.0, 0.0, width.0, height.0));
    root.add_attr(Width(width.0));
    root.add_attr(Height(height.0));

    // Compute our scale (transform 0 based microsecond to 0 based pixels):
    let times = get_times_interval(entries);
    let times = match times {
        Some(t) => t,
        None => return "".to_string(),
    };
    // We add some space for the right last grid labels.
    let pixels = Interval::new(Pixel(0.0), width - margin_right);
    let scale_x = Scale::new(times, pixels);

    let style = svg::style(include_str!("../resources/waterfall_svg.css"));
    root.add_child(style);

    let filters = filters();
    root.add_child(filters);

    let grid = grid(times, 20, scale_x, y, height);
    root.add_child(grid);

    // We construct entry from last to first so the detail SVG of any entry is not overridden
    // by the next entry SGV.
    entries
        .iter()
        .enumerate()
        .rev()
        .map(|(index, e)| {
            let entry_node = hurl_file.entries.get(index).unwrap();
            let entry_url = entry_url(id, entry_node);
            let offset_y = Pixel((index as f64) * entry_height.0 + margin_top.0);
            entry(e, times, scale_x, offset_y, width, &entry_url)
        })
        .for_each(|e| root.add_child(e));

    root.to_string()
}

/// Returns the grid SVG with tick on "nice" times.
fn grid(
    times: Interval<DateTime<Utc>>,
    ticks_number: usize,
    scale_x: Scale,
    y: Pixel,
    height: Pixel,
) -> Element {
    let mut grid = svg::group();
    grid.add_attr(Class("grid".to_string()));

    // We compute in which unit we're going to draw the grid
    let duration = times.end - times.start;
    let duration = duration.num_microseconds().unwrap() as f64;
    let duration = Microsecond(duration);
    let delta = Microsecond(duration.0 / ticks_number as f64);
    let (start, end) = match delta.0 {
        d if d < 1_000.0 => (TimeUnit::zero_mc(), TimeUnit::Microsecond(duration)),
        d if d < 1_000_000.0 => {
            let end = Millisecond::from(duration);
            (TimeUnit::zero_ms(), TimeUnit::Millisecond(end))
        }
        _ => {
            let end = Second::from(duration);
            (TimeUnit::zero_s(), TimeUnit::Second(end))
        }
    };
    let nice_scale = NiceScale::new(start.as_f64(), end.as_f64(), ticks_number);

    let mut t = start;
    let mut values = vec![];
    while t < end {
        let x = scale_x.to_pixel(Microsecond::from(t));
        // We want a integer pixel value:
        values.push((x.0.round(), t));
        t = t.add_raw(nice_scale.get_tick_spacing());
    }

    // First, draw the vertical lines:
    let mut lines = svg::group();
    lines.add_attr(Class("grid-line".to_string()));
    lines.add_attr(Stroke("#ccc".to_string()));
    values
        .iter()
        .map(|(x, _)| svg::line(*x, y.0, *x, height.0))
        .for_each(|l| lines.add_child(l));
    grid.add_child(lines);

    // Then, draw labels
    let mut labels = svg::group();
    labels.add_attr(Class("grid-labels".to_string()));
    labels.add_attr(Fill("#777".to_string()));
    values
        .iter()
        .map(|(x, t)| svg::text(*x + 5.0, 20.0, &format!("{} {}", t.as_f64(), t.unit())))
        .for_each(|l| labels.add_child(l));
    grid.add_child(labels);

    grid
}

/// Returns the SVG of this `entry`.
/// `times` is the time interval of the complete run, `scale_x` allows to convert
/// between times and pixel for the X-axis.
/// The entry is offset on the Y-Axis by `offset_y` pixels, and right boxed to a
/// maximum of `max_width` pixel.
fn entry(
    entry: &EntryResult,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    offset_y: Pixel,
    max_width: Pixel,
    url: &str,
) -> Element {
    let mut group = svg::group();
    group.add_attr(Class("entry".to_string()));
    entry
        .calls
        .iter()
        .map(|c| call(c, times, scale_x, offset_y, max_width, url))
        .for_each(|c| group.add_child(c));
    group
}

/// Returns the SVG of this `call`.
/// `times` is the time interval of the complete run, `scale_x` allows to convert
/// between times and pixel for the X-axis.
/// The entry is offset on the Y-Axis by `offset_y` pixels, and right boxed to a
/// maximum of `max_width` pixel.
fn call(
    call: &Call,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    offset_y: Pixel,
    max_width: Pixel,
    entry_url: &str,
) -> Element {
    let mut call_elt = svg::group();
    call_elt.add_attr(Class("call".to_string()));

    let summary = call_summary(call, times, scale_x, offset_y);
    call_elt.add_child(summary);
    let detail = call_detail(call, times, scale_x, offset_y, max_width, entry_url);
    call_elt.add_child(detail);

    call_elt
}

/// Returns the SVG summary of this `call`.
/// The summary is a suit of boxes for each call timings (see [`crate::http::Timings`]).
fn call_summary(
    call: &Call,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    offset_y: Pixel,
) -> Element {
    let mut group = svg::group();
    group.add_attr(Class("call-summary".to_string()));

    let y = offset_y;
    let height = Pixel(20.0);

    // DNS
    let dns_x = (call.timings.begin_call - times.start).to_std().unwrap();
    let dns_x = duration_to_pixel(dns_x, scale_x);
    let dns_width = duration_to_pixel(call.timings.name_lookup, scale_x);
    if dns_width.0 > 0.0 {
        let elt = svg::rect(dns_x.0, y.0, dns_width.0, height.0, "#1d9688");
        group.add_child(elt);
    }

    // TCP Handshake
    let tcp_x = duration_to_pixel(call.timings.name_lookup, scale_x) + dns_x;
    let tcp_width = duration_to_pixel(call.timings.connect - call.timings.name_lookup, scale_x);
    if tcp_width.0 > 0.0 {
        let elt = svg::rect(tcp_x.0, y.0, tcp_width.0, height.0, "#fa7f03");
        group.add_child(elt);
    }

    // SSL
    let ssl_x = duration_to_pixel(call.timings.connect, scale_x) + dns_x;
    let ssl_width = duration_to_pixel(call.timings.app_connect - call.timings.connect, scale_x);
    if ssl_width.0 > 0.0 {
        let elt = svg::rect(ssl_x.0, y.0, ssl_width.0, height.0, "#9933ff");
        group.add_child(elt);
    }

    // Wait
    let wait_x = duration_to_pixel(call.timings.pre_transfer, scale_x) + dns_x;
    let wait_width = duration_to_pixel(
        call.timings.start_transfer - call.timings.pre_transfer,
        scale_x,
    );
    if wait_width.0 > 0.0 {
        let elt = svg::rect(wait_x.0, y.0, wait_width.0, height.0, "#18c852");
        group.add_child(elt);
    }

    // Data transfer
    let data_transfer_x = duration_to_pixel(call.timings.start_transfer, scale_x) + dns_x;
    let data_transfer_width =
        duration_to_pixel(call.timings.total - call.timings.start_transfer, scale_x);
    if data_transfer_width.0 > 0.0 {
        let elt = svg::rect(
            data_transfer_x.0,
            y.0,
            data_transfer_width.0,
            height.0,
            "#36a9f4",
        );
        group.add_child(elt);
    }

    group
}

/// Returns the SVG detail of this `call`.
/// Each call timings value is displayed under the summary (see [`crate::http::Timings`]).
fn call_detail(
    call: &Call,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    offset_y: Pixel,
    max_width: Pixel,
    entry_url: &str,
) -> Element {
    let mut group = svg::group();
    group.add_attr(Class("call-detail".to_string()));

    let x = (call.timings.begin_call - times.start).to_std().unwrap();
    let x = duration_to_pixel(x, scale_x) + Pixel(6.0);
    let y = offset_y + Pixel(20.0);
    let width = Pixel(600.0);
    let height = Pixel(235.0);
    let x = if x + width > max_width {
        max_width - width - Pixel(10.0)
    } else {
        x
    };

    // Background detail:
    let mut elt = svg::rect(x.0, y.0, width.0, height.0, "white");
    elt.add_attr(Filter("url(#shadow)".to_string()));
    elt.add_attr(Stroke("#cc".to_string()));
    elt.add_attr(StrokeWidth(1.0));
    group.add_child(elt);

    let x = x + Pixel(16.0);
    let y = y + Pixel(20.0);
    let delta_y = Pixel(30.0);

    // URL + method
    let method = &call.request.method;
    let url = &call.request.url;
    let url = if url.len() > 64 {
        format!("{}...", &url[0..64])
    } else {
        url.to_string()
    };
    let elt = svg::text(x.0, y.0 + 14.0, &format!("{method} {url}"));
    let mut a = svg::a(entry_url);
    a.add_child(elt);
    group.add_child(a);

    // DNS
    let y = y + delta_y;
    let duration = call.timings.name_lookup.as_micros();
    let duration = Microsecond(duration as f64);
    let elt = legend(x, y, "DNS lookup", Some("#1d9688"), duration);
    group.add_child(elt);

    // TCP handshake
    let y = y + delta_y;
    let duration = (call.timings.connect - call.timings.name_lookup).as_micros();
    let duration = Microsecond(duration as f64);
    let elt = legend(x, y, "TCP handshake", Some("#fa7f03"), duration);
    group.add_child(elt);

    // SSL handshake
    let y = y + delta_y;
    let duration = (call.timings.app_connect - call.timings.connect).as_micros();
    let duration = Microsecond(duration as f64);
    let elt = legend(x, y, "SSL handshake", Some("#9933ff"), duration);
    group.add_child(elt);

    // Wait
    let y = y + delta_y;
    let duration = (call.timings.start_transfer - call.timings.pre_transfer).as_micros();
    let duration = Microsecond(duration as f64);
    let elt = legend(x, y, "Wait", Some("#18c852"), duration);
    group.add_child(elt);

    // Data transfer
    let y = y + delta_y;
    let duration = (call.timings.total - call.timings.start_transfer).as_micros();
    let duration = Microsecond(duration as f64);
    let elt = legend(x, y, "Data transfer", Some("#36a9f4"), duration);
    group.add_child(elt);

    // Total
    let y = y + delta_y;
    let duration = call.timings.total.as_micros();
    let duration = Microsecond(duration as f64);
    let mut elt = legend(x, y, "Total", None, duration);
    elt.add_attr(Class("call-detail-total".to_string()));
    group.add_child(elt);

    group
}

fn legend(x: Pixel, y: Pixel, text: &str, color: Option<&str>, duration: Microsecond) -> Element {
    let dx_label = Pixel(36.0);
    let dy_label = Pixel(17.0);
    let dx_duration = Pixel(180.0);

    let mut group = svg::group();

    if let Some(color) = color {
        let color_elt = svg::rect(x.0, y.0, 20.0, 20.0, color);
        group.add_child(color_elt);
    }

    let mut text_elt = svg::text((x + dx_label).0, (y + dy_label).0, text);
    text_elt.add_attr(Fill("#555".to_string()));
    group.add_child(text_elt);

    let duration = duration.human_string();
    let mut duration_elt = svg::text((x + dx_duration).0, (y + dy_label).0, &duration);
    duration_elt.add_attr(Fill("#333".to_string()));
    group.add_child(duration_elt);

    group
}

fn duration_to_pixel(duration: Duration, scale_x: Scale) -> Pixel {
    let value = duration.as_micros();
    let value = Microsecond(value as f64);
    scale_x.to_pixel(value)
}

fn filters() -> Element {
    let mut defs = svg::defs();

    let mut filter = svg::filter();
    filter.add_attr(Id("shadow".to_string()));

    let mut shadow = svg::fe_drop_shadow();
    shadow.add_attr(DX(0.0));
    shadow.add_attr(DY(4.0));
    shadow.add_attr(StdDeviation(4.0));
    shadow.add_attr(FloodOpacity(0.25));
    filter.add_child(shadow);
    defs.add_child(filter);
    defs
}

fn entry_url(id: &str, entry: &Entry) -> String {
    let line = entry.request.space0.source_info.start.line;
    format!("{id}.html#l{line}")
}

impl Microsecond {
    fn human_string(&self) -> String {
        match self.0 {
            d if d < 0.0 => "_".to_string(),
            d if d < 1_000.0 => format!("{d:.1} µs"),
            d if d < 1_000_000.0 => format!("{:.1} ms", d / 1_000.0),
            d => format!("{:.1} s", d / 1_000_000.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::report::html::waterfall::unit::Microsecond;
    use crate::report::html::waterfall::unit::{Interval, Pixel, Scale};
    use crate::report::html::waterfall::{grid, legend};
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn legend_svg() {
        let x = Pixel(20.0);
        let y = Pixel(30.0);
        let text = "Hellow world";
        let color = "red";
        let duration = Microsecond(2000.0);

        let elt = legend(x, y, text, Some(color), duration);
        assert_eq!(
            elt.to_string(),
            "<g>\
                <rect x=\"20\" y=\"30\" width=\"20\" height=\"20\" fill=\"red\" />\
                <text x=\"56\" y=\"47\" fill=\"#555\">Hellow world</text>\
                <text x=\"200\" y=\"47\" fill=\"#333\">2.0 ms</text>\
            </g>"
        );
    }

    #[test]
    fn grid_svg() {
        let start = Utc.with_ymd_and_hms(2022, 1, 1, 8, 0, 0).unwrap();
        let end = start + Duration::seconds(1);
        let times = Interval { start, end };
        let start = Pixel(0.0);
        let end = Pixel(1000.0);
        let pixels = Interval { start, end };
        let scale_x = Scale::new(times, pixels);
        let ticks_number = 10;
        let y = Pixel(0.0);
        let height = Pixel(100.0);

        let elt = grid(times, ticks_number, scale_x, y, height);
        assert_eq!(
            elt.to_string(),
            "<g class=\"grid\">\
                <g class=\"grid-line\" stroke=\"#ccc\">\
                    <line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"100\" />\
                    <line x1=\"100\" y1=\"0\" x2=\"100\" y2=\"100\" />\
                    <line x1=\"200\" y1=\"0\" x2=\"200\" y2=\"100\" />\
                    <line x1=\"300\" y1=\"0\" x2=\"300\" y2=\"100\" />\
                    <line x1=\"400\" y1=\"0\" x2=\"400\" y2=\"100\" />\
                    <line x1=\"500\" y1=\"0\" x2=\"500\" y2=\"100\" />\
                    <line x1=\"600\" y1=\"0\" x2=\"600\" y2=\"100\" />\
                    <line x1=\"700\" y1=\"0\" x2=\"700\" y2=\"100\" />\
                    <line x1=\"800\" y1=\"0\" x2=\"800\" y2=\"100\" />\
                    <line x1=\"900\" y1=\"0\" x2=\"900\" y2=\"100\" />\
                    </g>\
                <g class=\"grid-labels\" fill=\"#777\">\
                    <text x=\"5\" y=\"20\">0 ms</text>\
                    <text x=\"105\" y=\"20\">100 ms</text>\
                    <text x=\"205\" y=\"20\">200 ms</text>\
                    <text x=\"305\" y=\"20\">300 ms</text>\
                    <text x=\"405\" y=\"20\">400 ms</text>\
                    <text x=\"505\" y=\"20\">500 ms</text>\
                    <text x=\"605\" y=\"20\">600 ms</text>\
                    <text x=\"705\" y=\"20\">700 ms</text>\
                    <text x=\"805\" y=\"20\">800 ms</text>\
                    <text x=\"905\" y=\"20\">900 ms</text>\
                </g>\
            </g>"
        );
    }
}
