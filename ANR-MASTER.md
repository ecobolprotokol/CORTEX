# ANR — Architecture Contract

## Final Architectural Baseline v1.1

**Status:** `ARCHITECTURE CONTRACT`  
**Baseline:** Final Architectural Baseline v1.1  
**Bahasa normatif:** Dokumen ini menggunakan ketentuan wajib, larangan, dan kebolehan.  
**Sifat dokumen:** Dokumen ini adalah kontrak arsitektur, bukan roadmap, bukan milestone, bukan fase implementasi, dan bukan rencana kerja proyek.

---

## 0. Pernyataan Kontrak

Dokumen ini mengunci arsitektur ANR sebagai baseline final.

Setiap implementasi, spesifikasi turunan, pengujian, dan deployment WAJIB tunduk pada kontrak ini.

Dokumen ini:

1. Menetapkan batas arsitektural yang tidak boleh dilanggar.
2. Menetapkan invariant sistem.
3. Menetapkan interface logis antar subsistem.
4. Menetapkan aturan conformance.
5. Menetapkan aturan perubahan arsitektur.

Dokumen ini TIDAK mendefinisikan:

1. Roadmap.
2. Fase implementasi.
3. Sprint.
4. Milestone.
5. Jadwal.
6. Urutan pengerjaan tim.

Jika terdapat konflik antara dokumen turunan dan Architecture Contract ini, maka Architecture Contract ini yang menang.

---

## 1. Definisi Normatif

Dalam dokumen ini:

| Istilah | Arti |
|---|---|
| WAJIB / HARUS | Mutlak harus dipenuhi. |
| DILARANG / TIDAK BOLEH | Mutlak tidak boleh dilakukan. |
| SEBAIKNYA | Direkomendasikan kuat; penyimpangan harus dapat dipertanggungjawabkan. |
| BOLEH | Opsional. |
| ANR | Autonomous Neural Runtime. |
| Runtime | Proses executable `anr`. |
| Brain | Persistent neural memory bernama `brain.anr`. |
| Cortex | Logical memory untuk knowledge. |
| Cerebellum | Logical memory untuk skill. |
| Hippocampus | Logical memory untuk experience. |
| Cell | Unit neural terkecil. |
| Column | Kumpulan Cell. |
| Block | Unit konteks/sequence/episode/prediksi. |
| Synapse | Koneksi antar Cell dan/atau Column. |
| Episode | Rekaman pengalaman. |
| Skill | Kemampuan prosedural yang telah divalidasi. |
| Knowledge | Pengetahuan/pola yang telah digeneralisasi. |
| Brain Seed | Input provisioning awal. |
| Generation | Nomor versi transaksi persisten brain. |
| HOT/WARM/COLD | Tier penyimpanan logis. |
| Safety Boundary | Lapisan keselamatan yang memvalidasi aksi. |
| Maintenance Budget | Budget komputasi untuk pekerjaan latar belakang. |

---

## 2. Ruang Lingkup Kontrak

Kontrak ini mencakup:

1. Arsitektur runtime ANR.
2. Struktur persistent memory `brain.anr`.
3. Arsitektur neural core.
4. Pembagian Cortex, Cerebellum, dan Hippocampus.
5. Learning, replay, consolidation, retention, dan GC.
6. Perception, sensor, camera, audio, dan robotics.
7. Plugin dan HAL.
8. Decision engine dan safety layer.
9. Storage, recovery, transaction, dan integrity.
10. Scheduler, bounded queue, dan backpressure.
11. SIMD dan compute model.
12. CLI, diagnostics, dan telemetry lokal.
13. Security boundary.
14. Conformance dan change control.

Kontrak ini tidak mencakup:

1. Detail algoritma numerik tingkat implementasi yang tidak mengubah arsitektur.
2. UI/UX.
3. Cloud service.
4. Model bisnis.
5. Rencana rilis.

---

## 3. Interpretasi Kontrak

### 3.1 Prioritas Interpretasi

Jika terjadi ambiguitas, urutan prioritas berikut WAJIB digunakan:

1. Safety.
2. Integritas persistent state.
3. Bounded memory dan bounded queue.
4. Determinisme control path.
5. Single-binary dan single-brain deployment.
6. Offline-first.
7. Learning throughput.
8. Convenience.

### 3.2 Logical vs Physical

Diagram dan istilah seperti Cortex, Cerebellum, dan Hippocampus adalah entitas logis.

Istilah tersebut TIDAK BOLEH diinterpretasikan sebagai file terpisah dalam deployment production.

### 3.3 Kontrak Bukan Roadmap

Setiap kalimat dalam dokumen ini yang menyatakan urutan logis proses, misalnya:

```text
sense → perceive → decide → act
```

harus dibaca sebagai urutan arsitektural, bukan jadwal atau fase implementasi.

---

# BAGIAN I — DEPLOYMENT CONTRACT

---

## 4. Deployment Artifact Contract

### 4.1 Artifact Wajib

Deployment produksi ANR WAJIB hanya membutuhkan dua artifact utama:

```text
/opt/anr/
├── anr
└── brain.anr
```

### 4.2 Makna Artifact

| Artifact | Fungsi |
|---|---|
| `anr` | Seluruh executable runtime. |
| `brain.anr` | Satu-satunya persistent neural memory. |

### 4.3 Larangan File Persistent Neural Terpisah

Deployment produksi TIDAK BOLEH mewajibkan file neural terpisah seperti:

```text
cortex.cx
cerebellum.cm
hippocampus.hs
```

atau bentuk setara lainnya.

Cortex, Cerebellum, dan Hippocampus WAJIB direpresentasikan sebagai logical sections di dalam `brain.anr`.

### 4.4 Single Binary Requirement

Binary `anr` WAJIB mencakup fungsi inti berikut:

1. Runtime.
2. Neural Core.
3. Cortex.
4. Cerebellum.
5. Hippocampus.
6. Learning.
7. Replay.
8. Consolidation.
9. Memory Manager.
10. Garbage Collector.
11. Storage.
12. Recovery.
13. SIMD abstraction.
14. Perception.
15. Plugin system.
16. Hardware abstraction.
17. Decision engine.
18. Safety layer.
19. Actuator interface.
20. CLI.
21. Diagnostics.
22. Brain provisioning.
23. Brain validation.

### 4.5 Dependensi Eksternal yang Dilarang

Operasi inti ANR TIDAK BOLEH mewajibkan:

1. Python.
2. Node.js.
3. Database server.
4. LLM server.
5. Cloud inference.
6. External model server.
7. Message broker eksternal.
8. Service terpisah untuk fungsi inti.

### 4.6 Offline-First

ANR WAJIB beroperasi secara offline-first.

Fungsi inti sistem TIDAK BOLEH bergantung pada konektivitas cloud.

Telemetry, diagnostics, learning, dan persistence WAJIB dapat berjalan lokal tanpa jaringan eksternal.

### 4.7 GPU Optional

GPU BOLEH digunakan sebagai akselerator opsional.

Namun:

1. GPU TIDAK BOLEH menjadi dependensi wajib.
2. ANR WAJIB dapat berjalan tanpa GPU.
3. Ketiadaan GPU TIDAK BOLEH menyebabkan kegagalan arsitektural.

---

# BAGIAN II — PERSISTENT BRAIN CONTRACT

---

## 5. Single Brain Contract

### 5.1 Satu-Satunya Persistent Neural Memory

