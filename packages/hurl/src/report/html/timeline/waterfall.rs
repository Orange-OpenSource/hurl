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
use std::iter::zip;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::http::Call;
use crate::report::html::timeline::nice::NiceScale;
use crate::report::html::timeline::svg::Attribute::{
    Class, Fill, Filter, FloodOpacity, FontFamily, FontSize, FontWeight, Height, Href, Id, Opacity,
    StdDeviation, Stroke, StrokeWidth, TextDecoration, ViewBox, Width, DX, DY, X, Y,
};
use crate::report::html::timeline::svg::{new_a, Element};
use crate::report::html::timeline::unit::{
    Byte, Interval, Microsecond, Millisecond, Pixel, Px, Scale, Second, TimeUnit,
};
use crate::report::html::timeline::util::{
    new_failure_icon, new_retry_icon, new_stripes, new_success_icon, trunc_str,
};
use crate::report::html::timeline::{svg, CallContext, CallContextKind, CALL_HEIGHT, CALL_INSET};
use crate::report::html::Testcase;
use crate::util::redacted::Redact;

/// Returns the start and end date for these entries.
fn get_times_interval(calls: &[&Call]) -> Option<Interval<DateTime<Utc>>> {
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

impl Testcase {
    /// Returns the SVG string of this list of `calls`.
    pub fn get_waterfall_svg(
        &self,
        calls: &[&Call],
        call_ctxs: &[CallContext],
        secrets: &[&str],
    ) -> String {
        // Compute our scale (transform 0 based microsecond to 0 based pixels):
        let times = get_times_interval(calls);
        let times = match times {
            Some(t) => t,
            None => return String::new(),
        };

        let margin_top = 50.px();
        let margin_bottom = 250.px();
        let width = 1138.px();
        let height = (CALL_HEIGHT * calls.len()) + margin_top + margin_bottom;
        let height = Pixel::max(100.px(), height);

        let mut root = svg::new_svg();
        root.add_attr(ViewBox(0.0, 0.0, width.0, height.0));
        root.add_attr(Width(width.0.to_string()));
        root.add_attr(Height(height.0.to_string()));

        // Add styles, filters, symbols for success and failure icons:
        let elt = svg::new_style(include_str!("../resources/waterfall.css"));
        root.add_child(elt);
        let elt = new_filters();
        root.add_child(elt);
        let elt = new_success_icon("success");
        root.add_child(elt);
        let elt = new_failure_icon("failure");
        root.add_child(elt);
        let elt = new_retry_icon("retry");
        root.add_child(elt);

        // We add some space for the right last grid labels.
        let pixels_x = Interval::new(0.px(), width);
        let pixels_y = Interval::new(margin_top, height);
        let scale_x = Scale::new(times, pixels_x);

        let ticks_number = 10;
        let grid = new_grid(
            calls,
            times,
            ticks_number,
            scale_x,
            pixels_x,
            pixels_y,
            CALL_HEIGHT,
        );
        root.add_child(grid);

        let elts = zip(calls, call_ctxs).map(|(call, call_ctx)| {
            new_call(call, call_ctx, times, scale_x, pixels_x, pixels_y, secrets)
        });

        // We construct SVG calls from last to first so the detail of any call is not overridden
        // by the next call.
        elts.rev().for_each(|e| root.add_child(e));

        root.to_string()
    }
}

/// Returns the grid SVG with tick on "nice" times and stripes background.
fn new_grid(
    calls: &[&Call],
    times: Interval<DateTime<Utc>>,
    ticks_number: usize,
    scale_x: Scale,
    pixels_x: Interval<Pixel>,
    pixels_y: Interval<Pixel>,
    call_height: Pixel,
) -> Element {
    let mut grid = svg::new_group();
    let elt = new_stripes(calls.len(), call_height, pixels_x, pixels_y, "#f5f5f5");
    grid.add_child(elt);
    let elt = new_vert_lines(times, ticks_number, scale_x, pixels_y);
    grid.add_child(elt);
    grid
}

/// Returns the verticals lines with labels for the time ticks.
fn new_vert_lines(
    times: Interval<DateTime<Utc>>,
    ticks_number: usize,
    scale_x: Scale,
    pixels_y: Interval<Pixel>,
) -> Element {
    let mut group = svg::new_group();

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
        let x = x.0.round();
        values.push((x, t));
        t = t.add_raw(nice_scale.get_tick_spacing());
    }

    // Draw the vertical lines:
    let mut lines = svg::new_group();
    lines.add_attr(Class("grid-ticks".to_string()));
    lines.add_attr(Stroke("#ccc".to_string()));
    values.iter().for_each(|(x, _)| {
        if *x <= 0.0 {
            return;
        }
        let elt = svg::new_line(*x, 0.0, *x, pixels_y.end.0);
        lines.add_child(elt);
    });
    group.add_child(lines);

    // Finally, draw labels
    let mut labels = svg::new_group();
    labels.add_attr(FontSize("15px".to_string()));
    labels.add_attr(FontFamily("sans-serif".to_string()));
    labels.add_attr(Fill("#777".to_string()));
    values
        .iter()
        .map(|(x, t)| svg::new_text(*x + 5.0, 20.0, &format!("{} {}", t.as_f64(), t.unit())))
        .for_each(|l| labels.add_child(l));
    group.add_child(labels);

    group
}

