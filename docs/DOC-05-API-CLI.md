# CORTEX — 05 API & CLI Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-05 |
| **Title** | API & CLI Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Interface Contract |
| **Scope** | All external interfaces: HTTP API, CLI, environment variables |
| **Parent Document** | CORTEX-DOC-04 Algorithm Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per API version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Update cross-references for BLAKE3 migration |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| API Design Lead | _____________ | _____________ |
| CLI Design Lead | _____________ | _____________ |
| Security Review | _____________ | _____________ |

### Document Purpose

This document defines **all official interfaces** through which external software and humans communicate with CORTEX. It constitutes the interface contract: every API endpoint, every CLI command, every request/response format, every authentication mechanism, and every error response.

### Document Scope

This specification covers:

- Complete HTTP API endpoint definitions with request/response schemas.
- Complete CLI command definitions with arguments, options, and output formats.
- Authentication and authorization mechanisms.
- Error response formats and exit codes.
- Machine-readable output formats.
- API versioning and compatibility rules.
- Rate limits, timeouts, and resource constraints.

This specification does NOT cover:

- Internal algorithm logic (governed by DOC-04).
- Data structure definitions (governed by DOC-03).
- Module architecture (governed by DOC-02).
- System requirements (governed by DOC-01).

---

## 1. Interface Principles

| # | Principle | Implication |
|---|---|---|
| IP-001 | Single binary serves all interfaces | No separate API server binary; CLI and API from same `cortex` binary |
| IP-002 | Policy gate on all mutations | Every state-changing operation passes through policy evaluation |
| IP-003 | Authentication on all API endpoints | Bearer token required; no anonymous access |
| IP-004 | No arbitrary state mutation | API requests map to defined cognitive operations, not raw state writes |
| IP-005 | Consistent error model | All errors return structured JSON with error kind, message, and code |
| IP-006 | Versioned API | URL path includes version prefix (`/v1/`) |
| IP-007 | Machine-readable by default | CLI supports `--json` flag for structured output |
| IP-008 | Idempotent reads | GET and query operations are side-effect-free |
| IP-009 | Explicit side effects | POST operations clearly indicate state mutation |
| IP-010 | Bounded responses | All responses have bounded size; no unbounded streaming |
| IP-011 | Provenance in responses | Knowledge responses include provenance and verification status |
| IP-012 | Graceful degradation | Disabled subsystems return defined empty/default responses |
| IP-013 | No secret exposure | API keys, internal paths, and sensitive config never in responses |
| IP-014 | CLI mirrors API for primary operations | Core API operations have CLI equivalents |
| IP-015 | Deterministic error codes | Same error condition always produces same error code |

---

## 2. API Architecture

### 2.1 API Server Model

```
┌─────────────────────────────────────────────────────────────┐
│                    CORTEX BINARY                             │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              API SERVER (embedded)                    │   │
│  │                                                     │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────────────┐    │   │
│  │  │  HTTP   │  │  Auth   │  │  Request Router │    │   │
│  │  │ Listener│  │  Layer  │  │                 │    │   │
│  │  └────┬────┘  └────┬────┘  └───────┬─────────┘    │   │
│  │       │            │               │               │   │
│  │       ↓            ↓               ↓               │   │
│  │  ┌─────────────────────────────────────────────┐   │   │
│  │  │           REQUEST HANDLER                    │   │   │
│  │  │                                             │   │   │
│  │  │  Validate → Authenticate → Policy Check     │   │   │
│  │  │  → Cognitive Operation → Response           │   │   │
│  │  └─────────────────────┬───────────────────────┘   │   │
│  │                        │                           │   │
│  └────────────────────────┼───────────────────────────┘   │
│                           │                               │
│                           ↓                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              COGNITIVE RUNTIME                       │   │
│  │  (Language → Neural → Memory → World → Reasoning    │   │
│  │   → Planning → Verification → Learning → Persist)   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 API Server Configuration

| Property | Source | Default |
|---|---|---|
| Enabled | `[api].enabled` | `true` |
| Bind address | `[api].bind` | `"127.0.0.1:8080"` |
| API key env var | `[api].api_key_env` | `"CORTEX_API_KEY"` |
| Protocol | Fixed | HTTP/1.1 |
| TLS | Not included | N/A (use reverse proxy) |
| Max request body | Fixed | 1 MB |
| Max response body | Fixed | 10 MB |
| Request timeout | Fixed | 30 seconds |
| Concurrent connections | Fixed | 8 |

### 2.3 API Server Lifecycle

```
cortex serve
    │
    ↓
┌─────────────────────────┐
│ 1. Load configuration   │
│ 2. Load/initialize state│
│ 3. Initialize runtime   │
│ 4. Read API key from env│
│ 5. Bind TCP listener    │
│ 6. Accept connections   │
│ 7. Process requests     │
│ 8. Graceful shutdown    │
└─────────────────────────┘
```

---

## 3. API Versioning

### 3.1 Version Strategy

| Property | Value |
|---|---|
| Versioning method | URL path prefix |
| Current version | `v1` |
| Base URL | `http://{bind}/v1/` |
| Version lifecycle | Major versions are additive; old versions deprecated, not removed immediately |
| Breaking changes | Require new major version (`v2`) |
| Non-breaking changes | Additive fields, new endpoints within same version |

### 3.2 Version Rules

| Rule | Description |
|---|---|
| VER-001 | All endpoints include version prefix: `/v1/{endpoint}` |
| VER-002 | New fields in responses are backward-compatible |
| VER-003 | Removed or renamed fields require new major version |
| VER-004 | New endpoints may be added within same version |
| VER-005 | Deprecated endpoints return `Warning` header |
| VER-006 | Client MUST specify version; no versionless endpoints |

---

## 4. Request Model

### 4.1 Common Request Structure

All API requests follow this structure:

```http
{METHOD} /v1/{endpoint} HTTP/1.1
Host: {bind_address}
Authorization: Bearer {API_KEY}
Content-Type: application/json
Content-Length: {length}

{JSON_BODY}
```

### 4.2 Request Headers

| Header | Required | Value | Description |
|---|---|---|---|
| `Authorization` | YES | `Bearer {API_KEY}` | Authentication token |
| `Content-Type` | YES (POST) | `application/json` | Request body format |
| `Accept` | NO | `application/json` | Response format (default: JSON) |
| `X-Request-ID` | NO | UUID | Client-side request tracking |
| `X-Timeout-Ms` | NO | Integer | Client-requested timeout override (capped at 30s) |

### 4.3 Request Body Rules

| Rule | Description |
|---|---|
| REQ-001 | All POST bodies are JSON |
| REQ-002 | Maximum body size: 1 MB |
| REQ-003 | Invalid JSON returns 400 error |
| REQ-004 | Unknown fields are ignored (forward compatibility) |
| REQ-005 | Missing required fields return 422 error |
| REQ-006 | String fields are UTF-8 |
| REQ-007 | Numeric fields follow JSON number specification |

---

## 5. Response Model

### 5.1 Common Response Structure

All successful responses:

```json
{
  "success": true,
  "data": { ... },
  "metadata": {
    "request_id": "uuid",
    "timestamp": 1723550400000,
    "duration_ms": 42,
    "state_updated": true,
    "version": "1.0.0"
  }
}
```

### 5.2 Response Fields

| Field | Type | Description |
|---|---|---|
| `success` | bool | Whether the operation succeeded |
| `data` | object | Operation-specific response data |
| `metadata.request_id` | string (UUID) | Request identifier |
| `metadata.timestamp` | u64 | Response timestamp (ms since epoch) |
| `metadata.duration_ms` | u64 | Processing duration |
| `metadata.state_updated` | bool | Whether cognitive state was mutated |
| `metadata.version` | string | CORTEX version |

### 5.3 Response Headers

| Header | Value | Description |
|---|---|---|
| `Content-Type` | `application/json` | Response format |
| `X-Request-ID` | UUID | Echoed request ID |
| `X-CORTEX-Version` | String | CORTEX version |
| `X-State-Updated` | `true`/`false` | Whether state was mutated |

---

## 6. Error Model

### 6.1 Error Response Structure

All error responses:

```json
{
  "success": false,
  "error": {
    "code": "CORTEX_ERR_001",
    "kind": "InputError",
    "message": "Input exceeds maximum length",
    "details": {
      "max_length": 65536,
      "provided_length": 70000
    },
    "recoverable": true,
    "request_id": "uuid"
  }
}
```

### 6.2 Error Fields

| Field | Type | Description |
|---|---|---|
| `error.code` | string | Machine-readable error code |
| `error.kind` | string | Error taxonomy category |
| `error.message` | string | Human-readable description |
| `error.details` | object | Additional context (optional) |
| `error.recoverable` | bool | Whether retry may succeed |
| `error.request_id` | string | Request identifier |

### 6.3 Error Code Registry

> **Canonical definition:** See DOC-00 §7. Error kinds and severity levels are defined normatively in DOC-00.

