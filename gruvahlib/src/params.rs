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

use crate::oscillator::WaveType;
use crate::saturation::SaturationType;

pub enum Param {
    Pitch(i32, PitchParam),

    AmpAttack(f32),
    AmpDecay(f32),
    AmpSustain(f32),
    AmpRelease(f32),
    AmpExponentialFactorA(f32),

    Phase(f32),
    WaveType(WaveType),

    Drive(f32),
    SaturationType(SaturationType),
}

pub enum PitchParam {
    Octave(i32),
    Note(i32),
    Timing(f32),
}

impl Param {

    pub(crate) fn new(parameter_id: &str, new_value: f32) -> Self {
        match parameter_id {
            str if str.contains("octave") => {
                let note_number = get_note_number(parameter_id);
                Param::Pitch(note_number, PitchParam::Octave(new_value as i32))
            }

            str if str.contains("note") => {
                let note_number = get_note_number(parameter_id);
                Param::Pitch(note_number, PitchParam::Note(new_value as i32))
            }

            str if str.contains("timing") => {
                let note_number = get_note_number(parameter_id);
                Param::Pitch(note_number, PitchParam::Timing(new_value))
            }

            "amp_attack" => Param::AmpAttack(new_value),
            "amp_decay" => Param::AmpDecay(new_value),
            "amp_sustain" => Param::AmpSustain(new_value),
            "amp_release" => Param::AmpRelease(new_value),
            "amp_exponential_factor_a" => Param::AmpExponentialFactorA(new_value),

            "phase" => Param::Phase(new_value),
            "waveType" =>
                match new_value as i32 {
                    0 => Param::WaveType(WaveType::Sine),
                    1 => Param::WaveType(WaveType::Wave909),
                    _ => {
                        panic!("Invalid wave type: {}", new_value);
                    }
                },

            "driveDb" => Param::Drive(new_value),
            "saturationType" => {
                match new_value as i32 {
                    0 => Param::SaturationType(SaturationType::None),
                    1 => Param::SaturationType(SaturationType::Soft),
                    2 => Param::SaturationType(SaturationType::Clip),
                    3 => Param::SaturationType(SaturationType::ExtremeClip),
                    _ => {
                        panic!("Invalid saturation type: {}", new_value);
                    }
                }
            }
            _ => {
                panic!("Invalid parameter id: {}", parameter_id);
            }
        }
    }
}

fn get_note_number(parameter_id: &str) -> i32 {
    let note_number = parameter_id.chars().last().unwrap().to_digit(10).unwrap() as i32;
    let note_number = note_number - 1;
    note_number
}
