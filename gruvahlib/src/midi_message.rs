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


#[derive(Debug, Clone, Copy)]
pub enum MidiAction {
    NoteOn,
    NoteOff,
}

#[derive(Debug, Clone, Copy)]
pub struct MidiMessage {
    timestamp: i32,
    midi_action: MidiAction,
    note_pitch: u8,
    _velocity: u8,
}

impl MidiMessage {
    pub(crate) fn get_timestamp(&self) -> i32 {
        self.timestamp
    }
}

impl MidiMessage {
    fn new(timestamp: i32, raw_midi_data: &u8, note_pitch: u8, velocity: u8) -> Self {
        let midi_action = raw_midi_data & 0xF0;

        // velocity = 0 means note off, at least in the standalone application
        if velocity == 0 {
            return MidiMessage {
                timestamp,
                midi_action: MidiAction::NoteOff,
                note_pitch,
                _velocity: velocity,
            };
        }

        match midi_action {
            0x90 => MidiMessage {
                timestamp,
                midi_action: MidiAction::NoteOn,
                note_pitch,
                _velocity: velocity,
            },
            0x80 => MidiMessage {
                timestamp,
                midi_action: MidiAction::NoteOff,
                note_pitch,
                _velocity: velocity,
            },
            _ => panic!("Unknown midi action: {}", midi_action),
        }
    }

    pub fn get_midi_frequency(&self) -> f32 {
        get_midi_frequency_from_pitch(self.note_pitch)
    }


    pub fn get_midi_action(&self) -> MidiAction {
        self.midi_action.clone()
    }
}

pub fn get_midi_frequency_from_pitch(note_pitch: u8) -> f32 {
    let midi_note = note_pitch as f32;
    let midi_frequency = 440.0 * 2.0_f32.powf((midi_note - 69.0) / 12.0);
    midi_frequency
}

pub fn get_midi_note_from_octave_and_note_number(octave: i32, note_number: i32) -> u8 {
    (octave * 12 + note_number + 12) as u8
}

#[no_mangle]
pub extern "C" fn create_midi_message(timestamp: i32, raw_midi_data: &u8, note_pitch: u8, velocity: u8) -> *const MidiMessage {
    Box::into_raw(Box::new(MidiMessage::new(timestamp, raw_midi_data, note_pitch, velocity)))
}

#[no_mangle]
pub extern "C" fn destroy_midi_message(midi_message: *const MidiMessage) {
    assert!(!midi_message.is_null());
}

// midi note tests
#[cfg(test)]
mod test {
    use crate::midi_message::{get_midi_note_from_octave_and_note_number, MidiMessage};

    fn test_midi_note_to_frequency(note: u8, expected_frequency: f32) {
        let midi_message = MidiMessage::new(0, &0x90, note, 0);
        let midi_frequency = midi_message.get_midi_frequency();
        let midi_frequency_rounded = (midi_frequency * 100.0).round() / 100.0;
        assert_eq!(midi_frequency_rounded, expected_frequency);
    }

    #[test]
    fn test_midi_note_to_frequency_0() {
        test_midi_note_to_frequency(0, 8.18);
    }

    #[test]
    fn test_midi_note_to_frequency_24() {
        test_midi_note_to_frequency(24, 32.70);
    }

    #[test]
    fn test_midi_note_to_frequency_60() {
        test_midi_note_to_frequency(60, 261.63);
    }

    #[test]
    fn test_midi_note_octave_and_note_number() {
        let midi_note = get_midi_note_from_octave_and_note_number(2, 3);
        assert_eq!(midi_note, 39);
    }

    #[test]
    fn test_midi_note_octave_and_note_number_2() {
        let midi_note = get_midi_note_from_octave_and_note_number(3, 0);
        assert_eq!(midi_note, 48);
    }

    #[test]
    fn test_midi_note_octave_and_note_number_3() {
        let midi_note = get_midi_note_from_octave_and_note_number(1, 1);
        assert_eq!(midi_note, 25);
    }

}