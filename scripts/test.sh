#!/bin/bash

# Default values
N=3
START_PORT=8001
LOG_DIR="backend_logs"
FIELD_SIZE_W=20
FIELD_SIZE_H=20
RELEASE_MODE=false

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -n|--num-backends)
            N=$2
            shift 2
            ;;
        -p|--start-port)
            START_PORT=$2
            shift 2
            ;;
        -w|--width)
            FIELD_SIZE_W=$2
            shift 2
            ;;
        -h|--height)
            FIELD_SIZE_H=$2
            shift 2
            ;;
        -r|--release)
            RELEASE_MODE=$2
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

mkdir -p $LOG_DIR
BACKENDS=()

# Start backends
for ((i=0; i<N; i++)); do
    PORT=$((START_PORT + i))
    LOG_FILE="$LOG_DIR/backend_$PORT.log"
    if [ "$RELEASE_MODE" = true ]; then
        ROCKET_PORT=$PORT cargo run --release simple > "$LOG_FILE" 2>&1 &
    else
        ROCKET_PORT=$PORT cargo run simple > "$LOG_FILE" 2>&1 &
    fi
    PID=$!
    BACKENDS+=($PID)
    echo "Started backend on port $PORT with PID $PID (logging to $LOG_FILE)"
done

# Wait a bit to ensure backends start properly
sleep 5

# Construct benchmark command
BENCH_CMD="./battlesnake play -W $FIELD_SIZE_W -H $FIELD_SIZE_H"
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
