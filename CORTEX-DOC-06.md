# CORTEX — 06 Build & Release Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-06 |
| **Title** | Build & Release Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Build Contract |
| **Scope** | Build pipeline, CI, release process, binary artifacts, versioning |
| **Parent Document** | CORTEX-DOC-01 Technical Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Replace SHA-256 with BLAKE3 for all checksum operations |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Build Engineer | _____________ | _____________ |

### Document Purpose

This document defines **how CORTEX is built, tested, and released**. It constitutes the build contract: toolchain requirements, build pipeline stages, CI gates, release artifact specification, and versioning policy.

---

## 1. Toolchain Requirements

### 1.1 Required Toolchain

| Tool | Version | Purpose | Required |
|---|---|---|---|
| Rust stable | As specified in `rust-toolchain.toml` | Compilation | YES |
| cargo | Bundled with Rust | Build system | YES |
| rustfmt | Bundled with Rust | Code formatting | YES (CI) |
| clippy | Bundled with Rust | Linting | YES (CI) |
| cargo-audit | Latest | Dependency vulnerability audit | YES (CI) |
| cargo-deny | Latest | License and advisory checking | YES (CI) |

### 1.2 Toolchain Pinning

```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu"]
```

### 1.3 Minimum Supported Rust Version (MSRV)

| Property | Value |
|---|---|
| MSRV | Defined in `rust-toolchain.toml` |
| Edition | 2021+ |
| Verification | CI builds with MSRV toolchain |

---

## 2. Build Pipeline

### 2.1 Pipeline Stages

```
Source Code (src/)
    │
    ↓
┌─────────────────────────────────────────┐
│ STAGE 1: FORMAT CHECK                    │
│   cargo fmt --check                      │
│   Gate: PASS/FAIL (no warnings)          │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 2: CLIPPY LINT                     │
│   cargo clippy -- -D warnings            │
│   Gate: PASS/FAIL (no warnings)          │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 3: UNIT TESTS                      │
│   cargo test --lib                       │
│   Gate: ALL PASS                         │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 4: INTEGRATION TESTS               │
│   cargo test --test '*'                  │
│   Gate: ALL PASS                         │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 5: SECURITY AUDIT                  │
│   cargo audit                            │
│   cargo deny check                       │
│   Gate: NO CRITICAL/HIGH VULNERABILITIES │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 6: RELEASE BUILD                   │
│   cargo build --release                  │
│   Gate: SUCCESS, binary artifact produced│
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 7: BINARY VALIDATION               │
│   ./cortex --version                     │
│   ./cortex init --force                  │
│   ./cortex status                        │
│   Gate: Binary operational               │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│ STAGE 8: ARTIFACT PACKAGING              │
│   Package binary + config template       │
│   Compute BLAKE3 checksum              │
│   Gate: Artifact integrity verified      │
└─────────────────────────────────────────┘
```

### 2.2 Build Configuration

```toml
# Cargo.toml [profile.release]
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### 2.3 Build Output

| Artifact | Path | Description |
|---|---|---|
| Binary | `target/release/cortex` | Optimized release binary |
| Binary (debug) | `target/debug/cortex` | Debug binary (testing) |
| Checksum | `target/release/cortex.blake3` | BLAKE3 checksum file |
| License report | `target/release/licenses.html` | Dependency licenses |

---

## 3. Continuous Integration

### 3.1 CI Trigger Conditions

| Trigger | Action |
|---|---|
| Push to main | Full pipeline (stages 1-8) |
| Pull request | Stages 1-5 (lint, test, audit) |
| Tag push (v*) | Full pipeline + release artifact |
| Manual dispatch | Configurable stages |

### 3.2 CI Gates

| Gate | Condition | Action on Failure |
|---|---|---|
| Format | `cargo fmt --check` returns 0 | Block merge |
| Lint | `cargo clippy` returns 0 | Block merge |
| Unit tests | All pass | Block merge |
| Integration tests | All pass | Block merge |
| Security audit | No critical/high CVEs | Block merge |
| Release build | Binary produced | Block release |
| Binary validation | Binary starts and responds | Block release |

### 3.3 CI Environment

| Property | Value |
|---|---|
| OS | Linux (Ubuntu latest) |
| Architecture | x86_64 |
| Rust toolchain | As specified in `rust-toolchain.toml` |
| Cache | `~/.cargo/registry`, `~/.cargo/git`, `target/` |
| Timeout | 30 minutes per stage |

---

## 4. Release Process

### 4.1 Release Triggers

| Trigger | Action |
|---|---|
| Git tag `v*` | Automated release pipeline |
| Manual workflow dispatch | Configurable release |

### 4.2 Release Steps

```
1. Update version in Cargo.toml
2. Update version in documentation headers
3. Commit: "Release vX.Y.Z"
4. Create git tag: vX.Y.Z
5. Push tag: git push origin vX.Y.Z
6. CI pipeline triggers:
   a. Full build pipeline (stages 1-8)
   b. Package release artifact
   c. Compute checksums
   d. Create GitHub release with:
      - Binary artifact
      - Checksum file
      - Release notes (auto-generated from commits)
      - Changelog