Seluruh persistent neural state WAJIB berada dalam satu file:

```text
brain.anr
```

### 5.2 Isi Logis brain.anr

Secara logis, `brain.anr` WAJIB memuat:

```text
brain.anr
├── Cortex
├── Cerebellum
└── Hippocampus
```

### 5.3 Struktur Fisik Minimum

`brain.anr` WAJIB memiliki struktur logis berikut:

```text
brain.anr
│
├── Header
├── Runtime Metadata
├── Allocation Table
│
├── Cortex Section
│   ├── Hot
│   ├── Warm
│   ├── Cold
│   └── Index
│
├── Cerebellum Section
│   ├── Hot
│   ├── Warm
│   ├── Cold
│   └── Index
│
├── Hippocampus Section
│   ├── Hot
│   ├── Warm
│   ├── Cold
│   └── Index
│
├── Global Index
└── Integrity
```

### 5.4 Header Contract

Header `brain.anr` WAJIB memuat sekurang-kurangnya:

```text
MAGIC
FORMAT_VERSION
FLAGS
HEADER_SIZE

TOTAL_SIZE

CORTEX_OFFSET
CORTEX_SIZE

CEREBELLUM_OFFSET
CEREBELLUM_SIZE

HIPPOCAMPUS_OFFSET
HIPPOCAMPUS_SIZE

INDEX_OFFSET
INDEX_SIZE

METADATA_OFFSET
METADATA_SIZE

GENERATION

CHECKSUM
```

Detail bit-level BOLEH ditentukan pada Implementation Specification selama tidak melanggar kontrak ini.

### 5.5 Integrity Contract

Setiap operasi commit pada `brain.anr` WAJIB memiliki mekanisme integritas.

Integritas minimal WAJIB mencakup:

1. Validitas magic.
2. Validitas version.
3. Validitas header size.
4. Validitas offsets.
5. Validitas sizes.
6. Validitas section boundaries.
7. Validitas generation.
8. Checksum.

Runtime TIDAK BOLEH mempercayai isi `brain.anr` sebelum validasi selesai.

---

# BAGIAN III — MEMORY SUBSYSTEM CONTRACT

---

## 6. Tiga Subsistem Memori

ANR WAJIB memisahkan tiga fungsi memori:

```text
Cortex      = knowledge
Cerebellum  = skill
Hippocampus = experience
```

### 6.1 Hubungan Fungsional

```text
Experience
    │
    ▼
Hippocampus
    │
    ├──── knowledge ────► Cortex
    │
    └──── skill ────────► Cerebellum
```

---

## 7. Cortex Contract

### 7.1 Fungsi Cortex

Cortex adalah long-term knowledge memory.

Cortex WAJIB digunakan untuk:

1. Pattern generalization.
2. Semantic association.
3. Stable relationships.
4. Contextual knowledge.
5. Knowledge yang telah divalidasi.

### 7.2 Larangan Cortex

Cortex TIDAK BOLEH digunakan sebagai:

1. Raw event log.
2. Temporary buffer.
3. High-churn episode storage.
4. Action command log.

### 7.3 Karakteristik Cortex

Cortex WAJIB memiliki sifat:

1. Long-term.
2. Generalized.
3. Persistent.
4. Sparse.
5. Associative.
6. Conservative GC.

### 7.4 Cortex Promotion

Knowledge BOLEH masuk ke Cortex melalui:

1. Initial provisioning.
2. Consolidation dari Hippocampus.
3. Import yang tervalidasi.

Promotion knowledge WAJIB mempertimbangkan:

1. Recurrence.
2. Confidence.
3. Stability.
4. Context diversity.
5. Prediction error.
6. Contradiction state.

---

## 8. Cerebellum Contract

### 8.1 Fungsi Cerebellum

Cerebellum adalah procedural capability memory.

Cerebellum WAJIB menyimpan:

1. Skill.
2. Action pattern.
3. Procedural capability.
4. Predictive action mapping.
5. Error-corrected behavior.

### 8.2 Karakteristik Cerebellum

Cerebellum WAJIB memiliki sifat:

1. Procedural.
2. Action-oriented.
3. Predictive.
4. Error-driven.
5. Persistent.
6. Very conservative GC.

### 8.3 Skill Formation

Skill WAJIB dibentuk melalui pola:

```text
State
 ↓
Action
 ↓
Feedback
 ↓
Error
 ↓
Correction
 ↓
Repeated success
 ↓
Skill
```

### 8.4 Skill Promotion

Skill BOLEH dipromosikan ke Cerebellum jika memenuhi minimal:

1. Repeated execution.
2. High success.
3. Low error.
4. Stable context.
5. Validation.

Satu keberhasilan tunggal TIDAK BOLEH otomatis menjadikan skill permanent.

---

## 9. Hippocampus Contract

### 9.1 Fungsi Hippocampus

Hippocampus adalah episodic memory.

Hippocampus WAJIB menyimpan experience/episode.

Episode BOLEH berisi:

1. Timestamp.
2. Context.
3. Observation reference.
4. Internal state.
5. Action.
6. Result.
7. Reward.
8. Prediction.
9. Prediction error.
10. Novelty.
11. Importance.
12. References.
13. Provenance.

### 9.2 Karakteristik Hippocampus

Hippocampus WAJIB memiliki sifat:

1. High-write.
2. High-churn.
3. Temporary.
4. Episodic.
5. GC-friendly.

### 9.3 Hippocampus Bukan Knowledge Permanen

Hippocampus TIDAK BOLEH diperlakukan sebagai penyimpanan knowledge permanen utama.

Hippocampus adalah sumber pengalaman untuk replay, learning, dan consolidation.

---

## 10. Memory Isolation Contract

### 10.1 Allocation Independent

Setiap subsistem memori WAJIB memiliki allocation sendiri:

```text
Cortex
├── min
├── target
└── max

Cerebellum
├── min
├── target
└── max

Hippocampus
├── min
├── target
└── max
```

### 10.2 Larangan Hard Stealing

Hippocampus TIDAK BOLEH menghabiskan reserved memory milik Cortex atau Cerebellum.

Cortex dan Cerebellum TIDAK BOLEH mengambil hard allocation minimum milik Hippocampus.

### 10.3 Resource Isolation

Memory Manager WAJIB menjamin:

1. Setiap section memiliki maximum limit.
2. Tidak ada section yang melewati max-nya.
3. Pressure pada satu section tidak boleh secara langsung menghancurkan section lain.
4. GC section tidak boleh melanggar isolation.

---

# BAGIAN IV — NEURAL CORE CONTRACT

---

## 11. Neural Core Hierarchy

Neural Core WAJIB menggunakan primitive berikut:

```text
Cell
  │
  ▼
Column
  │
  ▼
Block
```

Synapse WAJIB menyediakan koneksi:

```text
Cell/Column
     │
     ▼
Synapse
     │
     ▼
Cell/Column
```

---

## 12. Cell Contract

### 12.1 Cell sebagai Unit Terkecil

Cell adalah unit neural terkecil.

Cell WAJIB memiliki logical state minimal:

1. activation.
2. potential.
3. threshold.
4. state.
5. refractory state.
6. activity timestamp.
7. usage.

### 12.2 Implementasi Cell

Implementasi TIDAK BOLEH menggunakan object-per-cell allocation berlebihan pada hot path.

Cell WAJIB dapat direpresentasikan sebagai index dalam array SoA.

