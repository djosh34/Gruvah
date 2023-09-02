// Copyright (c) 2023 Joshua Azimullah
//
// This file is part of Gruvah.
//
// Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.

#ifndef GruvahPLUGIN_H_INCLUDED
#define GruvahPLUGIN_H_INCLUDED

#include <JuceHeader.h>
#include <memory>

#include "PluginBase.h"
#include "gruvah.h"


class Gruvah : public PluginBase<Gruvah>
{
public:
    Gruvah();

    static void addParameters (Parameters& params);
    void prepareToPlay (double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;
    void processBlock (AudioBuffer<float>& buffer, MidiBuffer& midiBuffer) override;

    void updateParameter(const String &parameterID, float newValue);
    void initUpdateParameters() override;
    void afterSetStateInformation() override;

private:

    std::unique_ptr<kick_synth::KickSynth, decltype(&kick_synth::destroy)> distProc { nullptr, &kick_synth::destroy};

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (Gruvah)

    void processMidi(MidiBuffer &midiBuffer, kick_synth::KickSynth *distProc);
    float getCorrectedValue(String param);

    void setMidiNoteLabel(int block);
};

#endif // GruvahPLUGIN_H_INCLUDED
