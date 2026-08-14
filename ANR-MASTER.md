ANR — Master Technical Specification

Final Architectural Baseline v1.1

Status: Final Architectural Baseline
Target: IoT, embedded, robot, edge autonomous system
Implementation: Rust
Deployment: single binary + single persistent brain file
Core: Cell → Column → Block → Synapse
Learning: non-Transformer
Compute: CPU + SIMD, GPU optional
Operation: offline-first, no mandatory cloud


---

1. Executive Definition

ANR adalah Autonomous Neural Runtime yang menjalankan neural system lokal pada perangkat embedded/robot.

ANR mempunyai dua artefak deployment utama:

/opt/anr/
├── anr
└── brain.anr

anr adalah seluruh executable runtime.

brain.anr adalah satu-satunya persistent neural memory.

Secara logical:

brain.anr
│
├── Cortex
├── Cerebellum
└── Hippocampus

Ketiga subsystem tersebut bukan file terpisah.


---

2. Core Philosophy

ANR memisahkan tiga fungsi memori:

Cortex
= apa yang AI tahu

Cerebellum
= bagaimana AI melakukannya

Hippocampus
= apa yang AI alami/lakukan

Hubungan:

Experience
    │
    ▼
Hippocampus
    │
    ├──── knowledge ────► Cortex
    │
    └──── skill ────────► Cerebellum


---

3. Architectural Model

WORLD
  │
  ▼
SENSORS / CAMERA / AUDIO
  │
  ▼
PLUGIN + HAL
  │
  ▼
PERCEPTION
  │
  ▼
NEURAL CORE
  │
  ├── Cell
  ├── Column
  ├── Block
  └── Synapse
  │
  ├────────────┬─────────────┐
  ▼            ▼             ▼
CORTEX     CEREBELLUM   HIPPOCAMPUS
Knowledge      Skill       Experience
  │            │             │
  └────────────┼─────────────┘
               ▼
        DECISION ENGINE
               │
               ▼
         SAFETY LAYER
               │
               ▼
           ACTUATORS
               │
               ▼
             WORLD
               │
               ▼
           FEEDBACK
               │
               ▼
          HIPPOCAMPUS
               │
               ▼
           LEARNING
               │
               ▼
        CONSOLIDATION
          /         \
         ▼           ▼
      CORTEX     CEREBELLUM
         │           │
         └─────┬─────┘
               ▼
          MEMORY GC
               │
               ▼
           brain.anr


---

4. Single Binary Requirement

Binary anr mencakup:

Runtime
Neural Core
Cortex
Cerebellum
Hippocampus
Learning
Replay
Consolidation
Memory Manager
Garbage Collector
Storage
Recovery
SIMD
Perception
Plugin system
Hardware abstraction
Decision
Safety
Actuator interface
CLI
Diagnostics
Brain provisioning
Brain validation

Tidak membutuhkan service terpisah untuk operasi inti.

Tidak membutuhkan:

Python
Node.js
database server
LLM server
cloud inference
external model server


---

5. Single Brain Requirement

Persistent neural state harus berada pada:

brain.anr

Tidak ada:

cortex.cx
cerebellum.cm
hippocampus.hs

sebagai persistent deployment files.

Terminologi tersebut hanya boleh digunakan untuk logical sections dan conceptual architecture.


---

6. Neural Core

Hierarchy:

Cell
  │
  ▼
Column
  │
  ▼
Block

Synapse menyediakan koneksi:

Cell/Column
     │
     ▼
Synapse
     │
     ▼
Cell/Column


---

7. Cell

Cell adalah unit neural terkecil.

Logical state:

activation
potential
threshold
state
refractory state
activity timestamp
usage

Implementasi harus menghindari object-per-cell allocation yang berlebihan.


---

8. Column

Column adalah kumpulan Cell dengan representasi lokal.

Fungsi:

local representation
competition
sparse activation
association
temporal activity

Contoh:

Column
├── Cell 0
├── Cell 1
├── Cell 2
└── Cell N


---

9. Block

