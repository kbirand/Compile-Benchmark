# Dependencies

This benchmark includes **120+ real-world Rust crates** across various categories to simulate realistic compile workloads.

## Web Frameworks
| Crate | Version | Description |
|-------|---------|-------------|
| axum | 0.7 | Modular web framework |
| actix-web | 4 | High-performance web framework |
| rocket | 0.5 | Web framework with focus on ease of use |
| warp | 0.3 | Composable web framework |
| hyper | 1 | Low-level HTTP library |
| poem | 2 | Full-featured web framework |
| salvo | 0.65 | Web framework with JWT and compression |
| tide | 0.16 | Async web framework |
| viz | 0.8 | Fast web framework |
| gotham | 0.7 | Flexible web framework |
| thruster | 1 | Fast HTTP framework |

## Async Runtimes & Utilities
| Crate | Version | Description |
|-------|---------|-------------|
| tokio | 1 | Async runtime (full features) |
| async-std | 1 | Alternative async runtime |
| smol | 2 | Small async runtime |
| futures | 0.3 | Async primitives |
| async-trait | 0.1 | Async trait support |
| pollster | 0.3 | Minimal async executor |
| blocking | 1 | Blocking task pool |
| tokio-stream | 0.1 | Stream utilities |
| tokio-util | 0.7 | Additional utilities |
| futures-lite | 2 | Lightweight futures |
| async-stream | 0.3 | Stream macros |

## Serialization
| Crate | Version | Description |
|-------|---------|-------------|
| serde | 1 | Serialization framework |
| serde_json | 1 | JSON support |
| serde_yaml | 0.9 | YAML support |
| toml | 0.8 | TOML support |
| ron | 0.8 | Rusty Object Notation |
| rmp-serde | 1 | MessagePack |
| bincode | 1 | Binary encoding |
| postcard | 1 | Embedded-friendly format |
| ciborium | 0.2 | CBOR support |
| speedy | 0.8 | Fast serialization |
| rkyv | 0.7 | Zero-copy deserialization |
| borsh | 1 | Binary serialization |
| flexbuffers | 2 | FlexBuffers support |
| serde_with | 3 | Serde helpers |

## Database & ORM
| Crate | Version | Description |
|-------|---------|-------------|
| sqlx | 0.7 | Async SQL toolkit |
| diesel | 2 | ORM and query builder |
| sea-orm | 0.12 | Async ORM |
| redis | 0.25 | Redis client |

## Cryptography
| Crate | Version | Description |
|-------|---------|-------------|
| sha2 | 0.10 | SHA-2 hashes |
| sha3 | 0.10 | SHA-3 hashes |
| blake2 | 0.10 | BLAKE2 hashes |
| blake3 | 1 | BLAKE3 hash |
| argon2 | 0.5 | Password hashing |
| ed25519-dalek | 2 | Ed25519 signatures |
| x25519-dalek | 2 | X25519 key exchange |
| chacha20poly1305 | 0.10 | ChaCha20-Poly1305 AEAD |
| aes-gcm | 0.10 | AES-GCM AEAD |
| k256 | 0.13 | secp256k1 curve |
| p256 | 0.13 | P-256 curve |
| p384 | 0.13 | P-384 curve |
| rsa | 0.9 | RSA encryption |
| dsa | 0.6 | DSA signatures |
| ring | 0.17 | Crypto primitives |
| rustls | 0.22 | TLS implementation |
| jsonwebtoken | 9 | JWT tokens |

## Blockchain
| Crate | Version | Description |
|-------|---------|-------------|
| ethers | 2 | Ethereum library |
| web3 | 0.19 | Web3 implementation |

## Math & Scientific
| Crate | Version | Description |
|-------|---------|-------------|
| nalgebra | 0.32 | Linear algebra |
| ndarray | 0.15 | N-dimensional arrays |
| num | 0.4 | Numeric traits |
| rust_decimal | 1 | Decimal arithmetic |
| bigdecimal | 0.4 | Arbitrary precision |
| statrs | 0.16 | Statistics |
| peroxide | 0.34 | Scientific computing |
| rulinalg | 0.4 | Linear algebra |
| mathru | 0.13 | Math utilities |

