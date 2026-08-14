# NFSC-1.0 — End-to-End Technical Requirements & Boundaries

**Final Architectural Baseline — Companion Document v1.0**

| Field | Value |
|---|---|
| Parent document | NFSC-1.0 Final Architectural Baseline v1.0 (50 pasal) |
| Document class | Normative requirements & hard boundaries |
| Status | **FINAL** |
| Mode | Lossless |
| Implementation | Rust |
| Keyword convention | RFC 2119 (`MUST`, `MUST NOT`, `SHOULD`, `MAY`) |

Dokumen ini menerjemahkan architectural baseline menjadi **end-to-end technical requirements** yang terukur dan **boundaries** yang tidak boleh dilanggar. Setiap requirement memiliki ID unik untuk traceability.

---

## 0. Scope & Document Authority

### 0.1 Scope

\[
\boxed{
\text{Dokumen ini mengatur seluruh jalur data NFSC-1.0 dari byte input pertama hingga byte output terakhir, termasuk decode kebalikannya.}
}
\]

Mencakup:

1. Input ingestion → block partitioning
2. SIMD analysis → classification → decision
3. Transform → entropy coding → output assembly
4. Bitstream format → file layout
5. Decode → inverse transform → reconstruction
6. Performance, memory, concurrency, correctness, portability

### 0.2 Authority Hierarchy

Jika terjadi konflik antar dokumen:

\[
\boxed{
\text{Core Invariants (§45 baseline)}
>
\text{Dokumen ini (requirements)}
>
\text{Architectural guidance}
>
\text{Implementation convenience}
}
\]

Core invariants `I₁ … I₁₁` tidak dapat di-override oleh requirement manapun.

### 0.3 RFC 2119 Keywords

| Keyword | Makna |
|---|---|
| `MUST` | Wajib, pelanggaran = non-compliant |
| `MUST NOT` | Dilarang keras |
| `SHOULD` | Direkomendasikan kuat, boleh dilanggar dengan justifikasi tertulis |
| `SHOULD NOT` | Tidak direkomendasikan kuat |
| `MAY` | Opsional |

---

## 1. System Boundary Definition

### 1.1 In-Scope

\[
\boxed{\text{NFSC-1.0 ADALAH:}}
\]

| # | Cakupan |
|---|---|
| S-1 | Lossless byte-stream compressor |
| S-2 | Block-parallel encoder/decoder |
| S-3 | SIMD-accelerated analysis & transform |
| S-4 | Static rANS entropy coder (8-way interleaved) |
| S-5 | Transform-aware fast classifier |
| S-6 | Self-describing bitstream format |
| S-7 | Deterministic, integer-only arithmetic |
| S-8 | Three performance profiles (Fast/Balanced/Max) |

### 1.2 Out-of-Scope (Non-Goals)

\[
\boxed{\text{NFSC-1.0 BUKAN:}}
\]

| # | Non-goal | Implikasi |
|---|---|---|
| N-1 | Lossy codec | Tidak ada quality parameter |
| N-2 | Streaming/online codec dengan unbounded input | Block-based; input size diketahui atau di-frame |
| N-3 | Encrypted codec | Keamanan bukan tanggung jawab NFSC |
| N-4 | Error-correcting codec | Deteksi boleh, koreksi tidak wajib |
| N-5 | Content-aware semantic compressor | Tidak ada domain-specific model (JPEG-style, etc.) |
| N-6 | Distributed/network codec | Single-process, multicore |
| N-7 | GPU accelerator | CPU-only pada 1.0 |
| N-8 | Adaptive global model | Block independence dipertahankan |

### 1.3 Hard System Boundary

\[
\boxed{
\partial_{\mathrm{NFSC}}
=
\{\text{byte stream in}\}
\rightarrow
\{\text{NFSC file}\}
\rightarrow
\{\text{byte stream out}\}
}
\]

Segala sesuatu di luar boundary ini (filesystem, network, UI, encryption) adalah **external concern** dan `MUST NOT` mempengaruhi correctness atau determinisme codec.

---

## 2. End-to-End Data Contract

### 2.1 Input Contract

| ID | Requirement | Level |
|---|---|---|
| `E2E-IN-001` | Input `MUST` diperlakukan sebagai immutable byte sequence | MUST |
| `E2E-IN-002` | Input size `MUST` tercatat dalam header (`original_size`) | MUST |
| `E2E-IN-003` | Empty input (0 byte) `MUST` menghasilkan valid NFSC file dengan `block_count = 0` | MUST |
| `E2E-IN-004` | Codec `MUST NOT` memodifikasi input buffer | MUST |
| `E2E-IN-005` | Input `MAY` berasal dari memory-mapped file, slice, atau buffer | MAY |

### 2.2 Output Contract

| ID | Requirement | Level |
|---|---|---|
| `E2E-OUT-001` | Output `MUST` self-describing (decode tanpa side information) | MUST |
| `E2E-OUT-002` | Output `MUST` dimulai dengan magic bytes yang valid | MUST |
| `E2E-OUT-003` | Compressed size `MUST NOT` melebihi `raw_size + metadata_bound` untuk block RAW | MUST |
| `E2E-OUT-004` | Output `MUST` deterministic untuk input + profile yang sama | MUST |
| `E2E-OUT-005` | Encoder `MUST NOT` menulis di luar reserved output region | MUST |

