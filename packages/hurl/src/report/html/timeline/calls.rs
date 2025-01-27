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

use crate::http::Call;
use crate::report::html::timeline::svg::Attribute::{
    Class, Fill, FontFamily, FontSize, Height, Href, TextDecoration, ViewBox, Width, X, Y,
};
use crate::report::html::timeline::svg::{Element, ElementKind};
use crate::report::html::timeline::unit::{Pixel, Px};
use crate::report::html::timeline::util::{
    new_failure_icon, new_retry_icon, new_success_icon, trunc_str,
};
use crate::report::html::timeline::{svg, CallContext, CallContextKind, CALL_HEIGHT};
use crate::report::html::Testcase;
use crate::util::redacted::Redact;

impl Testcase {
    /// Returns a SVG view of `calls` list using contexts `call_ctxs`.
    pub fn get_calls_svg(
        &self,
        calls: &[&Call],
        call_ctxs: &[CallContext],
        secrets: &[&str],
    ) -> String {
        let margin_top = 50.px();
        let margin_bottom = 250.px();

        let call_height = 24.px();
        let width = 260.px();
        let height = call_height * calls.len() + margin_top + margin_bottom;
        let height = Pixel::max(100.px(), height);

        let mut root = svg::new_svg();
        root.add_attr(ViewBox(0.0, 0.0, width.0, height.0));
        root.add_attr(Width(width.0.to_string()));
        root.add_attr(Height(height.0.to_string()));

        // Add styles, symbols for success and failure icons:
        let elt = svg::new_style(include_str!("../resources/calls.css"));
        root.add_child(elt);

        let symbol = new_success_icon("success");
        root.add_child(symbol);
        let symbol = new_failure_icon("failure");
        root.add_child(symbol);
        let symbol = new_retry_icon("retry");
        root.add_child(symbol);

        // Add a flat background.
        let mut elt = Element::new(ElementKind::Rect);
        elt.add_attr(Class("calls-back".to_string()));
        elt.add_attr(X(0.0));
        elt.add_attr(Y(0.0));
        elt.add_attr(Width("100%".to_string()));
        elt.add_attr(Height("100%".to_string()));
        elt.add_attr(Fill("#fbfafd".to_string()));
        root.add_child(elt);

        if !calls.is_empty() {
            // Add horizontal lines
            let x = 0.px();
            let y = margin_top;
            let elt = new_grid(calls, y, width, height);
            root.add_child(elt);

            // Add calls info
            let elt = new_calls(calls, call_ctxs, x, y, secrets);
            root.add_child(elt);
        }

        root.to_string()
    }
}

/// Returns an SVG view of a list of `call`.
/// For instance:
///
/// `✅ GET www.google.fr 303 <run>`
fn new_calls(
    calls: &[&Call],
    call_ctxs: &[CallContext],
    offset_x: Pixel,
    offset_y: Pixel,
    secrets: &[&str],
) -> Element {
    let mut group = svg::new_group();
    group.add_attr(Class("calls-list".to_string()));
    group.add_attr(FontSize("13px".to_string()));
    group.add_attr(FontFamily("sans-serif".to_string()));
    group.add_attr(Fill("#777".to_string()));

    let margin_left = 13.px();

    zip(calls, call_ctxs)
        .enumerate()
        .for_each(|(index, (call, call_ctx))| {
            let mut x = offset_x + margin_left;
            let y = offset_y + (CALL_HEIGHT * index) + CALL_HEIGHT - 7.px();

            // Icon success / failure
            let mut elt = svg::new_use();
            let icon = match call_ctx.kind {
                CallContextKind::Success => "#success",
                CallContextKind::Failure => "#failure",
                CallContextKind::Retry => "#retry",
            };
            elt.add_attr(Href(icon.to_string()));
            elt.add_attr(X(x.0 - 6.0));
            elt.add_attr(Y(y.0 - 11.0));
            elt.add_attr(Width("13".to_string()));
            elt.add_attr(Height("13".to_string()));
            group.add_child(elt);

            x += 12.px();

            // URL
            let url = &call.request.url.to_string().redact(secrets);
            let url = url.strip_prefix("http://").unwrap_or(url);
            let url = url.strip_prefix("https://").unwrap_or(url);
            let text = format!("{} {url}", call.request.method);
            let text = trunc_str(&text, 24);
            let mut elt = svg::new_text(x.0, y.0, &text);
            if call_ctx.kind == CallContextKind::Failure {
                elt.add_attr(Fill("red".to_string()));
            }
            group.add_child(elt);

            // Status code
            x += 180.px();
            let text = format!("{}", call.response.status);
            let mut elt = svg::new_text(x.0, y.0, &text);
            if call_ctx.kind == CallContextKind::Failure {
                elt.add_attr(Fill("red".to_string()));
            }
            group.add_child(elt);

            // Source
            x += 28.px();
            let href = format!(
                "{}#e{}:c{}",
                call_ctx.run_filename, call_ctx.entry_index, call_ctx.call_entry_index
            );
            let mut a = svg::new_a(&href);
            let mut text = svg::new_text(x.0, y.0, "run");
            text.add_attr(Fill("royalblue".to_string()));
            text.add_attr(TextDecoration("underline".to_string()));
            a.add_child(text);
            group.add_child(a);
        });

    group
}

/// Returns a SVG view of the grid calls.
fn new_grid(calls: &[&Call], offset_y: Pixel, width: Pixel, height: Pixel) -> Element {
    let mut group = svg::new_group();
    group.add_attr(Class("calls-grid".to_string()));
    let nb_lines = 2 * (calls.len() / 2) + 2;
    (0..nb_lines).for_each(|index| {
        let y = CALL_HEIGHT * index + offset_y - (index % 2).px();
        let elt = svg::new_rect(0.0, y.0, width.0, 1.0, "#ddd");
        group.add_child(elt);
    });
    // Right borders:
    let elt = svg::new_rect(width.0 - 1.0, 0.0, 1.0, height.0, "#ddd");
    group.add_child(elt);
    group
}