| Code | Kind | HTTP Status | Severity | Description |
|---|---|---|---|---|
| `CORTEX_ERR_001` | InputError | 400 | Recoverable | Invalid input format |
| `CORTEX_ERR_002` | InputError | 400 | Recoverable | Input exceeds maximum length |
| `CORTEX_ERR_003` | EncodingError | 400 | Recoverable | Invalid UTF-8 encoding |
| `CORTEX_ERR_004` | AuthenticationError | 401 | Recoverable | Missing or invalid API key |
| `CORTEX_ERR_005` | AuthorizationError | 403 | Recoverable | Operation denied by policy |
| `CORTEX_ERR_006` | NotFoundError | 404 | Recoverable | Endpoint not found |
| `CORTEX_ERR_007` | ValidationError | 422 | Recoverable | Request validation failed |
| `CORTEX_ERR_008` | LanguageError | 500 | Recoverable | Language processing error |
| `CORTEX_ERR_009` | MemoryError | 500 | Recoverable/StateCorruption | Memory operation error |
| `CORTEX_ERR_010` | WorldModelError | 500 | Recoverable | World model error |
| `CORTEX_ERR_011` | ReasoningError | 500 | Recoverable | Reasoning error |
| `CORTEX_ERR_012` | PlanningError | 500 | Recoverable | Planning error |
| `CORTEX_ERR_013` | VerificationError | 500 | Recoverable | Verification error |
| `CORTEX_ERR_014` | LearningError | 500 | Recoverable | Learning error |
| `CORTEX_ERR_015` | PersistenceError | 500 | StateCorruption/Fatal | Persistence error |
| `CORTEX_ERR_016` | PolicyError | 403 | Recoverable | Policy denial |
| `CORTEX_ERR_017` | ResourceError | 503 | Recoverable | Resource exhaustion |
| `CORTEX_ERR_018` | NetworkError | 502 | Recoverable | Network operation failed |
| `CORTEX_ERR_019` | RuntimeError | 500 | Fatal | Internal runtime error |
| `CORTEX_ERR_020` | ConfigError | 500 | Configuration | Configuration error |
| `CORTEX_ERR_021` | RateLimitError | 429 | Recoverable | Rate limit exceeded |
| `CORTEX_ERR_022` | TimeoutError | 504 | Recoverable | Operation timed out |
| `CORTEX_ERR_023` | StateError | 409 | Recoverable | Invalid state transition |
| `CORTEX_ERR_024` | SerializationError | 500 | Recoverable | Serialization/deserialization error |
| `CORTEX_ERR_025` | SubsystemDisabled | 503 | Recoverable | Requested subsystem is disabled |

### 6.4 HTTP Status Code Mapping

| HTTP Status | Usage |
|---|---|
| 200 | Successful operation |
| 201 | Resource created (checkpoint) |
| 400 | Bad request (invalid input) |
| 401 | Unauthorized (missing/invalid API key) |
| 403 | Forbidden (policy denial) |
| 404 | Not found (invalid endpoint) |
| 422 | Unprocessable entity (validation failure) |
| 429 | Too many requests (rate limit) |
| 500 | Internal server error |
| 501 | Not implemented (disabled subsystem) |
| 502 | Bad gateway (network error) |
| 503 | Service unavailable (resource exhaustion) |
| 504 | Gateway timeout |

---

## 7. Authentication/Authorization Interface

### 7.1 Authentication Mechanism

| Property | Value |
|---|---|
| Method | Bearer token |
| Token source | Environment variable (`CORTEX_API_KEY`) |
| Token format | Arbitrary string (recommended: ≥ 32 chars) |
| Token transmission | `Authorization: Bearer {token}` header |
| Token storage | NEVER in `.cx`; NEVER in logs |
| Token rotation | Requires restart with new env var |

### 7.2 Authentication Flow

```
Client Request
    │
    ↓
┌─────────────────────────┐
│ Extract Authorization    │
│ header                   │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│ Parse Bearer token       │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│ Compare with expected    │
│ key (from env var)       │
└────────────┬────────────┘
             │
             ├── Match → Continue to handler
             │
             └── No match → 401 Unauthorized
```

### 7.3 Authorization Model

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

### 7.4 Security Rules

| Rule | Description |
|---|---|
| AUTH-001 | All endpoints require valid Bearer token |
| AUTH-002 | No anonymous access to any endpoint |
| AUTH-003 | Failed auth returns 401 with no information leakage |
| AUTH-004 | API key is compared using constant-time comparison |
| AUTH-005 | API key is NEVER logged |
| AUTH-006 | API key is NEVER included in responses |
| AUTH-007 | API key is NEVER persisted in `.cx` |
| AUTH-008 | Rate limiting applies per connection |

---

## 8. Core API

### 8.1 Inference Endpoint

The primary cognitive processing endpoint.

```
POST /v1/inference
```

**Request:**

```json
{
  "input": "Explain what gravity is.",
  "context": {
    "conversation_id": "optional-session-id",
    "prior_context": []
  },
  "options": {
    "max_tokens": 1024,
    "verify": true,
    "include_reasoning": false,
    "include_confidence": true
  }
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `input` | string | YES | Natural language input |
| `context.conversation_id` | string | NO | Session identifier |
| `context.prior_context` | array | NO | Prior conversation turns |
| `options.max_tokens` | u32 | NO | Max generation tokens (default: config) |
| `options.verify` | bool | NO | Enable verification (default: true) |
| `options.include_reasoning` | bool | NO | Include reasoning trace (default: false) |
| `options.include_confidence` | bool | NO | Include confidence in response (default: true) |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "output": "Gravity is a fundamental force of attraction...",
    "confidence": 0.84,
    "verification_status": "SUPPORTED",
    "intent_detected": "Question",
    "reasoning_trace": null,
    "state_updated": true,
    "learning_applied": true
  },
  "metadata": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": 1723550400000,
    "duration_ms": 156,
    "state_updated": true,
    "version": "1.0.0"
  }
}
```

| Response Field | Type | Description |
|---|---|---|
| `data.output` | string | Generated response text |
| `data.confidence` | float | Overall confidence [0.0, 1.0] |
| `data.verification_status` | string | Verification status enum |
| `data.intent_detected` | string | Detected input intent |
| `data.reasoning_trace` | object/null | Reasoning details (if requested) |
| `data.state_updated` | bool | Whether cognitive state was mutated |
| `data.learning_applied` | bool | Whether learning was applied |

### 8.2 Batch Inference Endpoint

```
POST /v1/inference/batch
```

**Request:**

```json
{
  "inputs": [
    {"input": "What is gravity?"},
    {"input": "What is electromagnetism?"}
  ],
  "options": {
    "max_tokens": 512,
    "verify": true
  }
}
```

**Constraints:**
- Maximum batch size: 8
- Each input processed sequentially through cognitive pipeline
- Total timeout: 30 seconds per batch

**Response (200):**

```json
{
  "success": true,
  "data": {
    "results": [
      {"output": "...", "confidence": 0.84, "verification_status": "SUPPORTED"},
      {"output": "...", "confidence": 0.79, "verification_status": "SUPPORTED"}
    ],
    "total_processed": 2,
    "state_updated": true
  }
}
```

---

## 9. Context API

### 9.1 Get Context

```
GET /v1/context
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "conversation_id": "session-123",
    "turn_count": 5,
    "active_concepts": ["gravity", "physics", "force"],
    "active_hypotheses": [],
    "tokens_used": 128,
    "context_window_size": 4096,
    "temporal_context": {
      "current_time": 1723550400000,
      "sequence_position": 5
    }
  }
}
```

### 9.2 Reset Context

```
POST /v1/context/reset
```

**Request:**

```json
{
  "scope": "conversation"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `scope` | string | YES | `"conversation"` or `"full"` |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "reset_scope": "conversation",
    "state_updated": true
  }
}
```

---

## 10. Observation API

### 10.1 Submit Observation

```
POST /v1/observe
```

**Request:**

```json
{
  "observation": "The temperature outside is 35 degrees Celsius.",
  "source": "user",
  "kind": "UserInput",
  "importance": 0.5,
  "context": {}
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `observation` | string | YES | Observation text |
| `source` | string | NO | Source identifier (default: "user") |
| `kind` | string | NO | Observation kind (default: "UserInput") |
| `importance` | float | NO | Importance [0.0, 1.0] (default: 0.5) |
| `context` | object | NO | Additional context |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "observation_id": "obs-uuid",
    "stored": true,
    "episode_created": true,
    "state_updated": true
  }
}
```

### 10.2 Get Recent Observations

```
GET /v1/observations?limit=10&offset=0
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `limit` | u32 | 10 | Max results |
| `offset` | u32 | 0 | Pagination offset |
| `kind` | string | all | Filter by kind |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "observations": [
      {
        "id": "obs-uuid",
        "text": "...",
        "kind": "UserInput",
        "timestamp": 1723550400000,
        "importance": 0.5
      }
    ],
    "total": 42,
    "limit": 10,
    "offset": 0
  }
}
```

---

## 11. Memory API

### 11.1 Query Memory

```
POST /v1/memory/query
```

**Request:**

