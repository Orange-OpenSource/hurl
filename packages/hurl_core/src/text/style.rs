/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(unused)]
pub struct Style {
    pub fg: Option<Color>,
    pub bold: bool,
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
}

impl Style {
    pub fn new() -> Style {
        let fg = None;
        let bold = false;
        Style { fg, bold }
    }

    pub fn red(mut self) -> Style {
        self.fg = Some(Color::Red);
        self
    }

    pub fn green(mut self) -> Style {
        self.fg = Some(Color::Green);
        self
    }

    pub fn blue(mut self) -> Style {
        self.fg = Some(Color::Blue);
        self
    }

    pub fn yellow(mut self) -> Style {
        self.fg = Some(Color::Yellow);
        self
    }

    pub fn bold(mut self) -> Style {
        self.bold = true;
        self
    }
}