Block adalah unit konteks neural yang lebih besar.

Digunakan untuk:

context
sequence
temporal representation
episode representation
local prediction
association

Block neural bukan filesystem block.


---

10. Synapse

Minimal:

source
target
weight
state

Optional:

strength
usage
age
last_active
plasticity

Mendukung:

Hebbian reinforcement
temporal association
decay
strengthening
weakening
pruning


---

11. Sparse Computation

ANR menggunakan sparse/local computation.

Tidak seluruh neural graph harus dihitung setiap cycle.

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

Tujuan:

low CPU usage,

low memory bandwidth,

cache locality,

embedded suitability.



---

12. Data Layout

Production layout:

Structure of Arrays

Contoh:

activations[]
thresholds[]
states[]
weights[]
targets[]
timestamps[]
usage[]

bukan:

Cell object
Cell object
Cell object
...

Tujuan:

cache locality
SIMD
prefetching
sequential access
low overhead


---

13. Cortex

Cortex adalah long-term knowledge memory.

Fungsi:

pattern generalization
semantic association
stable relationships
contextual knowledge

Pipeline:

Episode A ─┐
Episode B ─┼──► Pattern ──► Cortex
Episode C ─┘

Karakteristik:

long-term
generalized
persistent
sparse
associative
conservative GC

Cortex tidak digunakan sebagai raw event log.


---

14. Cerebellum

Cerebellum menyimpan procedural capability.

Pipeline:

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

Karakteristik:

procedural
action-oriented
predictive
error-driven
persistent
very conservative GC


---

15. Hippocampus

Hippocampus adalah episodic memory.

Episode dapat berisi:

timestamp
context
observation reference
internal state
action
result
reward
prediction
prediction error
novelty
importance
references

Karakteristik:

high-write
high-churn
temporary
episodic
GC-friendly


---

16. Initial Brain Provisioning

ANR harus mendukung initial brain provisioning.

Pengguna tidak perlu menyediakan tiga file memory.

Sumber initial brain:

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

Kemudian:

Initial Brain Seed
       │
       ▼
Brain Builder
       │
       ▼
brain.anr


---

17. Brain Seed

Brain Seed adalah input provisioning, bukan persistent runtime memory format.

Sumber dapat berasal dari:

knowledge
procedures
demonstrations
experience datasets
predefined patterns
hardware-specific capabilities

Brain Builder mengubahnya menjadi neural representation yang sesuai dengan ANR.


---

18. Brain Builder

Tetap single binary.

Contoh:

anr brain init
anr brain build
anr brain verify
anr brain inspect
anr brain install

Tidak ada executable tambahan wajib.

Pipeline:

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


---

19. Initial Cortex

Knowledge yang sudah diketahui dapat langsung diprovisioning ke Cortex.

Knowledge Source
      ↓
Pattern Extraction
      ↓
Generalization
      ↓
Neural Encoding
      ↓
Cortex

Tidak perlu memasukkannya sebagai episode terlebih dahulu jika knowledge tersebut memang sudah tervalidasi.


---

20. Initial Cerebellum

Skill tervalidasi dapat langsung diprovisioning.

Procedure / Demonstration
        ↓
Skill Extraction
        ↓
Validation
        ↓
Procedural Encoding
        ↓
Cerebellum

Contoh:

move_forward
turn_left
turn_right
stop
grasp
release


---

21. Initial Hippocampus

Opsional.

Digunakan untuk:

demonstrations
important experiences
initial contextual episodes
training experiences

Episode initial tetap dapat terkena retention/GC.

Jadi initial experience tidak otomatis permanent.


---

22. Initial Brain vs Learned Brain

brain.anr
                    │
          ┌─────────┴─────────┐
          ▼                   ▼
   Initial State         Learned State
          │                   │
    provisioned            acquired
          │                   │
          └─────────┬─────────┘
                    ▼
               Current Brain

Metadata origin dapat mencatat:

seed
learned
consolidated
imported


---

23. Autonomous Loop

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


---

24. Perception

Pipeline:

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


