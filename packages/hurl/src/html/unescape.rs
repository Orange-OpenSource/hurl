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

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::html::entities::HTML5_ENTITIES_REF;

// Ref https://html.spec.whatwg.org/#decimal-character-reference-start-state

static INVALID_CHAR: [(u32, &str); 34] = [
    (0x00, "\u{fffd}"), // REPLACEMENT CHARACTER
    (0x0d, "\r"),       // CARRIAGE RETURN
    (0x80, "\u{20ac}"), // EURO SIGN
    (0x81, "\u{81}"),   // <control>
    (0x82, "\u{201a}"), // SINGLE LOW-9 QUOTATION MARK
    (0x83, "\u{0192}"), // LATIN SMALL LETTER F WITH HOOK
    (0x84, "\u{201e}"), // DOUBLE LOW-9 QUOTATION MARK
    (0x85, "\u{2026}"), // HORIZONTAL ELLIPSIS
    (0x86, "\u{2020}"), // DAGGER
    (0x87, "\u{2021}"), // DOUBLE DAGGER
    (0x88, "\u{02c6}"), // MODIFIER LETTER CIRCUMFLEX ACCENT
    (0x89, "\u{2030}"), // PER MILLE SIGN
    (0x8a, "\u{0160}"), // LATIN CAPITAL LETTER S WITH CARON
    (0x8b, "\u{2039}"), // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
    (0x8c, "\u{0152}"), // LATIN CAPITAL LIGATURE OE
    (0x8d, "\u{8d}"),   // <control>
    (0x8e, "\u{017d}"), // LATIN CAPITAL LETTER Z WITH CARON
    (0x8f, "\u{8f}"),   // <control>
    (0x90, "\u{90}"),   // <control>
    (0x91, "\u{2018}"), // LEFT SINGLE QUOTATION MARK
    (0x92, "\u{2019}"), // RIGHT SINGLE QUOTATION MARK
    (0x93, "\u{201c}"), // LEFT DOUBLE QUOTATION MARK
    (0x94, "\u{201d}"), // RIGHT DOUBLE QUOTATION MARK
    (0x95, "\u{2022}"), // BULLET
    (0x96, "\u{2013}"), // EN DASH
    (0x97, "\u{2014}"), // EM DASH
    (0x98, "\u{02dc}"), // SMALL TILDE
    (0x99, "\u{2122}"), // TRADE MARK SIGN
    (0x9a, "\u{0161}"), // LATIN SMALL LETTER S WITH CARON
    (0x9b, "\u{203a}"), // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
    (0x9c, "\u{0153}"), // LATIN SMALL LIGATURE OE
    (0x9d, "\u{9d}"),   // <control>
    (0x9e, "\u{017e}"), // LATIN SMALL LETTER Z WITH CARON
    (0x9f, "\u{0178}"), // LATIN CAPITAL LETTER Y WITH DIAERESIS
];

lazy_static! {
    static ref INVALID_CHAR_REF: HashMap<u32, &'static str> =
        INVALID_CHAR.iter().copied().collect();
}

static INVALID_CODEPOINTS: [u32; 126] = [
    // 0x0001 to 0x0008
    0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, // 0x000E to 0x001F
    0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
    0x1e, 0x1f, // 0x007F to 0x009F
    0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
    0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e,
    0x9f, // 0xFDD0 to 0xFDEF
    0xfdd0, 0xfdd1, 0xfdd2, 0xfdd3, 0xfdd4, 0xfdd5, 0xfdd6, 0xfdd7, 0xfdd8, 0xfdd9, 0xfdda, 0xfddb,
    0xfddc, 0xfddd, 0xfdde, 0xfddf, 0xfde0, 0xfde1, 0xfde2, 0xfde3, 0xfde4, 0xfde5, 0xfde6, 0xfde7,
    0xfde8, 0xfde9, 0xfdea, 0xfdeb, 0xfdec, 0xfded, 0xfdee, 0xfdef, // Others
    0xb, 0xfffe, 0xffff, 0x1fffe, 0x1ffff, 0x2fffe, 0x2ffff, 0x3fffe, 0x3ffff, 0x4fffe, 0x4ffff,
    0x5fffe, 0x5ffff, 0x6fffe, 0x6ffff, 0x7fffe, 0x7ffff, 0x8fffe, 0x8ffff, 0x9fffe, 0x9ffff,
    0xafffe, 0xaffff, 0xbfffe, 0xbffff, 0xcfffe, 0xcffff, 0xdfffe, 0xdffff, 0xefffe, 0xeffff,
    0xffffe, 0xfffff, 0x10fffe, 0x10ffff,
];

lazy_static! {
    static ref CHAR_REF: Regex = Regex::new(concat!(
        r"&(#\d+;?",
        r"|#[xX][\da-fA-F]+;?",
        r"|[^\t\n\f <&#;]{1,32};?)",
    ))
    .unwrap();
}

