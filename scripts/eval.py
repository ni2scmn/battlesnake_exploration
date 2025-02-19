import subprocess
import re
import time
from datetime import datetime, timedelta, date

def start_backend():
    return subprocess.Popen(["cargo", "run", "--release"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

def stop_backend(proc):
    proc.terminate()
    proc.wait()

def run_battlesnake():
    result = subprocess.run(["scripts/battlesnake", "play", "-W", "100", "-H", "100", "--name", "'Python Starter Project'", "--url", "http://localhost:8000", "-g", "solo"], capture_output=True, text=True)
    return result.stderr

def extract_turns(output):
    match = re.search(r"Game completed after (\d+) turns.", output)
    return match.group(1) if match else "Unknown"

def calculate_average_turn_time(log_lines):
    timestamps = []
    for line in log_lines.strip().split('\n'):
        match = re.search(r'INFO ([\d:.]+) Turn:', line)
        if match:
            parsed_time = datetime.strptime(match.group(1), "%H:%M:%S.%f").time()
            parsed_dtime = datetime.combine(date.today(), parsed_time)
            timestamps.append(parsed_dtime)

    if len(timestamps) < 2:
        return "Not enough data to calculate average turn time."

        # Compute differences between consecutive times
    diffs = [timestamps[i+1] - timestamps[i] for i in range(len(timestamps)-1)]

    # Calculate average difference
    avg_turn_time = sum(diffs, timedelta()) / len(diffs) if diffs else timedelta()
    return avg_turn_time.total_seconds()


def main():
    backend_proc = start_backend()
    time.sleep(2)  # Give some time for backend to start

    try:
        for i in range(100):
            output = run_battlesnake()
            turns = extract_turns(output)
            avg_turn_time = calculate_average_turn_time(output)
            print(f"Game {i+1}: {turns} turns")
            print(f"Average turn time: {avg_turn_time} seconds")
    finally:
        stop_backend(backend_proc)

if __name__ == "__main__":
    main()
