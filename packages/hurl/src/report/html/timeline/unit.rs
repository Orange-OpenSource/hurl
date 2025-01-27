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
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use chrono::{DateTime, Utc};

/// Represents a second, millisecond or microsecond.
#[derive(Copy, Clone, PartialEq)]
pub enum TimeUnit {
    Second(Second),
    Millisecond(Millisecond),
    Microsecond(Microsecond),
}

impl TimeUnit {
    pub fn zero_s() -> Self {
        TimeUnit::Second(Second(0.0))
    }

    pub fn zero_ms() -> Self {
        TimeUnit::Millisecond(Millisecond(0.0))
    }

    pub fn zero_mc() -> Self {
        TimeUnit::Microsecond(Microsecond(0.0))
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            TimeUnit::Second(s) => s.0,
            TimeUnit::Millisecond(ms) => ms.0,
            TimeUnit::Microsecond(mc) => mc.0,
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            TimeUnit::Second(_) => "s",
            TimeUnit::Millisecond(_) => "ms",
            TimeUnit::Microsecond(_) => "µs",
        }
    }

    pub fn add_raw(self, raw: f64) -> TimeUnit {
        match self {
            TimeUnit::Second(s) => TimeUnit::Second(Second(s.0 + raw)),
            TimeUnit::Millisecond(ms) => TimeUnit::Millisecond(Millisecond(ms.0 + raw)),
            TimeUnit::Microsecond(mc) => TimeUnit::Microsecond(Microsecond(mc.0 + raw)),
        }
    }
}

impl PartialOrd for TimeUnit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = Microsecond::from(*self);
        let right = Microsecond::from(*other);
        Some(left.0.total_cmp(&right.0))
    }
}

/// Represents a second.
#[derive(Copy, Clone, PartialEq)]
pub struct Second(pub f64);

/// Represents a microsecond.
impl From<Microsecond> for Second {
    fn from(value: Microsecond) -> Self {
        Second(value.0 / 1_000_000.0)
    }
}

/// Represents a millisecond.
#[derive(Copy, Clone, PartialEq)]
pub struct Millisecond(pub f64);

impl From<Microsecond> for Millisecond {
    fn from(value: Microsecond) -> Self {
        Millisecond(value.0 / 1_000.0)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Microsecond(pub f64);

impl Sub for Microsecond {
    type Output = Microsecond;

    fn sub(self, rhs: Self) -> Self {
        Microsecond(self.0 - rhs.0)
    }
}

impl From<Second> for Microsecond {
    fn from(value: Second) -> Self {
        Microsecond(value.0 * 1_000_000.0)
    }
}

impl From<Millisecond> for Microsecond {
    fn from(value: Millisecond) -> Self {
        Microsecond(value.0 * 1_000.0)
    }
}

impl From<TimeUnit> for Microsecond {
    fn from(value: TimeUnit) -> Self {
        match value {
            TimeUnit::Second(s) => s.into(),
            TimeUnit::Millisecond(ms) => ms.into(),
            TimeUnit::Microsecond(mc) => mc,
        }
    }
}

/// Represents a byte.
#[derive(Copy, Clone, PartialEq)]
pub struct Byte(pub f64);

/// Represents a logic pixel.
#[derive(Copy, Clone, PartialEq)]
pub struct Pixel(pub f64);

pub trait Px {
    fn px(self) -> Pixel;
}

impl Px for f64 {
    fn px(self) -> Pixel {
        Pixel(self)
    }
}

impl Px for usize {
    fn px(self) -> Pixel {
        Pixel(self as f64)
    }
}

impl Sub for Pixel {
    type Output = Pixel;
    fn sub(self, rhs: Self) -> Self {
        Pixel(self.0 - rhs.0)
    }
}

impl Add for Pixel {
    type Output = Pixel;
    fn add(self, rhs: Self) -> Self {
        Pixel(self.0 + rhs.0)
    }
}

impl Mul<f64> for Pixel {
    type Output = Pixel;
    fn mul(self, rhs: f64) -> Pixel {
        Pixel(self.0 * rhs)
    }
}

impl Mul<usize> for Pixel {
    type Output = Pixel;
    fn mul(self, rhs: usize) -> Pixel {
        Pixel(self.0 * rhs as f64)
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Pixel) {
        *self = *self + rhs;
    }
}

impl SubAssign for Pixel {
    fn sub_assign(&mut self, rhs: Pixel) {
        *self = *self - rhs;
    }
}

impl From<Pixel> for f64 {
    fn from(value: Pixel) -> Self {
        value.0
    }
}

impl From<f64> for Pixel {
    fn from(value: f64) -> Self {
        Pixel(value)
    }
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
}

impl Pixel {
    pub fn max(v1: Pixel, v2: Pixel) -> Pixel {
        Pixel(f64::max(v1.0, v2.0))
    }
    pub fn min(v1: Pixel, v2: Pixel) -> Pixel {
        Pixel(f64::min(v1.0, v2.0))
    }
}

#[derive(Copy, Clone)]
pub struct Interval<Idx: Copy> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx: Copy> Interval<Idx> {
    pub fn new(start: Idx, end: Idx) -> Interval<Idx> {
        Interval { start, end }
    }
}

/// Structure that hold a time interval and a pixel interval.
/// This can be used to easily convert a time to a pixel value.
#[derive(Copy, Clone)]
pub struct Scale {
    times: Interval<Microsecond>,
    pixels: Interval<Pixel>,
}

impl Scale {
    /// Returns a new scale from `times` to `pixels`.
    pub fn new(times: Interval<DateTime<Utc>>, pixels: Interval<Pixel>) -> Self {
        let duration = times.end - times.start;
        let start = Microsecond(0.0);
        let end = Microsecond(duration.num_microseconds().unwrap() as f64);
        let times = Interval { start, end };
        Scale { times, pixels }
    }

    /// Returns a pixel value of `time`.
    pub fn to_pixel(self, time: Microsecond) -> Pixel {
        let pixel = (time.0 - self.times.start.0) * (self.pixels.end.0 - self.pixels.start.0)
            / (self.times.end.0 - self.times.start.0);
        Pixel(pixel)
    }
}
