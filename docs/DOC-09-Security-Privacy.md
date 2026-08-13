# CORTEX — 09 Security & Privacy Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-09 |
| **Title** | Security & Privacy Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Security Contract |
| **Scope** | Security architecture, threat model, privacy controls, compliance |
| **Parent Document** | CORTEX-DOC-01 Technical Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Replace SHA-256 with BLAKE3 for all hashing operations |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Security Review | _____________ | _____________ |

### Document Purpose

This document defines **CORTEX's security architecture, threat model, privacy controls, and compliance requirements**. It constitutes the security contract: security boundaries, authentication, authorization, data protection, and privacy guarantees.

---

## 1. Security Architecture

### 1.1 Security Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  Layer 1: Input Validation                                       │
│  All inputs validated before entering cognitive pipeline         │
│  • Format validation (JSON, TOML)                                │
│  • Size validation (bounded inputs)                              │
│  • Type validation (correct schema)                              │
├─────────────────────────────────────────────────────────────────┤
│  Layer 2: Authentication (API)                                   │
│  Bearer token required for all API endpoints                     │
│  • Token from environment variable                               │
│  • Constant-time comparison                                      │
│  • Never logged or persisted                                     │
├─────────────────────────────────────────────────────────────────┤
│  Layer 3: Policy Gate                                            │
│  All consequential operations pass through PolicyEngine          │
│  • Operation classification                                      │
│  • Risk estimation                                               │
│  • ALLOW/LIMIT/DENY decision                                    │
├─────────────────────────────────────────────────────────────────┤
│  Layer 4: State Invariants                                       │
│  Invalid state transitions fail before persistence               │
│  • Reference integrity                                           │
│  • Topology validity                                             │
│  • Provenance presence                                           │
├─────────────────────────────────────────────────────────────────┤
│  Layer 5: Persistence Integrity                                  │
│  .cx checksum verification before load                           │
│  • BLAKE3 file checksum                                        │
│  • Per-section checksum                                          │
│  • Atomic write (temp→flush→verify→replace)                      │
├─────────────────────────────────────────────────────────────────┤
│  Layer 6: Secret Isolation                                       │
│  API keys in environment only; never in .cx or cognitive state   │
│  • Environment variable injection                                │
│  • Never serialized                                              │
│  • Never logged                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Security Boundaries

| Boundary | Protected | Protector |
|---|---|---|
| API authentication | API endpoints | Bearer token validation |
| Policy gate | State mutations | PolicyEngine |
| State invariants | .cx integrity | Invariant validation |
| Persistence integrity | .cx data | Checksum verification |
| Secret isolation | API keys | Environment-only injection |
| Self-modification levels | Policy/algorithm state | Level classification |

---

## 2. Threat Model

### 2.1 Threat Categories

| Category | Description | Mitigation |
|---|---|---|
| Unauthorized access | Unauthenticated API access | Bearer token authentication |
| Policy bypass | Learning modifies policy | Level 3 restriction |
| State corruption | Tampered .cx file | Checksum verification |
| Data exfiltration | Cognitive state leaked | No external transmission |
| Denial of service | Resource exhaustion | Bounded operations, rate limits |
| Injection attacks | Malformed input | Input validation pipeline |
| Privilege escalation | Learning elevates capabilities | Self-modification levels |
| Information leakage | Secrets in logs/responses | Secret isolation |

### 2.2 Threat Matrix

| Threat | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Brute-force API key | Low | High | Rate limiting, strong keys |
| Malicious input | Medium | Medium | Input validation, policy gate |
| .cx file tampering | Low | High | Checksum verification |
| Memory exhaustion | Medium | Medium | Memory budgets, pressure response |
| Disk exhaustion | Low | Medium | Checkpoint retention limits |
| Learning manipulation | Low | High | Stability guard, policy gate |
| Internet content poisoning | Medium | Medium | Provenance tracking, verification |

### 2.3 Security Assumptions

| # | Assumption |
|---|---|
| SA-001 | The operating system is trusted |
| SA-002 | The filesystem provides atomic rename |
| SA-003 | Environment variables are set by trusted administrator |
| SA-004 | The binary itself is not tampered |
| SA-005 | Network transport is not encrypted (use reverse proxy for TLS) |

---

## 3. Authentication

### 3.1 Authentication Mechanism

| Property | Value |
|---|---|
| Method | Bearer token |
| Token source | Environment variable `CORTEX_API_KEY` |
| Token format | Arbitrary string (recommended: ≥ 32 chars) |
| Token transmission | `Authorization: Bearer {token}` header |
| Token comparison | Constant-time comparison |
| Token storage | NEVER in `.cx`; NEVER in logs |
| Token rotation | Requires restart with new env var |

### 3.2 Authentication Flow

```
Client Request
    │
    ↓
Extract Authorization header
    │
    ↓
Parse Bearer token
    │
    ↓
Compare with expected key (constant-time)
    │
    ├── Match → Continue to handler
    │
    └── No match → 401 Unauthorized (no information leakage)
```