---

## 13. Column Contract

Column adalah kumpulan Cell dengan representasi lokal.

Column WAJIB mendukung:

1. Local representation.
2. Competition.
3. Sparse activation.
4. Association.
5. Temporal activity.

Contoh logical:

```text
Column
├── Cell 0
├── Cell 1
├── Cell 2
└── Cell N
```

---

## 14. Block Contract

Block adalah unit konteks neural yang lebih besar.

Block WAJIB dapat digunakan untuk:

1. Context.
2. Sequence.
3. Temporal representation.
4. Episode representation.
5. Local prediction.
6. Association.

Block neural TIDAK BOLEH disamakan dengan filesystem block.

---

## 15. Synapse Contract

### 15.1 Field Minimum Synapse

Synapse WAJIB memiliki minimal:

1. source.
2. target.
3. weight.
4. state.

### 15.2 Field Opsional Synapse

Synapse BOLEH memiliki:

1. strength.
2. usage.
3. age.
4. last_active.
5. plasticity.

### 15.3 Synapse Behavior

Synapse WAJIB mendukung:

1. Hebbian reinforcement.
2. Temporal association.
3. Decay.
4. Strengthening.
5. Weakening.
6. Pruning.

---

## 16. Sparse Computation Contract

ANR WAJIB menggunakan sparse/local computation.

Runtime TIDAK BOLEH menghitung seluruh neural graph setiap cycle, kecuali pada mode debug atau validasi yang dinyatakan eksplisit.

Alur komputasi yang diharapkan:

```text
Input
 ↓
Active representation
 ↓
Active Columns
 ↓
Active Cells
 ↓
Relevant Synapses
 ↓
Active Blocks
```

Tujuan normatif:

1. Low CPU usage.
2. Low memory bandwidth.
3. Cache locality.
4. Embedded suitability.

---

## 17. Data Layout Contract

### 17.1 Structure of Arrays

Production data layout WAJIB menggunakan Structure of Arrays untuk hot neural data.

Contoh:

```text
activations[]
potentials[]
thresholds[]
states[]
weights[]
targets[]
timestamps[]
usage[]
```

### 17.2 Larangan Object-per-Cell

Hot path TIDAK BOLEH bergantung pada layout:

```text
Cell object
Cell object
Cell object
...
```

jika layout tersebut menyebabkan:

1. Cache miss tinggi.
2. Overhead alokasi besar.
3. Fragmentasi.
4. Ketidakmampuan SIMD.

### 17.3 Tujuan Data Layout

Data layout WAJIB mendukung:

1. Cache locality.
2. SIMD.
3. Prefetching.
4. Sequential access.
5. Low overhead.

---

# BAGIAN V — AUTONOMOUS RUNTIME CONTRACT

---

## 18. Autonomous Loop Contract

ANR WAJIB mendukung autonomous loop berikut:

```text
SENSE
 ↓
PERCEIVE
 ↓
REPRESENT
 ↓
ASSOCIATE
 ↓
PREDICT
 ↓
DECIDE
 ↓
SAFETY CHECK
 ↓
ACT
 ↓
OBSERVE RESULT
 ↓
RECORD EXPERIENCE
 ↓
LEARN
 ↓
CONSOLIDATE
 ↓
GC
 ↓
REPEAT
```

### 18.1 Sifat Loop

Loop WAJIB:

1. Berjalan lokal.
2. Tidak bergantung cloud.
3. Menghormati safety boundary.
4. Menggunakan bounded resources.
5. Memberikan feedback ke Hippocampus.

### 18.2 Posisi Safety

Safety check WAJIB terjadi setelah decision dan sebelum actuator.

Tidak ada actuator command yang boleh mencapai actuator tanpa melewati safety layer.

---

## 19. Boot Contract

Runtime boot WAJIB mengikuti urutan logis:

```text
POWER ON
   ↓
Load Config
   ↓
Open brain.anr
   ↓
Validate
   ↓
Recover
   ↓
Detect CPU/SIMD
   ↓
Initialize Memory
   ↓
Initialize HAL
   ↓
Initialize Plugins
   ↓
Initialize Neural Core
   ↓
Initialize Scheduler
   ↓
RUN
```

### 19.1 Boot Validation

Runtime TIDAK BOLEH langsung mempercayai isi brain sebelum validasi.

Validasi boot WAJIB mencakup:

1. Magic.
2. Version.
3. Header size.
4. Offsets.
5. Sizes.
6. Section boundaries.
7. Generation.
8. Checksum.

---

## 20. Shutdown Contract

### 20.1 Graceful Shutdown

Graceful shutdown WAJIB mengikuti pola:

```text
Stop new non-critical work
 ↓
Flush experience
 ↓
Finish safe transaction
 ↓
Update metadata
 ↓
Checksum
 ↓
Commit
 ↓
Shutdown
```

### 20.2 Emergency Shutdown

Emergency shutdown WAJIB memprioritaskan:

1. Actuator safety.
2. Persistent integrity.
3. Safe state.

Emergency shutdown TIDAK BOLEH mengorbankan safety demi menyelesaikan transaksi.

---

## 21. Degraded Mode Contract

ANR WAJIB dapat melanjutkan operasi terbatas pada kondisi failure.

Contoh degraded mode:

| Failure | Degraded Behavior |
|---|---|
| Camera failure | Disable vision. |
| Sensor failure | Disable affected pathway. |
| Plugin failure | Restart atau disable plugin. |
| Hippocampus pressure | Aggressive consolidation/GC. |
| Storage failure | Volatile/degraded mode jika diizinkan. |

### 21.1 Syarat Degraded Mode

Degraded mode TIDAK BOLEH:

1. Menghapus safety layer.
2. Melewati validasi integritas.
3. Menyebabkan unbounded memory.
4. Menyebabkan actuator tidak terkendali.

---

# BAGIAN VI — PERCEPTION CONTRACT

---

## 22. Perception Pipeline

Perception WAJIB mengikuti pipeline logis:

```text
Sensor
 ↓
Acquire
 ↓
Timestamp
 ↓
Validate
 ↓
Preprocess
 ↓
Encode
 ↓
Fuse
 ↓
Neural Representation
```

### 22.1 Output Perception

Output perception WAJIB berupa representasi neural yang dapat dikonsumsi oleh Neural Core.

Perception TIDAK BOLEH bergantung langsung pada detail hardware spesifik tanpa melalui HAL/plugin.

---

## 23. Sensor Architecture Contract

Sensor WAJIB masuk melalui plugin/HAL.

```text
Sensor
  ↓
Sensor Plugin
  ↓
HAL
  ↓
Perception
```

### 23.1 Sensor Plugin Interface

Sensor plugin WAJIB memiliki logical interface minimal:

```rust
trait SensorPlugin {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn read(&mut self) -> Result<SensorFrame>;
    fn capabilities(&self) -> SensorCapabilities;
}
```

Interface spesifik BOLEH lebih kaya, tetapi tidak boleh mengurangi kemampuan minimal di atas.

---

## 24. Sensor Frame Contract

Sensor frame WAJIB memiliki representasi logis minimal:

```text
SensorFrame
├── sensor_id
├── timestamp
├── sequence
├── payload
├── dimensions
├── format
├── quality
└── flags
```

---

## 25. Camera Architecture Contract

Camera adalah perception source khusus.

Pipeline camera WAJIB:

