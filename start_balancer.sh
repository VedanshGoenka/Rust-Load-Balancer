#!/bin/bash

usage() {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  --port <port>              Load balancer port (default: 8000)"
  echo "  --servers <urls>           Comma-separated list of backend server URLs"
  echo "  --interval <seconds>       Health check interval in seconds (default: 5)"
  echo "  --algorithm <algorithm>    Load balancing algorithm (default: round-robin)"
  echo "                            Options: round-robin, least-connections, random"
  exit 1
}

# Default Args
PORT=8000
SERVERS="http://localhost:8001,http://localhost:8002,http://localhost:8003"
INTERVAL=5
ALGORITHM="round-robin"

# Parse Command Line Args
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --port) PORT="$2"; shift ;;
    --servers) SERVERS="$2"; shift ;;
    --interval) INTERVAL="$2"; shift ;;
    --algorithm) ALGORITHM="$2"; shift ;;
    -h|--help) usage ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

echo "Starting load balancer..."
echo "Port: ${PORT}"
echo "Backend servers: ${SERVERS}"
echo "Health check interval: ${INTERVAL}s"
echo "Algorithm: ${ALGORITHM}"

cargo run -- balancer \
  --port "$PORT" \
  --servers "$SERVERS" \
  --interval "$INTERVAL" \
  --algorithm "$ALGORITHM" 