```json
{
  "query_type": "All",
  "text": "gravity",
  "concept_ids": [],
  "time_range": null,
  "max_results": 10,
  "min_confidence": 0.3
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `query_type` | string | YES | `"Semantic"`, `"Episodic"`, `"Procedural"`, `"Associative"`, `"All"` |
| `text` | string | NO | Text query |
| `concept_ids` | array | NO | Filter by concept IDs |
| `time_range` | [u64, u64] | NO | Timestamp range filter |
| `max_results` | u32 | NO | Max results (default: 10) |
| `min_confidence` | float | NO | Minimum confidence filter (default: 0.0) |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "episodic": [
      {
        "id": "ep-001",
        "observation": "...",
        "timestamp": 1723550400000,
        "importance": 0.7,
        "confidence": 0.85,
        "relevance_score": 0.92
      }
    ],
    "semantic": [
      {
        "id": "kn-001",
        "concept": "gravity",
        "properties": [{"name": "type", "value": "fundamental_force"}],
        "confidence": 0.88,
        "verification_status": "SUPPORTED",
        "relevance_score": 0.95,
        "provenance": {
          "category": "UserProvided",
          "timestamp": 1723550400000,
          "source_quality": 0.8
        }
      }
    ],
    "procedural": [],
    "associative": [],
    "total_results": 3,
    "relevance_scores": {"ep-001": 0.92, "kn-001": 0.95}
  }
}
```

### 11.2 Get Memory Statistics

```
GET /v1/memory/stats
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "working": {"active_concepts": 5, "active_hypotheses": 2},
    "episodic": {"count": 150, "capacity_bytes": 536870912, "usage_bytes": 12345678},
    "semantic": {"count": 89, "capacity_bytes": 536870912, "usage_bytes": 8765432},
    "procedural": {"count": 12, "capacity_bytes": 268435456, "usage_bytes": 1234567},
    "associative": {"count": 234, "capacity_bytes": 268435456, "usage_bytes": 4567890},
    "total_usage_bytes": 26913567,
    "pressure": "Low"
  }
}
```

### 11.3 Forget Operation

```
POST /v1/memory/forget
```

**Request:**

```json
{
  "policy": "moderate",
  "target": "episodic"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `policy` | string | YES | `"moderate"`, `"aggressive"`, `"emergency"` |
| `target` | string | NO | `"episodic"`, `"semantic"`, `"associative"`, `"all"` |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "episodic_forgotten": 15,
    "semantic_forgotten": 0,
    "associative_forgotten": 3,
    "bytes_freed": 2048576,
    "state_updated": true
  }
}
```

---

## 12. World Model API

### 12.1 Query World Model

```
POST /v1/world/query
```

**Request:**

```json
{
  "query": "entities",
  "filters": {
    "kind": "Person",
    "name_contains": "Ali"
  },
  "max_results": 10
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "entities": [
      {
        "id": "ent-001",
        "kind": "Person",
        "name": "Ali",
        "properties": [],
        "confidence": 0.9,
        "provenance": {"category": "UserProvided", "timestamp": 1723550400000}
      }
    ],
    "relations": [],
    "total_entities": 1,
    "total_relations": 0
  }
}
```

### 12.2 Get World State

```
GET /v1/world/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "entity_count": 42,
    "relation_count": 78,
    "active_events": 3,
    "uncertainty_level": 0.35,
    "last_updated": 1723550400000
  }
}
```

### 12.3 Predict Transition

```
POST /v1/world/predict
```

**Request:**

```json
{
  "action": {
    "kind": "Respond",
    "parameters": {}
  },
  "horizon": 3
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "predicted_states": [
      {"step": 1, "confidence": 0.7, "uncertainty": 0.3},
      {"step": 2, "confidence": 0.5, "uncertainty": 0.5},
      {"step": 3, "confidence": 0.3, "uncertainty": 0.7}
    ],
    "overall_confidence": 0.5,
    "horizon": 3
  }
}
```

---

## 13. Reasoning API

### 13.1 Submit Reasoning Query

```
POST /v1/reasoning/query
```

**Request:**

```json
{
  "question": "Why does water boil at lower temperature at high altitude?",
  "max_steps": 16,
  "include_hypotheses": true
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "conclusion": {
      "proposition": "Water boils at lower temperature at high altitude because atmospheric pressure is lower",
      "confidence": 0.82,
      "evidence_strength": 0.75,
      "reasoning_steps": 8,
      "bounded": false
    },
    "hypotheses": [
      {
        "id": "hyp-001",
        "proposition": "...",
        "confidence": 0.82,
        "reasoning_type": "Causal",
        "evidence_count": 3,
        "counter_evidence_count": 0
      }
    ],
    "contradictions": [],
    "budget_remaining": 8
  }
}
```

### 13.2 Get Reasoning State

```
GET /v1/reasoning/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "active_hypotheses": 2,
    "budget_remaining": 32,
    "contradiction_count": 0,
    "last_conclusion": null
  }
}
```

---

## 14. Planning API

### 14.1 Request Plan

```
POST /v1/planning/plan
```

**Request:**

```json
{
  "goal": "Explain the water cycle to a student",
  "max_depth": 4,
  "max_branches": 8,
  "include_risk": true
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "plan": {
      "id": "plan-001",
      "goal": "Explain the water cycle to a student",
      "steps": [
        {"action": "Respond", "description": "Define water cycle"},
        {"action": "Respond", "description": "Explain evaporation"},
        {"action": "Respond", "description": "Explain condensation"},
        {"action": "Respond", "description": "Explain precipitation"}
      ],
      "estimated_cost": 0.3,
      "estimated_risk": 0.1,
      "uncertainty": 0.2,
      "confidence": 0.85
    },
    "alternatives_considered": 3,
    "state_updated": false
  }
}
```

### 14.2 Get Planning State

```
GET /v1/planning/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "active_goals": 0,
    "candidate_plans": 0,
    "selected_plan": null,
    "budget_remaining": 8
  }
}
```

---

## 15. Prediction API

### 15.1 Get Current Prediction

```
GET /v1/prediction/current
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "prediction": {
      "target": "NextState",
      "confidence": 0.65,
      "timestamp": 1723550400000,
      "resolved": false
    },
    "recent_predictions": [
      {
        "target": "NextToken",
        "confidence": 0.72,
        "resolved": true,
        "error": 0.15
      }
    ],
    "average_prediction_error": 0.23
  }
}
```

### 15.2 Get Prediction History

```
GET /v1/prediction/history?limit=20
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "predictions": [
      {
        "timestamp": 1723550400000,
        "target": "NextState",
        "confidence": 0.65,
        "resolved": true,
        "error_magnitude": 0.18
      }
    ],
    "total": 150,
    "average_error": 0.23
  }
}
```

---

## 16. Action API

### 16.1 Execute Action

```
POST /v1/action/execute
```

**Request:**

```json
{
  "action": {
    "kind": "Respond",
    "parameters": {
      "text": "The water cycle consists of..."
    }
  },
  "require_policy_check": true
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "action_id": "act-001",
    "executed": true,
    "policy_decision": "Allowed",
    "outcome": {
      "success": true,
      "description": "Response generated",
      "confidence": 0.85
    },
    "state_updated": true
  }
}
```

---

## 17. Verification API

### 17.1 Verify Claim

```
POST /v1/verify
```

**Request:**

```json
{
  "claim": "Water boils at 100 degrees Celsius at standard atmospheric pressure",
  "include_evidence": true
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "claim": "Water boils at 100 degrees Celsius at standard atmospheric pressure",
    "verification_status": "VERIFIED",
    "confidence": {
      "belief": 0.92,
      "evidence_strength": 0.88,
      "source_quality": 0.9,
      "consistency": 0.95,
      "uncertainty": 0.05,
      "prediction_reliability": 0.0
    },
    "evidence": [
      {
        "source": "SemanticMemory",
        "strength": 0.9,
        "polarity": "Supports"
      }
    ],
    "contradictions": [],
    "state_updated": false
  }
}
```

### 17.2 Get Verification State

```
GET /v1/verification/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "pending_claims": 3,
    "verified_claims": 42,
    "contradicted_claims": 2,
    "confidence_threshold": 0.80
  }
}
```

---

## 18. Learning API

### 18.1 Submit Experience

```
POST /v1/experience
```

**Request:**

```json
{
  "observation": "User asked about gravity",
  "action": "Responded with explanation",
  "outcome": "User confirmed understanding",
  "feedback": "positive",
  "prediction_error": 0.12,
  "source": "user"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `observation` | string | YES | What was observed |
| `action` | string | NO | Action taken |
| `outcome` | string | NO | Observed outcome |
| `feedback` | string | NO | Feedback type: `"positive"`, `"negative"`, `"neutral"` |
| `prediction_error` | float | NO | Explicit prediction error |
| `source` | string | NO | Source identifier |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "experience_id": "exp-uuid",
    "learning_applied": true,
    "learning_signal_magnitude": 0.003,
    "attribution": "MemoryError",
    "state_updated": true
  }
}
```

### 18.2 Trigger Learning

```
POST /v1/learn
```

**Request:**

```json
{
  "mode": "full",
  "include_replay": true,
  "include_consolidation": true
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `mode` | string | NO | `"full"`, `"replay_only"`, `"consolidation_only"` |
| `include_replay` | bool | NO | Include replay (default: true) |
| `include_consolidation` | bool | NO | Include consolidation (default: true) |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "learning_events": 15,
    "replay_events": 5,
    "consolidation_events": 2,
    "average_error_reduction": 0.08,
    "state_updated": true
  }
}
```

### 18.3 Get Learning State

```
GET /v1/learning/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "enabled": true,
    "total_learning_events": 1523,
    "total_replay_events": 342,
    "total_consolidation_events": 89,
    "average_prediction_error": 0.23,
    "learning_rate": 0.001,
    "plasticity_rate": 0.01,
    "next_consolidation_at": 2000
  }
}
```

---

## 19. Self Model API

### 19.1 Get Self Model

