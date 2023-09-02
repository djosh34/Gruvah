// Copyright (c) 2023 Joshua Azimullah
//
// This file is part of Gruvah.
//
// Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.

#include "GruvahPlugin.h"
#include "ParamListener.h"


static const juce::String midiNoteNames[] = { "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" };

namespace
{
    const String octave_1 = "octave_1";
    const String note_1 = "note_1";
    const String timing_1 = "timing_1";

    const String octave_2 = "octave_2";
    const String note_2 = "note_2";
    const String timing_2 = "timing_2";

    const String octave_3 = "octave_3";
    const String note_3 = "note_3";
    const String timing_3 = "timing_3";

    const String octave_4 = "octave_4";
    const String note_4 = "note_4";
    const String timing_4 = "timing_4";

    const String amp_attack = "amp_attack";
    const String amp_decay = "amp_decay";
    const String amp_sustain = "amp_sustain";
    const String amp_release = "amp_release";
    const String amp_exponential_factor_a = "amp_exponential_factor_a";

    const String phaseTag = "phase";
    const String waveTypeTag = "waveType";

    const String driveTag = "driveDb";
    const String saturationTypeTag = "saturationType";
    AudioProcessorValueTreeState::Listener* paramListener;

}

float Gruvah::getCorrectedValue(String parameterId) {
    float new_value = vts.getParameterAsValue(parameterId).getValue().operator double();
//    printf("Parameter changed: %s, %f\n", parameterId.toRawUTF8(), new_value);

    return new_value;
}

Gruvah::Gruvah() = default;

void Gruvah::addParameters (Parameters& params)
{
    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { octave_1, 1}, "Octave 1", 0, 10, 8));
    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { note_1, 1}, "Note 1", 0, 11, 0));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { timing_1, 1}, "Timing 1 (ms)", 0.0f, 10.0f, 0.0f));

    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { octave_2, 1}, "Octave 2", 0, 10, 4));
    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { note_2, 1}, "Note 2", 0, 11, 7));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { timing_2, 1}, "Timing 2 (ms)", 0.0f, 10.0f, 2.12f));

    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { octave_3, 1}, "Octave 3", 0, 10, 3));
    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { note_3, 1}, "Note 3", 0, 11, 5));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { timing_3, 1}, "Timing 3 (ms)", 0.0f, 50.0f, 16.55f));

    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { octave_4, 1}, "Octave 4", 0, 10, 1));
    params.push_back (std::make_unique<AudioParameterInt> (ParameterID { note_4, 1}, "Note 4", 0, 11, 9));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { timing_4, 1}, "Timing 4 (ms)", 0.0f, 300.0f, 69.09f));

    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { amp_attack, 1}, "Amp Attack (ms)", 0.0f, 10.0f, 0.65f));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { amp_decay, 1}, "Amp Decay (ms)", 0.0f, 50.0f, 10.0f));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { amp_sustain, 1}, "Amp Sustain %", 0.0f, 100.0f, 100.0f));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { amp_release, 1}, "Amp Release (ms)", 0.0f, 1000.0f, 419.43f));
    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { amp_exponential_factor_a, 1}, "Amp Exponential Factor A", 1.0f, 10.0f, 4.31f));

    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { phaseTag, 1}, "Phase", 0.0f, 1.0f, 0.0f));
    params.push_back (std::make_unique<AudioParameterChoice> (ParameterID { waveTypeTag, 1}, "Wave Type", StringArray ("Sine", "909"), 0));

    params.push_back (std::make_unique<AudioParameterFloat> (ParameterID { driveTag, 1}, "Drive", 0.0f, 24.0f, 0.0f));
    params.push_back (std::make_unique<AudioParameterChoice> (ParameterID { saturationTypeTag, 1}, "Saturation Type", StringArray ("None", "Soft", "Clip", "ExtremeClip"), 0));
}

