# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial repository architecture implementation
- Python package structure under `src/cortex/`
- Documentation hierarchy under `docs/`
- Test architecture with 9 categories
- Schema definitions for `.cx`, API, and configuration
- Configuration profiles (defaults, development, testing, production)
- Build, test, audit, and release scripts
- Deployment configurations (Docker, Kubernetes, systemd, reverse-proxy)
- Performance benchmarks
- Usage examples
- Migration artifacts
- GitHub CI/CD workflows

### Changed
- Migrated from Rust/Cargo to Python/pyproject.toml architecture
- Relocated documentation from root to `docs/`
- Restructured tests from flat to hierarchical categories