## Parsing
| Crate | Version | Description |
|-------|---------|-------------|
| syn | 2 | Rust syntax parsing |
| proc-macro2 | 1 | Proc-macro utilities |
| quote | 1 | Code generation |
| nom | 7 | Parser combinators |
| pest | 2 | PEG parser |
| logos | 0.14 | Lexer generator |
| chumsky | 0.9 | Parser combinators |
| lalrpop-util | 0.20 | LALR parser |
| peg | 0.8 | PEG parser |
| winnow | 0.6 | Streaming parser |
| tree-sitter | 0.22 | Incremental parsing |

## ECS / Game Development
| Crate | Version | Description |
|-------|---------|-------------|
| bevy_ecs | 0.13 | Bevy's ECS |
| specs | 0.20 | Parallel ECS |
| legion | 0.4 | High-performance ECS |
| hecs | 0.10 | Minimal ECS |

## Template Engines
| Crate | Version | Description |
|-------|---------|-------------|
| tera | 1 | Jinja2-like templates |
| handlebars | 5 | Handlebars templates |
| askama | 0.12 | Type-safe templates |
| minijinja | 1 | Minimal Jinja2 |
| liquid | 0.26 | Liquid templates |
| upon | 0.8 | Simple templates |
| markup | 0.15 | HTML macro |

## HTTP Clients
| Crate | Version | Description |
|-------|---------|-------------|
| reqwest | 0.11 | HTTP client |
| ureq | 2 | Blocking HTTP client |
| attohttpc | 0.28 | Lightweight client |
| isahc | 1 | Async HTTP client |
| surf | 2 | Async HTTP client |

## GraphQL
| Crate | Version | Description |
|-------|---------|-------------|
| async-graphql | 7 | GraphQL server |
| async-graphql-axum | 7 | Axum integration |
| juniper | 0.16 | GraphQL library |

## gRPC
| Crate | Version | Description |
|-------|---------|-------------|
| tonic | 0.11 | gRPC framework |
| prost | 0.12 | Protocol Buffers |
| tarpc | 0.34 | RPC framework |
| capnp | 0.19 | Cap'n Proto |
| capnp-rpc | 0.19 | Cap'n Proto RPC |

## Compression
| Crate | Version | Description |
|-------|---------|-------------|
| flate2 | 1 | DEFLATE/gzip |
| zstd | 0.13 | Zstandard |
| brotli | 6 | Brotli |
| lzma-rs | 0.3 | LZMA |
| lz4_flex | 0.11 | LZ4 (pure Rust) |
| snap | 1 | Snappy |
| weezl | 0.1 | LZW |
| miniz_oxide | 0.7 | DEFLATE |

## Image & Graphics
| Crate | Version | Description |
|-------|---------|-------------|
| image | 0.24 | Image processing |
| tiny-skia | 0.11 | 2D rendering |
| resvg | 0.40 | SVG rendering |
| usvg | 0.40 | SVG parsing |
| fontdb | 0.16 | Font database |

## Audio
| Crate | Version | Description |
|-------|---------|-------------|
| rodio | 0.18 | Audio playback |
| symphonia | 0.5 | Audio decoding |

## Document Processing
| Crate | Version | Description |
|-------|---------|-------------|
| printpdf | 0.7 | PDF generation |
| lopdf | 0.32 | PDF manipulation |
| calamine | 0.24 | Excel reading |
| rust_xlsxwriter | 0.64 | Excel writing |
| csv | 1 | CSV processing |
| pulldown-cmark | 0.10 | Markdown parsing |
| comrak | 0.21 | CommonMark |

## CLI & TUI
| Crate | Version | Description |
|-------|---------|-------------|
| clap | 4 | Argument parsing |
| indicatif | 0.17 | Progress bars |
| dialoguer | 0.11 | User prompts |
| console | 0.15 | Terminal utilities |
| colored | 2 | Colored output |

