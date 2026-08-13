# .cx Binary Format Specification

Binary layout specification for the CORTEX state file format.

## Format Overview

The `.cx` file is a binary, versioned, section-oriented cognitive state container. It uses BLAKE3-256 for integrity verification and zstd for compression.

## File Header

| Offset | Size | Field | Description |
|---|---|---|---|
| 0 | 8 | magic | `b"CORTEX\0\0"` |
| 8 | 4 | format_version | u32 — format version |
| 12 | 4 | architecture_version | u32 — architecture version |
| 16 | 4 | algorithm_version | u32 — algorithm version |
| 20 | 32 | config_hash | BLAKE3-256 hash of configuration |
| 52 | 16 | state_id | UUID v4 |
| 68 | 8 | created_at | u64 — timestamp ms |
| 76 | 8 | last_checkpoint | u64 — timestamp ms |
| 84 | 8 | total_sections | u64 — number of sections |
| 92 | 32 | file_checksum | BLAKE3-256 of entire file (excluded from checksum computation) |

**Total header size: 124 bytes**

## Section Layout

Each section follows this structure:

| Offset | Size | Field | Description |
|---|---|---|---|
| 0 | 2 | section_type | u16 — section type identifier |
| 2 | 2 | section_version | u16 — section version |
| 4 | 4 | flags | u32 — section flags |
| 8 | 8 | offset | u64 — data offset from file start |
| 16 | 8 | length | u64 — data length in bytes |
| 24 | 16 | checksum | BLAKE3-128 truncated checksum of data |

**Total section header size: 40 bytes**

## Section Types

| Type ID | Name | Description |
|---|---|---|
| 0x0001 | HEADER | File header (duplicate for redundancy) |
| 0x0002 | ARCHITECTURE | Architecture metadata |
| 0x0010 | LANGUAGE | Language state |
| 0x0020 | NEURAL | Neural state |
| 0x0021 | CELLS | Cell data |
| 0x0022 | COLUMNS | Column data |
| 0x0023 | FIELDS | Field data |
| 0x0030 | WORKING_MEMORY | Working memory state |
| 0x0031 | EPISODIC_MEMORY | Episodic memory data |
| 0x0032 | SEMANTIC_MEMORY | Semantic memory data |
| 0x0033 | PROCEDURAL_MEMORY | Procedural memory data |
| 0x0034 | ASSOCIATIVE_MEMORY | Associative memory data |
| 0x0040 | WORLD_MODEL | World model state |
| 0x0050 | REASONING | Reasoning state |
| 0x0060 | PLANNING | Planning state |
| 0x0070 | VERIFICATION | Verification state |
| 0x0080 | LEARNING | Learning state |
| 0x0090 | SELF_MODEL | Self model state |
| 0x00A0 | PROVENANCE | Provenance data |
| 0x00B0 | CHECKPOINT_METADATA | Checkpoint metadata |
| 0x00C0 | INTEGRITY | Integrity verification data |

## Write Protocol

```
1. Serialize state to temporary buffer
2. Compute BLAKE3-256 checksum of buffer
3. Write temporary file: cortex.cx.tmp
4. Flush to disk
5. Verify checksum of written file
6. Atomic rename: cortex.cx.tmp → cortex.cx
```

## Read Protocol

```
1. Read file header
2. Verify magic bytes
3. Verify file checksum
4. Read section headers
5. Verify per-section checksums
6. Deserialize sections
7. Reconstruct CortexState
```
