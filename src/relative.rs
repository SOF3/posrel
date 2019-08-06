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

use std::ops::{Add, Div, Mul, Sub};

use derive_more::*;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Display, From, Neg, PartialEq)]
pub struct RelativeVector<T: Scalar>(pub(crate) Vector<T>);

impl<T: Scalar> From<RelativeVector<T>> for Vector<T> {
    fn from(from: RelativeVector<T>) -> Self {
        from.0
    }
}

impl<T: Scalar> Add for RelativeVector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<T: Scalar> Add<PositionVector<T>> for RelativeVector<T> {
    type Output = PositionVector<T>;

    fn add(self, other: PositionVector<T>) -> Self::Output {
        PositionVector(self.0 + other.0)
    }
}

impl<T: Scalar> Sub for RelativeVector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<T: Scalar> Sub<PositionVector<T>> for RelativeVector<T> {
    type Output = PositionVector<T>;

    fn sub(self, other: PositionVector<T>) -> Self::Output {
        PositionVector(self.0 - other.0)
    }
}

impl<T: Scalar> Mul<T> for RelativeVector<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self(self.0 * other)
    }
}

impl<T: Scalar> Div<T> for RelativeVector<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self(self.0 / other)
    }
}
