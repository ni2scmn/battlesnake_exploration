#!/bin/bash

N=10  # Change this to the number of backends you want to start
START_PORT=8001
BACKENDS=()

# Start backends
for ((i=0; i<N; i++)); do
    PORT=$((START_PORT + i))
    ROCKET_PORT=$PORT cargo run --release simple &
    BACKENDS+=($!)  # Store process ID
    echo "Started backend on port $PORT with PID ${BACKENDS[-1]}"
done

# Wait a bit to ensure backends start properly
sleep 5

# Construct benchmark command
BENCH_CMD="./battlesnake play -W 100 -H 100"
for ((i=0; i<N; i++)); do
    PORT=$((START_PORT + i))
    BENCH_CMD+=" --name '$((i+1))' --url http://localhost:$PORT"
done
BENCH_CMD+=" -g solo --browser"

# Run benchmark
eval $BENCH_CMD

# Terminate all backends
for PID in "${BACKENDS[@]}"; do
    echo "Stopping backend with PID $PID"
    kill $PID
done

