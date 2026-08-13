# CORTEX — 08 Deployment & Operations Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-08 |
| **Title** | Deployment & Operations Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Operations Contract |
| **Scope** | Deployment procedures, operations runbook, monitoring, upgrade, backup |
| **Parent Document** | CORTEX-DOC-01 Technical Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Update cross-references for BLAKE3 migration |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Operations Lead | _____________ | _____________ |

### Document Purpose

This document defines **how CORTEX is deployed, operated, monitored, upgraded, and recovered**. It constitutes the operations contract: deployment procedures, operational runbook, monitoring requirements, upgrade procedures, and disaster recovery. Concurrency model and durability semantics are defined in DOC-00.

---

## 1. Deployment Requirements

### 1.1 Minimum Deployment Artifacts

| Artifact | Required | Description |
|---|---|---|
| `cortex` binary | YES | Single executable |
| `cortex.toml` | YES | Configuration file |
| `cortex.cx` | NO | Auto-created on first boot |
| `checkpoints/` | NO | Created on first checkpoint |

### 1.2 Deployment Directory Structure

```
/opt/cortex/                    # Default deployment directory
├── cortex                      # Binary (executable)
├── cortex.toml                 # Configuration
├── cortex.cx                   # Cognitive state (auto-created)
└── checkpoints/                # Checkpoint directory (auto-created)
    ├── checkpoint_000001.cx
    ├── checkpoint_000002.cx
    └── ...
```

### 1.3 Deployment Prerequisites

| Requirement | Description |
|---|---|
| Linux x86_64 | Primary target platform |
| POSIX filesystem | Atomic rename support required |
| 2 GB+ RAM | For default memory configuration |
| 1 GB+ disk | For binary, config, state, and checkpoints |
| Network (optional) | Required only if internet features enabled |
| Environment variables | For API key injection |

---

## 2. Deployment Procedures

### 2.1 Fresh Installation

```
PROCEDURE: FreshInstall
INPUT: cortex binary, cortex.toml
OUTPUT: Operational CORTEX instance
STEPS:
  1. Create deployment directory: mkdir -p /opt/cortex
  2. Copy binary: cp cortex /opt/cortex/cortex
  3. Set permissions: chmod +x /opt/cortex/cortex
  4. Copy configuration: cp cortex.toml /opt/cortex/cortex.toml
  5. Set API key (if API enabled): export CORTEX_API_KEY="..."
  6. Navigate to deployment directory: cd /opt/cortex
  7. First boot: ./cortex run (or ./cortex serve)
  8. Verify: ./cortex status
  9. Expected: status reports "ready"
VALIDATION:
  - Binary starts without error
  - cortex.cx is created
  - Status reports "ready"
  - Full cognitive pipeline processes input
```

### 2.2 Upgrade Procedure

```
PROCEDURE: Upgrade
INPUT: New cortex binary, new cortex.toml (optional)
OUTPUT: Upgraded CORTEX instance
STEPS:
  1. Stop running CORTEX instance
  2. Backup current state: cp cortex.cx cortex.cx.bak
  3. Backup current config: cp cortex.toml cortex.toml.bak
  4. Replace binary: cp new_cortex /opt/cortex/cortex
  5. Replace config (if changed): cp new_cortex.toml /opt/cortex/cortex.toml
  6. Start new version: ./cortex run
  7. Verify migration: ./cortex status
  8. Expected: state migrated if needed; status "ready"
VALIDATION:
  - Binary starts without error
  - State is migrated (if version changed)
  - Status reports "ready"
  - All memories preserved
ROLLBACK:
  - Stop new version
  - Restore binary: cp cortex.bak /opt/cortex/cortex
  - Restore state: cp cortex.cx.bak /opt/cortex/cortex.cx
  - Restore config: cp cortex.toml.bak /opt/cortex/cortex.toml
  - Start old version
```

### 2.3 Backup Procedure

