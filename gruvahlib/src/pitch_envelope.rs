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

use crate::midi_message::{get_midi_frequency_from_pitch, get_midi_note_from_octave_and_note_number};
use crate::params::PitchParam;

#[derive(Clone, Copy, Debug)]
struct PitchEnvelopePart {
    frequency: f32,
    octave: i32,
    note: i32,
    timing: usize,
}


impl PitchEnvelopePart {
    pub(crate) fn recalculate_frequency(&mut self) {
        let octave = self.octave;
        let note = self.note;

        // println!("octave: {}, note: {}", octave, note);

        let midi_note_number = get_midi_note_from_octave_and_note_number(octave, note);
        self.frequency = get_midi_frequency_from_pitch(midi_note_number as u8);
    }
}

impl Default for PitchEnvelopePart {
    fn default() -> Self {
        Self {
            frequency: 20.0,
            octave: 0,
            note: 0,
            timing: 0,
        }
    }
}

const PITCH_ENVELOPE_SIZE: usize = 4;

#[derive(Debug)]
pub struct PitchEnvelope {
    sample_rate: usize,
    current_sample: usize,
    pitches: [(i32, PitchEnvelopePart); PITCH_ENVELOPE_SIZE],
    previous_pitch: PitchEnvelopePart,
    next_pitch: PitchEnvelopePart,
    previous_total_timing: usize,
    current_note_number: i32,
}

impl PitchEnvelope {
    pub(crate) fn new(sample_rate: usize) -> Self {
        let mut pitches = [(0, Default::default()); PITCH_ENVELOPE_SIZE];

        for i in 0..PITCH_ENVELOPE_SIZE {
            pitches[i].0 = i as i32;
        }


        Self {
            sample_rate,
            current_sample: 0,
            pitches,
            previous_pitch: Default::default(),
            next_pitch: Default::default(),
            previous_total_timing: 0,
            current_note_number: 0, // note number is the index of which pitch envelope part we are currently in the pitches array
        }
    }
}

impl PitchEnvelope {
    pub(crate) fn note_on(&mut self) {
        self.current_sample = 0;
        self.current_note_number = 0;
        self.previous_total_timing = 0;
        self.previous_pitch = Default::default();
        self.next_pitch = self.pitches[0].1;
    }

    pub(crate) fn note_off(&self) {
        // do nothing when note off
    }
}

impl PitchEnvelope {
    pub(crate) fn set_pitch(&mut self, note_number: i32, pitch_param: PitchParam) {
        let mut pitch_envelope_part = self.pitches[note_number as usize].1;

        match pitch_param {
            PitchParam::Octave(octave) => {
                pitch_envelope_part.octave = octave;
            },
            PitchParam::Note(note_number) => {
                pitch_envelope_part.note = note_number;

            },
            PitchParam::Timing(timing) => {
                let mut timing = (timing / 1000.0 * self.sample_rate as f32) as usize;
                if timing == 0 {
                    timing = 1;
                }


                pitch_envelope_part.timing = timing;
            },
        }

        pitch_envelope_part.recalculate_frequency();

        // println!("{:?}", self);

        self.pitches[note_number as usize].1 = pitch_envelope_part;
    }

    pub(crate) fn set_frequency(&mut self, note_number: i32, frequency: f32) {
        let mut pitch_envelope_part = self.pitches[note_number as usize].1;

        pitch_envelope_part.frequency = frequency;

        self.pitches[note_number as usize].1 = pitch_envelope_part;
    }



    pub(crate) fn get_frequency(&mut self) -> f32 {
        let current_sample_delta = self.current_sample - self.previous_total_timing;
        let timing_fraction = match self.next_pitch.timing {
            0 => 1.0,
            _ => current_sample_delta as f32 / self.next_pitch.timing as f32,
        };

        let frequency = self.previous_pitch.frequency + (self.next_pitch.frequency - self.previous_pitch.frequency) * timing_fraction;


        if self.current_sample == self.next_pitch.timing + self.previous_total_timing {
            self.previous_pitch = self.next_pitch;

            if self.current_note_number + 1 < self.pitches.len() as i32 {
                self.next_pitch = self.pitches[self.current_note_number as usize + 1].1;
            }

            self.current_note_number += 1;
            self.previous_total_timing += self.previous_pitch.timing;
        }

        self.current_sample += 1;


        frequency


    }

}


#[cfg(test)]
mod test {
    use crate::params::PitchParam;
    use crate::pitch_envelope::PitchEnvelope;

    #[test]
    fn test_get_frequency_0() {
        let mut pitch_envelope = PitchEnvelope::new(1000);

        pitch_envelope.set_pitch(0, PitchParam::Timing(10.0));
        pitch_envelope.set_frequency(0, 100.0);

        pitch_envelope.set_pitch(1, PitchParam::Timing(20.0));
        pitch_envelope.set_frequency(1, 200.0);

        pitch_envelope.set_pitch(2, PitchParam::Timing(30.0));
        pitch_envelope.set_frequency(2, 300.0);

        pitch_envelope.set_pitch(3, PitchParam::Timing(40.0));
        pitch_envelope.set_frequency(3, 400.0);

        pitch_envelope.note_on();

        for i in 0..150 {
            let frequency = pitch_envelope.get_frequency();
            println!("i: {}, frequency: {:?}", i, frequency);
            if i == 0 {
                assert_eq!(frequency, 20.0);
            }

            if i == 5 {
                assert_eq!(frequency, 60.0);
            }

            if i == 10 {
                assert_eq!(frequency, 100.0);
            }

            if i == 26 {
                assert_eq!(frequency, 180.0);
            }

            if i == 36 {
                assert_eq!(frequency, 220.0);
            }

            if i == 60 {
                assert_eq!(frequency, 300.0);
            }

            if i == 80 {
                assert_eq!(frequency, 350.0);
            }

            if i == 120 {
                assert_eq!(frequency, 400.0);
            }
        }

    }
}
