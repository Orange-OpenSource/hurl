/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

#[allow(unused)]
pub enum TerminalColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

impl TerminalColor {
    pub fn format(self, v: String) -> String {
        match self {
            TerminalColor::Black => format!("\x1b[0;30m{}\x1b[0m", v),
            TerminalColor::Red => format!("\x1b[1;31m{}\x1b[0m", v),
            TerminalColor::Green => format!("\x1b[0;32m{}\x1b[0m", v),
            TerminalColor::Yellow => format!("\x1b[0;33m{}\x1b[0m", v),
            TerminalColor::Blue => format!("\x1b[0;34m{}\x1b[0m", v),
            TerminalColor::Magenta => format!("\x1b[0;35m{}\x1b[0m", v),
            TerminalColor::Cyan => format!("\x1b[0;36m{}\x1b[0m", v),
            TerminalColor::LightGray => format!("\x1b[0;37m{}\x1b[0m", v),
            TerminalColor::LightBlack => format!("\x1b[0;90m{}\x1b[0m", v),
            TerminalColor::LightRed => format!("\x1b[0;91m{}\x1b[0m", v),
            TerminalColor::LightGreen => format!("\x1b[0;92m{}\x1b[0m", v),
            TerminalColor::LightYellow => format!("\x1b[0;93m{}\x1b[0m", v),
            TerminalColor::LightBlue => format!("\x1b[0;94m{}\x1b[0m", v),
            TerminalColor::LightMagenta => format!("\x1b[0;95m{}\x1b[0m", v),
            TerminalColor::LightCyan => format!("\x1b[0;96m{}\x1b[0m", v),
            TerminalColor::LightWhite => format!("\x1b[0;97m{}\x1b[0m", v),
        }
    }
}
