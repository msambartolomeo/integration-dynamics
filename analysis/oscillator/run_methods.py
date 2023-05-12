import os
import subprocess

METHODS = [
    "euler",
    "euler-mod",
    "verlet",
    "verlet-leap-frog",
    "velocity-verlet",
    "beeman",
    "euler-predictor-corrector",
    "gear-predictor-corrector",
]

DELTA_T = [0.01, 0.001, 0.0001, 0.00001, 0.000001]

RESULTS_PATH = "./analysis/oscillator/data/methods/"


def main():
    os.makedirs(RESULTS_PATH, exist_ok=True)
    for method in METHODS:
        os.makedirs(RESULTS_PATH + method, exist_ok=True)
        for delta_t in DELTA_T:
            subprocess.run(
                [
                    "./target/release/oscillator",
                    method,
                    "--data-output-path",
                    RESULTS_PATH + method + f"/{delta_t}.txt",
                    "--simulation-delta-t",
                    str(delta_t),
                    "--output-delta-t",
                    str(delta_t),
                ]
            )


if __name__ == "__main__":
    main()