### 2.3 Roundtrip Contract

\[
\boxed{
\texttt{R-ROUNDTRIP-001:}\quad
\forall X,\ \forall \text{profile } P:\ 
D_P(E_P(X)) = X
}
\]

| ID | Requirement | Level |
|---|---|---|
| `R-ROUNDTRIP-001` | Bit-exact roundtrip untuk semua input valid | MUST |
| `R-ROUNDTRIP-002` | Roundtrip `MUST` berlaku lintas SIMD backend | MUST |
| `R-ROUNDTRIP-003` | Roundtrip `MUST` berlaku lintas profile (encode Fast → decode default, dst.) | MUST |
| `R-ROUNDTRIP-004` | Roundtrip `MUST` berlaku untuk ukuran 0 hingga maksimum yang didukung | MUST |

---

## 3. Functional Requirements (FR)

### 3.1 Pipeline Stages

\[
\texttt{FR-PIPE:}\quad
\text{INGEST}
\to
\text{PARTITION}
\to
\text{ANALYZE}
\to
\text{CLASSIFY}
\to
\text{TRANSFORM}
\to
\text{CODE}
\to
\text{ASSEMBLE}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-001` | Pipeline `MUST` mengikuti urutan §2 baseline | MUST | §2 |
| `FR-002` | Scan `MUST` single-pass bila memungkinkan | MUST | §2 |
| `FR-003` | Transform `MUST` single-pass per block | MUST | §2 |
| `FR-004` | Pass tambahan `MUST NOT` ada tanpa bukti gain signifikan | MUST NOT | §2 |
| `FR-005` | Setiap stage `MUST` memiliki input/output contract yang terdefinisi | MUST | — |

### 3.2 Block Partitioning

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-BLK-001` | Default block size `MUST` 1 MiB | MUST | §5 |
| `FR-BLK-002` | Supported block sizes: 256 KiB, 512 KiB, 1 MiB, 2 MiB | MUST | §5 |
| `FR-BLK-003` | Block size di luar supported set `MUST` ditolak pada encoder | MUST | §5 |
| `FR-BLK-004` | Block terakhir `MAY` lebih kecil dari block size | MAY | §5 |
| `FR-BLK-005` | Setiap block `MUST` independent pada entropy state | MUST | §27 |
| `FR-BLK-006` | Block partitioning `MUST NOT` bergantung pada konten | MUST | — |

### 3.3 Analysis & Classification

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-AN-001` | Analyzer `MUST` menghasilkan `BlockStats` lengkap dalam satu pass | MUST | §9 |
| `FR-AN-002` | Analyzer `MUST` memory-bandwidth oriented | MUST | §9 |
| `FR-AN-003` | Classifier `MUST` menghasilkan kandidat tunggal atau shortlist | MUST | §28 |
| `FR-AN-004` | Classifier `MUST NOT` menjalankan transform untuk evaluasi | MUST NOT | §28 |
| `FR-AN-005` | Classifier cost `MUST` << transform cost | MUST | §29 |

### 3.4 Transform Requirements

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-TR-001` | Semua transform `MUST` reversible | MUST | I₆ |
| `FR-TR-002` | Transform `MUST` memiliki scalar reference implementation | MUST | §35 |
| `FR-TR-003` | Transform `MUST NOT` mengalokasi per-symbol | MUST NOT | §17 |
| `FR-TR-004` | Transform `SHOULD` single-pass | SHOULD | §2 |
| `FR-TR-005` | Transform `MUST` dijalankan hanya jika `Score(T) > 0` | MUST | §29 |

### 3.5 Entropy Coding

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-EC-001` | Primary entropy coder `MUST` rANS | MUST | §20 |
| `FR-EC-002` | Interleave factor `MUST` K=8 pada production path | MUST | §20 |
| `FR-EC-003` | Model default `MUST` static frequency | MUST | §19 |
| `FR-EC-004` | Adaptive model `MUST NOT` aktif pada Fast/Balanced default | MUST NOT | §19 |
| `FR-EC-005` | Semua aritmetika `MUST` integer | MUST | I₂, §34 |
| `FR-EC-006` | Model table `MUST` precomputed sebelum symbol loop | MUST | §22 |

### 3.6 Mode & Escape Requirements

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `FR-MODE-001` | `MODE_RAW` `MUST` tersedia | MUST | §32 |
| `FR-MODE-002` | `MODE_CONSTANT` `MUST` tersedia | MUST | §12 |
| `FR-MODE-003` | `MODE_RLE` `MUST` tersedia | MUST | §13 |
| `FR-MODE-004` | Jika `L_C ≥ L_R`, block `MUST` disimpan sebagai RAW | MUST | §32 |
| `FR-MODE-005` | RAW decode `MUST` equivalent memcpy | MUST | §23 |
| `FR-MODE-006` | Codec `MUST NOT` memperbesar block tanpa alasan | MUST NOT | §32 |

---

## 4. Performance Requirements (PR)

### 4.1 Throughput Floor

\[
\boxed{
\texttt{PR-TP-001:}\quad V_E \ge 60\ \mathrm{MB/s}
}
\]

\[
\boxed{
\texttt{PR-TP-002:}\quad V_D \ge 60\ \mathrm{MB/s}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PR-TP-001` | Encode throughput ≥ 60 MB/s pada reference profile | MUST | §3 |
| `PR-TP-002` | Decode throughput ≥ 60 MB/s pada reference profile | MUST | §3 |
| `PR-TP-003` | Throughput `MUST` dihitung terhadap raw input size | MUST | §1 |
| `PR-TP-004` | Target internal `SHOULD` jauh di atas floor | SHOULD | §3 |
| `PR-TP-005` | Decode `SHOULD` ≥ encode untuk mayoritas corpus | SHOULD | §24 |

### 4.2 Reference Benchmark Conditions

\[
\boxed{
\texttt{PR-BENCH:}\quad
\text{1 thread} \land \text{SIMD on} \land \text{release build} \land \text{warm-up} \land \text{fixed CPU}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PR-BENCH-001` | Acceptance test `MUST` single-thread | MUST | §38 |
| `PR-BENCH-002` | SIMD `MUST` enabled saat benchmark | MUST | §38 |
| `PR-BENCH-003` | Build `MUST` release (optimized) | MUST | §38 |
| `PR-BENCH-004` | Warm-up `MUST` dilakukan sebelum pengukuran | MUST | §38 |
| `PR-BENCH-005` | Input `MUST` cukup besar untuk amortisasi startup | MUST | §38 |

### 4.3 Multicore Scaling

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PR-MC-001` | `V_P > V_1` untuk multicore vs single-core | MUST | §38 |
| `PR-MC-002` | Scaling `MUST` diukur hingga jumlah physical cores | MUST | §38 |
| `PR-MC-003` | Scheduler overhead `MUST` < 5% total runtime | MUST | §7 |

### 4.4 Component CPU Budget

\[
\texttt{PR-BUDGET:}
\]

| Component | Budget | Level | Ref |
|---|---|---|---|
| SIMD analyzer | ≤ 10% | MUST | §36 |
| Transform | ≤ 15% | MUST | §36 |
| Model construction | ≤ 10% | MUST | §36 |
| rANS | ≤ 50% | MUST | §36 |
| Assembly/scheduler | ≤ 10% | MUST | §36 |
| Lainnya | ≤ 5% | MUST | §36 |

| ID | Requirement | Level |
|---|---|---|
| `PR-BUDGET-001` | Total budget `MUST` ≤ 100% | MUST |
| `PR-BUDGET-002` | Jika rANS > 50%, `MUST` optimize rANS | MUST |
| `PR-BUDGET-003` | Jika transform > 15%, `MUST` reduce transform | MUST |

### 4.5 Efficiency Principle

\[
\boxed{
\texttt{PR-EFF-001:}\quad
\frac{\Delta L}{\Delta C_{\mathrm{CPU}}}
\text{ harus signifikan}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PR-EFF-001` | Gain per CPU cycle `MUST` positif dan signifikan | MUST | §37 |
| `PR-EFF-002` | Gain kecil dengan cost besar `MUST` ditolak | MUST | §37 |
| `PR-EFF-003` | Complexity `MUST NOT` naik hanya karena CPU tersedia | MUST NOT | §37 |

---

## 5. Memory Requirements (MR)

### 5.1 Allocation Discipline

\[
\boxed{
\texttt{MR-ALLOC-001:}\quad
\text{zero allocation in steady-state hot path}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `MR-ALLOC-001` | Tidak ada alokasi heap pada hot path steady-state | MUST | §18 |
| `MR-ALLOC-002` | Buffer `MUST` reusable antar block | MUST | §18 |
| `MR-ALLOC-003` | Alokasi hanya pada worker initialization | MUST | §18 |
| `MR-ALLOC-004` | `Vec<Vec<u8>>` `MUST NOT` pada hot path | MUST NOT | §17 |
| `MR-ALLOC-005` | Alokasi per-symbol `MUST NOT` ada | MUST NOT | §17 |

### 5.2 Copy Discipline

\[
\boxed{
\texttt{MR-COPY:}\quad
\text{direct output} > \text{in-place} > \text{single scratch} > \text{multiple temp}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `MR-COPY-001` | Prioritas zero-copy `MUST` diikuti | MUST | §17 |
| `MR-COPY-002` | Multiple temporary buffer `MUST NOT` pada hot path | MUST NOT | §17 |
| `MR-COPY-003` | Repeated realloc pada rANS output `MUST NOT` ada | MUST NOT | §26 |

### 5.3 Memory Boundaries

| ID | Requirement | Level |
|---|---|---|
| `MR-BOUND-001` | Peak memory `MUST` terukur dan dilaporkan | MUST |
| `MR-BOUND-002` | Peak memory `SHOULD` O(block_size × active_workers), bukan O(file_size) | SHOULD |
| `MR-BOUND-003` | Memory `MUST NOT` tumbuh dengan jumlah block | MUST NOT |
| `MR-BOUND-004` | Output reservation `MUST` menggunakan worst-case bound | MUST |

### 5.4 Worst-Case Output Bound

\[
\boxed{
\texttt{MR-WC-001:}\quad
L_{\mathrm{out},i}^{\max}
=
L_{R,i} + L_{\mathrm{overhead}}
}
\]

Setiap block `MUST` memiliki worst-case bound yang diketahui sebelum encoding, sehingga reservasi buffer dilakukan sekali.

---

## 6. Concurrency Requirements (CR)

### 6.1 Worker Model

\[
\boxed{
\texttt{CR-WORKER-001:}\quad
\text{worker-local state, no shared mutable state on hot path}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `CR-WORKER-001` | Setiap worker `MUST` punya state privat | MUST | §6 |
| `CR-WORKER-002` | Shared mutable state `MUST NOT` ada pada hot path | MUST NOT | §6 |
| `CR-WORKER-003` | Worker `MUST` menulis ke region output lokal/reserved | MUST | §25 |
| `CR-WORKER-004` | Shared Vec + lock + append `MUST NOT` pada hot path | MUST NOT | §25 |

### 6.2 Scheduler

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `CR-SCHED-001` | Scheduler `MUST` coarse-grained | MUST | §7 |
| `CR-SCHED-002` | Task granularity `MUST` block-level, bukan symbol/byte | MUST | §7 |
| `CR-SCHED-003` | `T_scheduler << T_encode` | MUST | §7 |
| `CR-SCHED-004` | Scheduler overhead < 5% | MUST | §7 |

### 6.3 Determinism under Parallelism

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `CR-DET-001` | Output `MUST` identik terlepas dari jumlah worker | MUST | I₃ |
| `CR-DET-002` | Output `MUST` identik terlepas dari scheduling order | MUST | I₃ |
| `CR-DET-003` | Block independence `MUST` menjamin deterministic parallel encode | MUST | §27 |

---

## 7. Correctness Requirements (XR)

### 7.1 Core Invariants (Mapping §45)

| ID | Invariant | Requirement | Level |
|---|---|---|---|
| `XR-INV-001` | `D(E(X)) = X` | Lossless roundtrip | MUST |
| `XR-INV-002` | Integer coding | No FP in entropy path | MUST |
| `XR-INV-003` | Deterministic | Same input → same output | MUST |
| `XR-INV-004` | SIMD correctness | `F_SIMD(X) = F_Scalar(X)` | MUST |
| `XR-INV-005` | Block independence | `B_i ⊥ B_j` | MUST |
| `XR-INV-006` | Reversible transform | All transforms invertible | MUST |
| `XR-INV-007` | Raw escape | RAW when no gain | MUST |
| `XR-INV-008` | No unnecessary model | Skip model when not needed | MUST |
| `XR-INV-009` | No SIMD regression | SIMD ≥ scalar correctness | MUST |
| `XR-INV-010` | `V_E ≥ 60 MB/s` | Encode floor | MUST |
| `XR-INV-011` | `V_D ≥ 60 MB/s` | Decode floor | MUST |

### 7.2 SIMD Correctness

\[
\boxed{
\texttt{XR-SIMD-001:}\quad
F_{\mathrm{SIMD}}(X) = F_{\mathrm{Scalar}}(X)
\quad \forall X
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `XR-SIMD-001` | Setiap SIMD kernel `MUST` punya scalar reference | MUST | §35 |
| `XR-SIMD-002` | Output SIMD `MUST` identik dengan scalar | MUST | §35 |
| `XR-SIMD-003` | Feature detection `MUST` sekali di startup | MUST | §8 |
| `XR-SIMD-004` | Feature detection `MUST NOT` dalam hot loop | MUST NOT | §8 |

### 7.3 Determinism

\[
\boxed{
\texttt{XR-DET-001:}\quad
E_{\mathrm{Scalar}}(X)
=
E_{\mathrm{AVX2}}(X)
=
E_{\mathrm{AVX512}}(X)
=
E_{\mathrm{NEON}}(X)
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `XR-DET-001` | Cross-backend determinism | MUST | §33 |
| `XR-DET-002` | Integer-only, no FP rounding dependency | MUST | §34 |
| `XR-DET-003` | No undefined overflow | MUST | §34 |
| `XR-DET-004` | No random seed dependency | MUST | §34 |
| `XR-DET-005` | No external model state | MUST | §34 |

### 7.4 Overflow & Integer Semantics

| ID | Requirement | Level |
|---|---|---|
| `XR-INT-001` | Semua overflow `MUST` terdefinisi (wrapping/checked/explicit) | MUST |
| `XR-INT-002` | Delta byte `MUST` menggunakan `mod 256` | MUST |
| `XR-INT-003` | Renormalization arithmetic `MUST` exact | MUST |

---

## 8. Bitstream / Format Requirements (BR)

### 8.1 File Layout

\[
\boxed{
\text{NFSC FILE}
=
\text{HEADER}
\,\|\,
\text{BLOCK TABLE}
\,\|\,
\text{BLOCK PAYLOADS}
}
\]

### 8.2 Header Requirements

| ID | Field | Requirement | Level |
|---|---|---|---|
| `BR-HDR-001` | magic | `MUST` ada, unique identifier | MUST |
| `BR-HDR-002` | version | `MUST` ada, 1.0 untuk dokumen ini | MUST |
| `BR-HDR-003` | profile | `MUST` tercatat | MUST |
| `BR-HDR-004` | flags | `MUST` ada untuk ekstensibilitas | MUST |
| `BR-HDR-005` | original_size | `MUST` akurat | MUST |
| `BR-HDR-006` | block_size | `MUST` dari supported set | MUST |
| `BR-HDR-007` | block_count | `MUST` akurat | MUST |

### 8.3 Block Table Requirements

| ID | Field | Requirement | Level |
|---|---|---|---|
| `BR-BT-001` | mode | `MUST` dari enum mode valid | MUST |
| `BR-BT-002` | offset | `MUST` menunjuk payload yang benar | MUST |
| `BR-BT-003` | raw_size | `MUST` akurat | MUST |
| `BR-BT-004` | compressed_size | `MUST` akurat | MUST |
| `BR-BT-005` | model_id | `MUST` konsisten dengan payload | MUST |

### 8.4 Format Integrity

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `BR-INT-001` | Metadata `MUST` minimal | MUST | §31 |
| `BR-INT-002` | Ketiga profile `MUST` menghasilkan bitstream kompatibel | MUST | §4 |
| `BR-INT-003` | Decoder `MUST` menolak file dengan magic invalid | MUST | — |
| `BR-INT-004` | Decoder `MUST` menolak version unsupported | MUST | — |
| `BR-INT-005` | Decoder `MUST` memvalidasi block table sebelum decode | MUST | — |
| `BR-INT-006` | Codec `MUST NOT` memperbesar block RAW | MUST NOT | §32 |

### 8.5 Mode Encoding

| ID | Requirement | Level |
|---|---|---|
| `BR-MODE-001` | Setiap mode `MUST` punya identifier unik dalam block table | MUST |
| `BR-MODE-002` | Mode `MUST` cukup untuk decoder memilih inverse path | MUST |
| `BR-MODE-003` | Mode `MUST NOT` memerlukan side information eksternal | MUST NOT |

---

## 9. Hot-Path Requirements (HR)

### 9.1 rANS Hot Loop Discipline

\[
\boxed{
\texttt{HR-HOT-001:}\quad
\text{no heap, no vcall, no FP, no log, no dyn dispatch on hot loop}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `HR-HOT-001` | No heap allocation in hot loop | MUST | §21 |
| `HR-HOT-002` | No virtual call in hot loop | MUST | §21 |
| `HR-HOT-003` | No floating point in hot loop | MUST | §21 |
| `HR-HOT-004` | No model selection in hot loop | MUST | §21 |
| `HR-HOT-005` | No logging in hot loop | MUST | §21 |
| `HR-HOT-006` | No dynamic dispatch in hot loop | MUST | §21, §43 |
| `HR-HOT-007` | No expensive branch in hot loop | MUST | §21 |

### 9.2 Abstraction Boundary

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `HR-ABS-001` | Hot loop `MUST` static dispatch / monomorphization | MUST | §43 |
| `HR-ABS-002` | Interface hanya di boundary (`SimdBackend`, `Transform`, `EntropyCoder`) | MUST | §43 |
| `HR-ABS-003` | Zero virtual dispatch in hot loop | MUST | §43 |

### 9.3 Decode Hot Path

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `HR-DEC-001` | Decode `MUST` path paling sederhana | MUST | §23 |
| `HR-DEC-002` | RAW → direct memcpy | MUST | §23 |
| `HR-DEC-003` | RLE → direct reconstruct | MUST | §23 |
| `HR-DEC-004` | DELTA+rANS → rANS decode + SIMD prefix | MUST | §23 |
| `HR-DEC-005` | Decoder `MUST NOT` melakukan transform yang tidak perlu | MUST NOT | §23 |

---

## 10. Profile Requirements (PF)

### 10.1 Profile Definitions

| ID | Profile | Priority | Level |
|---|---|---|---|
| `PF-FAST-001` | NFSC-Fast: Throughput > Compression | MUST | §4 |
| `PF-BAL-001` | NFSC-Balanced: Compression + Throughput (default) | MUST | §4 |
| `PF-MAX-001` | NFSC-Max: Compression > Throughput | MUST | §4 |

### 10.2 Profile Constraints

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PF-COMPAT-001` | Ketiga profile `MUST` bitstream-compatible | MUST | §4 |
| `PF-FLOOR-001` | Semua profile `MUST` memenuhi 60 MB/s floor | MUST | §30 |
| `PF-FAST-002` | Fast `MUST NOT` pakai adaptive model default | MUST NOT | §4 |
| `PF-MAX-002` | Max `MAY` evaluasi kandidat tambahan | MAY | §30 |
| `PF-MAX-003` | Max `MUST` tetap ≥ 60 MB/s | MUST | §30 |
| `PF-DEF-001` | Balanced `MUST` menjadi default production profile | MUST | §4 |

---

## 11. Failure Mode Requirements (FM)

### 11.1 Failure Boundary

\[
\boxed{
\texttt{FM-BOUND:}\quad
\text{failure } \Rightarrow \text{ graceful, deterministic, no corruption}
}
\]

### 11.2 Input Failure

| ID | Scenario | Required Behavior | Level |
|---|---|---|---|
| `FM-IN-001` | Input null/invalid | Return error, no panic | MUST |
| `FM-IN-002` | Input unreadable | Return error, cleanup | MUST |
| `FM-IN-003` | Input size mismatch | Return error | MUST |

### 11.2 Bitstream Failure

| ID | Scenario | Required Behavior | Level |
|---|---|---|---|
| `FM-BS-001` | Magic invalid | Reject dengan error jelas | MUST |
| `FM-BS-002` | Version unsupported | Reject dengan error jelas | MUST |
| `FM-BS-003` | Block table corrupt | Reject, no partial decode | MUST |
| `FM-BS-004` | Truncated payload | Reject dengan error | MUST |
| `FM-BS-005` | compressed_size mismatch | Reject | MUST |

### 11.3 Resource Failure

| ID | Scenario | Required Behavior | Level |
|---|---|---|---|
| `FM-RES-001` | OOM saat alloc awal | Return error, no partial state | MUST |
| `FM-RES-002` | Thread spawn failure | Degrade ke fewer workers, tetap correct | SHOULD |
| `FM-RES-003` | Output buffer full | Return error, no overflow | MUST |

### 11.4 Failure Discipline

| ID | Requirement | Level |
|---|---|---|
| `FM-DISC-001` | Codec `MUST NOT` panic pada input invalid | MUST |
| `FM-DISC-002` | Codec `MUST NOT` menghasilkan output corrupt pada failure | MUST |
| `FM-DISC-003` | Error `MUST` terstruktur (enum Result), bukan string ad-hoc | MUST |
| `FM-DISC-004` | Failure `MUST NOT` mempengaruhi determinisme run berikutnya | MUST |

---

## 12. Portability Requirements (PT)

### 12.1 Backend Matrix

| ID | Backend | Status | Level |
|---|---|---|---|
| `PT-BE-001` | Scalar | MUST tersedia (fallback universal) | MUST |
| `PT-BE-002` | AVX2 | MUST pada x86-64 modern | MUST |
| `PT-BE-003` | AVX-512 | MAY, jika tersedia | MAY |
| `PT-BE-004` | NEON | MUST pada ARM64 | MUST |

### 12.2 Portability Contract

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `PT-CON-001` | Scalar backend `MUST` selalu available | MUST | §8 |
| `PT-CON-002` | Dispatch `MUST` runtime, sekali di startup | MUST | §8 |
| `PT-CON-003` | Output `MUST` identik lintas backend | MUST | §33 |
| `PT-CON-004` | Codec `MUST` berjalan tanpa SIMD (degrade ke scalar) | MUST | §8 |
| `PT-CON-005` | CPU-specific behavior `MUST NOT` bocor ke bitstream | MUST NOT | §34 |

---

## 13. Safety Requirements (SR) — Rust-Specific

### 13.1 Memory Safety

\[
\boxed{
\texttt{SR-SAFE-001:}\quad
\text{safe Rust on all hot path}
}
\]

| ID | Requirement | Level |
|---|---|---|
| `SR-SAFE-001` | Hot path `MUST` safe Rust | MUST |
| `SR-SAFE-002` | `unsafe` `MUST` minimal dan terisolasi (FFI/SIMD intrinsics) | MUST |
| `SR-SAFE-003` | Setiap blok `unsafe` `MUST` punya safety comment | MUST |
| `SR-SAFE-004` | No `unsafe` untuk alokasi hot-path | MUST NOT |

### 13.2 Undefined Behavior Prevention

| ID | Requirement | Level |
|---|---|---|
| `SR-UB-001` | No data race | MUST |
| `SR-UB-002` | No undefined overflow | MUST |
| `SR-UB-003` | No uninitialized memory read | MUST |
| `SR-UB-004` | No out-of-bounds access | MUST |

### 13.3 Build Safety

| ID | Requirement | Level |
|---|---|---|
| `SR-BUILD-001` | `MUST` compile tanpa warning pada release | MUST |
| `SR-BUILD-002` | `SHOULD` lulus clippy tanpa error | SHOULD |
| `SR-BUILD-003` | `SHOULD` lulus Miri untuk bagian unsafe | SHOULD |

---

## 14. Testability Requirements (TR)

### 14.1 Required Test Suite

\[
\boxed{
\texttt{TR-SUITE:}\quad
\text{roundtrip} \land \text{determinism} \land \text{random} \land \text{corpus} \land \text{simd} \land \text{performance} \land \text{stress}
}
\]

| ID | Test Suite | Purpose | Level | Ref |
|---|---|---|---|---|
| `TR-RT-001` | roundtrip | `D(E(X)) = X` | MUST | §42 |
| `TR-DET-001` | determinism | Cross-backend identical | MUST | §42 |
| `TR-RND-001` | random | Fast rejection proof | MUST | §42 |
| `TR-COR-001` | corpus | Required corpus coverage | MUST | §42 |
| `TR-SIMD-001` | simd | SIMD = Scalar | MUST | §42 |
| `TR-PERF-001` | performance | 60 MB/s gate | MUST | §42 |
| `TR-STR-001` | stress | Edge cases, large input | MUST | §42 |

### 14.2 Required Corpus (§40)

| ID | Corpus | Purpose | Level |
|---|---|---|---|
| `TR-CORP-001` | constant | CONSTANT path | MUST |
| `TR-CORP-002` | uniform random | RAW fast rejection | MUST |
| `TR-CORP-003` | biased random | Static rANS | MUST |
| `TR-CORP-004` | RLE-heavy | RLE path | MUST |
| `TR-CORP-005` | delta-friendly | DELTA path | MUST |
| `TR-CORP-006` | structured binary | Shuffle path | MUST |
| `TR-CORP-007` | columnar | Columnar path | MUST |
| `TR-CORP-008` | text | General bias | MUST |
| `TR-CORP-009` | image-like | Structured multi-byte | MUST |
| `TR-CORP-010` | real-world binary | Production realism | MUST |

### 14.3 Required Metrics (§39)

| ID | Metric | Level |
|---|---|---|
| `TR-MET-001` | Original size | MUST |
| `TR-MET-002` | Compressed size | MUST |
| `TR-MET-003` | Compression ratio | MUST |
| `TR-MET-004` | Bits/byte | MUST |
| `TR-MET-005` | Encode MB/s | MUST |
| `TR-MET-006` | Decode MB/s | MUST |
| `TR-MET-007` | CPU cycles/byte | MUST |
| `TR-MET-008` | Peak memory | MUST |
| `TR-MET-009` | 1T (single-thread) | MUST |
| `TR-MET-010` | NT (multi-thread) | MUST |
| `TR-MET-011` | Scalar | MUST |
| `TR-MET-012` | SIMD | MUST |
| `TR-MET-013` | Roundtrip | MUST |
| `TR-MET-014` | 60 MB/s pass/fail | MUST |

---

## 15. Regression & Change Control (RC)

### 15.1 Regression Rule

\[
\boxed{
\texttt{RC-REG-001:}\quad
\text{no compression gain at disproportionate throughput cost}
}
\]

| ID | Requirement | Level | Ref |
|---|---|---|---|
| `RC-REG-001` | Setiap perubahan `MUST` dibandingkan ke baseline | MUST | §41 |
| `RC-REG-002` | +compression dengan −throughput besar `MUST NOT` diterima otomatis | MUST NOT | §41 |
| `RC-REG-003` | Performance floor `MUST` tetap 60 MB/s | MUST | §41 |
| `RC-REG-004` | Δcompression `MUST` dibayar gain masuk akal (Fast/Balanced) | MUST | §41 |

### 15.2 Change Gate

| ID | Gate | Condition | Level |
|---|---|---|---|
| `RC-GATE-001` | Correctness gate | Semua invariant lulus | MUST |
| `RC-GATE-002` | Determinism gate | Cross-backend identical | MUST |
| `RC-GATE-003` | Performance gate | ≥ 60 MB/s | MUST |
| `RC-GATE-004` | Regression gate | No unacceptable throughput drop | MUST |

---

## 16. Hard Boundary Summary

\[
\boxed{
\text{Batasan keras NFSC-1.0 yang TIDAK BOLEH dilanggar:}
}
\]

| # | Boundary | Value | Type |
|---|---|---|---|
| HB-1 | Roundtrip | `D(E(X)) = X` | Correctness |
| HB-2 | Encode throughput | ≥ 60 MB/s | Performance |
| HB-3 | Decode throughput | ≥ 60 MB/s | Performance |
| HB-4 | Integer-only | No FP entropy | Correctness |
| HB-5 | Determinism | Cross-backend identical | Correctness |
| HB-6 | Block independence | `B_i ⊥ B_j` | Architecture |
| HB-7 | Reversible transform | All invertible | Correctness |
| HB-8 | Raw escape | Wajib | Compression |
| HB-9 | Zero alloc hot path | No steady-state alloc | Memory |
| HB-10 | No shared mutable hot state | Worker-local | Concurrency |
| HB-11 | No virtual dispatch hot loop | Static dispatch | Performance |
| HB-12 | Block size set | {256K, 512K, 1M, 2M} | Format |
| HB-13 | Interleave factor | K = 8 | Coder |
| HB-14 | Primary coder | rANS | Coder |
| HB-15 | Lossless only | No lossy | Scope |
| HB-16 | CPU-only 1.0 | No GPU | Scope |
| HB-17 | Single-pass scan | No rescan without proof | Pipeline |
| HB-18 | No panic on invalid input | Graceful error | Safety |
| HB-19 | Safe Rust hot path | Minimal unsafe | Safety |
| HB-20 | Scheduler overhead | < 5% | Performance |

---

## 17. Acceptance Criteria (End-to-End)

\[
\boxed{
\text{NFSC-1.0 dinyatakan ACCEPTED jika dan hanya jika:}
}
\]

\[
\text{ACCEPTED}
\iff
\begin{cases}
\texttt{XR-INV-001..011} & \text{semua lulus} \\
\texttt{PR-TP-001, PR-TP-002} & \ge 60\ \mathrm{MB/s} \\
\texttt{R-ROUNDTRIP-001..004} & \text{semua lulus} \\
\texttt{XR-DET-001} & \text{cross-backend identical} \\
\texttt{TR-SUITE} & \text{semua suite lulus} \\
\texttt{TR-CORP-001..010} & \text{semua corpus covered} \\
\texttt{FM-DISC-001..003} & \text{graceful failure} \\
\texttt{SR-SAFE-001..004} & \text{memory safe}
\end{cases}
\]

Jika satu saja `MUST` requirement gagal:

\[
\boxed{
\text{NFSC-1.0} \Rightarrow \text{NON-COMPLIANT}
}
\]

---

## 18. Traceability Matrix (Baseline → Requirements)

| Baseline § | Requirement IDs |
|---|---|
| §1 Objective | `PR-TP-001..005` |
| §2 Pipeline | `FR-001..005`, `FR-PIPE` |
| §3 Performance Contract | `PR-TP-001..002` |
| §4 Profiles | `PF-*` |
| §5 Block | `FR-BLK-*` |
| §6 Worker | `CR-WORKER-*` |
| §7 Scheduler | `CR-SCHED-*`, `PR-MC-003` |
| §8 SIMD Dispatch | `XR-SIMD-003..004`, `PT-CON-*` |
| §9 Analyzer | `FR-AN-*` |
| §10 Raw Escape | `FR-MODE-004`, `HB-8` |
| §11 Fast Path | `FR-MODE-*` |
| §12-15 Transforms | `FR-TR-*` |
| §16 Columnar | `FR-TR-*`, `PF-MAX-002` |
| §17 Zero-Copy | `MR-COPY-*` |
| §18 Allocation | `MR-ALLOC-*` |
| §19 Model | `FR-EC-003..004` |
| §20 rANS | `FR-EC-001..002` |
| §21 Hot Loop | `HR-HOT-*` |
| §22 Precompute | `FR-EC-006` |
| §23 Decode Opt | `HR-DEC-*` |
| §24 Decode Priority | `PR-TP-005` |
| §25 Output | `CR-WORKER-003..004` |
| §26 Size Prediction | `MR-WC-001`, `MR-COPY-003` |
| §27 Block Independence | `FR-BLK-005`, `XR-INV-005` |
| §28 Decision | `FR-AN-003..005` |
| §29 Transform Budget | `FR-TR-005` |
| §30 Max Exception | `PF-MAX-*` |
| §31 Bitstream | `BR-*` |
| §32 Raw Escape | `FR-MODE-001, FR-MODE-004` |
| §33 Determinism | `XR-DET-*` |
| §34 Integer-Only | `XR-INT-*`, `XR-DET-002..005` |
| §35 SIMD Correctness | `XR-SIMD-*` |
| §36 Budget | `PR-BUDGET-*` |
| §37 Efficiency | `PR-EFF-*` |
| §38 Acceptance | `PR-BENCH-*`, `PR-MC-*` |
| §39 Metrics | `TR-MET-*` |
| §40 Corpus | `TR-CORP-*` |
| §41 Regression | `RC-*` |
| §42 Architecture | `TR-SUITE` |
| §43 Hot-Path Abstraction | `HR-ABS-*` |
| §44 Reference Impl | `XR-SIMD-001` |
| §45 Invariants | `XR-INV-*` |
| §46 Shannon | (objective, non-testable hard req) |
| §47 Equation | (normative formula) |
| §48 Pipeline | `FR-PIPE` |
| §49 Formula | (summary) |
| §50 Definition | (summary) |

---

## 19. Final Compliance Statement

\[
\boxed{
\text{NFSC-1.0 compliant}
\iff
\forall r \in \{\texttt{MUST}\}: r = \texttt{PASS}
}
\]

\[
\boxed{
\text{Performance floor absolut: } V_E \ge 60\ \mathrm{MB/s} \land V_D \ge 60\ \mathrm{MB/s}
}
\]

\[
\boxed{
\text{Prinsip utama: DO NOT SPEND CPU CYCLES UNLESS THEY CAN BUY COMPRESSION.}
}
\]

---

**Status: FINAL — End-to-End Technical Requirements & Boundaries v1.0**
**Parent: NFSC-1.0 Final Architectural Baseline v1.0**

---

Dokumen ini siap dipakai sebagai:

1. **Acceptance checklist** saat implementasi
2. **Traceability reference** untuk setiap PR/change
3. **Compliance gate** pada CI/CD
4. **Basis test generation** untuk suite §14