```text
Camera
 ↓
Camera Plugin
 ↓
Frame Buffer
 ↓
Preprocessing
 ↓
Visual Encoder
 ↓
Perception
 ↓
Neural Core
```

### 25.1 Tanggung Jawab Camera Plugin

Camera plugin WAJIB bertanggung jawab atas minimal:

1. Device discovery.
2. Stream.
3. Format.
4. Resolution.
5. Timestamp.
6. Buffer.
7. Frame dropping.

---

## 26. Camera Buffer Contract

Buffer camera WAJIB bounded.

Buffer WAJIB memiliki:

1. `max_frames`.
2. `max_frame_size`.
3. `drop_policy`.

TIDAK BOLEH ada unbounded frame queue.

Jika buffer penuh, policy WAJIB dipilih dari:

1. `drop_oldest`.
2. `drop_newest`.
3. `sample`.
4. `merge`.

Policy harus konsisten dan terdokumentasi.

---

## 27. Audio Contract

Audio, jika digunakan, WAJIB diperlakukan sebagai perception source.

Audio pipeline mengikuti prinsip yang sama:

```text
Audio Input
 ↓
Audio Plugin
 ↓
Buffer
 ↓
Preprocessing
 ↓
Encoder
 ↓
Perception
 ↓
Neural Core
```

Audio buffer WAJIB bounded dan memiliki drop/backpressure policy.

---

# BAGIAN VII — PLUGIN AND HAL CONTRACT

---

## 28. Plugin Architecture Contract

Plugin subsystem WAJIB mendukung domain berikut:

```text
plugins/
├── sensors/
├── vision/
├── audio/
└── robotics/
```

### 28.1 Single-Binary Compatibility

Mekanisme plugin WAJIB tetap memenuhi target single-binary deployment.

Plugin TIDAK BOLEH menjadi dependensi eksternal wajib yang merusak deployment:

```text
/opt/anr/
├── anr
└── brain.anr
```

### 28.2 Plugin Failure Isolation

Plugin failure WAJIB isolated.

```text
Plugin failure
 ↓
Error isolation
 ↓
Restart / disable
 ↓
Runtime continues
```

Plugin failure TIDAK BOLEH:

1. Menghancurkan seluruh runtime.
2. Merusak brain.anr.
3. Melewati safety layer.
4. Menyebabkan unbounded queue.

---

## 29. Hardware Abstraction Layer Contract

HAL WAJIB menyediakan abstraksi untuk:

1. GPIO.
2. I2C.
3. SPI.
4. UART.
5. USB.
6. PWM.
7. ADC.

Untuk robotika, HAL BOLEH menyediakan:

1. Motor.
2. Servo.
3. Encoder.
4. Gripper.
5. Lidar.

### 29.1 Dependency Rule

Neural Core TIDAK BOLEH bergantung langsung pada HAL.

Akses hardware WAJIB melalui:

```text
Plugin / HAL → Perception / Action
```

bukan:

```text
Neural Core → HAL
```

---

# BAGIAN VIII — DECISION AND SAFETY CONTRACT

---

## 30. Decision Engine Contract

Decision engine WAJIB menggunakan minimal:

1. Current neural state.
2. Cortex knowledge.
3. Cerebellum skills.
4. Hippocampus context.
5. Sensor state.
6. Goal.
7. Prediction.
8. Confidence.

### 30.1 Decision Pipeline

Decision pipeline WAJIB mengikuti pola:

```text
State
 ↓
Candidate Actions
 ↓
Evaluate
 ↓
Safety
 ↓
Select
 ↓
Act
```

### 30.2 Deterministic Control Path

Control path WAJIB reproducible dengan ketentuan:

```text
same input
+
same neural state
+
same configuration
```

menghasilkan keputusan yang sama dalam batas numeric platform.

---

## 31. Safety Layer Contract

### 31.1 Safety Boundary Wajib

Safety layer WAJIB ada sebagai boundary terpisah.

```text
Neural Decision
      ↓
Safety Constraint
      ↓
Actuator
```

### 31.2 Safety Authority

Safety WAJIB dapat melakukan:

1. allow.
2. reject.
3. clamp.
4. override.
5. emergency stop.

### 31.3 Priority Safety

Safety WAJIB memiliki priority lebih tinggi daripada learning.

Learning TIDAK BOLEH:

1. Menonaktifkan safety.
2. Melewati safety.
3. Mengubah safety limit tanpa otorisasi eksplisit.
4. Menghasilkan aksi yang melanggar safety constraint.

### 31.4 Emergency Stop

Emergency stop command WAJIB memiliki sifat:

1. Non-droppable.
2. Highest priority.
3. Tidak boleh kalah oleh backpressure biasa.
4. Harus menghasilkan safe actuator state.

---

## 32. Feedback Contract

Feedback WAJIB mengikuti pola:

```text
Action
 ↓
Environment
 ↓
Sensor
 ↓
Result
 ↓
Prediction Comparison
 ↓
Prediction Error
 ↓
Hippocampus
```

Prediction error WAJIB menjadi sinyal utama untuk pembelajaran adaptif.

---

# BAGIAN IX — LEARNING CONTRACT

---

## 33. Core Learning Contract

### 33.1 Learning Primitives

Core learning WAJIB menggunakan primitive berikut:

1. Hebbian association.
2. Temporal association.
3. Synaptic reinforcement.
4. Synaptic decay.
5. Prediction error.
6. Experience replay.
7. Consolidation.

### 33.2 Non-Transformer Core

Transformer TIDAK BOLEH menjadi core learning architecture.

Jika komponen Transformer digunakan, komponen tersebut:

1. Bersifat opsional.
2. Tidak boleh menggantikan Neural Core.
3. Tidak boleh menjadi jalur learning utama.
4. Tidak boleh melanggar offline-first dan resource boundedness.

Namun untuk conformance penuh, arsitektur inti tetap harus beroperasi tanpa bergantung pada Transformer.

---

## 34. Experience Replay Contract

### 34.1 Replay Flow

```text
Hippocampus
      ↓
Replay Selection
      ↓
Experience Replay
      │
      ├────► Cortex
      │
      └────► Cerebellum
```

### 34.2 Replay Priority

Replay selection WAJIB mempertimbangkan minimal:

1. prediction_error.
2. novelty.
3. importance.
4. reward.
5. failure.
6. recurrence.

### 34.3 Replay Scheduling

Replay WAJIB berjalan pada low/background priority.

Replay TIDAK BOLEH mengganggu control loop.

---

## 35. Adaptive Consolidation Contract

### 35.1 Tidak Menggunakan Threshold Statis Tunggal

Consolidation TIDAK BOLEH hanya menggunakan satu threshold statis tunggal.

### 35.2 Consolidation Inputs

Consolidation WAJIB mempertimbangkan minimal:

1. frequency.
2. success.
3. stability.
4. recurrence.
5. novelty.
6. prediction_error.
7. reward.
8. context_diversity.
9. relevance.

### 35.3 Consolidation Outputs

Consolidation WAJIB menghasilkan salah satu atau lebih keputusan berikut:

```text
KEEP
CONSOLIDATE → Cortex
CONSOLIDATE → Cerebellum
CONSOLIDATE → Both
COMPRESS
DELETE
```

---

## 36. Knowledge Promotion Contract

### 36.1 Knowledge Promotion Pattern

```text
Experience A
Experience B
Experience C
      ↓
Repeated Pattern
      ↓
Stable Relation
      ↓
Cross-context validation
      ↓
Cortex Candidate
      ↓
Promotion
```