/// Returns the SVG of this `call`.
/// `times` is the time interval of the complete run, `scale_x` allows to convert
/// between times and pixel for the X-axis.
fn new_call(
    call: &Call,
    call_ctx: &CallContext,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    pixels_x: Interval<Pixel>,
    pixels_y: Interval<Pixel>,
    secrets: &[&str],
) -> Element {
    let mut call_elt = svg::new_group();

    let summary = new_call_timings(call, call_ctx, times, scale_x, pixels_y);
    call_elt.add_child(summary);

    let detail = new_call_tooltip(call, call_ctx, times, scale_x, pixels_x, pixels_y, secrets);
    call_elt.add_child(detail);

    call_elt
}

/// Returns the SVG timings block of this `call`.
fn new_call_timings(
    call: &Call,
    call_ctx: &CallContext,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    pixels_y: Interval<Pixel>,
) -> Element {
    let mut group = svg::new_group();
    group.add_attr(Class("call-summary".to_string()));

    let offset_y = CALL_HEIGHT * (call_ctx.call_index - 1) + pixels_y.start;
    let y = offset_y + CALL_INSET;
    let height = CALL_HEIGHT - CALL_INSET * 2;

    // DNS
    let dns_x = (call.timings.begin_call - times.start)
        .to_std()
        .unwrap_or_default();
    let dns_x = to_pixel(dns_x, scale_x);
    let dns_width = to_pixel(call.timings.name_lookup, scale_x);
    if dns_width.0 > 0.0 {
        let elt = svg::new_rect(dns_x.0, y.0, dns_width.0, height.0, "#1d9688");
        group.add_child(elt);
    }

    // TCP Handshake
    let tcp_x = to_pixel(call.timings.name_lookup, scale_x) + dns_x;
    let tcp_width = call.timings.connect.checked_sub(call.timings.name_lookup);
    if let Some(tcp_width) = tcp_width {
        let tcp_width = to_pixel(tcp_width, scale_x);
        if tcp_width.0 > 0.0 {
            let elt = svg::new_rect(tcp_x.0, y.0, tcp_width.0, height.0, "#fa7f03");
            group.add_child(elt);
        }
    }

    // SSL
    let ssl_x = to_pixel(call.timings.connect, scale_x) + dns_x;
    let ssl_width = call.timings.app_connect.checked_sub(call.timings.connect);
    if let Some(ssl_width) = ssl_width {
        let ssl_width = to_pixel(ssl_width, scale_x);
        if ssl_width.0 > 0.0 {
            let elt = svg::new_rect(ssl_x.0, y.0, ssl_width.0, height.0, "#9933ff");
            group.add_child(elt);
        }
    }

    // Wait
    let wait_x = to_pixel(call.timings.pre_transfer, scale_x) + dns_x;
    let wait_width = call
        .timings
        .start_transfer
        .checked_sub(call.timings.pre_transfer);
    if let Some(wait_width) = wait_width {
        let wait_width = to_pixel(wait_width, scale_x);
        if wait_width.0 > 0.0 {
            let elt = svg::new_rect(wait_x.0, y.0, wait_width.0, height.0, "#18c852");
            group.add_child(elt);
        }
    }

    // Data transfer
    let data_transfer_x = to_pixel(call.timings.start_transfer, scale_x) + dns_x;
    let data_transfer_width = call.timings.total.checked_sub(call.timings.start_transfer);
    if let Some(data_transfer_width) = data_transfer_width {
        let data_transfer_width = to_pixel(data_transfer_width, scale_x);
        if data_transfer_width.0 > 0.0 {
            let elt = svg::new_rect(
                data_transfer_x.0,
                y.0,
                data_transfer_width.0,
                height.0,
                "#36a9f4",
            );
            group.add_child(elt);
        }
    }

    group
}

