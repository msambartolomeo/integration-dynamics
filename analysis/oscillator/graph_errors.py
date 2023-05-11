import os
import numpy as np
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
                times = []
                numerics = []
                analytics = []

                for line in f:
                    splits = line.split(" ")

                    times.append(float(splits[0]))
                    numerics.append(float(splits[1]))
                    analytics.append(float(splits[2]))

                data = {
                    'times': np.array(times),
                    'numeric': np.array(numerics),
                    'analytic': np.array(analytics),
                }

                # Add the data to the dict
                data_per_method[method] = data

    return data_per_method


def plot():
    data = read_method_data()

    # Plot error values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel('Error cuádratico medio')
    plt.xlabel('Metodo de integración')

    errors = []
    for method in data:
        dif = data[method]['numeric'] - data[method]['analytic']
        error = np.average(dif ** 2)
        errors.append(error)

    plt.bar(data.keys(), errors)
    # Scientific notation
    plt.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))

    fig.savefig(RESULTS_PATH + 'error_values.png')

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
