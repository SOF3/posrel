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

use std::ops::{Add, Sub};

use derive_more::*;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Display, From, PartialEq)]
pub struct PositionVector<T: Scalar>(pub(crate) Vector<T>);

impl<T: Scalar> From<PositionVector<T>> for Vector<T> {
    fn from(from: PositionVector<T>) -> Self {
        from.0
    }
}

impl<T: Scalar> Add<RelativeVector<T>> for PositionVector<T> {
    type Output = Self;

    fn add(self, other: RelativeVector<T>) -> Self {
        Self(self.0 + other.0)
    }
}

impl<T: Scalar> Sub<RelativeVector<T>> for PositionVector<T> {
    type Output = Self;

    fn sub(self, other: RelativeVector<T>) -> Self {
        Self(self.0 - other.0)
    }
}

impl<T: Scalar> Sub for PositionVector<T> {
    type Output = RelativeVector<T>;

    fn sub(self, other: PositionVector<T>) -> Self::Output {
        RelativeVector(self.0 - other.0)
    }
}

impl<T: IntScalar> Eq for PositionVector<T> {}