/// Returns the SVG detail of this `call`.
/// Each call timings value is displayed under the timings (see [`crate::http::Timings`]).
fn new_call_tooltip(
    call: &Call,
    call_ctx: &CallContext,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    pixels_x: Interval<Pixel>,
    pixels_y: Interval<Pixel>,
    secrets: &[&str],
) -> Element {
    let mut group = svg::new_group();
    group.add_attr(Class("call-detail".to_string()));
    group.add_attr(FontFamily("sans-serif".to_string()));
    group.add_attr(FontSize("17px".to_string()));

    let width = 600.px();
    let height = 235.px();
    let offset_x = (call.timings.begin_call - times.start)
        .to_std()
        .unwrap_or_default();
    let offset_x = to_pixel(offset_x, scale_x);
    let offset_y = CALL_HEIGHT * (call_ctx.call_index - 1) + pixels_y.start;
    let offset_y = offset_y + CALL_HEIGHT - CALL_INSET;
    let max_width = pixels_x.end - pixels_x.start;
    // We bound the tooltip background to the overall bounding box.
    let offset_x = Pixel::max(offset_x, 6.px());
    let offset_x = Pixel::min(offset_x, max_width - width - 6.px());

    let selection = new_call_sel(call, call_ctx, times, scale_x, pixels_y);
    group.add_child(selection);

    let mut x = offset_x;
    let mut y = offset_y;

    let mut elt = svg::new_rect(x.0, y.0, width.0, height.0, "white");
    elt.add_attr(Class("call-back".to_string()));
    elt.add_attr(Filter("url(#shadow)".to_string()));
    elt.add_attr(Stroke("#ccc".to_string()));
    elt.add_attr(StrokeWidth(1.0));
    group.add_child(elt);

    x += 14.px();
    y += 14.px();
    let delta_y = 30.px();

    let mut legend = svg::new_group();
    legend.add_attr(Class("call-legend".to_string()));
    legend.add_attr(Fill("#555".to_string()));

    // Icon + URL + method
    let mut elt = svg::new_use();
    let icon = match call_ctx.kind {
        CallContextKind::Success => "#success",
        CallContextKind::Failure => "#failure",
        CallContextKind::Retry => "#retry",
    };
    elt.add_attr(Href(icon.to_string()));
    elt.add_attr(X(x.0));
    elt.add_attr(Y(y.0));
    elt.add_attr(Width("20".to_string()));
    elt.add_attr(Height("20".to_string()));
    legend.add_child(elt);

    let url = call.request.url.to_string().redact(secrets);
    let text = format!("{} {}", call.request.method, url);
    let text = trunc_str(&text, 54);
    let text = format!("{text}  {}", call.response.status);
    let mut elt = svg::new_text(x.0 + 30.0, y.0 + 16.0, &text);
    if call_ctx.kind == CallContextKind::Failure {
        elt.add_attr(Fill("red".to_string()));
    }
    elt.add_attr(FontWeight("bold".to_string()));
    legend.add_child(elt);

    x += 12.px();
    y += 32.px();

    // DNS
    let duration = call.timings.name_lookup.as_micros();
    let duration = Microsecond(duration as f64);
    let elt = new_legend(x, y, "DNS lookup", Some("#1d9688"), duration);
    legend.add_child(elt);
    y += delta_y;

    // TCP handshake
    let duration = call.timings.connect.checked_sub(call.timings.name_lookup);
    if let Some(duration) = duration {
        let duration = Microsecond(duration.as_micros() as f64);
        let elt = new_legend(x, y, "TCP handshake", Some("#fa7f03"), duration);
        legend.add_child(elt);
        y += delta_y;
    }

    // SSL handshake
    let duration = call.timings.app_connect.checked_sub(call.timings.connect);
    if let Some(duration) = duration {
        let duration = Microsecond(duration.as_micros() as f64);
        let elt = new_legend(x, y, "SSL handshake", Some("#9933ff"), duration);
        legend.add_child(elt);
        y += delta_y;
    }

    // Wait
    let duration = call
        .timings
        .start_transfer
        .checked_sub(call.timings.pre_transfer);
    if let Some(duration) = duration {
        let duration = Microsecond(duration.as_micros() as f64);
        let elt = new_legend(x, y, "Wait", Some("#18c852"), duration);
        legend.add_child(elt);
        y += delta_y;
    }

    // Data transfer
    let duration = call.timings.total.checked_sub(call.timings.start_transfer);
    if let Some(duration) = duration {
        let duration = Microsecond(duration.as_micros() as f64);
        let elt = new_legend(x, y, "Data transfer", Some("#36a9f4"), duration);
        legend.add_child(elt);
        y += delta_y;
    }

    // Total
    let duration = call.timings.total.as_micros();
    let duration = Microsecond(duration as f64);
    let mut elt = new_legend(x, y, "Total", None, duration);
    elt.add_attr(FontWeight("bold".to_string()));
    legend.add_child(elt);
    y += delta_y;

    // Start and stop timestamps
    let start = (call.timings.begin_call - times.start)
        .to_std()
        .unwrap_or_default();
    let end = (call.timings.end_call - times.start)
        .to_std()
        .unwrap_or_default();
    x = offset_x + 380.px();
    y = offset_y + 64.px();
    let value = Microsecond(start.as_micros() as f64);
    let value = value.to_human_string();
    let elt = new_value("Start:", &value, x, y);
    legend.add_child(elt);
    y += delta_y;
    let value = Microsecond(end.as_micros() as f64);
    let value = value.to_human_string();
    let elt = new_value("Stop:", &value, x, y);
    legend.add_child(elt);

    y += delta_y;
    let value = Byte(call.response.body.len() as f64);
    let value = value.to_human_string();
    let elt = new_value("Transferred:", &value, x, y);
    legend.add_child(elt);

    // Run URL
    y += 56.px();
    let href = format!(
        "{}#e{}:c{}",
        call_ctx.run_filename, call_ctx.entry_index, call_ctx.call_entry_index
    );
    let elt = new_link(x, y, "(view run)", &href);
    legend.add_child(elt);

    // Source URL
    let href = format!("{}#l{}", call_ctx.source_filename, call_ctx.line);
    let elt = new_link(x + 90.px(), y, "(view source)", &href);
    legend.add_child(elt);

    // Timings explanation
    y += delta_y;
    let elt = new_link(
        x,
        y,
        "Explanation",
        "https://hurl.dev/docs/response.html#timings",
    );
    legend.add_child(elt);
    group.add_child(legend);

    group
}

