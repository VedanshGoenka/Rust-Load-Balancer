#!/bin/bash

usage() {
  echo "Usage: $0 --url <url> --num-clients <num_clients> --read-write-ratio <read_write_ratio>"
  echo "  --url       The server URL to send requests to."
  echo "  --num-clients   The number of clients to simulate."
  echo "  --read-write_ratio     The read/write request ratio, in .% scale (e.g. 0.5, 1.0)"
  exit 1
}

# Default Args
URL="http://localhost:8000/"
NUM_CLIENTS=10
READ_WRITE_RATIO=0.5


# Parse Command Line Args
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --url) URL="$2"; shift ;;
    --clients) NUM_CLIENTS="$2"; shift ;;
    --ratio) READ_WRITE_RATIO="$2"; shift ;;
    *) echo "Unknown parameter: $1"; usage ;;
  esac
  shift
done

# Validate URL provided
if [[ -z "$URL" ]]; then
  echo "Error: --url is required"
  usage
fi

cargo run -- --url "$URL" --num-clients "$NUM_CLIENTS" --read-write-ratio "$READ_WRITE_RATIO"
