import os

import matplotlib.pyplot as plt
import numpy as np

RESULTS_PATH = "./analysis/billiards/parallel_universes/figs/"

DIR = "./analysis/billiards/parallel_universes/data/"

DELTAS = [0.01, 0.001, 0.0001, 0.00001, 0.000001]
BALL_COUNT = 16


def read_position_data():
    data: dict[float, dict[float, list[list[float]]]] = {}

    for file in os.listdir(DIR):
        if file.endswith(".txt"):
            # remove extension
            delta_t, _ = os.path.splitext(file)
            delta_t = float(delta_t)

            data[delta_t] = {}

            # Read the file and get the times
            with open(DIR + file, "r") as f:
                time = 0
                for line in f:
                    try:
                        time = float(line)
                        data[delta_t][time] = []
                    except ValueError:
                        splits = line.split(" ")
                        data[delta_t][time].append(
                            np.array([float(splits[0]), float(splits[1])])
                        )

    return data


def get_phi(data):
    phi = []
    for i in range(len(DELTAS) - 1):
        phi.append([])
        times = []
        for time in data[DELTAS[i]].keys():
            times.append(time)

            phi_t = 0
            for j in range(BALL_COUNT):
                phi_t += np.linalg.norm(
                    data[DELTAS[i + 1]][time][j] - data[DELTAS[i]][time][j]
                )

            phi[i].append(phi_t)

    return phi, times


def plot():
    data = read_position_data()

    phis, times = get_phi(data)

    # Plot curves values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.xlabel("Tiempo transcurrido (s)")
    plt.ylabel("Φ")
    plt.yscale("log")

    for i, phi in enumerate(phis):
        plt.plot(times, phi, label=f"k={i + 2}")

    plt.legend()
    fig.savefig(RESULTS_PATH + "phi.png")

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
