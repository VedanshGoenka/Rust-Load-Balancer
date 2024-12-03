# Rust Load Balancer - CIS 1905 Final Project

A high-performance HTTP load balancer implementation in Rust, featuring multiple load balancing strategies and detailed metrics tracking.

## Features

### Load Balancing Strategies

- **Round Robin**: Simple rotation through servers with request distribution tracking
- **Least Connections**: Routes based on active connection count with success rate monitoring
- **Weighted Round Robin**: Supports server weights (random 1-10 if not specified) with distribution tracking
- **IP Hash**: Consistent hashing based on client IP for session affinity

### Metrics and Monitoring

- Real-time metrics for each algorithm:
  - Round Robin: Request counts and distribution percentages
  - Least Connections: Active connections, total requests, success rates
  - Weighted Round Robin: Server weights, request distribution
  - IP Hash: Request distribution and IP mappings
- Metrics accessible via HTTP endpoint (/metrics)
- Automatic metrics display on shutdown

### Performance Features

- Async I/O with Tokio
- Connection pooling
- Configurable connection limits
- Graceful shutdown handling

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/vedanshgoenka/rust-load-balancer.git
cd rust-load-balancer
```

2. Run the complete setup (load balancer, servers, and test load):

```bash
./start_all.sh
```

3. Try different algorithms:

```bash
./start_all.sh --algorithm round-robin
./start_all.sh --algorithm least-connections
./start_all.sh --algorithm weighted-round-robin
./start_all.sh --algorithm ip-hash
```

## Component Scripts

- `start_all.sh`: Launches complete setup with load testing
- `start_balancer.sh`: Starts only the load balancer
- `start_server.sh`: Launches backend servers
- `start_generator.sh`: Runs load testing
- `start_client_server.sh`: Starts both client and server components

## Configuration

### Load Balancer

- Port: Default 8000
- Algorithms: round-robin, least-connections, weighted-round-robin, ip-hash
- Connection limit: 500 concurrent connections

### Backend Servers

- Default ports: 8001-8020
- Configurable response delays for GET/POST
- Health check support

### Load Generator

- Configurable request count
- Adjustable concurrent clients
- GET/POST ratio control

## Metrics

Access metrics via:

1. HTTP endpoint: `curl http://localhost:8000/metrics`
2. Automatic display on Ctrl+C
3. Final metrics after test completion

Example metrics output:

```bash
127.0.0.1:8001: Active: 5, Total: 100, Success: 95, Rate: 95.0%
127.0.0.1:8002: Weight: 8, Requests: 150, Distribution: 30.0%
```

## Dependencies

```toml
[dependencies]
tokio = { version = "1.28", features = ["full"] }
clap = { version = "4.2", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
futures = "0.3"
rand = "0.8"
```

## Project Structure

```txt
rust_load_balancer/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs           # Library exports
│   ├── algorithms/      # Load balancing algorithms
│   ├── balancer/       # Load balancer core
│   ├── server/         # Backend server
│   ├── client/         # Client implementation
│   └── generator/      # Load generator
├── scripts/
│   ├── start_all.sh
│   ├── start_balancer.sh
│   ├── start_server.sh
│   └── start_generator.sh
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Authors

- Richard Xue <xrichard@seas.upenn.edu>
- Vedansh Goenka <vedanshg@seas.upenn.edu>
