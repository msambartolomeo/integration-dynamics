import os
import subprocess

import numpy as np

Y_MAX_POS = 0.56
Y_MIN_POS = 0.42
Y_STEP = 0.007


RUNS_PER_POS = 100

BALLS_TO_WAIT_FOR = 8
DELTA_T = 0.0001


RESULTS_PATH = f"./analysis/billiards/lucky_shot/data/"

os.makedirs(RESULTS_PATH, exist_ok=True)
for y_pos in np.arange(Y_MIN_POS, Y_MAX_POS, Y_STEP):
    y_pos_str = str(round(y_pos, 3))
    y_offset_str = str(round(Y_MAX_POS - y_pos, 3))
    print(f"Starting runs for Y position {y_pos_str} and offset {y_offset_str}")
    times = []
    for run_idx in range(RUNS_PER_POS):
        run = subprocess.run(
            [
                "./target/release/billiards",
                "gear-predictor-corrector",
                "-w" + y_offset_str,
                "--simulation-delta-t",
                str(DELTA_T),
                "--output-delta-t",
                "0.01",
                "-b",
                str(BALLS_TO_WAIT_FOR),
            ],
            capture_output=True,
            text=True,
        )

        times.append(run.stdout.split(" ")[2])

    with open(RESULTS_PATH + y_pos_str + ".txt", "a") as f:
        for time in times:
            f.write(time)
