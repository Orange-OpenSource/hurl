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

/// A structure that holds "nice" numbers given a minimum and maximum values, and a number of ticks.
/// The code is derived from "Graphics Gems, Volume 1" by Andrew S. Glassner
/// See:
/// - <https://github.com/erich666/GraphicsGems/blob/master/gems/Label.c>
/// - <https://stackoverflow.com/questions/8506881/nice-label-algorithm-for-charts-with-minimum-ticks>
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NiceScale {
    min_value: f64,
    max_value: f64,
    max_ticks: usize,
    range: f64,
    tick_spacing: f64,
    nice_min: f64,
    nice_max: f64,
}

impl NiceScale {
    pub fn new(min_value: f64, max_value: f64, max_ticks: usize) -> Self {
        let range = max_value - min_value;
        let range = nice_number(range, false);
        let tick_spacing = range / ((max_ticks - 1) as f64);
        let tick_spacing = nice_number(tick_spacing, true);
        let nice_min = (min_value / tick_spacing).floor() * tick_spacing;
        let nice_max = (max_value / tick_spacing).ceil() * tick_spacing;
        NiceScale {
            min_value,
            max_value,
            max_ticks,
            range,
            tick_spacing,
            nice_min,
            nice_max,
        }
    }

    pub fn get_tick_spacing(&self) -> f64 {
        self.tick_spacing
    }
}

/// Returns a 'nice' number approximately equal to `range`.
/// Rounds the number if `round` is true, otherwise take the ceiling.
fn nice_number(range: f64, round: bool) -> f64 {
    let exponent = range.log10().floor() as i32;
    let fraction = range / 10_f64.powi(exponent);
    let nice_fraction = if round {
        if fraction < 1.5 {
            1.0
        } else if fraction < 3.0 {
            2.0
        } else if fraction < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * 10_f64.powi(exponent)
}

#[cfg(test)]
mod tests {
    use crate::report::html::timeline::nice::NiceScale;

    #[test]
    fn test_nice_scale() {
        let ns = NiceScale::new(0.0, 500.0, 20);
        assert_eq!(
            ns,
            NiceScale {
                min_value: 0.0,
                max_value: 500.0,
                max_ticks: 20,
                range: 500.0,
                tick_spacing: 20.0,
                nice_min: 0.0,
                nice_max: 500.0,
            }
        );

        let ns = NiceScale::new(0.0, 1700.0, 20);
        assert_eq!(
            ns,
            NiceScale {
                min_value: 0.0,
                max_value: 1700.0,
                max_ticks: 20,
                range: 2000.0,
                tick_spacing: 100.0,
                nice_min: 0.0,
                nice_max: 1700.0,
            }
        );
    }
}
