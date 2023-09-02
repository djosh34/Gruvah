// Copyright (c) 2023 Joshua Azimullah
//
// This file is part of Gruvah.
//
// Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.

#include <JuceHeader.h>
#include "GruvahPlugin.h"

class ParamListener : public AudioProcessorValueTreeState::Listener
{
public:

    ParamListener(Gruvah* processor) : processor(processor) {}

    void parameterChanged (const String &parameterID, float newValue);
private:
    Gruvah* processor;

};
