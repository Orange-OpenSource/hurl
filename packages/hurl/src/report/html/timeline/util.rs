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
use crate::report::html::timeline::svg;
use crate::report::html::timeline::svg::Attribute::{Class, Fill, Id, ViewBox};
use crate::report::html::timeline::svg::Element;
use crate::report::html::timeline::unit::{Interval, Pixel, Px};

/// Truncates a `text` if there are more than `max_len` chars (with ellipsis).
pub fn trunc_str(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[0..max_len])
    } else {
        text.to_string()
    }
}

/// Returns the stripe background SVG (1 call over 2)
pub fn new_stripes(
    nb_stripes: usize,
    stripe_height: Pixel,
    pixels_x: Interval<Pixel>,
    pixels_y: Interval<Pixel>,
    color: &str,
) -> Element {
    let mut group = svg::new_group();
    group.add_attr(Class("grid-strip".to_string()));
    let x = pixels_x.start;
    let width = pixels_x.end - pixels_x.start;

    // We want to have an odd number of stripes to have a filled strip at the bottom.
    let nb_calls = 2 * (nb_stripes / 2) + 1;
    (0..nb_calls)
        .step_by(2)
        .map(|index| {
            svg::new_rect(
                x.0,
                (index as f64) * stripe_height.0 + pixels_y.start.0,
                width.0,
                stripe_height.0,
                color,
            )
        })
        .for_each(|r| group.add_child(r));
    group
}

/// Returns the SVG success icon identified by `id`.
pub fn new_success_icon(id: &str) -> Element {
    new_icon(id, 512.px(), 512.px(), "M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209L241 337c-9.4 9.4-24.6 9.4-33.9 0l-64-64c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l47 47L335 175c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9z", "#10bb00")
}

/// Returns the SVG failure icon identified by `id`.
pub fn new_failure_icon(id: &str) -> Element {
    new_icon(id, 512.px(), 512.px(), "M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zm0-384c13.3 0 24 10.7 24 24V264c0 13.3-10.7 24-24 24s-24-10.7-24-24V152c0-13.3 10.7-24 24-24zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z", "red")
}

/// Returns the SVG retry icon identified by id.
pub fn new_retry_icon(id: &str) -> Element {
    new_icon(id, 512.px(), 512.px(), "M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zm0-384c13.3 0 24 10.7 24 24V264c0 13.3-10.7 24-24 24s-24-10.7-24-24V152c0-13.3 10.7-24 24-24zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z", "gold")
}

/// Returns a SVG icon identified by `id`, with a `width` pixel by `height` pixel size, `path` and `color`.
fn new_icon(id: &str, width: Pixel, height: Pixel, path: &str, color: &str) -> Element {
    let mut symbol = svg::new_symbol();
    symbol.add_attr(Id(id.to_string()));
    symbol.add_attr(ViewBox(0.0, 0.0, width.0, height.0));
    let mut path = svg::new_path(path);
    path.add_attr(Fill(color.to_string()));
    symbol.add_child(path);
    symbol
}

#[cfg(test)]
mod tests {
    use crate::report::html::timeline::unit::{Interval, Px};
    use crate::report::html::timeline::util::{new_stripes, trunc_str};

    #[test]
    fn truncates() {
        assert_eq!(trunc_str("foo", 32), "foo");
        assert_eq!(trunc_str("abcdefgh", 3), "abc...");
    }

    #[test]
    fn create_stripes() {
        let elt = new_stripes(
            10,
            1.px(),
            Interval::new(0.px(), 10.px()),
            Interval::new(0.px(), 10.px()),
            "green",
        );
        assert_eq!(
            elt.to_string(),
            "<g class=\"grid-strip\">\
                <rect x=\"0\" y=\"0\" width=\"10\" height=\"1\" fill=\"green\" />\
                <rect x=\"0\" y=\"2\" width=\"10\" height=\"1\" fill=\"green\" />\
                <rect x=\"0\" y=\"4\" width=\"10\" height=\"1\" fill=\"green\" />\
                <rect x=\"0\" y=\"6\" width=\"10\" height=\"1\" fill=\"green\" />\
                <rect x=\"0\" y=\"8\" width=\"10\" height=\"1\" fill=\"green\" />\
                <rect x=\"0\" y=\"10\" width=\"10\" height=\"1\" fill=\"green\" />\
            </g>"
        );
    }
}
