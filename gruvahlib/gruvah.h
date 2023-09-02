#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace kick_synth {

struct KickSynth;

struct MidiMessage;

extern "C" {

KickSynth *create(uintptr_t sample_rate);

void destroy(KickSynth *kick_synth);

void process(KickSynth *kick_synth, float *block_left, float *block_right, uintptr_t num_samples);

void process_mono(KickSynth *kick_synth, float *block, uintptr_t num_samples);

void process_midi_message(KickSynth *kick_synth, const MidiMessage *midi_message);

void update_param(KickSynth *kick_synth, const char *parameter_id, float new_value);

const MidiMessage *create_midi_message(int32_t timestamp,
                                       const uint8_t *raw_midi_data,
                                       uint8_t note_pitch,
                                       uint8_t velocity);

void destroy_midi_message(const MidiMessage *midi_message);

} // extern "C"

} // namespace kick_synth
