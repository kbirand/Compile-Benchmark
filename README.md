# Compile Benchmark

A realistic Rust web service application designed for compile-time benchmarking across different operating systems and hardware.

## Purpose

This project simulates a real-world web application with common dependencies to provide a meaningful compile-time benchmark. It includes:

- **Web Framework** (Axum) - HTTP server with routing
- **Database** (SQLx) - Async database with compile-time checked queries
- **Serialization** (Serde) - JSON/YAML/TOML parsing with derive macros
- **Authentication** (JWT, Argon2) - Token-based auth with password hashing
- **HTTP Client** (Reqwest) - External API calls
- **Caching** (Moka) - In-memory async cache
- **CLI** (Clap) - Command-line argument parsing with derive macros
- **Templating** (Tera) - HTML template rendering
- **Validation** (Validator) - Input validation with derive macros
- And many more common production dependencies...

## Prerequisites

### Install Rust

**All Platforms:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows (alternative):**
Download and run [rustup-init.exe](https://rustup.rs/)

After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

## Running the Benchmark

### Clean Build (Full Benchmark)

This measures the complete compile time from scratch:

**Windows (PowerShell):**
```powershell
cd compile-benchmark
cargo clean
Measure-Command { cargo build --release 2>&1 | Out-Default }
```

**macOS/Linux:**
```bash
cd compile-benchmark
cargo clean
time cargo build --release
```

### Quick Build (Debug Mode)

For faster iteration during testing:

```bash
cargo clean
time cargo build
```

## Expected Compile Times

| System | Debug Build | Release Build |
|--------|-------------|---------------|
| Modern Desktop (8+ cores) | 1-2 min | 2-4 min |
| Laptop (4 cores) | 2-3 min | 4-6 min |
| Older Hardware | 3-5 min | 6-10 min |

*Times vary based on CPU, RAM, and disk speed.*

## Benchmark Script

For consistent benchmarking, use the provided scripts:

**Windows (benchmark.ps1):**
```powershell
.\benchmark.ps1
```

**macOS/Linux (benchmark.sh):**
```bash
chmod +x benchmark.sh
./benchmark.sh
```

## What's Being Compiled

The project structure simulates a real e-commerce/content platform:

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── database.rs       # Database connection & migrations
├── auth.rs           # Authentication (JWT, password hashing)
├── cache.rs          # Caching layer
├── error.rs          # Error types and handling
├── middleware.rs     # HTTP middleware
├── api.rs            # API client
├── templates.rs      # Template engine
├── utils.rs          # Utility functions
├── models/           # Data models
│   ├── user.rs
│   ├── post.rs
│   ├── product.rs
│   ├── order.rs
│   └── analytics.rs
├── handlers/         # HTTP request handlers
│   ├── auth.rs
│   ├── users.rs
│   ├── posts.rs
│   ├── products.rs
│   ├── orders.rs
│   └── ...
└── services/         # Business logic
    ├── user_service.rs
    ├── post_service.rs
    ├── order_service.rs
    ├── email_service.rs
    ├── payment_service.rs
    └── ...
```

## Adjusting Compile Time

If you need longer/shorter compile times:

### Increase Compile Time
- Add more dependencies in `Cargo.toml`
- Enable more feature flags
- Add more modules with derive macros

### Decrease Compile Time
- Remove some dependencies
- Use `cargo build` instead of `cargo build --release`
- Reduce feature flags

## Tips for Accurate Benchmarking

1. **Close other applications** - Especially browsers and IDEs
2. **Disable antivirus scanning** for the project directory (temporarily)
3. **Run multiple times** - Take the average of 3+ runs
4. **Note system specs** - CPU, RAM, SSD vs HDD
5. **Check CPU temperature** - Throttling affects results
6. **Use the same Rust version** across systems:
   ```bash
   rustup default stable
   rustup update
   ```

## Verifying the Build

After compilation, verify it works:

```bash
./target/release/compile-benchmark --help
```

Or run it (it will start a web server):

```bash
./target/release/compile-benchmark --port 8080
```

## License

MIT - Use freely for benchmarking purposes.
