// posrel
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Scalar:
    Copy
    + Clone
    + Debug
    + Display
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + PartialEq
    + PartialOrd
    + Neg<Output = Self>
{
    const ZERO: Self;
}

impl Scalar for i8 {
    const ZERO: Self = 0;
}
impl Scalar for i16 {
    const ZERO: Self = 0;
}
impl Scalar for i32 {
    const ZERO: Self = 0;
}
impl Scalar for i64 {
    const ZERO: Self = 0;
}
impl Scalar for f32 {
    const ZERO: Self = 0.0;
}
impl Scalar for f64 {
    const ZERO: Self = 0.0;
}

pub trait IntScalar: Scalar + Eq + Ord {}

impl IntScalar for i8 {}
impl IntScalar for i16 {}
impl IntScalar for i32 {}
impl IntScalar for i64 {}