fn new_link(x: Pixel, y: Pixel, text: &str, href: &str) -> Element {
    let mut elt = svg::new_text(x.0, y.0, text);
    elt.add_attr(Fill("royalblue".to_string()));
    elt.add_attr(TextDecoration("underline".to_string()));
    let mut a = new_a(href);
    a.add_child(elt);
    a
}

/// Returns the highlighted span time of a call.
fn new_call_sel(
    call: &Call,
    call_ctx: &CallContext,
    times: Interval<DateTime<Utc>>,
    scale_x: Scale,
    pixels_y: Interval<Pixel>,
) -> Element {
    let offset_x_start = (call.timings.begin_call - times.start)
        .to_std()
        .unwrap_or_default();
    let offset_x_start = to_pixel(offset_x_start, scale_x);
    let offset_x_end = (call.timings.end_call - times.start)
        .to_std()
        .unwrap_or_default();
    let offset_x_end = to_pixel(offset_x_end, scale_x);
    let color = match call_ctx.kind {
        CallContextKind::Success | CallContextKind::Retry => "green",
        CallContextKind::Failure => "red",
    };
    let mut elt = svg::new_rect(
        offset_x_start.0,
        0.0,
        (offset_x_end - offset_x_start).0,
        pixels_y.end.0,
        color,
    );
    elt.add_attr(Opacity(0.05));
    elt.add_attr(Class("call-sel".to_string()));
    elt
}