```
PROCEDURE: Backup
INPUT: Running CORTEX instance
OUTPUT: Backup archive
STEPS:
  1. Create manual checkpoint: ./cortex checkpoint
  2. Copy state file: cp cortex.cx cortex.cx.backup.$(date +%Y%m%d)
  3. Copy config file: cp cortex.toml cortex.toml.backup.$(date +%Y%m%d)
  4. Copy checkpoints directory: cp -r checkpoints/ checkpoints.backup.$(date +%Y%m%d)
  5. Create archive: tar czf cortex-backup-$(date +%Y%m%d).tar.gz cortex.cx cortex.toml checkpoints/
VALIDATION:
  - Archive exists and is non-empty
  - Archive can be extracted
  - Extracted state passes validation: ./cortex migrate --dry-run
```

---

## 3. Operations Runbook

### 3.1 Common Operations

| Operation | Command | Expected Output |
|---|---|---|
| Check status | `./cortex status` | Status report with "ready" |
| Check health | `curl -H "Authorization: Bearer $KEY" http://localhost:8080/v1/health` | `{"healthy": true}` |
| Create checkpoint | `./cortex checkpoint` | Checkpoint created message |
| Inspect memory | `./cortex inspect memory` | Memory usage report |
| Inspect state | `./cortex inspect` | Full state summary |
| Trigger learning | `./cortex learn` | Learning cycle complete message |
| Validate state | `./cortex migrate --dry-run` | Validation report |

### 3.2 Troubleshooting Guide

| Symptom | Possible Cause | Resolution |
|---|---|---|
| Binary won't start | Invalid config | Check `cortex.toml` syntax; fix errors |
| Binary won't start | Missing config | Create or provide `cortex.toml` |
| State corrupt | Disk failure, crash | Restore from checkpoint: restart (auto-recovery) |
| API returns 401 | Invalid API key | Check `CORTEX_API_KEY` environment variable |
| API returns 503 | Resource exhaustion | Check memory usage; reduce load |
| High memory pressure | Memory budget exceeded | Increase budgets or trigger consolidation |
| Slow responses | High reasoning complexity | Reduce `reasoning.max_steps` |
| Learning not occurring | Policy disabled | Check `policy.learning` in config |

### 3.3 Log Analysis

| Log Level | Usage |
|---|---|
| ERROR | Failures requiring attention |
| WARN | Degraded operation, recoverable errors |
| INFO | Normal operational events |
| DEBUG | Detailed operational tracing |
| TRACE | Algorithm-level detail |

### 3.4 Diagnostic Commands

```bash
# Full status
./cortex status --verbose

# Memory inspection
./cortex inspect memory

# World model inspection
./cortex inspect world

# Learning state inspection
./cortex inspect learning

# Self model inspection
./cortex inspect self-model

# Policy state inspection
./cortex inspect policy

# State metadata inspection
./cortex inspect metadata

# Health check (API)
curl -H "Authorization: Bearer $KEY" http://localhost:8080/v1/health

# Metrics (API)
curl -H "Authorization: Bearer $KEY" http://localhost:8080/v1/metrics

# Diagnostics (API)
curl -H "Authorization: Bearer $KEY" http://localhost:8080/v1/diagnostics
```

---

## 4. Monitoring

### 4.1 Health Check Contract

| Property | Value |
|---|---|
| Endpoint | `GET /v1/health` |
| Frequency | Every 30 seconds recommended |
| Timeout | 5 seconds |
| Expected Response | `{"healthy": true, "checks": {...}}` |

### 4.2 Health Check Components

| Component | Check | Healthy Condition |
|---|---|---|
| State validity | `state_valid` | `.cx` passes integrity check |
| Memory budget | `memory_within_budget` | Usage < 95% of budget |
| Persistence | `persistence_operational` | Last save successful |
| Language | `language_operational` | Vocabulary size > 0 |
| Neural | `neural_operational` | Active cells > 0 |
| Policy | `policy_operational` | Policy engine responding |

### 4.3 Key Metrics

