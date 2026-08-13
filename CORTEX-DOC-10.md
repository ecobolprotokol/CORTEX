# CORTEX — 10 Configuration Reference

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-10 |
| **Title** | Configuration Reference |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Configuration Contract |
| **Scope** | All configuration parameters, validation rules, defaults, interactions |
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
| Configuration Lead | _____________ | _____________ |

### Document Purpose

This document defines **every configuration parameter available in `cortex.toml`**. It constitutes the configuration reference: parameter names, types, defaults, valid ranges, validation rules, interactions, and examples.

---

## 1. Configuration File Format

### 1.1 File Properties

| Property | Value |
|---|---|
| File name | `cortex.toml` |
| Format | TOML |
| Location | Same directory as `cortex` binary (default) |
| Mutability | Administrative only; learning SHALL NOT silently rewrite |
| Encoding | UTF-8 |

### 1.2 Configuration Discovery

```
Search order:
1. --config <path> (explicit CLI argument)
2. CORTEX_CONFIG environment variable
3. ./cortex.toml (current working directory)
4. /opt/cortex/cortex.toml (default install path)
5. Error: configuration not found
```

### 1.3 Configuration Validation Pipeline

```
Parse TOML → Schema Validation → Range Validation → Dependency Validation
→ Policy Validation → Runtime Initialization
```

Invalid configuration SHALL prevent startup with descriptive error.

---

## 2. `[model]` — Neural Architecture

```toml
[model]
cells = 4096
columns = 64
dimension = 256
precision = "f32"
sparsity_ratio = 0.05
```

### 2.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `cells` | u32 | 4096 | ≥ 256 | Total number of cells in the neural core |
| `columns` | u32 | 64 | ≥ 16 | Number of columns per field |
| `dimension` | u32 | 256 | ≥ 64 | Dimensionality of cell representation vectors |
| `precision` | string | `"f32"` | `"f32"`, `"f16"`, `"bf16"` | Floating-point precision for neural computation |
| `sparsity_ratio` | f32 | 0.05 | (0.0, 1.0] | Fraction of cells active at any time |

### 2.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| MDL-001 | `cells ≥ 256` | ConfigError: cells too small |
| MDL-002 | `columns ≥ 16` | ConfigError: columns too small |
| MDL-003 | `dimension ≥ 64` | ConfigError: dimension too small |
| MDL-004 | `sparsity_ratio ∈ (0.0, 1.0]` | ConfigError: invalid sparsity |
| MDL-005 | `cells % columns == 0` | ConfigError: cells must be divisible by columns |

### 2.3 Derived Values

| Value | Formula | Description |
|---|---|---|
| Cells per column | `cells / columns` | Cells in each column |
| Max active cells | `cells × sparsity_ratio` | Maximum simultaneously active cells |
| Field count | `cells / columns` | Number of neural fields |

---

## 3. `[language]` — Language Core

```toml
[language]
enabled = true
vocabulary_capacity = 65536
context_window = 4096
generation_limit = 1024
learning = true
```

### 3.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable language processing pipeline |
| `vocabulary_capacity` | u32 | 65536 | ≥ 256 | Maximum vocabulary size (number of symbols) |
| `context_window` | u32 | 4096 | ≥ 64 | Maximum context window in tokens |
| `generation_limit` | u32 | 1024 | ≥ 32 | Maximum generation output in tokens |
| `learning` | bool | true | true/false | Enable language learning (vocabulary expansion) |

### 3.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| LNG-001 | `vocabulary_capacity ≥ 256` | ConfigError: vocabulary too small |
| LNG-002 | `context_window ≥ 64` | ConfigError: context window too small |
| LNG-003 | `generation_limit ≥ 32` | ConfigError: generation limit too small |
| LNG-004 | `context_window ≥ generation_limit` | ConfigError: context must fit generation |

---

## 4. `[memory]` — Memory Budgets

```toml
[memory]
working_mb = 128
episodic_mb = 512
semantic_mb = 512
procedural_mb = 256
associative_mb = 256
```

### 4.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `working_mb` | u32 | 128 | ≥ 16 | Working memory budget in MB |
| `episodic_mb` | u32 | 512 | ≥ 32 | Episodic memory budget in MB |
| `semantic_mb` | u32 | 512 | ≥ 32 | Semantic memory budget in MB |
| `procedural_mb` | u32 | 256 | ≥ 16 | Procedural memory budget in MB |
| `associative_mb` | u32 | 256 | ≥ 16 | Associative memory budget in MB |

