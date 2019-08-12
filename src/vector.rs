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
use std::ops::{Div, Mul};

use operator_sugar::operator;

use crate::scalar::{IntScalar, Scalar};

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

    pub fn modulus_squared(self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn modulus(self) -> f64 {
        (self.modulus_squared() as f64).sqrt()
    }

    pub(super) fn bi_map<F: Fn(T, T) -> T>(self, other: Self, f: F) -> Self {
        Self(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    pub(super) fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

impl<T: IntScalar> Eq for Vector<T> {}

impl<T: Scalar> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

operator!({T: Scalar} Vector<T>, Vector<T>: a + b -> Vector<T> { a.bi_map(b, |x, y| x + y) });

operator!({T: Scalar} Vector<T>, Vector<T>: a - b -> Vector<T> { a.bi_map(b, |x, y| x - y) });

operator!({T: Scalar + Mul<U, Output = T>, U: Scalar} Vector<T>, U: a * b -> Vector<T> { a.map(|x| x * b) });

operator!({T: Scalar + Div<U, Output = T>, U: Scalar} Vector<T>, U: a / b -> Vector<T> { a.map(|x| x / b) });

operator!({T: Scalar} Vector<T>: -a -> Vector<T> { a.map(|x| -x) });