---

25. Sensor Architecture

Sensor masuk melalui plugin/HAL.

Sensor
  ↓
Sensor Plugin
  ↓
HAL
  ↓
Perception

Logical interface:

trait SensorPlugin {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn read(&mut self) -> Result<SensorFrame>;
    fn capabilities(&self) -> SensorCapabilities;
}


---

26. Camera Architecture

Camera merupakan perception source khusus.

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

Camera plugin bertanggung jawab atas:

device discovery
stream
format
resolution
timestamp
buffer
frame dropping


---

27. Camera Buffer

Buffer harus bounded.

max_frames
max_frame_size
drop_policy

Tidak boleh ada unbounded frame queue.

Jika buffer penuh:

drop_oldest
drop_newest
sample
merge

sesuai policy.


---

28. Sensor Frame

Logical representation:

SensorFrame
├── sensor_id
├── timestamp
├── sequence
├── payload
├── dimensions
├── format
├── quality
└── flags


---

29. Hardware Abstraction Layer

HAL menyediakan:

GPIO
I2C
SPI
UART
USB
PWM
ADC

Robot-specific:

motor
servo
encoder
gripper
lidar

Neural Core tidak boleh bergantung langsung pada HAL.


---

30. Plugin Architecture

Plugin subsystem:

plugins/
├── sensors/
├── vision/
├── audio/
└── robotics/

Plugin failure harus isolated.

Plugin failure
 ↓
Error isolation
 ↓
Restart / disable
 ↓
Runtime continues


---

31. Decision Engine

Decision menggunakan:

current neural state
Cortex knowledge
Cerebellum skills
Hippocampus context
sensor state
goal
prediction
confidence

Pipeline:

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


---

32. Safety Layer

Neural decision bukan satu-satunya safety boundary.

Neural Decision
      ↓
Safety Constraint
      ↓
Actuator

Safety dapat:

allow
reject
clamp
override
emergency stop

Safety mempunyai priority lebih tinggi daripada learning.


---

33. Feedback

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

Prediction error adalah sinyal utama untuk pembelajaran adaptif.


---

34. Learning

Core learning primitive:

Hebbian association
Temporal association
Synaptic reinforcement
Synaptic decay
Prediction error
Experience replay
Consolidation

Transformer bukan bagian core learning architecture.


---

35. Experience Replay

Hippocampus
      ↓
Replay Selection
      ↓
Experience Replay
      │
      ├────► Cortex
      │
      └────► Cerebellum

Prioritas berdasarkan:

prediction_error
novelty
importance
reward
failure
recurrence

Replay berjalan pada low/background priority.


---

36. Adaptive Consolidation

Tidak menggunakan satu threshold statis.

Input:

frequency
success
stability
recurrence
novelty
prediction_error
reward
context_diversity
relevance

Output:

KEEP
CONSOLIDATE → Cortex
CONSOLIDATE → Cerebellum
CONSOLIDATE → Both
COMPRESS
DELETE


---

37. Knowledge Promotion

Pattern:

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


---

38. Skill Promotion

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


---

39. Promotion Safety

Satu pengalaman tidak otomatis menjadi permanent knowledge/skill.

Promotion harus mempertimbangkan:

recurrence
confidence
stability
context diversity
success
failure rate
prediction error


---

40. Contradiction

Jika knowledge baru bertentangan:

New Experience
 ↓
Conflict Detection
 ↓
Contradiction
 ↓
Context Analysis

Kemungkinan:

same context
→ update

different context
→ contextualize

repeated contradiction
→ revise

Knowledge lama tidak langsung dihapus.


---

41. Skill Failure

Jika skill gagal:

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

Satu failure tidak otomatis menghapus skill.


---

42. Retention Engine

Retention menggunakan:

age
frequency
access_count
novelty
importance
reward
success
prediction_error
relevance
recurrence
context_diversity
consolidation_state

Conceptual:

High retention
    ↓
KEEP

Medium
    ↓
COMPRESS / COLD

Low
    ↓
DELETE


---

