import argparse
import json
import os
import re
import sys

from rich.console import Console
from rich.table import Table
from rich.text import Text


def load_json(file_path):
    with open(file_path, "r") as file:
        return json.load(file)


def find_latest_files(directory="logs"):
    files = [f for f in os.listdir(directory) if re.match(r'agg_results_\d{8}_\d{6}\.json', f)]
    files.sort(reverse=True)
    if len(files) < 2:
        print("Not enough benchmark files found.")
        sys.exit(1)
    return os.path.join(directory, files[1]), os.path.join(directory, files[0])


def compare_benchmarks(file1, file2, include_mismatches=True):
    console = Console()
    data1 = load_json(file1)
    data2 = load_json(file2)

    table = Table(title="Benchmark Comparison", show_lines=True)
    table.add_column("Test Case", justify="left", style="cyan", no_wrap=True)
    table.add_column("Avg Turns Δ", justify="right", style="magenta")
    table.add_column("Avg Turns %", justify="right", style="magenta")
    table.add_column("Turn Time Δ (ms)", justify="right", style="yellow")
    table.add_column("Turn Time %", justify="right", style="yellow")

    for test in sorted(set(data1.keys()) | set(data2.keys())):
        if not include_mismatches and (test not in data1 or test not in data2):
            continue

        d1 = data1.get(test, {"average_turns": 0, "average_turn_time": 0})
        d2 = data2.get(test, {"average_turns": 0, "average_turn_time": 0})

        turn_diff = d2["average_turns"] - d1["average_turns"]
        turn_percent = (turn_diff / d1["average_turns"] * 100) if d1["average_turns"] else 0
        time_diff = (d2["average_turn_time"] - d1["average_turn_time"]) * 1000
        time_percent = (time_diff / (d1["average_turn_time"] * 1000) * 100) if d1["average_turn_time"] else 0

        turn_style = "green" if turn_diff < 0 else "red"
        time_style = "green" if time_diff < 0 else "red"
        name_style = "purple" if test not in data1 or test not in data2 else "cyan"

        table.add_row(
            Text(test, style=name_style),
            Text(f"{turn_diff:+.2f}", style=turn_style),
            Text(f"{turn_percent:+.2f}%", style=turn_style),
            Text(f"{time_diff:+.5f}", style=time_style),
            Text(f"{time_percent:+.2f}%", style=time_style)
        )

    console.print(table)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Compare two benchmark JSON files.")
    parser.add_argument("file1", nargs="?", help="First benchmark JSON file")
    parser.add_argument("file2", nargs="?", help="Second benchmark JSON file")
    parser.add_argument("--include-mismatches", action="store_true", help="Include mismatched entries in output")

    args = parser.parse_args()

    if not args.file1 or not args.file2:
        args.file1, args.file2 = find_latest_files()

    compare_benchmarks(args.file1, args.file2, args.include_mismatches)