```

### 4.3 Release Artifact Package

```
cortex-v{VERSION}-x86_64-unknown-linux-gnu/
├── cortex                    # Binary
├── cortex.toml               # Default configuration template
├── README.md                 # Quick start guide
├── LICENSE                   # License file
├── CHANGELOG.md              # Version changelog
└── cortex.blake3            # BLAKE3 checksum of binary
```

### 4.4 Release Naming Convention

| Artifact | Pattern |
|---|---|
| Git tag | `v{MAJOR}.{MINOR}.{PATCH}` (e.g., `v1.0.0`) |
| Binary | `cortex` (no version in filename) |
| Package | `cortex-v{VERSION}-{TARGET}.tar.gz` |
| Checksum | `cortex-v{VERSION}-{TARGET}.blake3` |

---

## 5. Versioning Policy

### 5.1 Semantic Versioning

CORTEX follows Semantic Versioning 2.0.0:

```
MAJOR.MINOR.PATCH

MAJOR: Incompatible API changes, .cx format changes, or algorithm changes
       that break backward compatibility.
MINOR: New functionality added in a backward-compatible manner.
PATCH: Backward-compatible bug fixes.
```

### 5.2 Version Component Meaning

| Component | Increments When |
|---|---|
| MAJOR | API endpoint removed or renamed; `.cx` format incompatible; CLI command removed; Algorithm change requiring state migration from previous MAJOR |
| MINOR | New API endpoint added; New CLI command added; New configuration option; New algorithm variant |
| PATCH | Bug fix; Performance improvement; Documentation update; Dependency update |

### 5.3 Version Recording

| Location | Version Recorded |
|---|---|
| `Cargo.toml` | `version = "X.Y.Z"` |
| `.cx` header | `architecture_version`, `format_version`, `algorithm_versions` |
| API responses | `metadata.version` |
| CLI output | `cortex --version` |
| Documentation headers | Document version field |
| Git tag | `vX.Y.Z` |

### 5.4 Backward Compatibility Rules

| Rule | Description |
|---|---|
| BV-001 | PATCH versions are always backward compatible |
| BV-002 | MINOR versions add features without breaking existing interfaces |
| BV-003 | MAJOR versions may break any interface |
| BV-004 | `.cx` format changes within MAJOR use migration |
| BV-005 | `.cx` format changes across MAJOR require explicit migration tool |
| BV-006 | API deprecated endpoints return `Warning` header for at least one MINOR version |
| BV-007 | CLI removed commands produce helpful error message for one MAJOR version |

---

## 6. Dependency Management

### 6.1 Dependency Policy

| Category | Policy |
|---|---|
| New dependency | Requires review and approval |
| Version update (minor/patch) | Automated with CI verification |
| Version update (major) | Manual review required |
| Security advisory | Immediate update required |
| License change | Requires review and approval |

### 6.2 Dependency Audit

| Audit | Tool | Frequency |
|---|---|---|
| Vulnerability scan | `cargo audit` | Every CI run |
| License compliance | `cargo deny check licenses` | Every CI run |
| Advisory check | `cargo deny check advisories` | Every CI run |
| Outdated check | `cargo outdated` | Weekly |

### 6.3 Dependency Lock

| Rule | Description |
|---|---|
| DEP-001 | `Cargo.lock` is committed to repository |
| DEP-002 | `Cargo.lock` is regenerated only for intentional updates |
| DEP-003 | Dependencies are pinned to compatible versions (not exact) |
| DEP-004 | Dev-dependencies are excluded from release builds |

---

## 7. Build Reproducibility

### 7.1 Reproducibility Requirements

| Property | Requirement |
|---|---|
| Toolchain | Pinned in `rust-toolchain.toml` |
| Dependencies | Locked in `Cargo.lock` |
| Build profile | Defined in `Cargo.toml` |
| Environment | CI environment is deterministic |
| Timestamps | Not embedded in binary (strip = true) |
| Randomness | Not used in build process |

### 7.2 Reproducible Build Verification

```
1. Clone repository at exact commit
2. Use pinned toolchain
3. Run: cargo build --release
4. Compare binary checksum with release artifact
5. Checksums MUST match
```

---

## 8. Binary Specifications

### 8.1 Binary Requirements

| Property | Requirement |
|---|---|
| Type | Statically linked (preferred) or dynamically linked with system libs only |
| Target | `x86_64-unknown-linux-gnu` |
| Size | Implementation-defined; tracked per release |
| Stripped | YES (debug symbols removed) |
| LTO | YES (link-time optimization) |
| Panic | `abort` (no unwinding) |

### 8.2 Binary Validation

| Check | Method | Expected |
|---|---|---|
| Version output | `./cortex --version` | Matches release version |
| Help output | `./cortex --help` | Lists all commands |
| Init | `./cortex init --force` | Creates `cortex.cx` |
| Status | `./cortex status` | Reports "ready" |
| Checksum | BLAKE3 of binary | Matches published checksum |

### 8.3 Binary Compatibility

| Platform | Support |
|---|---|
| Linux x86_64 | Primary target |
| Linux aarch64 | Future consideration |
| macOS x86_64 | Future consideration |
| macOS aarch64 | Future consideration |
| Windows x86_64 | Not supported |

---

## 9. Release Notes

### 9.1 Release Notes Format

```markdown
# CORTEX vX.Y.Z

