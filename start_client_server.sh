#!/bin/bash

usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -p, --port <port>             Load balancer port (default: 8000)"
    echo "  -s, --servers <num>           Number of backend servers (default: 3)"
    echo "  -g, --get-delay <ms>          GET request delay (default: 100)"
    echo "  -o, --post-delay <ms>         POST request delay (default: 200)"
    exit 1
}

# Default values
LB_PORT=8000
NUM_SERVERS=3
GET_DELAY=100
POST_DELAY=200

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -p|--port) LB_PORT="$2"; shift ;;
        -s|--servers) NUM_SERVERS="$2"; shift ;;
        -g|--get-delay) GET_DELAY="$2"; shift ;;
        -o|--post-delay) POST_DELAY="$2"; shift ;;
        -h|--help) usage ;;
        *) echo "Unknown parameter: $1"; usage ;;
    esac
    shift
done

# Start backend servers
SERVER_LIST=""
for ((i=1; i<=NUM_SERVERS; i++)); do
    PORT=$((8000 + i))
    echo "Starting server $i on port $PORT..."
    cargo run -- server -p $PORT -g $GET_DELAY -o $POST_DELAY &
    if [ -n "$SERVER_LIST" ]; then
        SERVER_LIST="$SERVER_LIST,"
    fi
    SERVER_LIST="${SERVER_LIST}127.0.0.1:$PORT"
done

# Start load balancer
echo "Starting load balancer on port $LB_PORT with servers: $SERVER_LIST"
cargo run -- balancer -p $LB_PORT -s "$SERVER_LIST"

# Kill all background processes on exit
trap "pkill -P $$" EXIT 