// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;
use num_traits::Float;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<'a, T: 'a, U: 'a> {
    /// Outputs a value.
    pub source1: &'a NoiseFn<T, U>,

    /// Outputs a value.
    pub source2: &'a NoiseFn<T, U>,
}

impl<'a, T, U> Power<'a, T, U> {
    pub fn new(source1: &'a NoiseFn<T, U>, source2: &'a NoiseFn<T, U>) -> Power<'a, T, U> {
        Power {
            source1: source1,
            source2: source2,
        }
    }
}

impl<'a, T, U> NoiseFn<T, U> for Power<'a, T, U>
where
    T: Copy,
    U: Float,
{
    fn get(&self, point: T) -> U {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}