### 36.2 Syarat Knowledge Promotion

Knowledge promotion WAJIB mempertimbangkan:

1. Recurrence.
2. Confidence.
3. Stability.
4. Context diversity.
5. Prediction error.
6. Failure rate.
7. Contradiction.

Satu pengalaman tunggal TIDAK BOLEH otomatis menjadi permanent knowledge.

---

## 37. Skill Promotion Contract

### 37.1 Skill Promotion Pattern

```text
State
 ↓
Action
 ↓
Feedback
 ↓
Correction
 ↓
Repeated execution
 ↓
High success
 ↓
Low error
 ↓
Stable context
 ↓
Cerebellum Candidate
 ↓
Promotion
```

### 37.2 Syarat Skill Promotion

Skill promotion WAJIB mempertimbangkan:

1. Success rate.
2. Error rate.
3. Stability.
4. Context diversity.
5. Repetition.
6. Feedback quality.

Satu keberhasilan tunggal TIDAK BOLEH otomatis menjadi permanent skill.

---

## 38. Contradiction Contract

Jika knowledge baru bertentangan dengan knowledge lama, runtime WAJIB melakukan:

```text
New Experience
 ↓
Conflict Detection
 ↓
Contradiction
 ↓
Context Analysis
```

Kemungkinan resolusi:

| Kondisi | Resolusi |
|---|---|
| Same context | Update. |
| Different context | Contextualize. |
| Repeated contradiction | Revise. |

Knowledge lama TIDAK BOLEH langsung dihapus hanya karena satu contradiction.

---

## 39. Skill Failure Contract

Jika skill gagal:

```text
Skill
 ↓
Execution
 ↓
Failure
 ↓
Prediction Error
 ↓
Hippocampus
 ↓
Replay
 ↓
Adjustment
```

Satu failure TIDAK BOLEH otomatis menghapus skill.

Skill failure WAJIB digunakan sebagai sinyal evaluasi, bukan penghapusan langsung.

---

# BAGIAN X — RETENTION AND GC CONTRACT

---

## 40. Retention Engine Contract

Retention WAJIB menggunakan minimal:

1. age.
2. frequency.
3. access_count.
4. novelty.
5. importance.
6. reward.
7. success.
8. prediction_error.
9. relevance.
10. recurrence.
11. context_diversity.
12. consolidation_state.

### 40.1 Retention Decision

Secara konseptual:

```text
High retention
    ↓
KEEP

Medium retention
    ↓
COMPRESS / COLD

Low retention
    ↓
DELETE
```

---

## 41. Hippocampus Garbage Collection Contract

### 41.1 Memory Pressure States

Hippocampus memory pressure WAJIB memiliki state minimal:

```text
0–60%     NORMAL
60–75%    MONITOR
75–85%    CONSOLIDATE
85–95%    AGGRESSIVE GC
>95%      EMERGENCY GC
```

Threshold BOLEH dikonfigurasi, tetapi WAJIB mempertahankan urutan state yang monotonik dan keberadaan emergency state.

### 41.2 GC Bukan Sekadar TTL

GC TIDAK BOLEH hanya menghapus episode tertua.

GC WAJIB mempertimbangkan retention value.

---

## 42. GC Pipeline Contract

GC pipeline WAJIB mengikuti pola:

```text
Memory Pressure
 ↓
Stop low-priority allocation
 ↓
Evaluate retention
 ↓
Consolidate valuable episodes
 ↓
Compress medium-value episodes
 ↓
Delete low-value episodes
 ↓
Rebuild free list
 ↓
Update index
 ↓
Resume
```

### 42.1 GC dan Control Loop

GC WAJIB berjalan dalam maintenance budget.

GC TIDAK BOLEH memblokir control loop secara tidak terbatas.

---

## 43. HOT/WARM/COLD Contract

### 43.1 Tiering

`brain.anr` WAJIB mendukung logical tier:

```text
HOT
WARM
COLD
```

### 43.2 Definisi Tier

| Tier | Makna |
|---|---|
| HOT | Current/active data. |
| WARM | Frequently accessed but inactive. |
| COLD | Rarely accessed/compressed. |

### 43.3 Lokasi Tier

Semua tier WAJIB tetap berada dalam:

```text
brain.anr
```

Tier TIDAK BOLEH menghasilkan file persistent terpisah yang wajib dalam deployment.

---

# BAGIAN XI — STORAGE CONTRACT

---

## 44. Storage Validation Contract

Startup WAJIB memeriksa:

1. magic.
2. version.
3. header size.
4. offsets.
5. sizes.
6. section boundaries.
7. generation.
8. checksum.

Runtime TIDAK BOLEH langsung mmap dan mempercayai offset dari file tanpa validasi.

---

## 45. Transactional Write Contract

Penulisan `brain.anr` WAJIB transactional.

Pola minimal:

```text
Generation N
 ↓
Prepare N+1
 ↓
Write
 ↓
Flush
 ↓
Validate
 ↓
Checksum
 ↓
Commit
```

### 45.1 Power Loss Behavior

Jika power loss terjadi:

```text
N+1 invalid
 ↓
fallback
 ↓
Generation N
```

Runtime WAJIB dapat kembali ke generation terakhir yang valid.

---

## 46. Recovery Contract

Startup recovery WAJIB mengikuti pola:

```text
Open brain.anr
 ↓
Validate
 ↓
Find latest valid generation
 ↓
Recover
 ↓
Initialize runtime
```

### 46.1 Partial Corruption

Jika satu region corrupt, runtime WAJIB:

```text
isolate region
 ↓
recover if possible
 ↓
degraded operation
```

jika degraded operation aman.

Jika tidak aman, runtime WAJIB menolak melanjutkan operasi normal.

---

## 47. Memory Mapping Contract

`brain.anr` TIDAK WAJIB dimuat seluruhnya ke RAM.

Pola tiering:

```text
brain.anr
 │
 ├── HOT → RAM
 ├── WARM → cache
 └── COLD → storage
```

Implementasi BOLEH menggunakan:

1. mmap.
2. buffered I/O.
3. mekanisme platform lain.

Namun mekanisme tersebut TIDAK BOLEH melanggar validasi dan transactional contract.

---

## 48. Brain Update Contract

Update brain WAJIB transactional:

```text
New Brain
 ↓
Format Validation
 ↓
Compatibility Check
 ↓
Integrity Check
 ↓
Resource Check
 ↓
Atomic Install
 ↓
Generation++
```

Jika gagal:

```text
rollback
```

ke brain sebelumnya.

Update TIDAK BOLEH meninggalkan state setengah-valid yang tidak dapat direcovery.

---

# BAGIAN XII — PROVISIONING CONTRACT

---

## 49. Initial Brain Provisioning Contract

ANR WAJIB mendukung initial brain provisioning.

Pengguna TIDAK BOLEH diwajibkan menyediakan tiga file memory terpisah.

### 49.1 Sumber Initial Brain

```text
Knowledge
    │
    ▼
Cortex seed

Validated Skills
    │
    ▼
Cerebellum seed

Demonstrations / Experiences
    │
    ▼
Hippocampus seed
```

Kemudian:

```text
Initial Brain Seed
       │
       ▼
Brain Builder
       │
       ▼
brain.anr
```

---

## 50. Brain Seed Contract

Brain Seed adalah input provisioning.

