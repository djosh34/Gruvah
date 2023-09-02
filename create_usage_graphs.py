import matplotlib.pyplot as plt
import numpy as np


# Main function
def create_pitch_envelope_image():
    # Define pitch values and corresponding times
    timings = [20, 20, 30, 80]  # These are the times for each segment
    frequencies = [8000, 800, 400, 50]  # Corresponding pitch values
    ylim = max(frequencies) + 1000  # Set the y-axis limit

    # Labels for the points
    labels = ["C8", "G4", "F3", "A1"]




    plt.rcParams.update({
        "lines.color": "white",
        "patch.edgecolor": "white",
        "text.color": "black",
        "axes.facecolor": "white",
        "axes.edgecolor": "lightgray",
        "axes.labelcolor": "white",
        "xtick.color": "white",
        "ytick.color": "white",
        "grid.color": "lightgray",
        "figure.facecolor": "black",
        "figure.edgecolor": "black",
        "savefig.facecolor": "black",
        "savefig.edgecolor": "black"})

    # Create the pitch envelope graph
    plt.figure(figsize=(10, 6))
    plt.title('Pitch Envelope')
    plt.xlabel('Time')
    plt.ylabel('Pitch')

    ax = plt.gca()  # Get the current axes
    ax.set_facecolor('black')  # Set the background color to black

    # Remove numbers on the axes
    ax.set_xticks([])  # Remove x-axis labels
    ax.set_yticks([])  # Remove y-axis labels
    ax.set_ylim(0, ylim)  # Set the y-axis limits

    # Define offsets for text placement
    x_offset = 3  # Offset to the right
    y_offset = 400  # Offset upwards
    y_text_offset = 200  # Offset for the text

    # Initialize variables for the current position
    current_time = 0
    current_frequency = 0

    # Iterate through the timings and frequencies to draw lines and vertical bars
    for idx, (time, freq) in enumerate(zip(timings, frequencies)):
        # Draw a vertical line
        plt.axvline(current_time, color='gray', linestyle='--', linewidth=0.5)

        # Draw a line connecting the current point to the next
        plt.plot([current_time, current_time + time], [current_frequency, freq], marker='o', linestyle='-', color='white')


        # Update the current position
        current_time += time
        current_frequency = freq

        label = labels[idx]  # Get the label from the array
        plt.text(current_time + x_offset, freq + y_offset, label, color='white', verticalalignment='center', horizontalalignment='left')

    # Draw a vertical line
    plt.axvline(current_time, color='gray', linestyle='--', linewidth=0.5)

    # Remove axis spines (optional)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

    # Separate loop for <--> and text combinations at the top
    current_time = 0  # Reset the current time

    # Define the font properties
    font_properties = {
        'family': 'verdana',
        'size': 9,
    }

    for idx, time in enumerate(timings):  # Exclude the last timing
        timing_label = f'Timing {idx + 1}'
        plt.text(current_time + time / 2, ylim + y_text_offset, timing_label, fontdict=font_properties, color='white', horizontalalignment='center')

        arrow_props = dict(arrowstyle='<->', color='white')
        plt.annotate('', xy=(current_time, ylim), xytext=(current_time + time, ylim), arrowprops=arrow_props)

        current_time += time

    plt.savefig('pitch_envelope.png', facecolor='black', edgecolor='black')

def create_amp_envelope_image():
    plt.rcParams.update({
        "lines.color": "white",
        "patch.edgecolor": "white",
        "text.color": "black",
        "axes.facecolor": "white",
        "axes.edgecolor": "lightgray",
        "axes.labelcolor": "white",
        "xtick.color": "white",
        "ytick.color": "white",
        "grid.color": "lightgray",
        "figure.facecolor": "black",
        "figure.edgecolor": "black",
        "savefig.facecolor": "black",
        "savefig.edgecolor": "black"})

    # Define ADSR parameters
    attack = 0.05  # Attack time in seconds
    decay = 0.2   # Decay time in seconds
    sustain = 0.5 # Sustain level (between 0 and 1)
    release = 0.6 # Release time in seconds
    ylim = 1.1    # Set the y-axis limit
    y_text_offset = 0.05  # Offset for the text

    # Time values for each segment
    times = [0, attack, attack + decay, attack + decay + release]

    # Amplitude values for each segment
    amplitudes = [0, 1, sustain, 0]

    # Create the ADSR envelope graph
    plt.figure(figsize=(10, 6))
    ax = plt.gca()  # Get the current axes
    ax.set_facecolor('black')

    ax.set_xticks([])  # Remove x-axis labels
    ax.set_yticks([])  # Remove y-axis labels
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

    plt.title('ADSR Envelope')
    plt.xlabel('Time')
    plt.ylabel('Amplitude')

    # Plot the ADSR segments
    plt.plot(times, amplitudes, marker='o', linestyle='-', color='white')

    font_properties = {
        'family': 'verdana',
        'size': 9,
    }

    labels = ["Attack", "Decay", "Release"]

    for idx, (time, label) in enumerate(zip(times[:-1], labels)):  # Exclude the last timing
        plt.text((time + times[idx + 1]) / 2, ylim + y_text_offset, label, fontdict=font_properties, color='white', horizontalalignment='center')

        arrow_props = dict(arrowstyle='<->', color='white')
        plt.annotate('', xy=(time, ylim), xytext=(times[idx + 1], ylim), arrowprops=arrow_props)

    # Draw a vertical line for sustain
    arrow_props = dict(arrowstyle='<->', color='white')
    plt.annotate('', xy=(times[2], 0.02), xytext=(times[2], sustain - 0.02), arrowprops=arrow_props)
    plt.text(times[2] - 0.05, sustain / 2, "Sustain", fontdict=font_properties, color='white', horizontalalignment='center')

    # curved dashed release
    bounces = [6, 4, 2, 1.5]
    for bounce in bounces:
        x = np.linspace(0, 1, 50, endpoint=True)
        y = sustain * (1 - x) ** bounce

        x *= release
        x += times[2]
        plt.plot(x, y, linestyle='--', color='gray', linewidth=1)

    # explain curved dashed release line by having the word bounce with an array pointing to the line
    start_x = 0.3
    start_y = sustain * (1 - start_x) ** bounces[0]
    start_x *= release
    start_x += times[2]

    end_x = 0.4
    end_y = sustain * (1 - end_x)
    end_x *= release
    end_x += times[2]

    arrow_props = dict(arrowstyle='<->', color='white')
    plt.annotate('', xy=(start_x, start_y), xytext=(end_x, end_y), arrowprops=arrow_props)
    text_x = (start_x * 1.6 + end_x) / 2.6
    text_y = (start_y * 1.6 + end_y) / 2.6

    plt.text(text_x - 0.01, text_y + 0.01, "Bounce", rotation=63, fontdict=font_properties, color='white', horizontalalignment='center')

    # Draw a vertical line
    for time in times:
        plt.axvline(time, color='gray', linestyle='--', linewidth=0.5)

    ax.set_ylim(0, ylim)  # Set the y-axis limits
    # Show the envelope
    plt.savefig('amp_envelope.png', facecolor='black', edgecolor='black')


def main():
    print("Creating Pitch Envelope Image:")
    # create_pitch_envelope_image()

    print("\nCreating Amplitude Envelope Image:")
    create_amp_envelope_image()

if __name__ == "__main__":
    main()