### 4.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| MEM-001 | `working_mb ≥ 16` | ConfigError: working memory too small |
| MEM-002 | `episodic_mb ≥ 32` | ConfigError: episodic memory too small |
| MEM-003 | `semantic_mb ≥ 32` | ConfigError: semantic memory too small |
| MEM-004 | `procedural_mb ≥ 16` | ConfigError: procedural memory too small |
| MEM-005 | `associative_mb ≥ 16` | ConfigError: associative memory too small |

### 4.3 Derived Values

| Value | Formula | Description |
|---|---|---|
| Total memory budget | `working_mb + episodic_mb + semantic_mb + procedural_mb + associative_mb` | Total memory allocation |
| Default total | 1664 MB | Default total memory budget |

---

## 5. `[learning]` — Learning Parameters

```toml
[learning]
enabled = true
learning_rate = 0.001
plasticity = 0.01
replay = true
consolidation_interval = 1000
```

### 5.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable learning system |
| `learning_rate` | f32 | 0.001 | (0.0, 1.0] | Base learning rate (η) |
| `plasticity` | f32 | 0.01 | [0.0, 1.0] | Maximum weight update bound (plasticity) |
| `replay` | bool | true | true/false | Enable experience replay |
| `consolidation_interval` | u64 | 1000 | ≥ 1 | Episodes between consolidation cycles |

### 5.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| LRN-001 | `learning_rate ∈ (0.0, 1.0]` | ConfigError: invalid learning rate |
| LRN-002 | `plasticity ∈ [0.0, 1.0]` | ConfigError: invalid plasticity |
| LRN-003 | `consolidation_interval ≥ 1` | ConfigError: consolidation interval too small |

### 5.3 Derived Values

| Value | Formula | Description |
|---|---|---|
| Max replay count | `max(1, consolidation_interval / 10)` | Episodes replayed per cycle |
| Effective update bound | `learning_rate × plasticity` | Maximum single-update magnitude |

---

## 6. `[world]` — World Model

```toml
[world]
enabled = true
prediction_horizon = 8
```

### 6.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable world model |
| `prediction_horizon` | u32 | 8 | ≥ 1 | Maximum prediction steps ahead |

### 6.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| WLD-001 | `prediction_horizon ≥ 1` | ConfigError: prediction horizon too small |

---

## 7. `[reasoning]` — Reasoning Engine

```toml
[reasoning]
enabled = true
max_steps = 32
```

### 7.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable reasoning engine |
| `max_steps` | u32 | 32 | ≥ 1 | Maximum reasoning steps per evaluation |

### 7.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| RSN-001 | `max_steps ≥ 1` | ConfigError: max steps too small |

---

## 8. `[planning]` — Planning Engine

```toml
[planning]
enabled = true
max_depth = 8
max_branches = 16
```

### 8.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable planning engine |
| `max_depth` | u32 | 8 | ≥ 1 | Maximum planning depth (steps per plan) |
| `max_branches` | u32 | 16 | ≥ 1 | Maximum plan alternatives considered |

### 8.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| PLN-001 | `max_depth ≥ 1` | ConfigError: max depth too small |
| PLN-002 | `max_branches ≥ 1` | ConfigError: max branches too small |

---

## 9. `[verification]` — Verification Engine

```toml
[verification]
enabled = true
minimum_confidence = 0.80
```

### 9.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable verification engine |
| `minimum_confidence` | f32 | 0.80 | [0.0, 1.0] | Minimum confidence threshold for Verified status |

### 9.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| VER-001 | `minimum_confidence ∈ [0.0, 1.0]` | ConfigError: invalid confidence threshold |

---

## 10. `[internet]` — Internet Interface

```toml
[internet]
enabled = true
timeout_seconds = 15
max_response_mb = 4
```

### 10.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable internet access |
| `timeout_seconds` | u32 | 15 | ≥ 1 | HTTP request timeout in seconds |
| `max_response_mb` | u32 | 4 | ≥ 1 | Maximum response size in MB |

### 10.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| INT-001 | `timeout_seconds ≥ 1` | ConfigError: timeout too small |
| INT-002 | `max_response_mb ≥ 1` | ConfigError: max response too small |

