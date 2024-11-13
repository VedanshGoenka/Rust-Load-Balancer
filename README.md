# Rust Load Balancer

A high-performance HTTP load balancer implementation in Rust, featuring multiple balancing strategies and health checking capabilities.

## Features

- Multiple load balancing strategies:
  - Round Robin
  - Least Connections
  - Weighted Round Robin
- Health checking for backend servers
- Configuration via JSON/YAML
- Async I/O with Tokio
- Comprehensive logging and metrics

## Prerequisites

- Rust 1.70.0 or higher (install via [rustup](https://rustup.rs/))
- Cargo (included with rustup)
- Git

## Installation

1. Install Rust and Cargo:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rust_load_balancer.git
   cd rust_load_balancer
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Configuration

Create a configuration file `config.json`:

```json
{
  "listen_address": "127.0.0.1:8080",
  "backend_servers": [
    "127.0.0.1:8081",
    "127.0.0.1:8082"
  ],
  "health_check_interval": 30,
  "health_check_timeout": 5
}
```

## Usage

### Running the Load Balancer

Basic usage:

```bash
cargo run --release
```

With custom configuration:

```bash
cargo run --release -- --config path/to/config.json
```

### Development Commands

Run tests:

```bash
cargo test                 # Run unit tests
cargo test --test '*'     # Run integration tests
```

Generate documentation:

```bash
cargo doc --open
```

Run benchmarks:

```bash
cargo bench
```

Check code formatting:

```bash
cargo fmt -- --check
```

Run linter:

```bash
cargo clippy
```

## Crate Dependencies

### Main Dependencies

- **tokio** (1.28) - Asynchronous runtime
  - Features: full (TCP, time, IO, etc.)
  - Used for: Async network operations

- **serde** (1.0) - Serialization framework
  - Features: derive
  - Used for: Config serialization/deserialization

- **serde_json** (1.0) - JSON support
  - Used for: Configuration file parsing

- **thiserror** (1.0) - Error handling
  - Used for: Custom error types

- **tracing** (0.1) - Logging infrastructure
  - Used for: Application logging

- **tracing-subscriber** (0.3) - Logging implementation
  - Used for: Log formatting and output

- **config** (0.13) - Configuration management
  - Used for: Loading and parsing config files

- **clap** (4.2) - Command line parsing
  - Features: derive
  - Used for: CLI argument handling

### Development Dependencies

```toml
[dev-dependencies]
criterion = "0.5"     # For benchmarking
mockall = "0.11"      # For mocking in tests
```

## Project Structure

```txt
rust_load_balancer/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library entry point
│   ├── balancer/         # Load balancing logic
│   │   └── mod.rs
│   ├── config/           # Configuration management
│   │   └── mod.rs
│   ├── health/           # Health checking
│   │   └── mod.rs
│   └── server/           # Backend server management
│       └── mod.rs
├── tests/                # Integration tests
├── examples/             # Usage examples
├── benches/              # Performance benchmarks
└── docs/                # Additional documentation
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Ensure tests pass (`cargo test`)
4. Ensure code is formatted (`cargo fmt`)
5. Ensure clippy is happy (`cargo clippy`)
6. Commit your changes (`git commit -m 'Add some amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

### Common Issues

1. **Compilation Errors**

   ```bash
   cargo clean
   cargo build
   ```

2. **Permission Issues**

   ```bash
   chmod +x target/release/rust_load_balancer
   ```

3. **Port Already in Use**

   ```bash
   lsof -i :8080
   kill -9 <PID>
   ```

## Performance Tuning

For optimal performance:

1. Always use release builds:

   ```bash
   cargo run --release
   ```

2. Enable logging levels appropriately:

   ```bash
   RUST_LOG=info cargo run --release
   ```

## Support

For support, please:

1. Check existing issues
2. Create a new issue with:
   - Rust version (`rustc --version`)
   - OS details
   - Minimal reproducible example
