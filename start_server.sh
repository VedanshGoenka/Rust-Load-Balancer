#!/bin/bash

# Start multiple servers with different configurations
start_server() {
    local port=$1
    local get_delay=$2
    local post_delay=$3
    echo "Starting server on port $port (GET delay: ${get_delay}ms, POST delay: ${post_delay}ms)"
    cargo run -- server -p "$port" -g "$get_delay" -o "$post_delay" &
}

# Default configurations for different servers
# Format: "port get_delay post_delay"
declare -a servers=(
    "8001 100 200"    # Fast server
    "8002 200 400"    # Medium server
    "8003 300 600"    # Slow server
)

# Start all servers
for server in "${servers[@]}"; do
    read -r port get_delay post_delay <<< "$server"
    start_server "$port" "$get_delay" "$post_delay"
done

echo "All servers started. Press Enter to stop all servers..."
read -r

# Kill all background server processes
pkill -P $$