---

## 11. `[policy]` — Policy Configuration

```toml
[policy]
learning = true
internet_learning = true
self_modification = false
policy_modification = false
runtime_modification = false
```

### 11.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `learning` | bool | true | true/false | Allow learning operations |
| `internet_learning` | bool | true | true/false | Allow internet-sourced learning |
| `self_modification` | bool | false | true/false | Allow Level 2 algorithm adaptation |
| `policy_modification` | bool | false | true/false | Allow Level 3 policy modification |
| `runtime_modification` | bool | false | true/false | Allow runtime modification |

### 11.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| POL-001 | All parameters are valid TOML booleans | ConfigError |
| POL-002 | If `policy_modification = true`, emit warning | Warning |
| POL-003 | If `self_modification = true`, emit warning | Warning |

### 11.3 Security Warnings

| Condition | Warning |
|---|---|
| `self_modification = true` | Self-modification Level 2 enabled; algorithm adaptation allowed |
| `policy_modification = true` | Self-modification Level 3 enabled; policy modification allowed |
| `runtime_modification = true` | Runtime modification enabled |

---

## 12. `[api]` — API Server

```toml
[api]
enabled = true
bind = "127.0.0.1:8080"
api_key_env = "CORTEX_API_KEY"
```

### 12.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `enabled` | bool | true | true/false | Enable API server |
| `bind` | string | `"127.0.0.1:8080"` | Valid socket address | TCP bind address |
| `api_key_env` | string | `"CORTEX_API_KEY"` | Non-empty string | Environment variable name for API key |

### 12.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| API-001 | `bind` is valid socket address | ConfigError: invalid bind address |
| API-002 | `api_key_env` is non-empty string | ConfigError: invalid API key env var |
| API-003 | If `enabled = true`, `CORTEX_API_KEY` env var must exist | Startup error |

---

## 13. `[persistence]` — Persistence Configuration

```toml
[persistence]
state = "cortex.cx"
checkpoint_interval = 1000
```

### 13.1 Parameters

| Parameter | Type | Default | Range | Description |
|---|---|---|---|---|
| `state` | string | `"cortex.cx"` | Valid file path | Path to cognitive state file |
| `checkpoint_interval` | u64 | 1000 | ≥ 1 | Episodes between automatic checkpoints |

### 13.2 Validation Rules

| Rule | Condition | Error |
|---|---|---|
| PRS-001 | `state` is valid file path | ConfigError: invalid state path |
| PRS-002 | `checkpoint_interval ≥ 1` | ConfigError: checkpoint interval too small |

---

## 14. Configuration Interactions

### 14.1 Parameter Interactions

| Parameters | Interaction | Rule |
|---|---|---|
| `model.cells` × `model.columns` | `cells` must be divisible by `columns` | MDL-005 |
| `language.context_window` × `language.generation_limit` | `context_window ≥ generation_limit` | LNG-004 |
| `learning.learning_rate` × `learning.plasticity` | Effective bound = `learning_rate × plasticity` | LRN-DERIVED |
| `learning.consolidation_interval` × `learning.replay` | Replay budget = `max(1, consolidation_interval / 10)` | LRN-DERIVED |
| `model.sparsity_ratio` × `model.cells` | Max active = `cells × sparsity_ratio` | MDL-DERIVED |
| `api.enabled` × `CORTEX_API_KEY` | If API enabled, key must exist | API-003 |
| `internet.enabled` × `policy.internet_learning` | Internet requires both enabled | INT-DERIVED |

### 14.2 Disabled Subsystem Behavior

| Subsystem | `enabled = false` Behavior |
|---|---|
| `language.enabled = false` | Input treated as raw observation; output limited to structured responses |
| `world.enabled = false` | World model returns empty state; reasoning operates without world context |
| `reasoning.enabled = false` | Hypothesis generation skipped; conclusions based on direct memory retrieval |
| `planning.enabled = false` | No goal-directed planning; responses based on immediate reasoning only |
| `verification.enabled = false` | All claims remain provisional; minimum_confidence not applied |
| `learning.enabled = false` | No state mutation from experience; all learning signals discarded |
| `internet.enabled = false` | No network access; internet observation pipeline disabled |
| `api.enabled = false` | No API server started; only CLI is operational |

---

## 15. Configuration Examples

