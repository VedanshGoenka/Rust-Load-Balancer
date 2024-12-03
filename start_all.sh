#!/bin/bash

# Load balancer port
LB_PORT=8000
# Number of servers
NUM_SERVERS=20
# GET request delay
GET_DELAY=200
# POST request delay
POST_DELAY=100

# Test Loads
# Number of requests
NUM_REQUESTS=5000
# Number of concurrent clients
CONCURRENT_CLIENTS=50
# GET/POST ratio
GET_RATIO=0.7

# Function to check if a port is in use
check_port() {
    lsof -i ":$1" >/dev/null 2>&1
    return $?
}

# Function to cleanup all processes
cleanup() {
    echo "Cleaning up processes..."
    pkill -f "rust_load_balancer"
    sleep 1
    
    # Force kill any remaining processes
    if pgrep -f "rust_load_balancer" > /dev/null; then
        pkill -9 -f "rust_load_balancer"
    fi
    
    # Wait for ports to be free
    for ((i=0; i<=NUM_SERVERS; i++)); do
        PORT=$((LB_PORT + i))
        while check_port $PORT; do
            echo "Waiting for port $PORT to be free..."
            sleep 1
        done
    done
    
    echo "Cleanup complete."
}

# Clean up any existing processes before starting
cleanup

# Set up trap to cleanup on script exit
trap cleanup EXIT INT TERM

# Build the binary first
echo "Building project..."
cargo build || exit 1
BINARY="./target/debug/rust_load_balancer"

# Start servers
echo "Starting servers..."
for ((i=1; i<=NUM_SERVERS; i++)); do
    PORT=$((LB_PORT + i))
    if check_port $PORT; then
        echo "Error: Port $PORT is already in use"
        exit 1
    fi
    echo "Starting server on port $PORT..."
    $BINARY server -p $PORT -g $GET_DELAY -o $POST_DELAY &
done

# Wait a moment for servers to start
sleep 1

# Start load balancer
echo "Starting load balancer..."
if check_port $LB_PORT; then
    echo "Error: Load balancer port $LB_PORT is already in use"
    exit 1
fi

SERVER_LIST=""
for ((i=1; i<=NUM_SERVERS; i++)); do
    if [ -n "$SERVER_LIST" ]; then
        SERVER_LIST="$SERVER_LIST,"
    fi
    SERVER_LIST="${SERVER_LIST}127.0.0.1:$((LB_PORT + i))"
done

$BINARY balancer -p $LB_PORT -s "$SERVER_LIST" &

# Wait for load balancer to start
echo "Waiting for load balancer to initialize..."
sleep 1

# Verify all processes are running
if ! pgrep -f "rust_load_balancer server" > /dev/null; then
    echo "Error: Servers failed to start"
    exit 1
fi
if ! pgrep -f "rust_load_balancer balancer" > /dev/null; then
    echo "Error: Load balancer failed to start"
    exit 1
fi

# Run a test load
echo "Running test load..."
$BINARY generator \
    -u "http://localhost:$LB_PORT" \
    -n $NUM_REQUESTS \
    -c $CONCURRENT_CLIENTS \
    -r $GET_RATIO

echo "Test complete." 