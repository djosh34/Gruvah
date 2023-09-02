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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeState {
    NoteOn,
    NoteOff,
}



pub(crate) struct Envelope {
    sample_rate: usize,
    current_sample: i32,
    attack_sample_timing: TargetCurrentPair<i32>,
    decay_sample_timing: TargetCurrentPair<i32>,
    sustain: TargetCurrentPair<f32>,
    exponential_factor_a: TargetCurrentPair<f32>,
    release_sample_timing: TargetCurrentPair<i32>,
    envelope_state: EnvelopeState,
}


impl Envelope {
    pub fn new(sample_rate: usize) -> Self {
        Envelope {
            sample_rate,
            current_sample: 0,
            attack_sample_timing: Default::default(),
            decay_sample_timing: Default::default(),
            sustain: Default::default(),
            exponential_factor_a: Default::default(),
            release_sample_timing: Default::default(),
            envelope_state: EnvelopeState::NoteOff,
        }
    }

    fn convert_ms_to_samples(&self, ms: f32) -> i32 {
        let output = (ms * (1.0 / 1000.0) *  self.sample_rate as f32) as i32;

        // never return 0, because of clicks
        if output == 0 {
            return 1;
        }

        output
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_sample_timing.set_target(self.convert_ms_to_samples(attack_ms));
    }

    pub fn set_decay(&mut self, decay_ms: f32) {
        self.decay_sample_timing.set_target(self.convert_ms_to_samples(decay_ms));
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.sustain.set_target(sustain / 100.0);
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_sample_timing.set_target(self.convert_ms_to_samples(release_ms));
    }

    pub fn set_exponential_factor_a(&mut self, exponential_factor_a: f32) {
        self.exponential_factor_a.set_target(exponential_factor_a);
    }

    pub fn note_on(&mut self) {
        self.current_sample = 0;
        self.envelope_state = EnvelopeState::NoteOn;
    }

    pub fn note_off(&mut self) {
        // ignore this
    }

    pub(crate) fn process_sample(&mut self) -> f32 {
        if self.envelope_state == EnvelopeState::NoteOff {
            return 0.0
        }

        let output = self.get_output();
        self.update_to_target();

        self.current_sample += 1;

        // if output > 0.001 {
        //     print!("{:.2}, ", output);
        // }

        output
    }

    fn get_output(&mut self) -> f32 {
        let output= match self.current_sample {
            sample if sample < self.attack_sample_timing.get() => {
                let attack_sample = self.attack_sample_timing.get() as f32;

                if attack_sample == 0.0 {
                    return 1.0
                }

                let current_sample = self.current_sample as f32;
                current_sample / attack_sample
            },
            sample if sample < self.attack_sample_timing.get() + self.decay_sample_timing.get() => {
                let current_sample_delta = self.current_sample - self.attack_sample_timing.get();

                if self.decay_sample_timing.get() == 0 {
                    return self.sustain.get()
                }

                let decay_ratio = 1.0 - current_sample_delta as f32 / self.decay_sample_timing.get() as f32;
                let output = self.sustain.get() + (1.0 - self.sustain.get()) * decay_ratio;
                output
            },
            sample if sample < self.attack_sample_timing.get() + self.decay_sample_timing.get() + self.release_sample_timing.get() => {
                let current_sample_delta = self.current_sample - self.attack_sample_timing.get() - self.decay_sample_timing.get();

                if self.release_sample_timing.get() == 0 {
                    return 0.0
                }

                let release_ratio = 1.0 - current_sample_delta as f32 / self.release_sample_timing.get() as f32;
                let release_ratio = release_ratio.powf(self.exponential_factor_a.get());
                // println!("release_ratio: {}", release_ratio);
                let output = self.sustain.get() * release_ratio;
                output
            }
            _ =>  {
                self.envelope_state = EnvelopeState::NoteOff;
                0.0
            }
        };

        // if self.current_sample % 100 == 0 {
        //     println!("attack: {}, decay: {}, sustain: {}, release: {}, current_sample: {}, output: {}", self.attack_sample_timing, self.decay_sample_timing, self.sustain, self.release_sample_timing, self.current_sample, output);
        // }

        output
    }

    fn update_to_target(&mut self) {
        self.attack_sample_timing.update_to_target();
        self.decay_sample_timing.update_to_target();
        self.sustain.update_to_target();
        self.release_sample_timing.update_to_target();
        self.exponential_factor_a.update_to_target();
    }
}