/// Convert all named and numeric character references (e.g. &gt;, &#62;,
/// &x3e;) in the string `text` to the corresponding unicode characters.
/// This function uses the rules defined by the HTML 5 standard
/// for both valid and invalid character references, and the list of
/// HTML 5 named character references defined in html.entities.html5.
///
/// The code is adapted from the Python standard library:
/// <https://github.com/python/cpython/blob/main/Lib/html/__init__.py>
///
/// See MDN decoder tool: <https://mothereff.in/html-entities>
pub fn html_unescape(text: &str) -> String {
    if text.chars().any(|c| c == '&') {
        CHAR_REF
            .replace_all(text, |caps: &Captures| {
                let s = &caps[1];
                let s0 = s.chars().next().unwrap();
                if s0 == '#' {
                    // Numeric charref
                    let s1 = s.chars().nth(1).unwrap();
                    let num = if s1 == 'x' || s1 == 'X' {
                        let val = s[2..].trim_end_matches(';');
                        match u32::from_str_radix(val, 16) {
                            Ok(val) => val,
                            Err(_) => return "\u{FFFD}".to_string(),
                        }
                    } else {
                        let val = s[1..].trim_end_matches(';');
                        match val.parse::<u32>() {
                            Ok(val) => val,
                            Err(_) => return "\u{FFFD}".to_string(),
                        }
                    };
                    if let Some(char) = INVALID_CHAR_REF.get(&num) {
                        return char.to_string();
                    }
                    if (0xD800..=0xDFFF).contains(&num) || num > 0x10FFFF {
                        return "\u{FFFD}".to_string();
                    }
                    if INVALID_CODEPOINTS.contains(&num) {
                        return String::new();
                    }
                    char::from_u32(num).unwrap().to_string()
                } else {
                    if let Some(entity) = HTML5_ENTITIES_REF.get(s) {
                        return entity.to_string();
                    }
                    // Find the longest matching name (as defined by the standard)
                    for x in (1..s.len()).rev() {
                        let name = &s[..x];
                        if let Some(entity) = HTML5_ENTITIES_REF.get(name) {
                            return format!("{}{}", entity, &s[x..]);
                        }
                    }
                    format!("&{s}")
                }
            })
            .to_string()
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::html_unescape;

    /// Extracts from Python test suites: https://github.com/python/cpython/blob/main/Lib/test/test_html.py
    #[test]
    fn test_html_unescape() {
        fn check(text: &str, expected: &str) {
            assert_eq!(html_unescape(text), expected.to_string());
        }

        fn check_num(num: usize, expected: &str) {
            let text = format!("&#{num}");
            check(&text, expected);
            let text = format!("&#{num};");
            check(&text, expected);
            let text = format!("&#x{num:x}");
            check(&text, expected);
            let text = format!("&#x{num:x};");
            check(&text, expected);
        }

        check("Hurl&rlarr;", "Hurl⇄");

        // Check simple
        check(
            "Foo &#xA9; bar &#x1D306; baz &#x2603; qux",
            "Foo © bar 𝌆 baz ☃ qux",
        );

        // Check text with no character references
        check("no character references", "no character references");

        // Check & followed by invalid chars
        check("&\n&\t& &&", "&\n&\t& &&");

        // Check & followed by numbers and letters
        check("&0 &9 &a &0; &9; &a;", "&0 &9 &a &0; &9; &a;");

        // Check incomplete entities at the end of the string
        for x in ["&", "&#", "&#x", "&#X", "&#y", "&#xy", "&#Xy"].iter() {
            check(x, x);
            check(&format!("{x};"), &format!("{x};"));
        }

        // Check several combinations of numeric character references,
        // possibly followed by different characters

        // Format &#1234 (without ending semi-colon)
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#{num}"), &format!("{char}"));
            check(&format!("&#{num} "), &format!("{char} "));
            check(&format!("&#{num}X"), &format!("{char}X"));
        }

        // Format &#0001234 (without ending semi-colon)
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#{num:07}"), &format!("{char}"));
            check(&format!("&#{num:07} "), &format!("{char} "));
            check(&format!("&#{num:07}X"), &format!("{char}X"));
        }

        // Format &#1234;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#{num};"), &format!("{char}"));
            check(&format!("&#{num}; "), &format!("{char} "));
            check(&format!("&#{num};X"), &format!("{char}X"));
        }

        // Format &#0001234;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#{num:07};"), &format!("{char}"));
            check(&format!("&#{num:07}; "), &format!("{char} "));
            check(&format!("&#{num:07};X"), &format!("{char}X"));
        }

        // Format &#x1abc
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:x}"), &format!("{char}"));
            check(&format!("&#x{num:x} "), &format!("{char} "));
            check(&format!("&#x{num:x}X"), &format!("{char}X"));
        }

        // Format &#x001abc
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:06x}"), &format!("{char}"));
            check(&format!("&#x{num:06x} "), &format!("{char} "));
            check(&format!("&#x{num:06x}X"), &format!("{char}X"));
        }

        // Format &#x1abc;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:x};"), &format!("{char}"));
            check(&format!("&#x{num:x}; "), &format!("{char} "));
            check(&format!("&#x{num:x};X"), &format!("{char}X"));
        }

        // Format &#x001abc;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:06x};"), &format!("{char}"));
            check(&format!("&#x{num:06x}; "), &format!("{char} "));
            check(&format!("&#x{num:06x};X"), &format!("{char}X"));
        }

        // Format &#x1ABC
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:X}"), &format!("{char}"));
            check(&format!("&#x{num:X} "), &format!("{char} "));
            check(&format!("&#x{num:X}X"), &format!("{char}X"));
        }

        // Format &#x001ABC
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:06X}"), &format!("{char}"));
            check(&format!("&#x{num:06X} "), &format!("{char} "));
            check(&format!("&#x{num:06X}X"), &format!("{char}X"));
        }

        // Format &#x1ABC;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:X};"), &format!("{char}"));
            check(&format!("&#x{num:X}; "), &format!("{char} "));
            check(&format!("&#x{num:X};X"), &format!("{char}X"));
        }

        // Format &#x001ABC;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#x{num:06X};"), &format!("{char}"));
            check(&format!("&#x{num:06X}; "), &format!("{char} "));
            check(&format!("&#x{num:06X};X"), &format!("{char}X"));
        }

        // Format &#X1abc;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#X{num:x};"), &format!("{char}"));
            check(&format!("&#X{num:x}; "), &format!("{char} "));
            check(&format!("&#X{num:x};X"), &format!("{char}X"));
        }

        // Format &#X001abc;
        for (num, char) in [
            (65, 'A'),
            (97, 'a'),
            (34, '"'),
            (38, '&'),
            (0x2603, '\u{2603}'),
            (0x101234, '\u{101234}'),
        ]
        .iter()
        {
            check(&format!("&#X{num:06x};"), &format!("{char}"));
            check(&format!("&#X{num:06x}; "), &format!("{char} "));
            check(&format!("&#X{num:06x};X"), &format!("{char}X"));
        }

        // Check invalid code points
        for cp in [0xD800, 0xDB00, 0xDC00, 0xDFFF, 0x110000] {
            check_num(cp, "\u{FFFD}");
        }

        // Check more invalid code points
        for cp in [0x1, 0xb, 0xe, 0x7f, 0xfffe, 0xffff, 0x10fffe, 0x10ffff] {
            check_num(cp, "");
        }

        // Check invalid numbers
        for (num, ch) in [(0x0d, "\r"), (0x80, "\u{20ac}"), (0x95, "\u{2022}")] {
            check_num(num, ch);
        }

        // Check small numbers
        check_num(0, "\u{FFFD}");
        check_num(9, "\t");

        // Check a big number
        check_num(1000000000000000000, "\u{FFFD}");

        // Check that multiple trailing semicolons are handled correctly
        for e in ["&quot;;", "&#34;;", "&#x22;;", "&#X22;;"] {
            check(e, "\";");
        }

        // Check that semicolons in the middle don't create problems
        for e in ["&quot;quot;", "&#34;quot;", "&#x22;quot;", "&#X22;quot;"] {
            check(e, "\"quot;");
        }

        // Check triple adjacent charrefs
        for e in ["&quot", "&#34", "&#x22", "&#X22"] {
            // check(&e.repeat(3), "\"\"\"");
            check(&format!("{e};").repeat(3), "\"\"\"");
        }

        // Check that the case is respected
        for e in ["&amp", "&amp;", "&AMP", "&AMP;"] {
            check(e, "&");
        }
        for e in ["&Amp", "&Amp;"] {
            check(e, e);
        }

        // Check that nonexistent named entities are returned unchanged
        check("&svadilfari;", "&svadilfari;");

        // The following examples are in the html5 specs
        check("&notit", "¬it");
        check("&notit;", "¬it;");
        check("&notin", "¬in");
        check("&notin;", "∉");

        // A similar example with a long name
        check(
            "&notReallyAnExistingNamedCharacterReference;",
            "¬ReallyAnExistingNamedCharacterReference;",
        );

        // Longest valid name
        check("&CounterClockwiseContourIntegral;", "∳");

        // Check a charref that maps to two unicode chars
        check("&acE;", "\u{223e}\u{333}");
        check("&acE", "&acE");

        // See Python #12888
        check(&"&#123; ".repeat(1050), &"{ ".repeat(1050));
        // See Python #15156
        check(
            "&Eacuteric&Eacute;ric&alphacentauri&alpha;centauri",
            "ÉricÉric&alphacentauriαcentauri",
        );
        check("&co;", "&co;");
    }
}
