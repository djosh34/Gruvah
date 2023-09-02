cmake_minimum_required(VERSION 3.15)
project(gruvah-rs VERSION 1.0.1)

set(CMAKE_CXX_STANDARD 17)
add_subdirectory(modules)


juce_add_plugin(Gruvah
    COMPANY_NAME DJKaasstengel
    PLUGIN_MANUFACTURER_CODE KSGL
    PLUGIN_CODE GRUV
    NEEDS_MIDI_INPUT TRUE
    IS_SYNTH TRUE
    FORMATS AU VST3 Standalone #VST
    ProductName "Gruvah")

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

# Setup Rust
enable_language(Rust)
include(CMakeCargo)

add_subdirectory(gruvahlib)
include_directories(gruvahlib)

add_custom_command(TARGET Gruvah PRE_BUILD
    COMMAND cbindgen -d --lang c++ -o gruvah.h .
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/gruvahlib/"
    COMMENT "Generating Rust to C++ bindings..."
)

# Setup JUCE
juce_generate_juce_header(Gruvah)
add_subdirectory(src)

# Add foleys gui
juce_add_binary_data(GruvahData SOURCES
    ${CMAKE_SOURCE_DIR}/resources/magic.xml
)

target_compile_definitions(Gruvah
    PUBLIC
    JUCE_DISPLAY_SPLASH_SCREEN=0
    JUCE_REPORT_APP_USAGE=0
    JUCE_WEB_BROWSER=0
    JUCE_USE_CURL=0
    JUCE_VST3_CAN_REPLACE_VST2=0
    FOLEYS_SHOW_GUI_EDITOR_PALLETTE=0 # Set this to 1 to change the GUI
    )

# Set compiler flags
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic -Wno-unused-parameter -Wno-unused-variable -Wno-unused-function -Wno-unused-private-field -Wno-unused-but-set-variable -Wno-unused-lambda-capture -Wno-unused-value -Wno-unused")

target_link_libraries(Gruvah PRIVATE
    GruvahData
    juce::juce_audio_utils
    foleys_gui_magic
        gruvahlib
    )

if(WIN32)
    target_link_libraries(Gruvah PRIVATE userenv)
endif()
