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

mod kick_synth;
mod midi_message;
mod oscillator;
mod envelope;
mod params;
mod midi_message_queue;
mod saturation;
mod utils;
mod pitch_envelope;
mod target_pair;

use std::ffi::c_char;
pub use kick_synth::KickSynth;
use params::Param;


#[no_mangle]
pub extern "C" fn create(sample_rate: usize) -> *mut KickSynth {
    Box::into_raw(Box::new(KickSynth::new(sample_rate)))
}

#[no_mangle]
pub unsafe extern "C" fn destroy(kick_synth: *mut KickSynth) {
    assert!(!kick_synth.is_null());
    drop(Box::from_raw(kick_synth));
}

#[no_mangle]
pub unsafe extern "C" fn process(
    kick_synth: &mut KickSynth,
    block_left: *mut f32,
    block_right: *mut f32,
    num_samples: usize
) {
    let block_left = std::slice::from_raw_parts_mut(block_left, num_samples);
    kick_synth.process_block(block_left);

    let block_right = std::slice::from_raw_parts_mut(block_right, num_samples);

    // mono copy left to right
    for i in 0..num_samples {
        block_right[i] = block_left[i];
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_mono(
    kick_synth: &mut KickSynth,
    block: *mut f32,
    num_samples: usize
) {
    let block = std::slice::from_raw_parts_mut(block, num_samples);
    kick_synth.process_block(block);
}

#[no_mangle]
pub extern "C" fn process_midi_message(kick_synth: &mut KickSynth, midi_message: &midi_message::MidiMessage) {
    kick_synth.queue_midi_message(midi_message);
}

#[no_mangle]
pub unsafe extern "C" fn update_param(kick_synth: &mut KickSynth, parameter_id: *const c_char, new_value: f32) {

    let parameter_id = std::ffi::CStr::from_ptr(parameter_id).to_str().unwrap();
    // println!("Updating param: {} to {}", parameter_id, new_value);
    let param = Param::new(parameter_id, new_value);
    kick_synth.update_param(param);
}