43. Hippocampus Garbage Collection

Memory pressure:

0–60%     NORMAL
60–75%    MONITOR
75–85%    CONSOLIDATE
85–95%    AGGRESSIVE GC
>95%      EMERGENCY GC

GC tidak sekadar menghapus episode tertua.


---

44. GC Pipeline

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


---

45. Memory Allocation

Masing-masing memory mempunyai:

minimum
target
maximum

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

Tidak boleh saling mengambil hard allocation.


---

46. HOT/WARM/COLD

Logical storage tier:

HOT
WARM
COLD

HOT

Current/active data.

WARM

Frequently accessed but inactive.

COLD

Rarely accessed/compressed.

Semua tetap berada di:

brain.anr


---

47. brain.anr Structure

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


---

48. ANR Header

Baseline:

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

Format bit-level final dapat ditentukan pada implementation specification.


---

49. Storage Validation

Startup harus memeriksa:

magic
version
header size
offsets
sizes
section boundaries
generation
checksum

Tidak boleh langsung mmap dan mempercayai offset dari file.


---

50. Transactional Write

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

Power loss:

N+1 invalid
 ↓
fallback
 ↓
Generation N


---

51. Recovery

Startup:

Open brain.anr
 ↓
Validate
 ↓
Find latest valid generation
 ↓
Recover
 ↓
Initialize runtime

Jika satu region corrupt:

isolate region
 ↓
recover if possible
 ↓
degraded operation


---

52. Memory Mapping

brain.anr tidak wajib seluruhnya dimuat ke RAM.

brain.anr
 │
 ├── HOT → RAM
 ├── WARM → cache
 └── COLD → storage

Dapat menggunakan:

mmap

atau buffered I/O sesuai platform.


---

53. SIMD

Abstraction:

SIMD
 │
 ├── NEON
 ├── AVX2
 ├── AVX-512
 └── Scalar

CPU feature detection dilakukan saat startup.


---

54. SIMD Workloads

Prioritas vectorization:

activation
weighted accumulation
dot product
similarity
pattern matching
prediction
synaptic update
decay
normalization

Scalar fallback wajib tersedia.


---

55. Scheduler

Task priority:

REALTIME
HIGH
NORMAL
LOW
BACKGROUND

Mapping:

Safety / Control       REALTIME
Sensor / Perception    HIGH
Decision               HIGH
Experience Recording   NORMAL
Learning               LOW
Replay                 LOW
Consolidation          BACKGROUND
GC                     BACKGROUND
Compression            BACKGROUND


---

56. Maintenance Budget

Maintenance menggunakan budget.

maintenance
 ↓
execute limited work
 ↓
yield
 ↓
control
 ↓
resume

Targetnya agar:

GC
learning
replay
compression
consolidation

tidak mengganggu control loop.


---

57. Queue Architecture

Semua queue kritis harus bounded:

Sensor Queue
Camera Queue
Perception Queue
Action Queue
Experience Queue
Learning Queue
Maintenance Queue

Tidak ada unbounded queue pada production embedded configuration.


---

58. Backpressure

Queue penuh memiliki policy:

drop_oldest
drop_newest
sample
merge
compress
backpressure

Policy berbeda untuk sensor, camera, action, dan experience.


---

59. Resource Isolation

Memory Manager:

Memory Manager
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
    Cortex      Cerebellum   Hippocampus
       │            │            │
      max          max          max

Hippocampus tidak dapat menghabiskan reserved Cortex memory.


---

60. RAM Strategy

ANR harus dapat berjalan pada RAM terbatas.

Strategi:

sparse representation
SoA
bounded buffers
memory mapping
HOT/WARM/COLD
compression
lazy loading
bounded queues


---

61. Runtime Boot

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


---

62. Runtime Shutdown

Graceful:

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

Emergency shutdown memprioritaskan actuator safety dan persistent integrity.


---

63. Degraded Mode

ANR harus dapat melanjutkan operasi terbatas.

Contoh:

Camera failure
→ disable vision

Sensor failure
→ disable affected pathway