```
GET /v1/self-model
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "capabilities": {
      "language_accuracy": 0.78,
      "prediction_accuracy": 0.72,
      "verification_reliability": 0.81,
      "planning_success": 0.65,
      "memory_retrieval_success": 0.85,
      "reasoning_consistency": 0.79,
      "resource_availability": 0.9
    },
    "limitations": {
      "known_limitations": ["Limited domain knowledge in quantum physics"],
      "resource_constraints": ["Memory pressure: Low"],
      "capability_gaps": []
    },
    "prediction_accuracy": 0.72,
    "uncertainty_level": 0.28,
    "memory_health": {
      "pressure": "Low",
      "fragmentation": 0.1,
      "consolidation_backlog": 5
    },
    "last_updated": 1723550400000
  }
}
```

### 19.2 Get Capability Estimate

```
GET /v1/self-model/capability/{capability}
```

| Parameter | Values |
|---|---|
| `capability` | `"language"`, `"prediction"`, `"verification"`, `"planning"`, `"memory"`, `"reasoning"` |

**Response (200):**

```json
{
  "success": true,
  "data": {
    "capability": "prediction",
    "accuracy": 0.72,
    "confidence": 0.8,
    "sample_size": 1523,
    "trend": "improving"
  }
}
```

---

## 20. Policy API

### 20.1 Get Policy State

```
GET /v1/policy
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "learning_enabled": true,
    "internet_learning_enabled": true,
    "self_modification_allowed": false,
    "policy_modification_allowed": false,
    "runtime_modification_allowed": false,
    "risk_thresholds": {
      "auto_approve_below": 0.3,
      "limit_above": 0.6,
      "deny_above": 0.8
    }
  }
}
```

### 20.2 Evaluate Operation

```
POST /v1/policy/evaluate
```

**Request:**

```json
{
  "operation": {
    "classification": "CognitiveStateAdaptation",
    "description": "Update semantic memory with new knowledge",
    "target": "semantic_memory",
    "estimated_impact": 0.2,
    "reversibility": 0.8
  }
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "decision": "Allowed",
    "risk_estimate": {
      "score": 0.15,
      "level": "Low"
    },
    "constraints": null,
    "reason": null
  }
}
```

---

## 21. Internet Interface API

### 21.1 Fetch URL

```
POST /v1/internet/fetch
```

**Request:**

```json
{
  "url": "https://example.com/article",
  "method": "GET",
  "timeout_seconds": 10,
  "max_response_mb": 2
}
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "observation_id": "obs-uuid",
    "content_extracted": true,
    "content_length": 15234,
    "provenance": {
      "category": "Internet",
      "source_url": "https://example.com/article",
      "timestamp": 1723550400000,
      "verification_status": "UNKNOWN"
    },
    "stored_as_observation": true,
    "state_updated": true
  }
}
```

### 21.2 Get Internet State

```
GET /v1/internet/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "enabled": true,
    "total_requests": 42,
    "successful_requests": 38,
    "failed_requests": 4,
    "total_bytes_received": 1234567,
    "last_request": 1723550400000
  }
}
```

---

## 22. Persistence API

### 22.1 Create Checkpoint

```
POST /v1/checkpoint
```

**Request:** (empty body)

**Response (201):**

```json
{
  "success": true,
  "data": {
    "checkpoint_id": 42,
    "timestamp": 1723550400000,
    "file_size_bytes": 5242880,
    "episode_count": 150,
    "state_version": 1,
    "integrity_verified": true
  }
}
```

### 22.2 Get Checkpoint List

```
GET /v1/checkpoints
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "checkpoints": [
      {
        "id": 42,
        "timestamp": 1723550400000,
        "file_size_bytes": 5242880,
        "episode_count": 150
      },
      {
        "id": 41,
        "timestamp": 1723540000000,
        "file_size_bytes": 5100000,
        "episode_count": 140
      }
    ],
    "total": 2,
    "max_checkpoints": 10
  }
}
```

### 22.3 Validate State

```
POST /v1/persistence/validate
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "valid": true,
    "format_version": 1,
    "architecture_version": 1,
    "state_id": "uuid",
    "errors": [],
    "warnings": []
  }
}
```

---

## 23. State API

### 23.1 Get State Summary

```
GET /v1/state
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "state_id": "550e8400-e29b-41d4-a716-446655440000",
    "created_at": 1723450000000,
    "last_updated": 1723550400000,
    "architecture_version": 1,
    "episode_count": 150,
    "total_learning_events": 1523,
    "checkpoint_count": 5,
    "subsystems": {
      "language": {"enabled": true, "vocabulary_size": 2048},
      "neural": {"enabled": true, "active_cells": 205, "active_columns": 12},
      "memory": {"enabled": true, "total_items": 485},
      "world": {"enabled": true, "entities": 42, "relations": 78},
      "reasoning": {"enabled": true},
      "planning": {"enabled": true},
      "verification": {"enabled": true},
      "learning": {"enabled": true},
      "internet": {"enabled": true}
    }
  }
}
```

### 23.2 Inspect State

```
GET /v1/state/inspect?section=memory
```

| Parameter | Values |
|---|---|
| `section` | `"language"`, `"neural"`, `"memory"`, `"world"`, `"reasoning"`, `"planning"`, `"verification"`, `"learning"`, `"self_model"`, `"metadata"` |

**Response (200):** Section-specific detailed state.

---

## 24. Configuration API

### 24.1 Get Configuration (Read-Only)

```
GET /v1/config
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "model": {
      "cells": 4096,
      "columns": 64,
      "dimension": 256,
      "precision": "f32",
      "sparsity_ratio": 0.05
    },
    "language": {
      "enabled": true,
      "vocabulary_capacity": 65536,
      "context_window": 4096,
      "generation_limit": 1024
    },
    "memory": {
      "working_mb": 128,
      "episodic_mb": 512,
      "semantic_mb": 512,
      "procedural_mb": 256,
      "associative_mb": 256
    },
    "learning": {
      "enabled": true,
      "learning_rate": 0.001,
      "plasticity": 0.01
    },
    "api": {
      "enabled": true,
      "bind": "127.0.0.1:8080"
    }
  }
}
```

**Security Note:** Configuration API is READ-ONLY. Configuration changes require editing `cortex.toml` and restarting. API key environment variable name is shown but its value is NEVER exposed.

---

## 25. Health/Status API

### 25.1 Get Status

```
GET /v1/status
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "status": "ready",
    "uptime_seconds": 3600,
    "runtime_state": "Ready",
    "memory_usage": {
      "total_bytes": 26913567,
      "pressure": "Low"
    },
    "episode_count": 150,
    "prediction_error": 0.23,
    "learning_enabled": true,
    "world_model_size": 42,
    "language_vocabulary_size": 2048,
    "checkpoint_count": 5,
    "last_checkpoint": 1723550400000
  }
}
```

### 25.2 Health Check

```
GET /v1/health
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "healthy": true,
    "checks": {
      "state_valid": true,
      "memory_within_budget": true,
      "persistence_operational": true,
      "language_operational": true,
      "neural_operational": true,
      "policy_operational": true
    },
    "timestamp": 1723550400000
  }
}
```

---

## 26. Observability API

### 26.1 Get Metrics

```
GET /v1/metrics
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "prediction_error": {
      "current": 0.23,
      "average": 0.25,
      "trend": "decreasing"
    },
    "memory_retrieval_success": 0.85,
    "knowledge_stability": 0.92,
    "verification_confidence": 0.78,
    "reasoning_consistency": 0.79,
    "planning_success": 0.65,
    "language_prediction_quality": 0.72,
    "learning_rate_effective": 0.001,
    "forgetting_rate": 0.02,
    "consolidation_rate": 0.05
  }
}
```

### 26.2 Get Diagnostics

```
GET /v1/diagnostics
```

**Response (200):**

```json
{
  "success": true,
  "data": {
    "last_errors": [
      {
        "kind": "NetworkError",
        "message": "Timeout fetching URL",
        "timestamp": 1723550000000,
        "severity": "Recoverable"
      }
    ],
    "error_frequency": {
      "NetworkError": 3,
      "InputError": 1
    },
    "subsystem_errors": {
      "internet": 3,
      "input": 1
    },
    "total_errors": 4,
    "recovery_actions": 3
  }
}
```

---

## 27. CLI Architecture

### 27.1 CLI Design Model

```
cortex [COMMAND] [SUBCOMMAND] [ARGUMENTS] [OPTIONS]
```

### 27.2 CLI Execution Model

| Property | Value |
|---|---|
| Binary | `cortex` (same as API server) |
| Invocation | `cortex {command} {args} {options}` |
| Output format | Human-readable (default) or JSON (`--json`) |
| Exit codes | Standard (see §33) |
| Configuration | Reads `cortex.toml` from CWD or `--config` path |
| State | Reads/writes `cortex.cx` from configured path |

### 27.3 CLI Modes

| Mode | Command | Behavior |
|---|---|---|
| Interactive | `cortex run` | Continuous cognitive loop; reads stdin |
| Server | `cortex serve` | Starts API server |
| Single-shot | `cortex observe`, `cortex query`, etc. | Single operation; exits |
| Administrative | `cortex init`, `cortex migrate` | State management |

---

## 28. CLI Commands

### 28.1 Command Reference

