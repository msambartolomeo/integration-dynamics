import os
import matplotlib.pyplot as plt

RESULTS_PATH = "./analysis/oscillator/figs/"

DIR = "./analysis/oscillator/data/methods/"


def read_method_data() -> dict[str, dict[str, list[float]]]:
    data_per_method: dict[str, dict[str, list[float]]] = {}

    for file in os.listdir(DIR):
        if file.endswith(".txt"):
            # remove extension
            method, _ = os.path.splitext(file)

            # Read the file and get the times
            with open(DIR + file, "r") as f:
                data = {
                    'times': [],
                    'numerics': [],
                    'analytics': [],
                    'errors': [],
                }
                for line in f:
                    splits = line.split(" ")

                    data['times'].append(float(splits[0]))
                    data['numerics'].append(float(splits[1]))
                    data['analytics'].append(float(splits[2]))
                    data['errors'].append(float(splits[3]))

                # Add the data to the dict
                data_per_method[method] = data

    return data_per_method


def plot():
    data = read_method_data()

    # Plot error values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams.update({"font.size": 16})
    plt.xlabel('Tiempo transcurrido (s)')
    plt.ylabel('Error cuádratico medio')

    for method in data:
        plt.plot(data[method]['times'], data[method]['errors'], label=method)

    plt.legend()
    # Save second figure to a file
    fig.savefig(RESULTS_PATH + 'error_values.png')

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