## Breaking Changes
- [List breaking changes, if any]

## New Features
- [List new features]

## Bug Fixes
- [List bug fixes]

## Improvements
- [List improvements]

## Dependencies
- [List dependency changes]

## Migration Notes
- [List migration steps, if any]
```

### 9.2 Auto-Generated Content

| Content | Source |
|---|---|
| Breaking changes | Commit messages containing `BREAKING CHANGE:` |
| Features | Commit messages containing `feat:` |
| Bug fixes | Commit messages containing `fix:` |
| Improvements | Commit messages containing `improve:` |
| Dependencies | `cargo deny` output |

---

## 10. Build Invariants

### 10.1 Build Invariant List

| # | Invariant | Enforcement |
|---|---|---|
| BLD-001 | Release binary passes all stages of CI pipeline | CI gate |
| BLD-002 | Release binary checksum matches published checksum | Release verification |
| BLD-003 | Release binary starts and produces status output | Binary validation |
| BLD-004 | `.cx` created on first boot by release binary | Integration test |
| BLD-005 | All tests pass with release profile | CI gate |
| BLD-006 | No critical or high security vulnerabilities | Security audit |
| BLD-007 | All dependency licenses are compatible | License audit |
| BLD-008 | Binary is stripped of debug symbols | Build profile |
| BLD-009 | LTO is enabled for release builds | Build profile |

---

## 11. Traceability

### 11.1 Traceability to Requirements

| DOC-01 Requirement | DOC-06 Coverage |
|---|---|
| §20.1 Language & Toolchain | §1 Toolchain Requirements |
| §20.2 Dependency Constraints | §6 Dependency Management |
| §20.3 Build & Deployment | §2 Build Pipeline, §8 Binary Specifications |
| FR-PRS-001 through FR-PRS-006 | §8.2 Binary Validation |

### 11.2 Final Build Contract Statement

> **This document constitutes the build and release contract for CORTEX.** It defines how the system is built, tested, versioned, and released.
>
> The build contract ensures:
> - **Reproducible builds**: Pinned toolchain, locked dependencies, deterministic profiles.
> - **CI-gated releases**: Every release passes format, lint, test, audit, build, and validation gates.
> - **Semantic versioning**: Clear versioning policy with backward compatibility rules.
> - **Binary validation**: Every release binary is validated before publication.
> - **Dependency hygiene**: Automated vulnerability scanning and license compliance.
>
> **CORTEX build contract: 8 pipeline stages, 9 build invariants, semantic versioning, reproducible builds.**

---

*End of Document — CORTEX-DOC-06 Build & Release Specification v1.1.0*