### 3.3 Authentication Rules

| Rule | Description |
|---|---|
| AUTH-001 | All API endpoints require valid Bearer token |
| AUTH-002 | No anonymous access to any endpoint |
| AUTH-003 | Failed auth returns 401 with no information leakage |
| AUTH-004 | API key comparison uses constant-time algorithm |
| AUTH-005 | API key is NEVER logged |
| AUTH-006 | API key is NEVER included in responses |
| AUTH-007 | API key is NEVER persisted in `.cx` |
| AUTH-008 | Rate limiting applies per connection |

---

## 4. Authorization

### 4.1 Authorization Model

| Operation Class | Authorization | Policy Check |
|---|---|---|
| Read (query, status, inspect) | Authenticated | None |
| Inference (process input) | Authenticated | None |
| Observation (submit observation) | Authenticated | None |
| Learning (trigger learning) | Authenticated | `policy.learning` |
| Internet (fetch) | Authenticated | `policy.internet_learning` |
| State mutation (experience) | Authenticated | Policy evaluation |
| Checkpoint | Authenticated | None |
| Configuration read | Authenticated | None |
| Configuration write | NOT EXPOSED via API | N/A |

### 4.2 Self-Modification Levels

| Level | Scope | Default | Required For |
|---|---|---|---|
| 1 — Cognitive State Adaptation | Memory, language state, world model, learned parameters, procedures, associations | Allowed | Normal learning |
| 2 — Algorithm Adaptation | Learning, reasoning, language, runtime algorithms | Restricted | Meta-learning |
| 3 — Security / Policy Modification | Policy, authorization, risk boundary, security enforcement | Restricted (highest) | Administrative only |

### 4.3 Policy Gate Rules

| Rule | Description |
|---|---|
| PG-001 | All potentially consequential operations pass through PolicyEngine |
| PG-002 | Policy decisions are deterministic for same input |
| PG-003 | Ambiguous security decisions default to DENY |
| PG-004 | Policy is separate from learned knowledge |
| PG-005 | Learning SHALL NOT modify root policy |
| PG-006 | Self-modification Level 3 requires explicit administrative action |

---

## 5. Data Protection

### 5.1 Data Classification

| Data Type | Classification | Protection |
|---|---|---|
| API key | SECRET | Environment only; never persisted |
| Cognitive state (.cx) | CONFIDENTIAL | Checksum integrity; local filesystem |
| Configuration (.toml) | INTERNAL | Local filesystem |
| Observations | CONFIDENTIAL | Processed in memory; persisted in .cx |
| Logs | INTERNAL | Bounded; no secrets |
| Checkpoints | CONFIDENTIAL | Local filesystem; versioned |

### 5.2 Data Flow Security

```
User Input → Policy Check → Cognitive Processing → State Mutation → Persistence
     │              │              │                      │              │
     ↓              ↓              ↓                      ↓              ↓
  Validated    ALLOW/LIMIT    Bounded processing    Invariant check  Atomic write
```

### 5.3 Data at Rest

| Protection | Implementation |
|---|---|
| Integrity | BLAKE3 checksums on .cx sections and file |
| Confidentiality | Local filesystem permissions (operator responsibility) |
| Availability | Checkpoint-based backup |
| Authenticity | Checksum verification on load |

### 5.4 Data in Transit

| Protection | Implementation |
|---|---|
| API traffic | HTTP (use reverse proxy for HTTPS) |
| Internet fetches | HTTP/HTTPS (bounded by policy) |
| No external transmission | Cognitive state never transmitted externally |

---

## 6. Privacy Controls

### 6.1 Privacy Principles

| # | Principle | Implementation |
|---|---|---|
| PP-001 | Data minimization | Only necessary data is collected and stored |
| PP-002 | Purpose limitation | Data used only for cognitive operations |
| PP-003 | Storage limitation | Memory budgets bound storage; forgetting removes data |
| PP-004 | No external transmission | Cognitive state never leaves the system |
| PP-005 | Provenance transparency | All data carries origin information |
| PP-006 | User control | Users can trigger forgetting and state deletion |

### 6.2 Data Handling Rules

| Rule | Description |
|---|---|
| PRIV-001 | All persistent cognitive state resides in `.cx` on local filesystem |
| PRIV-002 | CORTEX SHALL NOT transmit cognitive state to external services |
| PRIV-003 | Internet observations are bounded and carry provenance |
| PRIV-004 | API keys and secrets SHALL NOT appear in `.cx`, logs, or cognitive state |
| PRIV-005 | User-provided information carries `UserProvided` provenance |
| PRIV-006 | Internet-derived information carries `Internet` provenance and is never treated as ground truth |

### 6.3 Forgetting & Deletion

