import os
import subprocess

DELTA_T = [0.01, 0.001, 0.0001, 0.00001, 0.000001]

RESULTS_PATH = "./analysis/billiards/parallel_universes/data/"


def main():
    os.makedirs(RESULTS_PATH, exist_ok=True)
    for delta_t in DELTA_T:
        subprocess.run(
            [
                "./target/release/billiards",
                "gear-predictor-corrector",
                "-f",
                "-i",
                "--data-output-path",
                RESULTS_PATH + f"/{delta_t}.txt",
                "--simulation-delta-t",
                str(delta_t),
                "--output-delta-t",
                "0.01",
                "-m100",
            ]
        )


if __name__ == "__main__":
    main()
