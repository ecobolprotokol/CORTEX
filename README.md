# CORTEX

A persistent, state-based, continually learning AI model.

## Quickstart

```bash
# Build
cargo build --release

# Run
./target/release/cortex run

# Observe
./target/release/cortex observe "input text"

# Query
./target/release/cortex query "question"

# Status
./target/release/cortex status
```

## Documentation

- [Technical Specification](docs/DOC-01-Requirements.md)
- [Software Design](docs/DOC-02-Architecture.md)
- [Data & State](docs/DOC-03-Data-Architecture.md)
- [Algorithms](docs/DOC-04-Algorithms.md)
- [API & CLI](docs/DOC-05-API-CLI.md)
- [Build & Release](docs/DOC-06-Build-Release.md)
- [Testing & Validation](docs/DOC-07-Testing-Validation.md)
- [Deployment & Operations](docs/DOC-08-Deployment-Operations.md)
- [Security & Privacy](docs/DOC-09-Security-Privacy.md)
- [Configuration Reference](docs/DOC-10-Configuration-Reference.md)
- [Repository Architecture](docs/DOC-11-Repository-Architecture.md)

## Development

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --check

# Bench
cargo bench
```

## License

Proprietary. All rights reserved.