fn new_legend(
    x: Pixel,
    y: Pixel,
    text: &str,
    color: Option<&str>,
    duration: Microsecond,
) -> Element {
    let dx_label = 36.px();
    let dy_label = 17.px();
    let dx_duration = 180.px();

    let mut group = svg::new_group();

    if let Some(color) = color {
        let color_elt = svg::new_rect(x.0, y.0, 20.0, 20.0, color);
        group.add_child(color_elt);
    }

    let text_elt = svg::new_text((x + dx_label).0, (y + dy_label).0, text);
    group.add_child(text_elt);

    let duration = duration.to_human_string();
    let duration_elt = svg::new_text((x + dx_duration).0, (y + dy_label).0, &duration);
    group.add_child(duration_elt);

    group
}

fn new_value(label: &str, value: &str, x: Pixel, y: Pixel) -> Element {
    let mut group = svg::new_group();
    let elt = svg::new_text(x.0, y.0, label);
    group.add_child(elt);

    let x = x + 100.px();
    let elt = svg::new_text(x.0, y.0, value);
    group.add_child(elt);

    group
}

/// Converts a `duration` to pixel, using `scale_x`.
fn to_pixel(duration: Duration, scale_x: Scale) -> Pixel {
    let value = duration.as_micros();
    let value = Microsecond(value as f64);
    scale_x.to_pixel(value)
}

/// Creates SVG filters for the waterfall (used by drop shadow of call tooltip).
fn new_filters() -> Element {
    let mut defs = svg::new_defs();

    let mut filter = svg::new_filter();
    filter.add_attr(Id("shadow".to_string()));

    let mut shadow = svg::new_fe_drop_shadow();
    shadow.add_attr(DX(0.0));
    shadow.add_attr(DY(4.0));
    shadow.add_attr(StdDeviation(4.0));
    shadow.add_attr(FloodOpacity(0.25));
    filter.add_child(shadow);
    defs.add_child(filter);
    defs
}

impl Microsecond {
    /// Returns a human readable string of a microsecond.
    fn to_human_string(self) -> String {
        match self.0 {
            d if d < 0.0 => "_".to_string(),
            d if d < 1_000.0 => format!("{d:.1} µs"),
            d if d < 1_000_000.0 => format!("{:.1} ms", d / 1_000.0),
            d => format!("{:.1} s", d / 1_000_000.0),
        }
    }
}

impl Byte {
    /// Returns a human readable string of a byte.
    fn to_human_string(self) -> String {
        match self.0 {
            d if d < 0.0 => "_".to_string(),
            d if d < 1_000.0 => format!("{d:.1} B"),
            d if d < 1_000_000.0 => format!("{:.1} kB", d / 1_000.0),
            d => format!("{:.1} MB", d / 1_000_000.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use super::*;
    use crate::report::html::timeline::unit::{Interval, Microsecond, Scale};

    #[test]
    fn legend_svg() {
        let x = 20.px();
        let y = 30.px();
        let text = "Hello world";
        let color = "red";
        let duration = Microsecond(2000.0);

        let elt = new_legend(x, y, text, Some(color), duration);
        assert_eq!(
            elt.to_string(),
            "<g>\
                <rect x=\"20\" y=\"30\" width=\"20\" height=\"20\" fill=\"red\" />\
                <text x=\"56\" y=\"47\">Hello world</text>\
                <text x=\"200\" y=\"47\">2.0 ms</text>\
            </g>"
        );
    }

    #[test]
    fn grid_vert_lines_svg() {
        let start = Utc.with_ymd_and_hms(2022, 1, 1, 8, 0, 0).unwrap();
        let end = start + Duration::try_seconds(1).unwrap();
        let times = Interval { start, end };
        let start = 0.px();
        let end = 1000.px();
        let pixels_x = Interval { start, end };
        let start = 0.px();
        let end = 100.px();
        let pixels_y = Interval { start, end };
        let scale_x = Scale::new(times, pixels_x);
        let ticks_number = 10;

        let elt = new_vert_lines(times, ticks_number, scale_x, pixels_y);
        assert_eq!(
            elt.to_string(),
            "<g>\
                <g class=\"grid-ticks\" stroke=\"#ccc\">\
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
                <g font-size=\"15px\" font-family=\"sans-serif\" fill=\"#777\">\
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
