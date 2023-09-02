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

use crate::midi_message::MidiMessage;

const MIDI_QUEUE_SIZE: usize = 16;

pub(crate) struct MidiMessageQueue {
    midi_messages: [Option<(i32, MidiMessage)>; MIDI_QUEUE_SIZE],
}

impl MidiMessageQueue {
    pub(crate) fn new() -> Self {
        MidiMessageQueue {
            midi_messages: [None; MIDI_QUEUE_SIZE]
        }
    }

    pub(crate) fn add_midi_message(&mut self, midi_message: MidiMessage) {
        for midi_message_item in self.midi_messages.iter_mut() {
            if midi_message_item.is_none() {
                *midi_message_item = Some((midi_message.get_timestamp(), midi_message));
                return;
            }
        }

        eprintln!("Do you really really need more than {} messages during one block???, dropping midi message: {:?}", MIDI_QUEUE_SIZE, midi_message);
    }

    // If there are more than one midi messages with the same timestamp, this will return the first one, and drop the rest
    pub(crate) fn get_latest_midi_message_if_timestamp(&mut self, timestamp: i32) -> Option<MidiMessage> {
        let mut output = None;

        for midi_message_item in self.midi_messages.iter_mut() {
            let Some((midi_message_timestamp, midi_message)) = midi_message_item else {
                continue;
            };

            if *midi_message_timestamp != timestamp {
                continue;
            }

            if output.is_some() {
                // Dropping midi message, since we have already one with the same timestamp
                *midi_message_item = None;
                continue;
            }

            let midi_message = *midi_message;
            *midi_message_item = None;
            output = Some(midi_message);
        }

        output
    }
}