| Operation | Effect | Command |
|---|---|---|
| Forget (moderate) | Removes low-value episodes | `cortex learn` with policy |
| Forget (aggressive) | Removes more episodes | API: `/v1/memory/forget` |
| State reset | Complete state deletion | `cortex init --force` |
| Checkpoint deletion | Removes checkpoint files | Manual file deletion |

---

## 7. Security Invariants

### 7.1 Invariant List

| # | Invariant | Enforcement |
|---|---|---|
| SEC-001 | All API endpoints require authentication | Bearer token check |
| SEC-002 | Policy gate enforces all consequential operations | PolicyEngine |
| SEC-003 | Learning cannot modify Level 3 (security/policy) | Self-modification levels |
| SEC-004 | API key is never persisted in .cx | Secret isolation |
| SEC-005 | API key is never logged | Logging rules |
| SEC-006 | .cx integrity is verified before loading | Checksum verification |
| SEC-007 | Fail-closed on ambiguous security decisions | Policy gate default |
| SEC-008 | State invariants are checked before persistence | Invariant validation |
| SEC-009 | Rate limiting prevents abuse | Per-connection rate limits |
| SEC-010 | Input validation prevents injection | Validation pipeline |

---

## 8. Security Testing

### 8.1 Security Test Categories

| Category | Tests | Purpose |
|---|---|---|
| Authentication | Unauthenticated access, invalid tokens | Verify auth enforcement |
| Authorization | Policy denial, level restrictions | Verify policy enforcement |
| Input validation | Malformed input, overflow, injection | Verify input handling |
| State integrity | Checksum verification, corruption detection | Verify persistence security |
| Secret isolation | API key in .cx, API key in logs | Verify secret handling |
| Rate limiting | Excessive requests | Verify rate limit enforcement |
| Fail-closed | Ambiguous security decisions | Verify default deny behavior |

### 8.2 Security Test Requirements

| ID | Requirement |
|---|---|
| SEC-T-001 | All authentication tests SHALL pass |
| SEC-T-002 | All authorization tests SHALL pass |
| SEC-T-003 | All input validation tests SHALL pass |
| SEC-T-004 | All state integrity tests SHALL pass |
| SEC-T-005 | All secret isolation tests SHALL pass |
| SEC-T-006 | All rate limiting tests SHALL pass |
| SEC-T-007 | All fail-closed tests SHALL pass |

---

## 9. Compliance Considerations

### 9.1 Applicable Standards

| Standard | Relevance | CORTEX Coverage |
|---|---|---|
| OWASP Top 10 | Web application security | Input validation, authentication, error handling |
| NIST Cybersecurity Framework | Security management | Identify, Protect, Detect, Respond, Recover |
| Data Protection Principles | Privacy | Data minimization, purpose limitation, storage limitation |

### 9.2 Compliance Mapping

| OWASP Category | CORTEX Mitigation |
|---|---|
| A01: Broken Access Control | Bearer token authentication, policy gate |
| A02: Cryptographic Failures | BLAKE3 checksums, secret isolation |
| A03: Injection | Input validation pipeline |
| A04: Insecure Design | Security architecture (6 layers) |
| A05: Security Misconfiguration | Configuration validation |
| A06: Vulnerable Components | Dependency audit (cargo audit) |
| A07: Authentication Failures | Constant-time comparison, rate limiting |
| A08: Data Integrity Failures | Checksum verification, atomic writes |
| A09: Logging Failures | Bounded diagnostics, no secrets in logs |
| A10: SSRF | Internet fetch policy gate |

---

## 10. Traceability

### 10.1 Traceability to Requirements

| DOC-01 Requirement | DOC-09 Coverage |
|---|---|
| SEC-001 through SEC-008 | §1-7 Security architecture, boundaries, invariants |
| SEC-API-001 through SEC-API-004 | §3-4 Authentication and authorization |
| SEC-INT-001 through SEC-INT-004 | §5.4 Data in transit |
| PRV-001 through PRV-006 | §6 Privacy controls |
| FR-POL-001 through FR-POL-006 | §4.2-4.3 Self-modification levels, policy gate |

### 10.2 Final Security Contract Statement

> **This document constitutes the security and privacy contract for CORTEX.** It defines security boundaries, authentication, authorization, data protection, and privacy guarantees.
>
> The security contract ensures:
> - **Authenticated access**: All API endpoints require Bearer token.
> - **Policy-gated operations**: All consequential operations pass through PolicyEngine.
> - **Self-modification levels**: Three levels of modification restriction.
> - **Secret isolation**: API keys never in state, logs, or responses.
> - **State integrity**: Checksum verification prevents corrupted state loading.
> - **Fail-closed**: Ambiguous security decisions default to DENY.
> - **Privacy by design**: No external transmission of cognitive state.
>
> **CORTEX security contract: 6 security layers, 10 security invariants, 3 self-modification levels, 7 authentication rules.**

---

*End of Document — CORTEX-DOC-09 Security & Privacy Specification v1.1.0*
