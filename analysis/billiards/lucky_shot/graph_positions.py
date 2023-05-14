import os

import matplotlib.pyplot as plt
import numpy as np

RESULTS_PATH = "./analysis/billiards/lucky_shot/figs/"

DIR = "./analysis/billiards/lucky_shot/data/"


def read_times() -> dict[float, list[float]]:
    times_per_pos: dict[float, list[float]] = {}

    for file in os.listdir(DIR):
        if file.endswith(".txt"):
            # remove extension
            y_pos, _ = os.path.splitext(file)
            y_pos = float(y_pos)

            times_per_pos[y_pos] = []

            with open(DIR + file, "r") as f:
                times = []
                for line in f:
                    times.append(float(line))

                times_per_pos[y_pos].append(times)

    # Sort by position ascending
    return dict(sorted(times_per_pos.items()))


def plot():
    times_per_pos = read_times()

    # Plot curves values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.xlabel("Posición inicial en Y de la bola blanca (cm)", fontsize=18)
    plt.ylabel("Tiempo total de simulación (s)", fontsize=18)

    for pos, times in times_per_pos.items():
        plt.errorbar(
            pos * 100,
            np.mean(times),
            yerr=np.std(times) / np.sqrt(len(times)),
            fmt="bx",
            ecolor="r",
            capsize=5,
        )

    fig.savefig(RESULTS_PATH + "positions.png")

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