| Command | Description | Mutates State | Requires Config |
|---|---|---|---|
| `cortex run` | Start interactive cognitive runtime | YES | YES |
| `cortex serve` | Start API server | YES | YES |
| `cortex observe <text>` | Submit observation | YES | YES |
| `cortex experience <json>` | Submit learning experience | YES | YES |
| `cortex learn` | Trigger learning cycle | YES | YES |
| `cortex query <text>` | Query cognitive state | NO | YES |
| `cortex inspect [section]` | Inspect state | NO | YES |
| `cortex verify <claim>` | Verify a claim | NO | YES |
| `cortex checkpoint` | Create checkpoint | YES | YES |
| `cortex status` | Show runtime status | NO | YES |
| `cortex init` | Initialize new state | YES | YES |
| `cortex migrate` | Migrate state format | YES | YES |
| `cortex help` | Show help | NO | NO |
| `cortex version` | Show version | NO | NO |

### 28.2 Command Details

#### `cortex run`

```bash
cortex run [OPTIONS]
```

Starts the interactive cognitive runtime. Reads input from stdin, processes through full cognitive pipeline, outputs response.

**Options:**
| Option | Description |
|---|---|
| `--config <path>` | Path to cortex.toml |
| `--json` | Output responses as JSON |
| `--quiet` | Suppress non-response output |
| `--max-turns <n>` | Exit after n turns (for scripting) |

**Example:**
```bash
$ cortex run
CORTEX v1.0.0 | State: cortex.cx | Ready
> What is gravity?
Gravity is a fundamental force of attraction between objects with mass...
> Explain further.
[Response...]
> ^C
Graceful shutdown. State saved.
```

#### `cortex serve`

```bash
cortex serve [OPTIONS]
```

Starts the embedded API server.

**Options:**
| Option | Description |
|---|---|
| `--config <path>` | Path to cortex.toml |
| `--bind <addr>` | Override bind address |
| `--port <port>` | Override port |

**Example:**
```bash
$ CORTEX_API_KEY="secret" cortex serve
CORTEX API server listening on 127.0.0.1:8080
```

#### `cortex observe`

```bash
cortex observe <text> [OPTIONS]
```

Submits an observation without generating a response.

**Options:**
| Option | Description |
|---|---|
| `--source <source>` | Source identifier |
| `--importance <float>` | Importance [0.0, 1.0] |
| `--json` | JSON output |

**Example:**
```bash
$ cortex observe "The temperature is 35 degrees today" --importance 0.6
Observation stored. Episode created. State updated.
```

#### `cortex experience`

```bash
cortex experience <json> [OPTIONS]
cortex experience --file <path> [OPTIONS]
```

Submits a learning experience.

**Options:**
| Option | Description |
|---|---|
| `--file <path>` | Read experience JSON from file |
| `--json` | JSON output |

**Example:**
```bash
$ cortex experience '{"observation":"User asked about gravity","outcome":"User confirmed","feedback":"positive"}'
Experience recorded. Learning applied. State updated.
```

#### `cortex learn`

```bash
cortex learn [OPTIONS]
```

Triggers a learning cycle.

**Options:**
| Option | Description |
|---|---|
| `--replay` | Include replay (default: true) |
| `--consolidation` | Include consolidation (default: true) |
| `--json` | JSON output |

**Example:**
```bash
$ cortex learn
Learning cycle complete: 15 events, 5 replays, 2 consolidations.
```

#### `cortex query`

```bash
cortex query <text> [OPTIONS]
```

Queries cognitive state.

**Options:**
| Option | Description |
|---|---|
| `--target <target>` | Query target: `memory`, `world`, `knowledge`, `episodes`, `procedures` |
| `--max-results <n>` | Max results (default: 10) |
| `--min-confidence <float>` | Minimum confidence filter |
| `--json` | JSON output |

**Example:**
```bash
$ cortex query "gravity" --target memory --max-results 5
Found 3 relevant memories:
  [0.95] Knowledge: gravity is a fundamental force (confidence: 0.88, SUPPORTED)
  [0.82] Episode: User asked about gravity (2024-01-15)
  [0.71] Association: gravity → physics (strength: 0.6)
```

#### `cortex inspect`

```bash
cortex inspect [SECTION] [OPTIONS]
```

Inspects internal state.

**Arguments:**
| Section | Description |
|---|---|
| `(none)` | Full state summary |
| `language` | Language core state |
| `neural` | Neural core state |
| `memory` | Memory system state |
| `world` | World model state |
| `reasoning` | Reasoning state |
| `learning` | Learning state |
| `self-model` | Self model state |
| `policy` | Policy state |
| `metadata` | State metadata |

**Example:**
```bash
$ cortex inspect memory
Memory System Status:
  Working:    5 active concepts, 2 hypotheses
  Episodic:   150 episodes (12.3 MB / 512 MB)
  Semantic:   89 knowledge items (8.7 MB / 512 MB)
  Procedural: 12 procedures (1.2 MB / 256 MB)
  Associative: 234 associations (4.5 MB / 256 MB)
  Pressure: Low
```

#### `cortex verify`

```bash
cortex verify <claim> [OPTIONS]
```

Verifies a claim.

**Options:**
| Option | Description |
|---|---|
| `--include-evidence` | Include evidence details |
| `--json` | JSON output |

**Example:**
```bash
$ cortex verify "Water boils at 100°C at standard pressure"
Claim: "Water boils at 100°C at standard pressure"
Status: VERIFIED
Confidence: 0.92
Evidence: 3 supporting, 0 contradicting
```

#### `cortex checkpoint`

```bash
cortex checkpoint [OPTIONS]
```

Creates a manual checkpoint.

**Example:**
```bash
$ cortex checkpoint
Checkpoint #42 created. Size: 5.2 MB. Episodes: 150.
```

#### `cortex status`

```bash
cortex status [OPTIONS]
```

Shows runtime status.

**Options:**
| Option | Description |
|---|---|
| `--json` | JSON output |
| `--verbose` | Detailed status |

**Example:**
```bash
$ cortex status
CORTEX v1.0.0
State: cortex.cx (valid)
Uptime: 1h 23m
Episodes: 150
Vocabulary: 2048 symbols
Prediction Error: 0.23
Learning: enabled
Memory Pressure: Low
Checkpoints: 5
```

#### `cortex init`

```bash
cortex init [OPTIONS]
```

Initializes a new cognitive state. **WARNING: Overwrites existing state.**

**Options:**
| Option | Description |
|---|---|
| `--force` | Overwrite without confirmation |
| `--config <path>` | Path to cortex.toml |

**Example:**
```bash
$ cortex init
WARNING: This will overwrite existing cortex.cx. Continue? [y/N] y
New state initialized. State ID: 550e8400-...
```

#### `cortex migrate`

```bash
cortex migrate [OPTIONS]
```

Migrates state format to current version.

**Options:**
| Option | Description |
|---|---|
| `--dry-run` | Show what would be migrated without applying |
| `--json` | JSON output |

**Example:**
```bash
$ cortex migrate --dry-run
State version: 1 → 2
Migration steps: 1
  - Add provenance section
Ready to migrate. Run without --dry-run to apply.
```

---

## 29. CLI Arguments

### 29.1 Global Arguments

| Argument | Applies To | Description |
|---|---|---|
| `<text>` | observe, query, verify | Positional text argument |
| `<json>` | experience | Positional JSON argument |
| `[section]` | inspect | Optional section name |

### 29.2 Argument Validation

| Rule | Description |
|---|---|
| ARG-001 | Text arguments are UTF-8 |
| ARG-002 | JSON arguments must be valid JSON |
| ARG-003 | Missing required arguments produce usage error |
| ARG-004 | Unknown arguments produce error with suggestion |
| ARG-005 | Maximum text argument length: 65536 characters |

---

## 30. CLI Options

### 30.1 Global Options

| Option | Short | Description | Default |
|---|---|---|---|
| `--config <path>` | `-c` | Path to cortex.toml | `./cortex.toml` |
| `--json` | `-j` | Output as JSON | false |
| `--quiet` | `-q` | Suppress non-essential output | false |
| `--verbose` | `-v` | Verbose output | false |
| `--help` | `-h` | Show help | N/A |
| `--version` | `-V` | Show version | N/A |

### 30.2 Command-Specific Options

See §28.2 for per-command options.

### 30.3 Option Rules

| Rule | Description |
|---|---|
| OPT-001 | Options are case-sensitive |
| OPT-002 | Short options use single dash: `-j` |
| OPT-003 | Long options use double dash: `--json` |
| OPT-004 | Options with values use `=` or space: `--config=path` or `--config path` |
| OPT-005 | Boolean options are flags: `--json` (no value needed) |
| OPT-006 | Unknown options produce error |
| OPT-007 | Conflicting options produce error |

---

## 31. CLI Output

### 31.1 Human-Readable Output (Default)

```
CORTEX v1.0.0 | State: cortex.cx | Ready
> What is gravity?
Gravity is a fundamental force of attraction between objects with mass.
It was described by Newton and refined by Einstein's general relativity.
Confidence: 0.84 | Status: SUPPORTED
```

### 31.2 JSON Output (`--json`)

```json
{
  "success": true,
  "data": {
    "output": "Gravity is a fundamental force...",
    "confidence": 0.84,
    "verification_status": "SUPPORTED",
    "state_updated": true
  },
  "metadata": {
    "timestamp": 1723550400000,
    "duration_ms": 156,
    "version": "1.0.0"
  }
}
```

