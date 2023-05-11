import os
import subprocess

import numpy as np


METHODS = ['euler', 'euler-mod', 'beeman', 'gear-predictor-corrector']

RUNS_PER_POS = 200


RESULTS_PATH = "./analysis/oscillator/data/methods/"


def main():

    os.makedirs(RESULTS_PATH, exist_ok=True)
    for method in METHODS:
        subprocess.run(
            [
                "./target/release/oscillator",
                method,
                "--data-output-path",
                RESULTS_PATH + f"{method}.txt",
            ]
        )


if __name__ == "__main__":
    main()