Brain Seed TIDAK BOLEH dianggap sebagai persistent runtime memory format.

Sumber Brain Seed BOLEH berasal dari:

1. knowledge.
2. procedures.
3. demonstrations.
4. experience datasets.
5. predefined patterns.
6. hardware-specific capabilities.

Brain Builder WAJIB mengubah seed menjadi neural representation yang sesuai dengan ANR.

---

## 51. Brain Builder Contract

Brain Builder WAJIB tetap berada dalam single binary.

Perintah minimal:

```text
anr brain init
anr brain build
anr brain verify
anr brain inspect
anr brain install
```

Tidak boleh ada executable tambahan wajib untuk provisioning inti.

### 51.1 Brain Builder Pipeline

Pipeline logis:

```text
Seed
 ↓
Validate
 ↓
Transform
 ↓
Build Neural Representation
 ↓
Allocate Sections
 ↓
Build Index
 ↓
Write brain.anr
 ↓
Integrity Check
```

---

## 52. Initial Cortex Contract

Knowledge yang sudah diketahui BOLEH langsung diprovisioning ke Cortex.

```text
Knowledge Source
      ↓
Pattern Extraction
      ↓
Generalization
      ↓
Neural Encoding
      ↓
Cortex
```

Knowledge yang sudah tervalidasi TIDAK WAJIB dimasukkan sebagai episode terlebih dahulu.

---

## 53. Initial Cerebellum Contract

Skill tervalidasi BOLEH langsung diprovisioning.

```text
Procedure / Demonstration
        ↓
Skill Extraction
        ↓
Validation
        ↓
Procedural Encoding
        ↓
Cerebellum
```

Contoh initial skills:

```text
move_forward
turn_left
turn_right
stop
grasp
release
```

---

## 54. Initial Hippocampus Contract

Initial Hippocampus bersifat opsional.

Initial Hippocampus BOLEH digunakan untuk:

1. demonstrations.
2. important experiences.
3. initial contextual episodes.
4. training experiences.

Episode initial TIDAK otomatis permanent.

Episode initial WAJIB tetap tunduk pada retention dan GC.

---

## 55. Initial Brain vs Learned Brain Contract

`brain.anr` WAJIB dapat merepresentasikan gabungan:

```text
brain.anr
    │
    ├─ Initial State
    └─ Learned State
```

Metadata origin WAJIB dapat mencatat minimal:

1. seed.
2. learned.
3. consolidated.
4. imported.

---

## 56. Factory Deployment Contract

Master brain BOLEH digunakan untuk banyak device:

```text
Master brain.anr
       │
 ┌─────┼─────┐
 ▼     ▼     ▼
 A     B     C
```

Setiap device BOLEH memiliki:

1. initial knowledge.
2. initial skills.
3. optional initial experiences.

Setelah beroperasi, setiap device BOLEH membentuk:

```text
Device-specific experience
       ↓
Device-specific brain.anr
```

---

# BAGIAN XIII — RESOURCE CONTRACT

---

## 57. Scheduler Contract

Scheduler WAJIB mendukung priority class:

```text
REALTIME
HIGH
NORMAL
LOW
BACKGROUND
```

Mapping minimal:

| Fungsi | Priority |
|---|---|
| Safety / Control | REALTIME |
| Sensor / Perception | HIGH |
| Decision | HIGH |
| Experience Recording | NORMAL |
| Learning | LOW |
| Replay | LOW |
| Consolidation | BACKGROUND |
| GC | BACKGROUND |
| Compression | BACKGROUND |

---

## 58. Maintenance Budget Contract

Maintenance WAJIB menggunakan budget.

```text
maintenance
 ↓
execute limited work
 ↓
yield
 ↓
control
 ↓
resume
```

Tujuan normatif:

1. GC tidak mengganggu control loop.
2. Learning tidak mengganggu control loop.
3. Replay tidak mengganggu control loop.
4. Compression tidak mengganggu control loop.
5. Consolidation tidak mengganggu control loop.

---

## 59. Queue Contract

### 59.1 Bounded Queues

Semua queue kritis WAJIB bounded:

1. Sensor Queue.
2. Camera Queue.
3. Perception Queue.
4. Action Queue.
5. Experience Queue.
6. Learning Queue.
7. Maintenance Queue.

TIDAK BOLEH ada unbounded queue pada production embedded configuration.

### 59.2 Backpressure Policy

Queue penuh WAJIB memiliki policy:

1. drop_oldest.
2. drop_newest.
3. sample.
4. merge.
5. compress.
6. backpressure.

Policy BOLEH berbeda per queue.

### 59.3 Safety Queue Exception

Safety-critical command TIDAK BOLEH diperlakukan seperti data sensor biasa.

Emergency stop WAJIB tidak boleh di-drop karena backpressure normal.

---

## 60. RAM Strategy Contract

ANR WAJIB dapat berjalan pada RAM terbatas.

Strategi WAJIB mencakup:

1. sparse representation.
2. SoA.
3. bounded buffers.
4. memory mapping.
5. HOT/WARM/COLD.
6. compression.
7. lazy loading.
8. bounded queues.

---

# BAGIAN XIV — COMPUTE CONTRACT

---

## 61. SIMD Contract

ANR WAJIB memiliki SIMD abstraction:

```text
SIMD
 │
 ├── NEON
 ├── AVX2
 ├── AVX-512
 └── Scalar
```

CPU feature detection WAJIB dilakukan saat startup.

### 61.1 Scalar Fallback

Scalar fallback WAJIB tersedia.

Sistem TIDAK BOLEH gagal hanya karena SIMD tertentu tidak tersedia.

---

## 62. SIMD Workloads Contract

Prioritas vectorization WAJIB mencakup:

1. activation.
2. weighted accumulation.
3. dot product.
4. similarity.
5. pattern matching.
6. prediction.
7. synaptic update.
8. decay.
9. normalization.

---

## 63. GPU Contract

GPU bersifat optional.

Jika GPU digunakan:

1. Harus opsional.
2. Harus dapat dinonaktifkan.
3. Tidak boleh mengubah invariant arsitektur.
4. Tidak boleh menjadi dependensi operasi inti.

---

# BAGIAN XV — RUST IMPLEMENTATION BOUNDARY CONTRACT

---

## 64. Language Contract

Implementasi ANR WAJIB menggunakan Rust.

Rust dipilih sebagai bahasa utama untuk runtime inti.

---

## 65. Repository Structure Contract

Struktur repository WAJIB mengikuti pola minimal:

```text
anr/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
│
├── src/
│   ├── main.rs
│   ├── error.rs
│   │
│   ├── core/
│   ├── neural/
│   ├── brain/
│   ├── learning/
│   ├── memory/
│   ├── storage/
│   ├── perception/
│   ├── plugins/
│   ├── hardware/
│   ├── action/
│   ├── simd/
│   └── interface/
│
├── tests/
├── benches/
├── examples/
└── docs/
```

---

## 66. Module Responsibility Contract

| Module | Responsibility |
|---|---|
| `core/` | lifecycle, scheduler, runtime. |
| `neural/` | Cell, Column, Block, Synapse. |
| `brain/` | Cortex, Cerebellum, Hippocampus. |
| `learning/` | Hebbian, temporal, reinforcement, replay, prediction. |
| `memory/` | allocation, retention, GC, tier, compression. |
| `storage/` | brain.anr, transaction, checksum, recovery. |
| `perception/` | sensor representation, preprocessing, fusion. |
| `plugins/` | sensor/camera/audio/robot plugins. |
| `hardware/` | HAL. |
| `action/` | decision, actuator, safety, feedback. |
| `simd/` | NEON/AVX/scalar. |
| `interface/` | CLI/diagnostics. |

