#!/bin/bash

usage() {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  --port <port>                Port for server (default: 8000)"
  echo "  --get-delay <ms>             GET request delay in ms (default: 1000)"
  echo "  --post-delay <ms>            POST request delay in ms (default: 500)"
  echo "  --num-clients <num>          Number of clients (default: 10)"
  echo "  --read-write-ratio <ratio>   Read/Write ratio (default: 0.5)"
  exit 1
}

# Default Args
PORT=8000
GET_DELAY=1000
POST_DELAY=500
NUM_CLIENTS=10
READ_WRITE_RATIO=0.5

# Parse Command Line Args
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --port) PORT="$2"; shift ;;
    --get-delay) GET_DELAY="$2"; shift ;;
    --post-delay) POST_DELAY="$2"; shift ;;
    --num-clients) NUM_CLIENTS="$2"; shift ;;
    --read-write-ratio) READ_WRITE_RATIO="$2"; shift ;;
    -h|--help) usage ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

# URL will be constructed based on port
URL="http://localhost:${PORT}"

echo "Starting server and generator..."
echo "Server: port=${PORT}, get_delay=${GET_DELAY}ms, post_delay=${POST_DELAY}ms"
echo "Generator: url=${URL}, num_clients=${NUM_CLIENTS}, ratio=${READ_WRITE_RATIO}"

cargo run -- both \
  --port "$PORT" \
  --get-delay "$GET_DELAY" \
  --post-delay "$POST_DELAY" \
  --url "$URL" \
  --num-clients "$NUM_CLIENTS" \
  --read-write-ratio "$READ_WRITE_RATIO" 