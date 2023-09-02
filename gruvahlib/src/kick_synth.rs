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

use crate::midi_message::{MidiAction, MidiMessage};
use crate::{envelope,  oscillator, pitch_envelope, saturation};
use crate::midi_message_queue::MidiMessageQueue;
use crate::params::Param;


pub struct KickSynth {
    oscillator: oscillator::Oscillator,
    amp_envelope: envelope::Envelope,
    pitch_envelope: pitch_envelope::PitchEnvelope,
    midi_message_queue: MidiMessageQueue,
    saturation: saturation::Saturation,
}

impl KickSynth {
    pub(crate) fn queue_midi_message(&mut self, midi_message: &MidiMessage) {
        println!("Queuing midi message: {:?}", midi_message);
        self.midi_message_queue.add_midi_message(midi_message.clone());
    }
}

impl KickSynth {
    pub(crate) fn update_param(&mut self, param: Param) {
        match param {
            Param::Pitch(note_number, pitch_param) => {
                self.pitch_envelope.set_pitch(note_number, pitch_param);
            }

            Param::AmpAttack(attack) => {
                self.amp_envelope.set_attack(attack);
            }
            Param::AmpDecay(decay) => {
                self.amp_envelope.set_decay(decay);
            }
            Param::AmpSustain(sustain) => {
                self.amp_envelope.set_sustain(sustain);
            }
            Param::AmpRelease(release) => {
                self.amp_envelope.set_release(release);
            }
            Param::AmpExponentialFactorA(factor) => {
                self.amp_envelope.set_exponential_factor_a(factor);
            }

            Param::Phase(phase) => {
                self.oscillator.set_phase(phase);
            }

            Param::WaveType(wave_type) => {
                self.oscillator.set_wave_type(wave_type);
            }

            Param::SaturationType(saturation_type) => {
                self.saturation.set_saturation_type(saturation_type);
            }
            Param::Drive(drive) => {
                self.saturation.set_drive(drive);
            }
        }
    }

}

impl KickSynth {

    fn process_midi_messages(&mut self, midi_message_timestamp: i32) {
        let midi_message = self.midi_message_queue.get_latest_midi_message_if_timestamp(midi_message_timestamp);


        let Some(midi_message) = midi_message else {
            return;
        };
        self.process_midi_message(&midi_message);
    }

    pub(crate) fn process_midi_message(&mut self, midi_message: &MidiMessage) {
        match midi_message.get_midi_action() {
            MidiAction::NoteOn => {
                self.oscillator.reset();
                self.amp_envelope.note_on();
                self.pitch_envelope.note_on();
            }
            MidiAction::NoteOff => {
                self.amp_envelope.note_off();
                self.pitch_envelope.note_off();
            }
        }

    }
}

impl KickSynth {
    pub fn new(sr: usize) -> Self {

        KickSynth {
            oscillator: oscillator::new(sr),
            amp_envelope: envelope::Envelope::new(sr),
            pitch_envelope: pitch_envelope::PitchEnvelope::new(sr),
            midi_message_queue: MidiMessageQueue::new(),
            saturation: saturation::Saturation::new(),
        }
    }

    pub fn process_block(&mut self, block: &mut [f32]) {

        for (i, x) in block.iter_mut().enumerate() {
            self.process_midi_messages(i as i32);

            let frequency = self.pitch_envelope.get_frequency();
            self.oscillator.set_frequency(frequency);

            *x = self.oscillator.process_sample() * self.amp_envelope.process_sample();

            // attenuation by -12db
            *x = *x * 0.25;
            *x = self.saturation.process_sample(*x);


            // Safety feature, shouldn't change the audio
            if x.abs() > 1.5 {
                eprintln!("Clipping! with value {}", x);
                *x = x.clamp(-1.5, 1.5);
            }


        }
    }
}