| Metric | Description | Healthy Range |
|---|---|---|
| `prediction_error` | Average prediction error | < 0.5 |
| `memory_pressure` | Memory pressure level | Low or Moderate |
| `learning_rate_effective` | Effective learning rate | > 0 |
| `verification_confidence` | Average verification confidence | > 0.5 |
| `reasoning_consistency` | Reasoning consistency score | > 0.6 |
| `episode_count` | Total episodes stored | Monotonically increasing |
| `total_learning_events` | Total learning events | Monotonically increasing |
| `checkpoint_count` | Total checkpoints | Monotonically increasing |
| `uptime_seconds` | Process uptime | > 0 |
| `error_count` | Total errors in session | Bounded, not rapidly increasing |

### 4.4 Alert Conditions

| Condition | Severity | Action |
|---|---|---|
| Health check fails | CRITICAL | Investigate immediately |
| Memory pressure = Critical | HIGH | Trigger emergency forgetting |
| Prediction error > 0.8 | WARNING | Review learning configuration |
| API authentication failures > 10/min | WARNING | Review access patterns |
| State save failure | CRITICAL | Check disk space; backup immediately |
| Checksum mismatch on load | CRITICAL | Initiate recovery procedure |

---

## 5. Backup & Recovery

### 5.1 Backup Strategy

| Backup Type | Frequency | Retention | Content |
|---|---|---|---|
| Checkpoint | Per `checkpoint_interval` | Configurable (default: 10) | Full cognitive state |
| Manual checkpoint | On demand | Until manual deletion | Full cognitive state |
| State backup | Before upgrades | 1 per upgrade | Full cognitive state |
| Config backup | Before changes | Until manual deletion | Configuration |

### 5.2 Recovery Priority

| Priority | Source | Condition |
|---|---|---|
| 1 | Current valid `.cx` | File exists, passes integrity check |
| 2 | Latest valid checkpoint | Checkpoint exists, passes integrity check |
| 3 | Previous valid checkpoint | Older checkpoint exists, passes integrity check |
| 4 | Fresh initialization | No valid state available |
| 5 | Safe stop | Initialization fails |

### 5.3 Recovery Procedures

#### 5.3.1 Automatic Recovery

```
TRIGGER: .cx fails integrity check on load
PROCEDURE:
  1. Log corruption event
  2. Attempt load from latest checkpoint
  3. If checkpoint valid → restore and continue
  4. If checkpoint corrupt → try next checkpoint
  5. If no valid checkpoint → initialize fresh state
  6. Log recovery action
RESULT: System operational with recovered or fresh state
```

#### 5.3.2 Manual Recovery

```
TRIGGER: Operator initiates recovery
PROCEDURE:
  1. Stop CORTEX instance
  2. Identify valid checkpoint: ls -la checkpoints/
  3. Copy checkpoint to cortex.cx: cp checkpoints/checkpoint_NNNNNN.cx cortex.cx
  4. Validate: ./cortex migrate --dry-run
  5. Start CORTEX: ./cortex run
  6. Verify: ./cortex status
RESULT: System operational with restored state
```

#### 5.3.3 Disaster Recovery

```
TRIGGER: Complete state loss (no valid .cx or checkpoints)
PROCEDURE:
  1. Stop CORTEX instance
  2. Remove corrupted cortex.cx: rm -f cortex.cx
  3. Start CORTEX: ./cortex run (creates fresh state)
  4. Verify: ./cortex status
  5. Restore from backup (if available): tar xzf cortex-backup-YYYYMMDD.tar.gz
  6. Validate restored state: ./cortex migrate --dry-run
  7. Start CORTEX: ./cortex run
RESULT: System operational (fresh or restored state)
NOTE: Learned knowledge is lost in fresh initialization
```

---

## 6. Upgrade Procedures

### 6.1 Upgrade Compatibility Matrix

| Change Type | Upgrade Type | State Migration Required |
|---|---|---|
| PATCH version | In-place upgrade | NO |
| MINOR version | In-place upgrade | MAYBE (if format changed) |
| MAJOR version | In-place upgrade with migration | YES |

