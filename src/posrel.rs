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

use derive_more::{Display, From, Neg};
use operator_sugar::operator;

use crate::scalar::{IntScalar, Scalar};
use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Display, From, PartialEq)]
pub struct PositionVector<T: Scalar>(pub(crate) Vector<T>);

impl<T: Scalar> From<PositionVector<T>> for Vector<T> {
    fn from(from: PositionVector<T>) -> Self {
        from.0
    }
}

impl<T: IntScalar> Eq for PositionVector<T> {}

#[derive(Clone, Copy, Debug, Display, From, Neg, PartialEq)]
pub struct RelativeVector<T: Scalar>(pub(crate) Vector<T>);

impl<T: Scalar> RelativeVector<T> {
    pub fn length_squared(self) -> T {
        self.0.modulus_squared()
    }

    pub fn length(self) -> f64 {
        self.0.modulus()
    }
}

impl<T: Scalar> From<RelativeVector<T>> for Vector<T> {
    fn from(from: RelativeVector<T>) -> Self {
        from.0
    }
}

operator!({T: Scalar} PositionVector<T>, RelativeVector<T>: a + b -> PositionVector<T> { PositionVector(a.0 + b.0) });

operator!({T: Scalar} RelativeVector<T>, PositionVector<T>: a + b -> PositionVector<T> { PositionVector(a.0 + b.0) });

operator!({T: Scalar} RelativeVector<T>, RelativeVector<T>: a + b -> RelativeVector<T> { RelativeVector(a.0 + b.0) });

operator!({T: Scalar} PositionVector<T>, RelativeVector<T>: a - b -> PositionVector<T> { PositionVector(a.0 - b.0) });

operator!({T: Scalar} RelativeVector<T>, PositionVector<T>: a - b -> PositionVector<T> { PositionVector(a.0 - b.0) });

operator!({T: Scalar} RelativeVector<T>, RelativeVector<T>: a - b -> RelativeVector<T> { RelativeVector(a.0 - b.0) });

operator!({T: Scalar} RelativeVector<T>, T: a * b -> RelativeVector<T> { RelativeVector(a.0 * b) });

operator!({T: Scalar} RelativeVector<T>, T: a / b -> RelativeVector<T> { RelativeVector(a.0 / b) });