### 15.1 Minimal Configuration

```toml
[model]
cells = 256
columns = 16
dimension = 64
precision = "f32"
sparsity_ratio = 0.05

[language]
enabled = true
vocabulary_capacity = 1024
context_window = 512
generation_limit = 256
learning = true

[memory]
working_mb = 16
episodic_mb = 32
semantic_mb = 32
procedural_mb = 16
associative_mb = 16

[learning]
enabled = true
learning_rate = 0.001
plasticity = 0.01
replay = true
consolidation_interval = 100

[world]
enabled = true
prediction_horizon = 4

[reasoning]
enabled = true
max_steps = 8

[planning]
enabled = false

[verification]
enabled = true
minimum_confidence = 0.70

[internet]
enabled = false

[policy]
learning = true
internet_learning = false
self_modification = false
policy_modification = false
runtime_modification = false

[api]
enabled = false
bind = "127.0.0.1:8080"
api_key_env = "CORTEX_API_KEY"

[persistence]
state = "cortex.cx"
checkpoint_interval = 100
```

### 15.2 Production Configuration

```toml
[model]
cells = 4096
columns = 64
dimension = 256
precision = "f32"
sparsity_ratio = 0.05

[language]
enabled = true
vocabulary_capacity = 65536
context_window = 4096
generation_limit = 1024
learning = true

[memory]
working_mb = 128
episodic_mb = 512
semantic_mb = 512
procedural_mb = 256
associative_mb = 256

[learning]
enabled = true
learning_rate = 0.001
plasticity = 0.01
replay = true
consolidation_interval = 1000

[world]
enabled = true
prediction_horizon = 8

[reasoning]
enabled = true
max_steps = 32

[planning]
enabled = true
max_depth = 8
max_branches = 16

[verification]
enabled = true
minimum_confidence = 0.80

[internet]
enabled = true
timeout_seconds = 15
max_response_mb = 4

[policy]
learning = true
internet_learning = true
self_modification = false
policy_modification = false
runtime_modification = false

[api]
enabled = true
bind = "127.0.0.1:8080"
api_key_env = "CORTEX_API_KEY"

[persistence]
state = "cortex.cx"
checkpoint_interval = 1000
```

---

## 16. Configuration Invariants

| # | Invariant | Enforcement |
|---|---|---|
| CFG-001 | All parameters have defined types | TOML parsing |
| CFG-002 | All parameters are within valid ranges | Range validation |
| CFG-003 | All parameter interactions are consistent | Dependency validation |
| CFG-004 | Disabled subsystems produce defined defaults | Runtime behavior |
| CFG-005 | Configuration is immutable after boot | Architecture |
| CFG-006 | Invalid configuration prevents startup | Validation pipeline |
| CFG-007 | Configuration hash is recorded in .cx | Persistence |
| CFG-008 | Policy configuration defaults are secure | Default values |

---

## 17. Traceability

### 17.1 Traceability to Requirements

| DOC-01 Requirement | DOC-10 Coverage |
|---|---|
| §13 Configuration Requirements | §1-14 Configuration parameters |
| §13.2 Configuration Sections | §2-13 Per-section parameters |
| §13.3 Configuration Validation Pipeline | §1.3 Validation pipeline |
| §13.4 Configuration Immutability Boundary | §16 CFG-005 |
| §13.5 Disabled Subsystem Behavior | §14.2 Disabled behavior |
| §23 Open Technical Parameters | Per-parameter defaults and ranges |

### 17.2 Final Configuration Contract Statement

> **This document constitutes the configuration reference for CORTEX.** It defines every configuration parameter, its type, default, valid range, validation rules, and interactions.
>
> The configuration contract ensures:
> - **Complete parameter coverage**: Every `cortex.toml` parameter is documented.
> - **Explicit validation rules**: Every parameter has defined validation.
> - **Clear defaults**: Every parameter has a documented default value.
> - **Interaction rules**: Parameter dependencies and interactions are explicit.
> - **Disabled subsystem behavior**: Every subsystem has defined behavior when disabled.
> - **Security defaults**: Policy defaults are secure (self-modification disabled).
>
> **CORTEX configuration reference: 12 sections, 35 parameters, 35 validation rules, 8 disabled subsystem behaviors.**

---

*End of Document — CORTEX-DOC-10 Configuration Reference v1.1.0*
