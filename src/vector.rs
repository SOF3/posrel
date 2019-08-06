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

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T: Scalar>(T, T, T);

impl<T: Scalar> Vector<T> {
    pub fn x(self) -> T {
        self.0
    }

    pub fn y(self) -> T {
        self.1
    }

    pub fn z(self) -> T {
        self.2
    }

    pub(super) fn bi_map<F: Fn(T, T) -> T>(self, other: Self, f: F) -> Self {
        Self(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    pub(super) fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

impl<T: Scalar> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.bi_map(other, |a, b| a + b)
    }
}

impl<T: Scalar> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.bi_map(other, |a, b| a - b)
    }
}

impl<T: Scalar, U: Scalar> Mul<U> for Vector<T>
where
    T: Mul<U, Output = T>,
{
    type Output = Self;

    fn mul(self, other: U) -> Self {
        self.map(|a| a * other)
    }
}

impl<T: Scalar, U: Scalar> Div<U> for Vector<T>
where
    T: Div<U, Output = T>,
{
    type Output = Self;

    fn div(self, other: U) -> Self {
        self.map(|a| a / other)
    }
}

impl<T: Scalar> Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|a| -a)
    }
}

impl<T: IntScalar> Eq for Vector<T> {}