Plugin failure
→ restart/disable plugin

Hippocampus pressure
→ aggressive consolidation/GC

Storage failure
→ volatile/degraded mode jika diizinkan


---

64. Security Boundary

brain.anr adalah data, bukan executable.

Runtime wajib:

validate offsets
validate lengths
validate versions
validate allocations
validate indexes
validate checksum

Tidak boleh mengeksekusi data neural sebagai code.


---

65. Rust Architecture

Repository:

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


---

66. Module Responsibilities

core/
→ lifecycle, scheduler, runtime

neural/
→ Cell, Column, Block, Synapse

brain/
→ Cortex, Cerebellum, Hippocampus

learning/
→ Hebbian, temporal, reinforcement, replay, prediction

memory/
→ allocation, retention, GC, tier, compression

storage/
→ brain.anr, transaction, checksum, recovery

perception/
→ sensor representation, preprocessing, fusion

plugins/
→ sensor/camera/audio/robot plugins

hardware/
→ HAL

action/
→ decision, actuator, safety, feedback

simd/
→ NEON/AVX/scalar

interface/
→ CLI/diagnostics


---

67. CLI

Minimum:

anr run
anr status
anr memory
anr inspect
anr learn
anr consolidate

Brain provisioning:

anr brain init
anr brain build
anr brain verify
anr brain inspect
anr brain install

Semua command tetap berada dalam executable:

anr


---

68. Brain Provisioning Workflow

Development:

Knowledge
Skills
Demonstrations
Experience
     │
     ▼
Brain Seed
     │
     ▼
anr brain build
     │
     ▼
brain.anr

Deployment:

anr
brain.anr
     │
     ▼
Device

Boot:

brain.anr
 ↓
Cortex
Cerebellum
Hippocampus
 ↓
RUN


---

69. Factory Deployment

Master brain:

Master brain.anr
       │
 ┌─────┼─────┐
 ▼     ▼     ▼
A      B     C

Setiap device kemudian memiliki:

initial knowledge
initial skills
optional initial experiences

dan setelah beroperasi:

Device-specific experience
       ↓
Device-specific brain.anr


---

70. Brain Update

Update brain harus transactional:

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

Jika gagal:

rollback

ke brain sebelumnya.


---

71. Hardware Targets

Minimum

ARM64
2 CPU cores
256–512 MB RAM
≥1 GB storage
NEON

Recommended

ARM64
4–8 CPU cores
2–4 GB RAM
16–32 GB storage
NEON

Edge

ARM64 / x86-64
4–8+ cores
4–16 GB RAM
32+ GB storage
NEON / AVX2

GPU optional.


---

72. Testing

Unit:

Cell
Column
Block
Synapse
Allocator
Retention
GC
ANR parser
Checksum
SIMD

Integration:

Sensor → Perception
Camera → Perception
Perception → Neural Core
Neural → Decision
Decision → Actuator
Experience → Hippocampus
Hippocampus → Consolidation
Storage → Recovery

Fault:

power loss
corrupt brain
full storage
full Hippocampus
camera failure
sensor failure
plugin failure
queue overflow


---

73. Benchmark

Benchmark utama:

Cell activation
Column activation
Block update
Synapse update
Similarity
Pattern matching
Prediction
Learning
Replay
Consolidation
GC
Compression
brain read
brain write
Recovery
Camera preprocessing
Sensor fusion

SIMD:

Scalar
NEON
AVX2
AVX-512

dibandingkan jika hardware mendukung.


---

74. Determinism

Control path harus reproducible:

same input
+
same neural state
+
same configuration

→ keputusan yang sama dalam batas numeric platform.

Learning dapat memiliki:

deterministic mode
adaptive mode


---

75. Diagnostics

anr status minimal menyediakan:

Runtime
CPU
RAM
Storage
SIMD backend

Cortex usage
Cerebellum usage
Hippocampus usage

HOT/WARM/COLD

Episode rate
Learning rate
Replay rate
Consolidation rate
GC rate

Sensor status
Camera status
Plugin status


