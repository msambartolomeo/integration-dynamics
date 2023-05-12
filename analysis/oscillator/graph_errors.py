import os

import matplotlib.pyplot as plt
import numpy as np

RESULTS_PATH = "./analysis/oscillator/figs/"

DIR = "./analysis/oscillator/data/methods/"


def read_method_data():
    data_per_method = {}

    for method in os.listdir(DIR):
        data_per_method[method] = {}
        for file in os.listdir(DIR + method):
            if file.endswith(".txt"):
                # remove extension
                delta_t, _ = os.path.splitext(file)
                delta_t = float(delta_t)

                # Read the file and get the times
                with open(DIR + method + "/" + file, "r") as f:
                    times = []
                    numerics = []
                    analytics = []

                    for line in f:
                        splits = line.split(" ")

                        times.append(float(splits[0]))
                        numerics.append(float(splits[1]))
                        analytics.append(float(splits[2]))

                    data = {
                        "times": np.array(times),
                        "numeric": np.array(numerics),
                        "analytic": np.array(analytics),
                    }

                    # Add the data to the dict
                    data_per_method[method][delta_t] = data

    return data_per_method


def plot():
    data = read_method_data()

    # Plot error values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Error cuádratico medio")
    plt.xlabel("Delta t (s)")
    plt.xscale("log")
    plt.yscale("log")

    for method in data:
        deltas = []
        errors = []
        for delta_t in data[method]:
            deltas.append(delta_t)

            dif = data[method][delta_t]["numeric"] - data[method][delta_t]["analytic"]
            error = np.average(dif**2)
            errors.append(error)

        plt.plot(
            deltas,
            errors,
            "-o",
            label=method,
        )

    plt.legend()
    fig.savefig(RESULTS_PATH + "error_values.png")

    analytic = data["euler"][0.0001]["analytic"]
    times = data["euler"][0.0001]["times"]

    protector = data["gear-predictor-corrector"][0.0001]["numeric"]
    verletto = data["verlet"][0.0001]["numeric"]
    beemovie = data["beeman"][0.0001]["numeric"]

    # Plot curves values
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.xlabel("Tiempo transcurrido (s)")
    plt.ylabel("Posición del oscilador (m)")

    plt.plot(times, analytic, label="Analítica")
    plt.plot(times, protector, label="Gear Predictor Corrector")
    plt.plot(times, verletto, label="Verlet")
    plt.plot(times, beemovie, label="Beeman")

    plt.legend()
    fig.savefig(RESULTS_PATH + "values.png")

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot()