### 6.2 Pre-Upgrade Checklist

| # | Check | Action |
|---|---|---|
| 1 | Backup current state | `./cortex checkpoint` then copy `.cx` |
| 2 | Backup configuration | Copy `cortex.toml` |
| 3 | Review release notes | Check for breaking changes |
| 4 | Check disk space | Ensure sufficient space for migration |
| 5 | Stop running instance | SIGTERM for graceful shutdown |

### 6.3 Post-Upgrade Validation

| # | Check | Command |
|---|---|---|
| 1 | Binary starts | `./cortex status` |
| 2 | State migrated | `./cortex inspect metadata` (check version) |
| 3 | Memory preserved | `./cortex inspect memory` (check episode count) |
| 4 | World model preserved | `./cortex inspect world` (check entity count) |
| 5 | Learning operational | `./cortex learn` |
| 6 | API operational | `curl /v1/status` |

---

## 7. Capacity Planning

### 7.1 Resource Sizing Guide

| Deployment Size | Cells | Memory Budget | Disk | Use Case |
|---|---|---|---|---|
| Small | 256 | 256 MB total | 1 GB | Development, testing |
| Medium | 1024 | 1 GB total | 5 GB | Single-user production |
| Large | 4096 | 2 GB total | 20 GB | Power user |
| Default | 4096 | 1.7 GB total | 10 GB | Standard deployment |

### 7.2 Growth Projections

| Metric | Growth Rate | Monitoring |
|---|---|---|
| Episode count | +1 per interaction | `episode_count` metric |
| Vocabulary size | +symbols per unknown word | `vocabulary_size` metric |
| `.cx` file size | Proportional to state | File size monitoring |
| Checkpoint count | +1 per checkpoint interval | `checkpoint_count` metric |
| Memory usage | Proportional to state size | `memory_pressure` metric |

---

## 8. Operational Invariants

| # | Invariant | Enforcement |
|---|---|---|
| OPS-001 | `.cx` is always recoverable from checkpoint | Backup strategy |
| OPS-002 | Upgrade preserves learned knowledge | Migration testing |
| OPS-003 | Graceful shutdown preserves state | SIGTERM handling |
| OPS-004 | Health check accurately reflects system state | Health check contract |
| OPS-005 | Monitoring metrics are bounded | Metric retention policy |
| OPS-006 | Backup archives are validated after creation | Backup verification |
| OPS-007 | Recovery procedures are tested regularly | DR testing |

---

## 9. Traceability

### 9.1 Traceability to Requirements

| DOC-01 Requirement | DOC-08 Coverage |
|---|---|
| §7.3 First Boot | §2.1 Fresh Installation |
| §7.4 Graceful Shutdown | §6 Upgrade Procedures |
| §7.5 Restart | §5.3 Recovery Procedures |
| §10.1 Reliability | §5 Backup & Recovery |
| §10.2 Availability | §4 Monitoring |
| §25 Deployment Contract | §1-3 Deployment & Operations |
| REL-004 Recovery priority | §5.2 Recovery Priority |

### 9.2 Final Operations Contract Statement

> **This document constitutes the deployment and operations contract for CORTEX.** It defines how CORTEX is deployed, operated, monitored, upgraded, and recovered.
>
> The operations contract ensures:
> - **Reproducible deployment**: Clear procedures for fresh installation and upgrade.
> - **Health monitoring**: Defined health check contract and key metrics.
> - **Backup strategy**: Checkpoint-based backup with configurable retention.
> - **Recovery priority**: Clear recovery cascade from current state to fresh initialization.
> - **Upgrade compatibility**: Defined upgrade procedures with pre/post validation.
> - **Capacity planning**: Resource sizing guidance for different deployment scenarios.
>
> **CORTEX operations contract: 3 deployment procedures, 5 recovery procedures, 10 key metrics, 7 operational invariants.**

---

*End of Document — CORTEX-DOC-08 Deployment & Operations Specification v1.1.0*