---

76. Telemetry

Internal metrics:

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

Tidak memerlukan cloud.


---

77. Performance Invariants

Prioritas:

1. Safety
2. Control latency
3. Memory boundedness
4. Cache locality
5. SIMD
6. Parallelism
7. Storage efficiency
8. Learning throughput


---

78. Architectural Invariants

ANR wajib mempertahankan:

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


---

79. Physical Deployment Model

/opt/anr/
│
├── anr
│
└── brain.anr

Tidak ada requirement runtime terhadap:

/cortex
/cerebellum
/hippocampus
/models
/plugins
/server
/cloud

Plugin/hardware support dikompilasi ke binary atau diimplementasikan melalui mekanisme plugin yang tetap memenuhi single-binary deployment target.


---

80. Final End-to-End Model

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


---

81. Final Definition

> ANR adalah embedded autonomous neural runtime berbasis Rust yang berjalan sebagai satu binary anr dan menggunakan satu persistent neural memory brain.anr. Neural Core menggunakan Cell–Column–Block–Synapse sebagai primitive arsitektur, tanpa Transformer sebagai core architecture.

Cortex merepresentasikan knowledge yang telah digeneralisasi, Cerebellum merepresentasikan skill/procedure yang telah terbukti, dan Hippocampus merepresentasikan pengalaman episodik. Sensor, kamera, audio, dan perangkat robot masuk melalui plugin/HAL menuju perception layer dan kemudian Neural Core.

Pengalaman baru masuk ke Hippocampus, dievaluasi berdasarkan novelty, recurrence, reward, prediction error, relevance, success, stability, dan context diversity, kemudian direplay dan dikonsolidasikan secara adaptif menjadi Cortex dan/atau Cerebellum. Hippocampus memiliki allocation independen, retention management, compression, serta automatic garbage collection berbasis memory pressure.

Initial Cortex, Cerebellum, dan optional Hippocampus dapat diprovisioning melalui Brain Seed dan anr brain build, kemudian semuanya dikompilasi menjadi satu brain.anr. Setelah deployment, brain tersebut menjadi persistent living state yang terus berubah melalui experience, learning, consolidation, dan GC.

ANR menggunakan bounded memory, bounded queues, SoA data layout, SIMD NEON/AVX2/AVX-512 dengan scalar fallback, transactional storage, checksum/integrity validation, crash recovery, memory isolation, priority scheduling, sensor/camera plugin architecture, HAL, dan safety layer. Control dan safety selalu memiliki prioritas di atas learning dan memory maintenance.

Target akhirnya adalah autonomous intelligence yang dapat merasakan lingkungan, membentuk pengalaman, mengubah pengalaman menjadi pengetahuan dan kemampuan, mempertahankan memory yang bernilai, membuang pengalaman yang tidak bernilai, dan melakukan seluruh proses tersebut secara lokal pada IoT, robot, dan edge hardware tanpa ketergantungan cloud.



Baseline final

ANR
                          │
            ┌─────────────┴─────────────┐
            │                           │
         anr                         brain.anr
       executable                  persistent brain
            │                           │
     ┌──────┴──────┐          ┌─────────┼─────────┐
     │             │          │         │         │
  Runtime       Plugins     Cortex  Cerebellum Hippocampus
     │             │          │         │         │
     └──────┬──────┘          └─────────┼─────────┘
            │                           │
            ▼                           ▼
       Perception                  Learning
            │                           │
            ▼                           ▼
     Cell → Column → Block       Consolidation + GC
            │                           │
            └─────────────┬─────────────┘
                          ▼
                       Decision
                          │
                          ▼
                        Action
                          │
                          ▼
                       Feedback
                          │
                          └──────────────► Hippocampus

Ini menjadi baseline arsitektur yang dapat langsung diturunkan menjadi tahap berikutnya: Implementation Specification, yang mengunci struktur Rust, memory layout, algoritma neural, allocator, binary layout brain.anr, serialization, SIMD implementation, dan API internal tanpa mengubah arsitektur di atas.
