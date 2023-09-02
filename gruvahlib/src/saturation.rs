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

use crate::target_pair::TargetCurrentPair;
use crate::utils::db_to_linear;

pub enum SaturationType {
    None,
    Soft,
    Clip,
    ExtremeClip,
}

pub(crate) struct Saturation {
    saturation_type: SaturationType,
    drive: TargetCurrentPair<f32>,
}

impl Saturation {
    pub fn new() -> Self {
        let mut drive = TargetCurrentPair::default();
        drive.set_target(1.0);

        Self {
            saturation_type: SaturationType::Soft,
            drive,
        }
    }

    pub(crate) fn process_sample(&mut self, input: f32) -> f32 {
        self.drive.update_to_target();

        match self.saturation_type {
            SaturationType::None => {
                input
            },
            SaturationType::Soft => {
                let output = input * self.drive.get();
                let output = output.tanh();
                output
            },
            SaturationType::Clip => {
                let mut output = input * self.drive.get();
                output = output.min(1.0);
                output = output.max(-1.0);
                output
            }
            SaturationType::ExtremeClip => {
                // TO THE MEGA, YEEAAAHHHHH, HARDCORE TO THE MEGA
                let mut output = input * self.drive.get() * self.drive.get();
                output = output.min(1.0);
                output = output.max(-1.0);
                output
            }
        }
    }

    pub fn set_saturation_type(&mut self, saturation_type: SaturationType) {
        self.saturation_type = saturation_type;
    }

    pub fn set_drive(&mut self, drive: f32) {
        let drive = db_to_linear(drive);
        // println!("drive: {}", drive);
        self.drive.set_target(drive);
    }
}