### 31.3 Output Rules

| Rule | Description |
|---|---|
| OUT-001 | Default output is human-readable text |
| OUT-002 | `--json` flag produces structured JSON |
| OUT-003 | JSON output goes to stdout |
| OUT-004 | Errors go to stderr |
| OUT-005 | Progress/status messages go to stderr (not stdout) |
| OUT-006 | `--quiet` suppresses all non-response output |
| OUT-007 | `--verbose` adds diagnostic information to stderr |

---

## 32. Exit Codes

### 32.1 Exit Code Registry

| Code | Meaning | Description |
|---|---|---|
| 0 | Success | Operation completed successfully |
| 1 | General error | Unspecified error |
| 2 | Usage error | Invalid arguments or options |
| 3 | Configuration error | Invalid or missing cortex.toml |
| 4 | State error | Invalid or corrupt cortex.cx |
| 5 | Authentication error | Invalid API key (serve mode) |
| 6 | Policy error | Operation denied by policy |
| 7 | Resource error | Resource exhaustion |
| 8 | Network error | Network operation failed |
| 9 | Persistence error | Save/load failure |
| 10 | Migration error | State migration failed |
| 11 | Timeout | Operation timed out |
| 12 | Interrupted | User interrupted (SIGINT) |
| 13 | Fatal error | Unrecoverable error |

### 32.2 Exit Code Rules

| Rule | Description |
|---|---|
| EXIT-001 | Exit code 0 ONLY for complete success |
| EXIT-002 | Non-zero exit code for any failure |
| EXIT-003 | Exit codes are deterministic for same error condition |
| EXIT-004 | SIGTERM triggers graceful shutdown (exit 0 if state saved) |
| EXIT-005 | SIGINT triggers graceful shutdown (exit 12) |
| EXIT-006 | SIGKILL cannot be handled (OS-level) |

---

## 33. Configuration Files

### 33.1 Configuration File Discovery

```
Search order:
1. --config <path> (explicit)
2. ./cortex.toml (current directory)
3. /opt/cortex/cortex.toml (default install path)
4. Error: configuration not found
```

### 33.2 Configuration File Format

TOML format as defined in CORTEX-DOC-01 §5.1.

### 33.3 Configuration Validation at CLI Level

| Check | Error Code |
|---|---|
| File exists | 3 |
| Valid TOML syntax | 3 |
| Schema validation | 3 |
| Range validation | 3 |
| Dependency validation | 3 |
| Policy validation | 3 |

---

## 34. Environment Variables

### 34.1 Environment Variable Registry

| Variable | Required | Description |
|---|---|---|
| `CORTEX_API_KEY` | YES (if API enabled) | API authentication token |
| `CORTEX_CONFIG` | NO | Override config file path |
| `CORTEX_STATE` | NO | Override state file path |
| `CORTEX_LOG_LEVEL` | NO | Log level: `error`, `warn`, `info`, `debug`, `trace` |
| `CORTEX_LOG_FILE` | NO | Log file path (default: stderr) |

### 34.2 Environment Variable Rules

| Rule | Description |
|---|---|
| ENV-001 | `CORTEX_API_KEY` is NEVER logged |
| ENV-002 | `CORTEX_API_KEY` is NEVER persisted in `.cx` |
| ENV-003 | `CORTEX_API_KEY` is NEVER included in API responses |
| ENV-004 | Missing `CORTEX_API_KEY` when API enabled prevents server start |
| ENV-005 | Environment variables override config file values |
| ENV-006 | Invalid environment variable values produce startup error |

### 34.3 Environment Variable Priority

```
Priority (highest to lowest):
1. CLI arguments (--config, --bind, etc.)
2. Environment variables (CORTEX_CONFIG, CORTEX_STATE, etc.)
3. Configuration file (cortex.toml)
4. Built-in defaults
```

---

## 35. Machine-Readable Output

### 35.1 JSON Output Schema

All `--json` output follows this schema:

```json
{
  "success": true|false,
  "data": { ... },
  "error": { ... },
  "metadata": {
    "timestamp": 1723550400000,
    "duration_ms": 42,
    "version": "1.0.0",
    "command": "query",
    "state_updated": true|false
  }
}
```

### 35.2 Machine-Readable Rules

| Rule | Description |
|---|---|
| MR-001 | JSON output is valid JSON (parseable by standard parsers) |
| MR-002 | JSON output is a single object (not array) |
| MR-003 | All fields are present (null if not applicable) |
| MR-004 | Timestamps are u64 milliseconds since epoch |
| MR-005 | Durations are u64 milliseconds |
| MR-006 | Sizes are u64 bytes |
| MR-007 | Confidence values are float [0.0, 1.0] |
| MR-008 | Enum values are strings (e.g., `"VERIFIED"`, not integer) |

### 35.3 Structured Output for Scripting

```bash
# Parse confidence from query result
CONFIDENCE=$(cortex query "gravity" --json | jq '.data.confidence')

# Check if learning is enabled
LEARNING=$(cortex status --json | jq '.data.learning_enabled')

# Get episode count
EPISODES=$(cortex status --json | jq '.data.episode_count')
```

---

## 36. Compatibility

### 36.1 API Compatibility

| Aspect | Guarantee |
|---|---|
| Endpoint paths | Stable within major version |
| Request fields | Additive only within major version |
| Response fields | Additive only within major version |
| Error codes | Stable; new codes may be added |
| Authentication | Stable within major version |
| HTTP methods | Stable within major version |

### 36.2 CLI Compatibility

| Aspect | Guarantee |
|---|---|
| Command names | Stable within major version |
| Command arguments | Additive only within major version |
| Command options | Additive only within major version |
| Exit codes | Stable within major version |
| Output format | Stable within major version |
| JSON schema | Additive only within major version |

### 36.3 Breaking Change Policy

| Change Type | Requires |
|---|---|
| New endpoint | Same version |
| New optional request field | Same version |
| New response field | Same version |
| New error code | Same version |
| Removed endpoint | New major version |
| Renamed field | New major version |
| Changed field type | New major version |
| Changed semantics | New major version |
| Removed command | New major version |
| Changed exit code meaning | New major version |

---

## 37. API Limits

### 37.1 Rate Limits

| Endpoint Category | Rate Limit | Window |
|---|---|---|
| Inference | 10 requests | 1 second |
| Observation | 20 requests | 1 second |
| Query | 50 requests | 1 second |
| Learning | 5 requests | 1 second |
| Checkpoint | 1 request | 10 seconds |
| Status/Health | 100 requests | 1 second |

### 37.2 Size Limits

| Resource | Limit |
|---|---|
| Request body | 1 MB |
| Response body | 10 MB |
| Input text | 65536 characters |
| Batch size | 8 requests |
| Query max_results | 100 |
| Observation text | 65536 characters |
| Experience JSON | 65536 characters |
| Claim text | 4096 characters |

### 37.3 Timeout Limits

| Operation | Timeout |
|---|---|
| Request processing | 30 seconds |
| Internet fetch | `internet.timeout_seconds` (default: 15) |
| Checkpoint creation | 60 seconds |
| State load | 30 seconds |
| Learning cycle | 60 seconds |

### 37.4 Concurrent Connection Limits

| Property | Limit |
|---|---|
| Max concurrent connections | 8 |
| Max queued connections | 32 |
| Connection timeout (idle) | 60 seconds |

### 37.5 Limit Enforcement

| Rule | Description |
|---|---|
| LIM-001 | Rate limit exceeded returns 429 with `Retry-After` header |
| LIM-002 | Size limit exceeded returns 400 |
| LIM-003 | Timeout returns 504 |
| LIM-004 | Connection limit returns 503 |
| LIM-005 | Limits are enforced per connection, not globally |

---

## 38. API/CLI Error Handling

### 38.1 Error Handling Pipeline

```
Error Occurs
    │
    ↓
┌─────────────────────────┐
│ Classify error kind      │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│ Map to error code        │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│ Determine HTTP status    │
│ (API) or exit code (CLI) │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│ Construct error response │
└────────────┬────────────┘
             │
             ├── API: Return JSON error response
             │
             └── CLI: Print to stderr, set exit code
```

### 38.2 Error Recovery Guidance

| Error Kind | Client Action |
|---|---|
| InputError | Fix input format/content |
| AuthenticationError | Provide valid API key |
| AuthorizationError | Operation not permitted; check policy |
| ValidationError | Fix request fields |
| ResourceError | Retry later; reduce request size |
| NetworkError | Retry; check network connectivity |
| TimeoutError | Retry with longer timeout or simpler query |
| PersistenceError | Check disk space; verify state file |
| SubsystemDisabled | Enable subsystem in configuration |
| RuntimeError | Restart CORTEX |

### 38.3 Error Response Examples

**API Error:**
```json
{
  "success": false,
  "error": {
    "code": "CORTEX_ERR_004",
    "kind": "AuthenticationError",
    "message": "Invalid or missing API key",
    "details": null,
    "recoverable": true,
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**CLI Error:**
```
$ cortex query "gravity"
Error: Configuration file not found: cortex.toml
Hint: Provide --config <path> or create cortex.toml in current directory.
Exit code: 3
```

---

## 39. Examples

### 39.1 Complete API Interaction Example

```bash
# 1. Start server
$ CORTEX_API_KEY="my-secret-key" cortex serve &
CORTEX API server listening on 127.0.0.1:8080