---

## 67. Dependency Boundary Contract

Arsitektur modul WAJIB menjaga boundary berikut:

1. Neural Core TIDAK BOLEH bergantung langsung pada HAL.
2. Learning TIDAK BOLEH mengirim perintah actuator tanpa decision/safety.
3. Storage TIDAK BOLEH mengubah semantic learning secara langsung.
4. Perception TIDAK BOLEH melewati validasi sensor.
5. Safety TIDAK BOLEH di-bypass oleh decision atau learning.
6. Plugins TIDAK BOLEH mengakses neural state secara tidak terkontrol.

---

## 68. Unsafe and Panic Contract

### 68.1 Unsafe

Unsafe Rust BOLEH digunakan hanya untuk:

1. SIMD.
2. Memory mapping.
3. FFI/platform.
4. Optimasi performance kritis.

Namun unsafe WAJIB:

1. Diisolasi.
2. Dibatasi.
3. Tidak merusak safe API boundary.
4. Tidak melanggar memory boundedness.

### 68.2 Panic

Panic pada control path dan safety path SEBAIKNYA dihindari.

Jika panic terjadi, runtime WAJIB berusaha masuk ke safe state atau degraded state jika memungkinkan.

---

# BAGIAN XVI — CLI AND DIAGNOSTICS CONTRACT

---

## 69. CLI Contract

CLI WAJIB berada dalam binary `anr`.

Perintah minimum runtime:

```text
anr run
anr status
anr memory
anr inspect
anr learn
anr consolidate
```

Perintah minimum brain provisioning:

```text
anr brain init
anr brain build
anr brain verify
anr brain inspect
anr brain install
```

Tidak boleh ada executable terpisah yang wajib untuk perintah-perintah tersebut.

---

## 70. Diagnostics Contract

`anr status` WAJIB menyediakan minimal:

1. Runtime state.
2. CPU.
3. RAM.
4. Storage.
5. SIMD backend.
6. Cortex usage.
7. Cerebellum usage.
8. Hippocampus usage.
9. HOT/WARM/COLD state.
10. Episode rate.
11. Learning rate.
12. Replay rate.
13. Consolidation rate.
14. GC rate.
15. Sensor status.
16. Camera status.
17. Plugin status.

---

## 71. Telemetry Contract

Telemetry internal WAJIB mencakup minimal:

```text
sensor_frames
dropped_frames

episodes_created
episodes_deleted

promotions
contradictions

prediction_errors
successful_actions
failed_actions

gc_cycles
bytes_reclaimed
bytes_compressed

brain_generation
brain_size
```

Telemetry TIDAK BOLEH mewajibkan cloud.

Telemetry WAJIB dapat diakses lokal melalui diagnostics atau file lokal opsional.

---

# BAGIAN XVII — SECURITY CONTRACT

---

## 72. Brain Data Security Contract

`brain.anr` adalah data, bukan executable.

Runtime WAJIB:

1. Validate offsets.
2. Validate lengths.
3. Validate versions.
4. Validate allocations.
5. Validate indexes.
6. Validate checksum.

Runtime TIDAK BOLEH mengeksekusi data neural sebagai code.

---

## 73. Untrusted Brain Contract

Brain file yang berasal dari sumber tidak terpercaya WAJIB diperlakukan sebagai untrusted data.

Runtime WAJIB:

1. Menolak brain yang tidak valid.
2. Menolak brain yang corrupt.
3. Menolak brain dengan version tidak kompatibel kecuali upgrade eksplisit.
4. Tidak menjalankan payload dari brain.

---

## 74. Plugin Security Contract

Plugin WAJIB:

1. Memiliki interface terbatas.
2. Tidak boleh mengakses brain secara langsung tanpa validasi.
3. Tidak boleh melewati safety layer.
4. Tidak boleh menyebabkan unbounded allocation.
5. Harus dapat di-disable jika gagal.

---

# BAGIAN XVIII — HARDWARE TARGET CONTRACT

---

## 75. Minimum Hardware Target

Target minimum:

```text
ARM64
2 CPU cores
256–512 MB RAM
≥1 GB storage
NEON
```

### 75.1 Scalar Fallback

Meskipun ARM64 minimum mengharapkan NEON, scalar fallback WAJIB tetap ada untuk portabilitas.

---

## 76. Recommended Hardware Target

Target recommended:

```text
ARM64
4–8 CPU cores
2–4 GB RAM
16–32 GB storage
NEON
```

---

## 77. Edge Hardware Target

Target edge:

```text
ARM64 / x86-64
4–8+ cores
4–16 GB RAM
32+ GB storage
NEON / AVX2
```

GPU optional.

---

# BAGIAN XIX — PERFORMANCE CONTRACT

---

## 78. Performance Priority Contract

Jika terjadi trade-off, prioritas WAJIB:

1. Safety.
2. Control latency.
3. Memory boundedness.
4. Cache locality.
5. SIMD.
6. Parallelism.
7. Storage efficiency.
8. Learning throughput.

Learning throughput TIDAK BOLEH mengorbankan safety atau control latency.

---

## 79. Determinism Contract

Control path WAJIB deterministic dalam batas:

```text
same input
+
same neural state
+
same configuration
```

Learning BOLEH memiliki:

1. deterministic mode.
2. adaptive mode.

Deterministic mode WAJIB tersedia untuk debugging, testing, dan reproduksi.

---

# BAGIAN XX — VERIFICATION CONTRACT

---

## 80. Testing Contract

Implementasi WAJIB memiliki pengujian untuk memastikan kontrak arsitektur.

### 80.1 Unit Tests

Unit tests WAJIB mencakup minimal:

1. Cell.
2. Column.
3. Block.
4. Synapse.
5. Allocator.
6. Retention.
7. GC.
8. ANR parser.
9. Checksum.
10. SIMD kernels.

### 80.2 Integration Tests

Integration tests WAJIB mencakup minimal:

```text
Sensor → Perception
Camera → Perception
Perception → Neural Core
Neural → Decision
Decision → Actuator
Experience → Hippocampus
Hippocampus → Consolidation
Storage → Recovery
```

### 80.3 Fault Tests

Fault tests WAJIB mencakup minimal:

1. power loss.
2. corrupt brain.
3. full storage.
4. full Hippocampus.
5. camera failure.
6. sensor failure.
7. plugin failure.
8. queue overflow.

---

## 81. Benchmark Contract

Benchmark WAJIB mencakup minimal:

1. Cell activation.
2. Column activation.
3. Block update.
4. Synapse update.
5. Similarity.
6. Pattern matching.
7. Prediction.
8. Learning.
9. Replay.
10. Consolidation.
11. GC.
12. Compression.
13. brain read.
14. brain write.
15. Recovery.
16. Camera preprocessing.
17. Sensor fusion.

Benchmark SIMD WAJIB membandingkan, jika hardware mendukung:

```text
Scalar
NEON
AVX2
AVX-512
```

---

# BAGIAN XXI — CONFORMANCE CONTRACT

---

## 82. Architectural Invariants

Implementasi ANR WAJIB mempertahankan invariant berikut:

