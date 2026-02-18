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
- And 120+ more real-world dependencies...

See **[DEPENDENCIES.md](DEPENDENCIES.md)** for the complete list of all crates organized by category.

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

**Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy Bypass -File .\benchmark.ps1
```

**macOS/Linux:**
```bash
chmod +x benchmark.sh
./benchmark.sh
```

The scripts will:
- Check/install Rust if needed
- Run clean debug and release builds
- Measure and display compile times
- Save results to `benchmark-results.txt` and `benchmark-results.json`

## Expected Compile Times

| System | Debug Build | Release Build |
|--------|-------------|---------------|
| Modern Desktop (8+ cores) | 1-2 min | 2-4 min |
| Laptop (4 cores) | 2-3 min | 4-6 min |
| Older Hardware | 3-5 min | 6-10 min |

*Times vary based on CPU, RAM, and disk speed.*

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

## Result Reporting

After the benchmark completes, results are automatically sent as a **JSON POST** request to the configured endpoint. Both scripts (`benchmark.sh` and `benchmark.ps1`) send the same payload format.

To change the endpoint, edit the URL in `benchmark-config.cfg` — both scripts read from this single file.

### Payload

```json
{
  "timestamp": "2026-02-16T16:00:00Z",
  "system": {
    "device": "Mac Studio",
    "os": "macOS 15.3",
    "cpu": "Apple M2 Pro",
    "gpu": "Apple M2 Pro",
    "cores": 12,
    "ram_gb": 32
  },
  "rust": {
    "rustc": "rustc 1.84.1 (e71f9a9a9 2025-01-27)",
    "cargo": "cargo 1.84.1 (66221abde 2025-01-22)"
  },
  "results": {
    "debug": {
      "time_seconds": 58.123,
      "avg_power": "12.5W",
      "energy": "0.20Wh"
    },
    "release": {
      "time_seconds": 112.456,
      "avg_power": "15.3W",
      "energy": "0.48Wh"
    }
  },
  "power_monitoring_enabled": true
}
```

> **Note:** `avg_power` and `energy` will be `"N/A"` if power monitoring is not available or the script is not run with `sudo`.

## License

MIT License with Attribution Requirement

Free to use for benchmarking purposes. **If sharing results on social media, blogs, or publications, please credit:**

```
Benchmark: github.com/kbirand/Compile-Benchmark
```