# 2. Submit observation
$ curl -X POST http://127.0.0.1:8080/v1/observe \
  -H "Authorization: Bearer my-secret-key" \
  -H "Content-Type: application/json" \
  -d '{"observation": "Water boils at 100°C at sea level", "source": "user"}'
{"success":true,"data":{"observation_id":"obs-001","stored":true,...}}

# 3. Query
$ curl -X POST http://127.0.0.1:8080/v1/inference \
  -H "Authorization: Bearer my-secret-key" \
  -H "Content-Type: application/json" \
  -d '{"input": "What temperature does water boil at?", "options": {"verify": true}}'
{"success":true,"data":{"output":"Water boils at approximately 100°C...","confidence":0.88,...}}

# 4. Verify claim
$ curl -X POST http://127.0.0.1:8080/v1/verify \
  -H "Authorization: Bearer my-secret-key" \
  -H "Content-Type: application/json" \
  -d '{"claim": "Water boils at 100°C at standard pressure"}'
{"success":true,"data":{"verification_status":"VERIFIED","confidence":{...}}}

# 5. Check status
$ curl http://127.0.0.1:8080/v1/status \
  -H "Authorization: Bearer my-secret-key"
{"success":true,"data":{"status":"ready","episode_count":1,...}}

# 6. Create checkpoint
$ curl -X POST http://127.0.0.1:8080/v1/checkpoint \
  -H "Authorization: Bearer my-secret-key"
{"success":true,"data":{"checkpoint_id":1,...}}
```

### 39.2 Complete CLI Interaction Example

```bash
# 1. Initialize new state
$ cortex init
New state initialized. State ID: 550e8400-e29b-41d4-a716-446655440000

# 2. Submit observations
$ cortex observe "The Earth orbits the Sun" --importance 0.8
Observation stored. Episode created.

$ cortex observe "The Moon orbits the Earth" --importance 0.7
Observation stored. Episode created.

# 3. Query
$ cortex query "What orbits the Sun?"
Found 1 relevant memory:
  [0.92] Knowledge: Earth orbits the Sun (confidence: 0.85, SUPPORTED)

# 4. Verify
$ cortex verify "The Earth orbits the Sun"
Claim: "The Earth orbits the Sun"
Status: SUPPORTED
Confidence: 0.85
Evidence: 1 supporting, 0 contradicting

# 5. Trigger learning
$ cortex learn
Learning cycle complete: 2 events, 0 replays, 0 consolidations.

# 6. Check status
$ cortex status
CORTEX v1.0.0
State: cortex.cx (valid)
Episodes: 2
Vocabulary: 15 symbols
Learning: enabled
Memory Pressure: Low

# 7. Create checkpoint
$ cortex checkpoint
Checkpoint #1 created. Size: 1.2 KB. Episodes: 2.

# 8. Inspect state
$ cortex inspect memory
Memory System Status:
  Working:    0 active concepts, 0 hypotheses
  Episodic:   2 episodes (0.5 KB / 512 MB)
  Semantic:   0 knowledge items (0 KB / 512 MB)
  Procedural: 0 procedures (0 KB / 256 MB)
  Associative: 0 associations (0 KB / 256 MB)
  Pressure: Low

# 9. JSON output for scripting
$ cortex status --json | jq '.data.episode_count'
2
```

### 39.3 Error Handling Examples

```bash
# Missing API key
$ curl http://127.0.0.1:8080/v1/status
{"success":false,"error":{"code":"CORTEX_ERR_004","kind":"AuthenticationError",...}}

# Invalid input
$ curl -X POST http://127.0.0.1:8080/v1/inference \
  -H "Authorization: Bearer my-secret-key" \
  -H "Content-Type: application/json" \
  -d '{"input": ""}'
{"success":false,"error":{"code":"CORTEX_ERR_001","kind":"InputError","message":"Empty input",...}}

# Policy denial
$ curl -X POST http://127.0.0.1:8080/v1/learn \
  -H "Authorization: Bearer my-secret-key" \
  -H "Content-Type: application/json" \
  -d '{}'
{"success":false,"error":{"code":"CORTEX_ERR_016","kind":"PolicyError","message":"Learning disabled by policy",...}}

# CLI configuration error
$ cortex status
Error: Configuration file not found: cortex.toml
Exit code: 3

# CLI state corruption
$ cortex status
Error: State file corrupt: checksum mismatch
Hint: Run 'cortex migrate' or restore from checkpoint.
Exit code: 4
```

---

## 40. Interface Contracts

### 40.1 API Contract Summary

| Endpoint | Method | Auth | Mutates | Policy | Bounded |
|---|---|---|---|---|---|
| `/v1/inference` | POST | YES | YES | None | generation_limit |
| `/v1/inference/batch` | POST | YES | YES | None | batch × generation_limit |
| `/v1/context` | GET | YES | NO | None | N/A |
| `/v1/context/reset` | POST | YES | YES | None | N/A |
| `/v1/observe` | POST | YES | YES | None | context_window |
| `/v1/observations` | GET | YES | NO | None | max_results |
| `/v1/memory/query` | POST | YES | NO | None | max_results |
| `/v1/memory/stats` | GET | YES | NO | None | N/A |
| `/v1/memory/forget` | POST | YES | YES | Policy | N/A |
| `/v1/world/query` | POST | YES | NO | None | max_results |
| `/v1/world/state` | GET | YES | NO | None | N/A |
| `/v1/world/predict` | POST | YES | NO | None | prediction_horizon |
| `/v1/reasoning/query` | POST | YES | NO | None | max_steps |
| `/v1/reasoning/state` | GET | YES | NO | None | N/A |
| `/v1/planning/plan` | POST | YES | NO | None | max_depth × max_branches |
| `/v1/planning/state` | GET | YES | NO | None | N/A |
| `/v1/prediction/current` | GET | YES | NO | None | N/A |
| `/v1/prediction/history` | GET | YES | NO | None | limit |
| `/v1/action/execute` | POST | YES | YES | Policy | N/A |
| `/v1/verify` | POST | YES | NO | None | N/A |
| `/v1/verification/state` | GET | YES | NO | None | N/A |
| `/v1/experience` | POST | YES | YES | Policy | N/A |
| `/v1/learn` | POST | YES | YES | Policy | consolidation_interval |
| `/v1/learning/state` | GET | YES | NO | None | N/A |
| `/v1/self-model` | GET | YES | NO | None | N/A |
| `/v1/self-model/capability/{cap}` | GET | YES | NO | None | N/A |
| `/v1/policy` | GET | YES | NO | None | N/A |
| `/v1/policy/evaluate` | POST | YES | NO | None | N/A |
| `/v1/internet/fetch` | POST | YES | YES | Policy | timeout, max_response |
| `/v1/internet/state` | GET | YES | NO | None | N/A |
| `/v1/checkpoint` | POST | YES | YES | None | N/A |
| `/v1/checkpoints` | GET | YES | NO | None | N/A |
| `/v1/persistence/validate` | POST | YES | NO | None | N/A |
| `/v1/state` | GET | YES | NO | None | N/A |
| `/v1/state/inspect` | GET | YES | NO | None | N/A |
| `/v1/config` | GET | YES | NO | None | N/A |
| `/v1/status` | GET | YES | NO | None | N/A |
| `/v1/health` | GET | YES | NO | None | N/A |
| `/v1/metrics` | GET | YES | NO | None | N/A |
| `/v1/diagnostics` | GET | YES | NO | None | N/A |

### 40.2 CLI Contract Summary

| Command | Mutates | Policy | Bounded | Exit Codes |
|---|---|---|---|---|
| `run` | YES | None | context_window | 0, 1, 3, 4, 12 |
| `serve` | YES | None | N/A | 0, 1, 3, 4, 5 |
| `observe` | YES | None | context_window | 0, 1, 2, 3, 4 |
| `experience` | YES | Policy | N/A | 0, 1, 2, 3, 4, 6 |
| `learn` | YES | Policy | consolidation | 0, 1, 3, 4, 6 |
| `query` | NO | None | max_results | 0, 1, 2, 3, 4 |
| `inspect` | NO | None | N/A | 0, 1, 2, 3, 4 |
| `verify` | NO | None | N/A | 0, 1, 2, 3, 4 |
| `checkpoint` | YES | None | N/A | 0, 1, 3, 4, 9 |
| `status` | NO | None | N/A | 0, 1, 3, 4 |
| `init` | YES | None | N/A | 0, 1, 3 |
| `migrate` | YES | None | N/A | 0, 1, 3, 4, 10 |

---

## 41. Open Technical Parameters

| Parameter | Current Value | Open Question | Resolution Path |
|---|---|---|---|
| Max request body | 1 MB | Sufficient for all use cases? | Operational evaluation |
| Max response body | 10 MB | Should this be configurable? | Operational evaluation |
| Request timeout | 30 seconds | Appropriate for complex queries? | Performance testing |
| Concurrent connections | 8 | Sufficient for target deployment? | Load testing |
| Rate limits | Per-endpoint | Should limits be configurable? | Operational evaluation |
| Batch size limit | 8 | Optimal batch size? | Performance testing |
| Max query results | 100 | Sufficient for retrieval? | Retrieval quality evaluation |
| CLI output format | Text/JSON | Should we support YAML, table? | User feedback |
| API TLS | Not included | Should CORTEX include TLS? | Security evaluation |
| WebSocket support | Not included | Should streaming be supported? | Use case evaluation |
| GraphQL support | Not included | Would GraphQL be beneficial? | Use case evaluation |
| API pagination | Offset-based | Should we use cursor-based? | Scale evaluation |
| CLI interactive mode | Line-based | Should we support rich TUI? | User experience evaluation |
| API documentation | Manual | Should we generate OpenAPI spec? | Developer experience |
| CLI auto-completion | Not included | Should we generate shell completions? | Developer experience |

---

## 42. Gap Resolution: Additional Interface Specifications

### 42.1 Error Recovery Cascade — API/CLI Behavior

When errors occur during API or CLI operations, the following cascade is followed:

```
Error Occurs
    │
    ↓
