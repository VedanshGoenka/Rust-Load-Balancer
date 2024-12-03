#!/bin/bash

usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -u, --url <url>              Load balancer URL (default: http://localhost:8000)"
    echo "  -n, --num-requests <num>      Number of requests (default: 100)"
    echo "  -c, --concurrent <num>        Number of concurrent clients (default: 5)"
    echo "  -r, --ratio <ratio>           GET/POST ratio (default: 0.7)"
    exit 1
}

# Default values
URL="http://localhost:8000"
NUM_REQUESTS=100
CONCURRENT_CLIENTS=5
GET_RATIO=0.7

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -u|--url) URL="$2"; shift ;;
        -n|--num-requests) NUM_REQUESTS="$2"; shift ;;
        -c|--concurrent) CONCURRENT_CLIENTS="$2"; shift ;;
        -r|--ratio) GET_RATIO="$2"; shift ;;
        -h|--help) usage ;;
        *) echo "Unknown parameter: $1"; usage ;;
    esac
    shift
done

echo "Starting load generator..."
echo "URL: $URL"
echo "Total requests: $NUM_REQUESTS"
echo "Concurrent clients: $CONCURRENT_CLIENTS"
echo "GET ratio: $GET_RATIO"

cargo run -- generator \
    -u "$URL" \
    -n "$NUM_REQUESTS" \
    -c "$CONCURRENT_CLIENTS" \
    -r "$GET_RATIO"
