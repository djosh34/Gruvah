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
use std::sync::Arc;
use num_complex::Complex;

pub enum WaveType {
    Sine,
    Wave909,
}

pub struct Oscillator {
    sample_rate: usize,
    frequency: f32,
    start_phase: f32,
    current_phase: f32,
    fft: Arc<dyn rustfft::Fft<f32>>,
    wave_table: [Complex<f32>; 1024],
}


impl Oscillator {
    pub(crate) fn set_phase(&mut self, phase: f32) {
        self.start_phase = phase;
    }

    fn set_wave_table(&mut self, bins: Vec<(usize, Complex<f32>)>) {
        self.wave_table = [Complex::new(0.0, 0.0); 1024];

        for bin in bins {
            self.wave_table[bin.0] = bin.1;
        }

        self.fft.process(&mut self.wave_table);
    }

    pub(crate) fn set_wave_type(&mut self, wave_type: WaveType) {
        match wave_type {
            WaveType::Sine => {
                let bins = vec![(1, Complex::new(1.0, 0.0))];
                self.set_wave_table(bins);
            }
            WaveType::Wave909 => {
                let bins = vec![
                    (1, Complex::new(1.0, 0.0)),
                    (2, Complex::new(0.2, 0.0)),
                    (2, Complex::new(0.1, 0.0)),
                ];
                self.set_wave_table(bins);
            }
        }
    }
}

impl Oscillator {
    fn get_value_from_wave_table(&self) -> f32 {
        let index = self.current_phase * self.wave_table.len() as f32;

        let index_floor = index.floor() as usize;
        let index_ceil = index.ceil() as usize;

        let value_floor = self.wave_table[index_floor % self.wave_table.len()].re;
        let value_ceil = self.wave_table[index_ceil % self.wave_table.len()].re;

        let linearly_interpolated_value = index - index_floor as f32;

        value_floor * (1.0 - linearly_interpolated_value) + value_ceil * linearly_interpolated_value
    }

    pub(crate) fn process_sample(&mut self) -> f32 {
        let frequency = self.frequency;

        let phase_increment = frequency / self.sample_rate as f32;
        self.current_phase += phase_increment;

        if self.current_phase > 1.0 {
            self.current_phase -= 1.0
        }

        self.get_value_from_wave_table()
    }

    pub(crate) fn set_frequency(&mut self, freq_hz: f32) {
        self.frequency = freq_hz;
    }

    pub(crate) fn reset(&mut self) {
        self.current_phase = self.start_phase;
    }
}

pub fn new(sample_rate: usize) -> Oscillator {
    let mut planner = rustfft::FftPlanner::new();
    let wave_table = [Complex::new(0.0, 0.0); 1024];

    let fft = planner.plan_fft_inverse(wave_table.len());

    Oscillator {
        sample_rate,
        frequency: 440.0,
        start_phase: 0.0,
        current_phase: 0.0,
        fft,
        wave_table,
    }
}
