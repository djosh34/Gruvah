/*
 * Copyright (c) 2023 Joshua Azimullah
 *
 * This file is part of Gruvah.
 *
 * Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.
 */

pub static TARGET_DIVISOR: f32 = 1000.0;

// Slowly update a current value to a target value in order to avoid clicks
pub struct TargetCurrentPair<T>
    where T: Copy {
    target: T,
    current: T,
}


impl<T: Copy> TargetCurrentPair<T> {
    pub(crate) fn get(&self) -> T {
        self.current
    }

    pub(crate) fn set_target(&mut self, target: T) {
        self.target = target;
    }

}

impl TargetCurrentPair<f32> {
    pub(crate) fn update_to_target(&mut self) {
        if self.current == self.target {
            return;
        }

        // println!("current: {}, target: {}", self.current, self.target);

        let difference = self.target - self.current;
        let delta = difference / TARGET_DIVISOR;

        if delta.abs() < 1.0e-6 {
            self.current = self.target;
        } else {
            self.current += delta;
        }
    }
}

impl TargetCurrentPair<i32> {
    pub(crate) fn update_to_target(&mut self) {
        if self.current == self.target {
            return;
        }


        let difference = self.target- self.current;
        let delta = difference as f32 / TARGET_DIVISOR;
        let delta = delta.signum() * delta.abs().ceil();

        self.current += delta as i32;

        // println!("current: {}, target: {}, delta: {}", self.current, self.target, delta);

    }
}

impl Default for TargetCurrentPair<f32> {
    fn default() -> Self {
        Self {
            target: 0.0,
            current: 0.0,
        }
    }
}

impl Default for TargetCurrentPair<i32> {
    fn default() -> Self {
        Self {
            target: 0,
            current: 0,
        }
    }
}