void Gruvah::prepareToPlay (double sampleRate, int samplesPerBlock)
{
    distProc.reset (kick_synth::create (sampleRate));


    paramListener = new ParamListener(this);

    // set each parameter into the newly created kick synth
    // The rust part doesn't "know" what the default values are, thus we need to set them here
    for (const auto& param : this->getParameters())
    {
        const auto* paramWithID = dynamic_cast<const AudioProcessorParameterWithID*>(param);


        float new_value = getCorrectedValue(paramWithID->getParameterID());
//        printf("param: %s, value: %f\n", paramWithID->getParameterID().toRawUTF8(), new_value);

        paramListener->parameterChanged(paramWithID->getParameterID(), new_value);
        vts.addParameterListener(paramWithID->getParameterID(), paramListener);
    }

}

void Gruvah::releaseResources()
{
}

void Gruvah::processMidi(MidiBuffer& midiBuffer, kick_synth::KickSynth *distProc)
{

    for (const MidiMessageMetadata metadata : midiBuffer)
    {
        juce::MidiMessage message = metadata.getMessage();

        const uint8_t *rawData = message.getRawData();

        if (!message.isNoteOnOrOff())
            continue;

        const kick_synth::MidiMessage *midiMessage = kick_synth::create_midi_message(message.getTimeStamp(), rawData, message.getNoteNumber(), message.getVelocity());
        kick_synth::process_midi_message (distProc, midiMessage);
        kick_synth::destroy_midi_message (midiMessage);
    }

}

void Gruvah::initUpdateParameters() {
    for (const auto& param : this->getParameters())
    {
        const auto* paramWithID = dynamic_cast<const AudioProcessorParameterWithID*>(param);
        float new_value = getCorrectedValue(paramWithID->getParameterID());
        updateParameter(paramWithID->getParameterID(), new_value);
    }
}

void Gruvah::afterSetStateInformation() {
    // This was to solve an ui bug where the labels for the midi notes would not be set after changing presets
    setMidiNoteLabel(1);
    setMidiNoteLabel(2);
    setMidiNoteLabel(3);
    setMidiNoteLabel(4);
}

void Gruvah::setMidiNoteLabel(int block) {
    String octave_id = "octave_" + String(block);
    String note_id = "note_" + String(block);

    String octave = magicState.getParameter(octave_id)->getCurrentValueAsText();
    String note = magicState.getParameter(note_id)->getCurrentValueAsText();

    int note_index = note.getIntValue(); // yes, this line makes me very uncomfortable. However, this is the only way I managed to get the labels working

    String midiNoteLetter = midiNoteNames[note_index];
    String midiNote = midiNoteLetter + octave;

    magicState.getPropertyAsValue("midi_note:" + String(block)).setValue(midiNote);
}

void Gruvah::processBlock (AudioBuffer<float>& buffer, MidiBuffer& midiBuffer)
{
    ScopedNoDenormals noDenormals;


    if (midiBuffer.getNumEvents() > 0) {
        this->processMidi(midiBuffer, distProc.get());
    }

    if (buffer.getNumChannels() == 1)
        kick_synth::process_mono (distProc.get(), buffer.getWritePointer (0), buffer.getNumSamples());
    else if (buffer.getNumChannels() == 2) {
        kick_synth::process (distProc.get(), buffer.getWritePointer (0), buffer.getWritePointer(1), buffer.getNumSamples());
    }
}

void Gruvah::updateParameter(const String &parameterID, float newValue) {
    if (distProc == nullptr) {
        return;
    }

    // Setting the labels for the midi notes
    if (parameterID.contains("octave_")) {
        String block = parameterID.substring(parameterID.length() - 1);
        String noteId = "note_" + block;

        int32_t octave = newValue;
        int32_t note = getCorrectedValue(noteId);

        String midiNoteLetter = midiNoteNames[note];
        String midiNote = midiNoteLetter + String(octave);

        magicState.getPropertyAsValue("midi_note:" + block).setValue(midiNote);
    }

    if (parameterID.contains("note_")) {
        String block = parameterID.substring(parameterID.length() - 1);
        String octaveId = "octave_" + block;

        int32_t octave = getCorrectedValue(octaveId);
        int32_t note = newValue;

        String midiNoteLetter = midiNoteNames[note];
        String midiNote = midiNoteLetter + String(octave);

        magicState.getPropertyAsValue("midi_note:" + block).setValue(midiNote);
    }

    kick_synth::update_param(distProc.get(), parameterID.toRawUTF8(), newValue);
}


// This creates new instances of the plugin...
AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new Gruvah();
}