```text
Rust
single binary
single brain.anr
offline-first
no mandatory cloud
no mandatory GPU
non-Transformer core

Cell → Column → Block
Cortex = Knowledge
Cerebellum = Skill
Hippocampus = Experience

independent memory allocation
automatic Hippocampus GC
adaptive consolidation
HOT/WARM/COLD
SIMD + scalar fallback
SoA
bounded queues
transactional persistence
recovery
sensor/camera plugin architecture
HAL
safety boundary
```

---

## 83. Conformance Criteria

Sebuah implementasi dianggap conformant jika:

1. Memenuhi seluruh ketentuan WAJIB dalam dokumen ini.
2. Tidak melanggar ketentuan DILARANG.
3. Dapat menjalankan deployment minimal:

```text
/opt/anr/
├── anr
└── brain.anr
```

4. Dapat boot dari `brain.anr` yang valid.
5. Dapat recover dari generation valid sebelumnya jika generation terbaru corrupt.
6. Dapat menjalankan autonomous loop secara lokal.
7. Dapat melakukan brain provisioning dalam binary yang sama.
8. Tidak memerlukan cloud untuk operasi inti.
9. Tidak memerlukan GPU untuk operasi inti.
10. Menjaga safety boundary pada setiap aksi.

---

# BAGIAN XXII — CHANGE CONTROL CONTRACT

---

## 84. Change Authority

Dokumen ini adalah Architecture Contract.

Perubahan terhadap dokumen ini WAJIB diperlakukan sebagai perubahan arsitektural.

Perubahan tidak boleh dilakukan secara implisit melalui:

1. Implementation detail.
2. Test case.
3. Configuration default.
4. Deployment script.
5. Plugin behavior.

---

## 85. Backward Compatibility

Perubahan brain format WAJIB memiliki:

1. `FORMAT_VERSION`.
2. Compatibility check.
3. Migration path jika didukung.
4. Rollback path jika update gagal.

Runtime WAJIB menolak brain yang tidak kompatibel jika tidak ada jalur upgrade yang aman dan eksplisit.

---

## 86. Contract Stability

Dokumen ini dimaksudkan sebagai baseline final yang stabil.

Dokumen ini TIDAK BOLEH berubah menjadi:

1. Roadmap.
2. Release plan.
3. Implementation schedule.
4. Sprint backlog.
5. Feature wishlist.

Jika diperlukan detail implementasi, dokumen turunan harus dibuat sebagai Implementation Specification dan harus tetap tunduk pada Architecture Contract ini.

---

# BAGIAN XXIII — END-TO-END ARCHITECTURAL MODEL

---

## 87. End-to-End Contractual Diagram

Model end-to-end berikut adalah normatif secara arsitektural:

```text
WORLD
                           │
                           ▼
              ┌────────────────────────┐
              │ Sensors / Camera / Audio│
              └────────────┬───────────┘
                           │
                           ▼
                    Plugin / HAL
                           │
                           ▼
                      Perception
                           │
                           ▼
                 ┌─────────────────┐
                 │   Neural Core   │
                 │                 │
                 │ Cell            │
                 │ Column          │
                 │ Block           │
                 │ Synapse         │
                 └────────┬────────┘
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       Cortex        Cerebellum       Hippocampus
      Knowledge         Skill           Episode
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                   Decision Engine
                          │
                          ▼
                     Safety Layer
                          │
                          ▼
                       Actuator
                          │
                          ▼
                        WORLD
                          │
                          ▼
                      Feedback
                          │
                          ▼
                    Hippocampus
                          │
                          ▼
                       Learning
                          │
                  ┌───────┴───────┐
                  ▼               ▼
                Replay      Prediction Error
                  │               │
                  └───────┬───────┘
                          ▼
                    Consolidation
                     /          \
                    ▼            ▼
                 Cortex      Cerebellum
                    │            │
                    └─────┬──────┘
                          ▼
                    Retention
                          │
                 ┌────────┼────────┐
                 ▼        ▼        ▼
               KEEP    COMPRESS   DELETE
                 │        │        │
                 └────────┼────────┘
                          ▼
                       brain.anr
```

---

## 88. Final Deployment Contract Diagram

```text
ANR
  │
  ├── anr
  │    ├── Runtime
  │    ├── Neural Core
  │    ├── Learning
  │    ├── Memory Manager
  │    ├── Storage
  │    ├── Perception
  │    ├── Plugin System
  │    ├── HAL
  │    ├── Decision Engine
  │    ├── Safety Layer
  │    ├── CLI
  │    └── Diagnostics
  │
  └── brain.anr
       ├── Cortex
       ├── Cerebellum
       └── Hippocampus
```

---

# BAGIAN XXIV — FINAL DEFINITION

---

## 89. Definisi Final Arsitektur

ANR adalah embedded autonomous neural runtime berbasis Rust yang berjalan sebagai satu binary `anr` dan menggunakan satu persistent neural memory `brain.anr`.

Neural Core menggunakan:

```text
Cell → Column → Block
```

dengan Synapse sebagai koneksi.

Transformer TIDAK menjadi core architecture.

Cortex merepresentasikan knowledge yang telah digeneralisasi.

Cerebellum merepresentasikan skill/procedure yang telah terbukti.

Hippocampus merepresentasikan pengalaman episodik.

Sensor, kamera, audio, dan perangkat robot masuk melalui plugin/HAL menuju perception layer dan kemudian Neural Core.

Pengalaman baru masuk ke Hippocampus, dievaluasi berdasarkan novelty, recurrence, reward, prediction error, relevance, success, stability, dan context diversity, kemudian direplay dan dikonsolidasikan secara adaptif menjadi Cortex dan/atau Cerebellum.

Hippocampus memiliki allocation independen, retention management, compression, serta automatic garbage collection berbasis memory pressure.

Initial Cortex, Cerebellum, dan optional Hippocampus dapat diprovisioning melalui Brain Seed dan `anr brain build`, kemudian semuanya dikompilasi menjadi satu `brain.anr`.

Setelah deployment, brain tersebut menjadi persistent living state yang terus berubah melalui experience, learning, consolidation, dan GC.

ANR menggunakan:

```text
bounded memory
bounded queues
SoA data layout
SIMD NEON/AVX2/AVX-512 dengan scalar fallback
transactional storage
checksum/integrity validation
crash recovery
memory isolation
priority scheduling
sensor/camera plugin architecture
HAL
safety layer
```

Control dan safety selalu memiliki prioritas di atas learning dan memory maintenance.

Tujuan akhir arsitektur ini adalah autonomous intelligence yang dapat:

1. Merasakan lingkungan.
2. Membentuk pengalaman.
3. Mengubah pengalaman menjadi pengetahuan dan kemampuan.
4. Mempertahankan memory yang bernilai.
5. Membuang pengalaman yang tidak bernilai.
6. Melakukan seluruh proses tersebut secara lokal pada IoT, robot, dan edge hardware tanpa ketergantungan cloud.

---

## 90. Penutup Kontraktual

Dokumen ini adalah Architecture Contract final untuk ANR.

Dokumen ini mengunci arsitektur dan menjadi acuan tertinggi untuk seluruh dokumen turunan teknis.

Setiap dokumen implementasi, pengujian, atau deployment WAJIB merujuk pada kontrak ini dan TIDAK BOLEH mengubah invariant yang telah ditetapkan di sini tanpa perubahan kontrak yang eksplisit.