## Observability
| Crate | Version | Description |
|-------|---------|-------------|
| tracing | 0.1 | Application tracing |
| tracing-subscriber | 0.3 | Tracing output |
| prometheus | 0.13 | Metrics |
| metrics | 0.22 | Metrics facade |
| opentelemetry | 0.22 | Telemetry |
| sentry | 0.32 | Error tracking |
| fern | 0.6 | Logging |
| log4rs | 1 | Logging framework |
| env_logger | 0.11 | Environment logger |

## Concurrency
| Crate | Version | Description |
|-------|---------|-------------|
| rayon | 1 | Data parallelism |
| crossbeam | 0.8 | Concurrent primitives |
| dashmap | 5 | Concurrent HashMap |
| flume | 0.11 | Channel library |
| kanal | 0.1 | Fast channels |

## Derive Macros
| Crate | Version | Description |
|-------|---------|-------------|
| derive_more | 0.99 | Additional derives |
| derive_builder | 0.20 | Builder pattern |
| typed-builder | 0.18 | Type-safe builder |
| getset | 0.1 | Getters/setters |
| smart-default | 0.7 | Default values |
| educe | 0.5 | Derive utilities |
| shrinkwraprs | 0.3 | Newtype derives |
| darling | 0.20 | Derive helpers |
| strum | 0.26 | Enum utilities |
| enum-iterator | 1 | Enum iteration |
| num_enum | 0.7 | Enum conversions |
| enum-map | 2 | Enum maps |
| enumflags2 | 0.7 | Enum flags |
| enum-assoc | 1 | Enum associations |
| bitfield | 0.17 | Bitfield macros |

## Validation
| Crate | Version | Description |
|-------|---------|-------------|
| validator | 0.16 | Validation derives |
| garde | 0.18 | Validation framework |
| nutype | 0.4 | Newtype validation |
| schemars | 0.8 | JSON Schema |

## Error Handling
| Crate | Version | Description |
|-------|---------|-------------|
| thiserror | 1 | Error derives |
| anyhow | 1 | Error handling |
| color-eyre | 0.6 | Colorful errors |
| miette | 7 | Diagnostic errors |
| eyre | 0.6 | Error reporting |

## Messaging & Protocols
| Crate | Version | Description |
|-------|---------|-------------|
| rumqttc | 0.24 | MQTT client |
| lapin | 2 | AMQP client |
| trust-dns-resolver | 0.23 | DNS resolver |
| hickory-resolver | 0.24 | DNS resolver |

## Bot Frameworks
| Crate | Version | Description |
|-------|---------|-------------|
| serenity | 0.12 | Discord library |
| octocrab | 0.34 | GitHub API |

## Email
| Crate | Version | Description |
|-------|---------|-------------|
| lettre | 0.11 | Email sending |

## Testing
| Crate | Version | Description |
|-------|---------|-------------|
| proptest | 1 | Property testing |
| quickcheck | 1 | QuickCheck |
| arbitrary | 1 | Arbitrary data |

## Other Utilities
| Crate | Version | Description |
|-------|---------|-------------|
| chrono | 0.4 | Date/time |
| chrono-tz | 0.9 | Timezones |
| uuid | 1 | UUID generation |
| rand | 0.8 | Random generation |
| regex | 1 | Regular expressions |
| url | 2 | URL parsing |
| base64 | 0.21 | Base64 encoding |
| moka | 0.12 | Caching |
| governor | 0.6 | Rate limiting |
| config | 0.14 | Configuration |
| cron | 0.12 | Cron expressions |
| job_scheduler | 1 | Job scheduling |
| quick-xml | 0.31 | XML parsing |
| xml-rs | 0.8 | XML processing |
| toml_edit | 0.22 | TOML editing |
| syntect | 5 | Syntax highlighting |

---

All dependencies are **pure Rust** with no native library requirements, ensuring cross-platform compatibility on Windows, macOS, and Linux.
