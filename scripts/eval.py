import json
import subprocess
import re
import time
from collections import defaultdict
from datetime import datetime, timedelta, date
from pathlib import Path
from tqdm import tqdm


def start_backend(strategy: str):
    return subprocess.Popen(
        ["cargo", "run", "--release", strategy],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def stop_backend(proc):
    proc.terminate()
    proc.wait()


def run_battlesnake(width: int, height: int):
    result = subprocess.run(
        [
            "scripts/battlesnake",
            "play",
            "-W",
            str(width),
            "-H",
            str(height),
            "--name",
            "'Battlesnake Exploration'",
            "--url",
            "http://localhost:8000",
            "-g",
            "solo",
        ],
        capture_output=True,
        text=True,
    )
    return result.stderr


def extract_turns(output):
    match = re.search(r"Game completed after (\d+) turns.", output)
    return int(match.group(1)) if match else -1


def calculate_average_turn_time(log_lines):
    timestamps = []
    for line in log_lines.strip().split("\n"):
        match = re.search(r"INFO ([\d:.]+) Turn:", line)
        if match:
            parsed_time = datetime.strptime(match.group(1), "%H:%M:%S.%f").time()
            parsed_dtime = datetime.combine(date.today(), parsed_time)
            timestamps.append(parsed_dtime)

    if len(timestamps) < 2:
        return timedelta()

        # Compute differences between consecutive times
    diffs = [timestamps[i + 1] - timestamps[i] for i in range(len(timestamps) - 1)]

    # Calculate average difference
    avg_turn_time = sum(diffs, timedelta()) / len(diffs) if diffs else timedelta()
    return avg_turn_time.total_seconds()


def main():
    strategies = ["random", "simple"]
    board_sizes = [(10, 10), (15, 15), (20, 20)]
    results = []
    aggregated_results = defaultdict(
        lambda: {"total_turns": 0, "total_games": 0, "total_time": 0.0}
    )
    log_dir = Path("logs")
    log_dir.mkdir(exist_ok=True)

    for strategy in tqdm(strategies, desc="Strategies", leave=True):
        backend_proc = start_backend(strategy)
        time.sleep(2)  # Allow backend to start

        try:
            for width, height in tqdm(board_sizes, desc="Board Sizes", leave=True):
                for i in tqdm(range(100), desc="Games", leave=False):
                    output = run_battlesnake(width, height)
                    turns = extract_turns(output)
                    avg_turn_time = calculate_average_turn_time(output)

                    game_result = {
                        "strategy": strategy,
                        "board_size": f"{width}x{height}",
                        "game_number": i + 1,
                        "turns": turns,
                        "avg_turn_time": avg_turn_time,
                    }
                    results.append(game_result)
                    key = (strategy, f"{width}x{height}")
                    aggregated_results[key]["total_turns"] += turns
                    aggregated_results[key]["total_games"] += 1
                    aggregated_results[key]["total_time"] += (
                        avg_turn_time if avg_turn_time else 0.0
                    )

        finally:
            stop_backend(backend_proc)

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    log_filename = log_dir / f"log_results_{timestamp}.json"
    with open(log_filename, "w") as f:
        json.dump(results, f, indent=4)

    aggregated_output = {}
    for (strategy, board_size), data in aggregated_results.items():
        avg_turns = data["total_turns"] / data["total_games"]
        avg_time = (
            data["total_time"] / data["total_games"] if data["total_games"] > 0 else 0.0
        )
        aggregated_output[f"{strategy}_{board_size}"] = {
            "average_turns": avg_turns,
            "average_turn_time": avg_time,
        }

    agg_filename = log_dir / f"agg_results_{timestamp}.json"
    with open(agg_filename, "w") as f:
        json.dump(aggregated_output, f, indent=4)


if __name__ == "__main__":
    main()
