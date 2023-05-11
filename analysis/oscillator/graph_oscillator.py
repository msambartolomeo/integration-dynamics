import os
import matplotlib.pyplot as plt

RESULTS_PATH = "./analysis/oscillator/figs/"


def read_data_from_file(path):
    with open(path, "r") as f:
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

    return data


def plot(path):
    data = read_data_from_file(path)

    times = data['times']
    numeric = data['numerics']
    analytic = data['analytics']

    # Plot numeric and analytic values on one figure
    fig = plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})
    plt.xlabel('Tiempo transcurrido (s)')
    plt.ylabel('Posicion del osciladór (m)')

    plt.plot(times, numeric, label='Numérica')
    plt.plot(times, analytic, label='Analítica')

    plt.legend()
    # Save first figure to a file
    fig.savefig(RESULTS_PATH + 'numeric_analytic_values.png')

    plt.show()


if __name__ == "__main__":
    os.makedirs(RESULTS_PATH, exist_ok=True)
    plot("oscillator.txt")