Classify Error Kind
    │
    ├── InputError → 400 Bad Request (API) / stderr + exit 2 (CLI)
    │
    ├── EncodingError → 400 Bad Request (API) / stderr + exit 2 (CLI)
    │
    ├── AuthenticationError → 401 Unauthorized (API) / stderr + exit 5 (CLI)
    │
    ├── AuthorizationError → 403 Forbidden (API) / stderr + exit 6 (CLI)
    │
    ├── ValidationError → 422 Unprocessable Entity (API) / stderr + exit 2 (CLI)
    │
    ├── PolicyError → 403 Forbidden (API) / stderr + exit 6 (CLI)
    │
    ├── ResourceError → 503 Service Unavailable (API) / stderr + exit 7 (CLI)
    │
    ├── NetworkError → 502 Bad Gateway (API) / stderr + exit 8 (CLI)
    │
    ├── PersistenceError → 500 Internal Server Error (API) / stderr + exit 9 (CLI)
    │
    ├── TimeoutError → 504 Gateway Timeout (API) / stderr + exit 11 (CLI)
    │
    ├── SubsystemDisabled → 501 Not Implemented (API) / stderr + exit 1 (CLI)
    │
    ├── ConfigError → 500 Internal Server Error (API) / stderr + exit 3 (CLI)
    │
    └── RuntimeError → 500 Internal Server Error (API) / stderr + exit 13 (CLI)
```

**Error Response Rules:**

| Rule | Description |
|---|---|
| ERR-API-001 | All API errors return structured JSON with code, kind, message |
| ERR-API-002 | All CLI errors print to stderr with error message and hint |
| ERR-API-003 | Recoverable errors include `recoverable: true` in response |
| ERR-API-004 | Fatal errors include `recoverable: false` in response |
| ERR-API-005 | Error messages do not leak internal state or secrets |
| ERR-API-006 | Error codes are deterministic for same error condition |
| ERR-API-007 | Exit codes match the error category (see DOC-05 §32) |

### 42.2 Configuration Validation API Response

When configuration validation is triggered (e.g., via `cortex status` or startup):

```json
{
  "success": false,
  "error": {
    "code": "CORTEX_ERR_020",
    "kind": "ConfigError",
    "message": "Configuration validation failed",
    "details": {
      "validation_errors": [
        {"field": "model.cells", "rule": "RangeViolation", "message": "Must be >= 256", "value": 128},
        {"field": "language.context_window", "rule": "DependencyViolation", "message": "Must be >= generation_limit"}
      ]
    },
    "recoverable": false,
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

---

## 43. API/CLI Completeness

### 42.1 Completeness Checklist

| Interface Category | Status | Coverage |
|---|---|---|
| API Architecture | ✅ Complete | Server model, config, lifecycle |
| API Versioning | ✅ Complete | URL-based versioning, rules |
| Request Model | ✅ Complete | Headers, body, validation |
| Response Model | ✅ Complete | Structure, fields, headers |
| Error Model | ✅ Complete | 25 error codes, HTTP mapping |
| Authentication | ✅ Complete | Bearer token, flow, rules |
| Core API (Inference) | ✅ Complete | Single + batch inference |
| Context API | ✅ Complete | Get, reset |
| Observation API | ✅ Complete | Submit, list |
| Memory API | ✅ Complete | Query, stats, forget |
| World Model API | ✅ Complete | Query, state, predict |
| Reasoning API | ✅ Complete | Query, state |
| Planning API | ✅ Complete | Plan, state |
| Prediction API | ✅ Complete | Current, history |
| Action API | ✅ Complete | Execute |
| Verification API | ✅ Complete | Verify, state |
| Learning API | ✅ Complete | Experience, learn, state |
| Self Model API | ✅ Complete | Full model, capability |
| Policy API | ✅ Complete | State, evaluate |
| Internet API | ✅ Complete | Fetch, state |
| Persistence API | ✅ Complete | Checkpoint, list, validate |
| State API | ✅ Complete | Summary, inspect |
| Configuration API | ✅ Complete | Read-only config |
| Health/Status API | ✅ Complete | Status, health |
| Observability API | ✅ Complete | Metrics, diagnostics |
| CLI Architecture | ✅ Complete | Model, modes |
| CLI Commands | ✅ Complete | 14 commands defined |
| CLI Arguments | ✅ Complete | Positional, validation |
| CLI Options | ✅ Complete | Global + per-command |
| CLI Output | ✅ Complete | Text, JSON, rules |
| Exit Codes | ✅ Complete | 14 codes defined |
| Configuration Files | ✅ Complete | Discovery, format, validation |
| Environment Variables | ✅ Complete | 5 variables, rules |
| Machine-Readable Output | ✅ Complete | JSON schema, rules |
| Compatibility | ✅ Complete | API + CLI guarantees |
| API Limits | ✅ Complete | Rate, size, timeout, connections |
| Error Handling | ✅ Complete | Pipeline, recovery, examples |
| Examples | ✅ Complete | API + CLI + error scenarios |
| Interface Contracts | ✅ Complete | 39 API + 12 CLI contracts |

### 42.2 Traceability to Requirements

| DOC-01 Requirement | DOC-05 Interface Coverage |
|---|---|
| FR-LANG-* | `/v1/inference`, `cortex run`, `cortex query` |
| FR-MEM-* | `/v1/memory/*`, `cortex query --target memory` |
| FR-WRLD-* | `/v1/world/*`, `cortex inspect world` |
| FR-RSN-* | `/v1/reasoning/*` |
| FR-PLN-* | `/v1/planning/*` |
| FR-VER-* | `/v1/verify`, `cortex verify` |
| FR-LRN-* | `/v1/experience`, `/v1/learn`, `cortex experience`, `cortex learn` |
| FR-SLF-* | `/v1/self-model` |
| FR-POL-* | `/v1/policy/*` |
| FR-INT-* | `/v1/internet/*` |
| FR-PRS-* | `/v1/checkpoint`, `cortex checkpoint` |
| FR-API-001 | §8-26 (all API endpoints) |
| FR-API-002 | §7 (Authentication) |
| FR-API-003 | §40.1 (API contract: no arbitrary mutation) |
| FR-API-004 | §2.2 (`api.enabled = false`) |
| FR-CLI-001 | §28 (all CLI commands) |

### 42.3 Traceability to Requirements

| DOC-01 Requirement | DOC-05 Section | API Endpoint | CLI Command |
|---|---|---|---|
| FR-API-001 | §2 API Endpoints | All `/v1/*` | — |
| FR-API-002 | §4 Authentication | `Authorization: Bearer` | `--api-key` |
| FR-API-003 | §5 Policy Gates | All mutation endpoints | All mutation commands |
| FR-API-004 | §2.1 Disabled API | — | CLI-only mode |
| FR-CLI-001 | §3 CLI Commands | — | All `cortex <command>` |
| FR-LANG-001 | §2.1.1 POST /v1/inference | `/v1/inference` | `cortex observe` |
| FR-MEM-007 | §2.1.5 POST /v1/memory/query | `/v1/memory/query` | `cortex query` |
| FR-VER-001 | §2.1.10 POST /v1/verify | `/v1/verify` | `cortex verify` |
| FR-PRS-001 | §2.1.14 POST /v1/checkpoint | `/v1/checkpoint` | `cortex checkpoint` |
| FR-INT-001 | §2.1.12 POST /v1/internet/fetch | `/v1/internet/fetch` | `cortex fetch` |

### 42.4 Final Interface Contract Statement

> **This document constitutes the interface contract for CORTEX.** It defines every external communication pathway: 39 HTTP API endpoints, 14 CLI commands, 5 environment variables, 14 exit codes, 25 error codes, and complete request/response schemas.
>
> The interface contract ensures:
> - **Authenticated access**: All API endpoints require Bearer token.
> - **Policy-gated mutations**: All state-changing operations pass through policy.
> - **No arbitrary state writes**: API maps to defined cognitive operations.
> - **Consistent error model**: Structured errors with codes, kinds, and recovery guidance.
> - **Versioned API**: URL-based versioning with compatibility guarantees.
> - **Machine-readable**: JSON output for scripting and automation.
> - **CLI parity**: Core API operations have CLI equivalents.
> - **Bounded operations**: All operations have explicit limits.
> - **Secret isolation**: API keys never in state, logs, or responses.
>
> **CORTEX interface architecture: 39 API endpoints, 14 CLI commands, 1 binary, 1 authentication mechanism, 1 error taxonomy.**

---

*End of Document — CORTEX-DOC-05 API & CLI Specification v1.1.0*
