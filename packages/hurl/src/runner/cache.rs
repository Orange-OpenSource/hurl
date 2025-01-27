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
use serde_json::Value;

use crate::runner::xpath::Document;

/// This is a cache to hold parsed structured data (XML/JSON/text), computed from an HTTP response
/// body bytes. This cache lives for a given request, and allows reusing parsed response for
/// multiple queries of the same type (for instance, two XPath queries will share their XML document
/// through this cache).
#[derive(Default)]
pub struct BodyCache {
    /// The parsed XML document.
    xml: Option<Document>,
    /// The parsed JSON body
    json: Option<Value>,
}

impl BodyCache {
    /// Creates a new empty cache.
    pub fn new() -> Self {
        BodyCache::default()
    }

    /// Returns a reference to a cached XML response.
    pub fn xml(&self) -> Option<&Document> {
        self.xml.as_ref()
    }

    /// Caches a XML document `doc`.
    pub fn set_xml(&mut self, xml: Document) {
        self.xml = Some(xml);
    }

    /// Returns a reference to a cached JSON response.
    pub fn json(&self) -> Option<&Value> {
        self.json.as_ref()
    }

    /// Caches a parsed JSON.
    pub fn set_json(&mut self, json: Value) {
        self.json = Some(json);
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::cache::BodyCache;
    use crate::runner::xpath::{Document, Format};
    use crate::runner::Value;

    #[test]
    fn add_and_retry_html() {
        let html = "<!DOCTYPE html> \
                    <html> \
                        <body> \
                            <h1>My First Heading</h1> \
                            <p>My first paragraph.</p> \
                        </body> \
                    </html>";
        let doc = Document::parse(html, Format::Html).unwrap();
        assert_eq!(
            doc.eval_xpath("string(//h1)").unwrap(),
            Value::String("My First Heading".to_string())
        );

        let mut cache = BodyCache::new();
        assert!(cache.xml().is_none());

        cache.set_xml(doc);
        let doc = cache.xml().unwrap();
        assert_eq!(
            doc.eval_xpath("string(//h1)").unwrap(),
            Value::String("My First Heading".to_string())
        );
    }
}
