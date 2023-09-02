// Copyright (c) 2023 Joshua Azimullah
//
// This file is part of Gruvah.
//
// Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.

# pragma once

#include "JuceHeader.h"
#include "BinaryData.h"

/**
 * Base class for plugin processors.
 * 
 * Children must override prepareToPlay
 * and releaseResources (from AudioProcessor),
 * as well as processBlock, and addParameters.
 * */
template<class Processor>
class PluginBase : public AudioProcessor
{
public:
    PluginBase();
    ~PluginBase() override;

    const String getName() const override { return JucePlugin_Name; }

    bool acceptsMidi() const override { return true; }
    bool producesMidi() const override { return false; } 
    bool isMidiEffect() const override { return false; }

    double getTailLengthSeconds() const override { return 0.0; }
    
    int getNumPrograms() override { return 1; }
    int getCurrentProgram() override { return 0; }
    void setCurrentProgram (int) override {}
    const String getProgramName (int) override { return {}; }
    void changeProgramName (int, const String&) override {}

    bool isBusesLayoutSupported (const BusesLayout& layouts) const override;
    void prepareToPlay (double sampleRate, int samplesPerBlock) override;
    virtual void processBlock (AudioBuffer<float>&, MidiBuffer&) override = 0;

    bool hasEditor() const override { return true; }

    virtual void initUpdateParameters() {}
    virtual void afterSetStateInformation() {}

    AudioProcessorEditor* createEditor() override {
        initUpdateParameters();
        return new foleys::MagicPluginEditor (magicState);
    }

    void getStateInformation (MemoryBlock& data) override {
        magicState.getStateInformation (data);
    }

    void setStateInformation (const void* data, int sizeInBytes) override {
        magicState.setStateInformation (data, sizeInBytes, getActiveEditor());
        magicState.setGuiValueTree( BinaryData::magic_xml, BinaryData::magic_xmlSize );
        afterSetStateInformation();
    }

protected:
    using Parameters = std::vector<std::unique_ptr<RangedAudioParameter>>;
    AudioProcessorValueTreeState vts;
    foleys::MagicProcessorState magicState { *this };

private:
    AudioProcessorValueTreeState::ParameterLayout createParameterLayout();

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (PluginBase)
};

// check is template class has addParameters
template<typename T>
class HasAddParameters
{
    typedef char one;
    typedef long two;

    template <typename C> static one test (decltype(&C::addParameters));
    template <typename C> static two test (...);

public:
    enum { value = sizeof (test<T> (0)) == sizeof (char) };
};

template<class Processor>
PluginBase<Processor>::PluginBase() :
    AudioProcessor (BusesProperties()
        .withOutput ("Output", AudioChannelSet::stereo(), true)),
    vts (*this, nullptr, Identifier ("Parameters"), createParameterLayout())
{
        magicState.setGuiValueTree( BinaryData::magic_xml, BinaryData::magic_xmlSize );
}

template<class Processor>
PluginBase<Processor>::~PluginBase() {}

template<class Processor>
AudioProcessorValueTreeState::ParameterLayout PluginBase<Processor>::createParameterLayout()
{
    Parameters params;

    static_assert (HasAddParameters<Processor>::value,
        "Processor class MUST contain a static addParameters function!");
    Processor::addParameters (params);

    return { params.begin(), params.end() };
}

template<class Processor>
bool PluginBase<Processor>::isBusesLayoutSupported (const BusesLayout& layouts) const
{
    // only supports mono and stereo (for now)
    if (layouts.getMainOutputChannelSet() != AudioChannelSet::mono()
     && layouts.getMainOutputChannelSet() != AudioChannelSet::stereo())
        return false;

    return true;
}

template<class Processor>
void PluginBase<Processor>::prepareToPlay (double sampleRate, int samplesPerBlock)
{
    setRateAndBufferSizeDetails (sampleRate, samplesPerBlock);
    magicState.prepareToPlay (sampleRate, samplesPerBlock);
}
