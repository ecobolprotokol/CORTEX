# Cross-Document Traceability Matrix

Traceability across all CORTEX documents (DOC-01 through DOC-11).

## Document Hierarchy

```
DOC-01 (Technical Specification) ← ROOT
├── DOC-02 (Software Design Specification)
│   ├── DOC-03 (Data & State Specification)
│   │   └── DOC-04 (Algorithm Specification)
│   │       ├── DOC-05 (API & CLI Specification)
│   │       └── DOC-07 (Testing & Validation Specification)
│   └── DOC-11 (Repository Architecture)
├── DOC-06 (Build & Release Specification)
├── DOC-08 (Deployment & Operations Specification)
├── DOC-09 (Security & Privacy Specification)
└── DOC-10 (Configuration Reference)
```

## Cross-Document References

| From | To | Section | Relationship |
|---|---|---|---|
| DOC-01 | DOC-02 | §1.4 | Requirements → Architecture |
| DOC-01 | DOC-03 | §12 | Requirements → Data format |
| DOC-01 | DOC-05 | §14 | Requirements → Interface contracts |
| DOC-01 | DOC-06 | §20 | Requirements → Technology constraints |
| DOC-01 | DOC-08 | §7, §10 | Requirements → Deployment/Operations |
| DOC-01 | DOC-09 | §15, §16 | Requirements → Security/Privacy |
| DOC-01 | DOC-10 | §13 | Requirements → Configuration |
| DOC-02 | DOC-03 | §10 | Architecture → Type system |
| DOC-02 | DOC-11 | §5 | Architecture → Repository structure |
| DOC-03 | DOC-04 | (implicit) | Data structures → Algorithm input/output |
| DOC-04 | DOC-01 | §21 | Algorithms → Acceptance criteria |
| DOC-05 | DOC-04 | (implicit) | API/CLI → Algorithm invocation |
| DOC-06 | DOC-01 | §20 | Build → Technology constraints |
| DOC-06 | DOC-11 | §8-9 | Build → Repository artifacts |
| DOC-07 | DOC-01 | §21 | Testing → Acceptance criteria |
| DOC-07 | DOC-04 | §2 | Testing → Algorithm correctness |
| DOC-08 | DOC-01 | §7, §10 | Operations → Requirements |
| DOC-09 | DOC-01 | §15, §16 | Security → Requirements |
| DOC-10 | DOC-01 | §13 | Configuration → Requirements |
| DOC-11 | DOC-02 | §4-5 | Repository → Architecture |

## Requirement Domain Coverage

| Domain | DOC-01 Req IDs | DOC-02 Coverage | DOC-03 Coverage | DOC-04 Coverage | DOC-07 Coverage |
|---|---|---|---|---|---|
| Language | FR-LANG-001 to FR-LANG-015 | §16 Language Core | §8 LanguageState | §8 Language algorithms | Unit tests |
| Neural | FR-NEUR-001 to FR-NEUR-009 | §15 Neural Core | §8 NeuralState | §9 Neural algorithms | Unit tests |
| Memory | FR-MEM-001 to FR-MEM-011 | §17 Memory System | §9 Memory data model | §10-11 Memory algorithms | Unit + integration |
| World | FR-WRLD-001 to FR-WRLD-007 | §18 World Model | §10 World data | §12-13 World algorithms | Unit tests |
| Reasoning | FR-RSN-001 to FR-RSN-006 | §19 Reasoning Engine | §11 Reasoning state | §14-15 Reasoning algorithms | Unit tests |
| Planning | FR-PLN-001 to FR-PLN-004 | §20 Planning Engine | §12 Planning state | §17 Planning algorithms | Unit tests |
| Verification | FR-VER-001 to FR-VER-006 | §21 Verification Engine | §13 Verification state | §18 Verification algorithms | Unit tests |
| Learning | FR-LRN-001 to FR-LRN-009 | §22 Learning System | §14 Learning state | §19-20 Learning algorithms | Unit + stability |
| Self Model | FR-SLF-001 to FR-SLF-004 | §23 Self Model | §15 Self-model state | §21 Self-model algorithms | Unit tests |
| Policy | FR-POL-001 to FR-POL-006 | §24 Policy Engine | §16 Policy state | §22 Policy algorithms | Security tests |
| Internet | FR-INT-001 to FR-INT-005 | §25 Internet Interface | (via Observation) | §23 Internet algorithms | Integration tests |
| Persistence | FR-PRS-001 to FR-PRS-006 | §26-28 Persistence | §23 Persistence format | §24 Persistence algorithms | Round-trip tests |
| API | FR-API-001 to FR-API-004 | §29 API Server | (via Request/Response) | — | API contract tests |
| CLI | FR-CLI-001 | §30 CLI Layer | — | — | CLI tests |
