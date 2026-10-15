# Scope 01 · Foundation

[Kembali ke index](index.md)

Fondasi yang dipijak semua journey: stack, tooling, sumber runtime, model domain dan data, protokol IPC, operation engine, CLI, mesin provider, supervisor, secrets, dan GUI shell. Fase ini ditutup dengan walking skeleton yang membuktikan semuanya sudah tersambung.

## Foundation

### 1. Stack & arsitektur · needs a decision

Putuskan stack dan susunan monorepo, lalu scaffold proyek yang bisa dijalankan, supaya setiap journey dibangun di atas struktur nyata. PRD sudah mengunci Rust untuk core, daemon, dan CLI, SwiftUI untuk macOS, dan SQLite untuk registry; spec ini mengunci sisanya (toolchain, pembagian crate, encoding IPC) tanpa membuat crate kosong yang belum perlu.

**Done when:** keputusan stack tercatat di spec (termasuk OQ-005), workspace Rust dan proyek SwiftUI kosong bisa build dan jalan di Mac Apple Silicon, dan arah dependency dari domain ke adapter sesuai PRD bagian 15.

**Trace:** REQ-001, REQ-050, NFR-010, OQ-005, PRD bagian 15 dan 16

- [ ] Decide the stack (spec): `/architect stack & arsitektur`
- [ ] Scaffold from the decision: `/develop stack & arsitektur`

### 2. Coding standards & tooling

Catat konvensi dan pilihan tooling dari proyek hasil scaffold, lalu pasang format, lint, typecheck, dan CI dasar yang wajib lulus di setiap PR.

**Done when:** root `AGENTS.md` mencerminkan stack nyata, dan pipeline PR menjalankan format, lint, build, unit test, validasi schema, serta audit lisensi dependency dengan hasil bersih.

**Trace:** REQ-051, PRD bagian 27

- [ ] Capture conventions and tooling choices: `/audit`
- [ ] Install the tooling: `/develop coding standards & tooling`

### 3. Sumber & packaging runtime · needs a decision

Tentukan dari mana artefak runtime enam provider berasal dan bagaimana mereka dipaketkan supaya bisa dipindah dan dipasang tanpa Homebrew global. Ini risiko terbesar proyek (RISK-01), jadi harus jelas sebelum install runtime (fitur 17) dibangun.

**Done when:** tiap provider wajib punya sumber artefak, dependency closure, versi konkret, dan status lisensi yang tercatat; PostgreSQL 17 dan 18 berjalan dari artefak managed di Mac uji tanpa Homebrew global; provider yang belum punya artefak managed dicatat `BLOCKED` dengan alasan (gate G0).

**Trace:** OQ-003, OQ-004, RISK-01, RISK-12, AC-078, gate G0, PRD bagian 20

- [ ] Design it (spec): `/architect sumber & packaging runtime`

### 4. Domain core & platform abstraction · needs a decision

Model domain (software, provider revision, runtime, instance, environment, operation, lease), tiga state machine yang terpisah, invariant bisnis, dan trait platform dengan fake platform untuk test. Core tidak boleh tahu soal SwiftUI, WinUI, atau command macOS.

**Done when:** domain bisa build tanpa dependency GUI atau API OS; state runtime, lifecycle instance, dan readiness/health terpisah; fake platform menjalankan test invariant di CI; capability Windows yang belum dibuat mengembalikan not supported.

**Trace:** REQ-001, REQ-050, BR-001 sampai BR-025 (sebagai invariant), NFR-010, AC-050, AC-074, PRD bagian 6 dan 19.1

- [ ] Design it (spec): `/architect domain core & platform abstraction`

### 5. Data model registry · needs a decision

Skema SQLite untuk registry: entitas, relasi, constraint, migrasi, dan revision per resource. Model data adalah hal paling mahal untuk diulang, jadi ia diputuskan sebelum fitur apa pun menyimpan state.

**Done when:** semua entitas logis di PRD bagian 17 ada dengan foreign key dan transaksi aktif; migrasi berurutan teruji dari versi sebelumnya; secret hanya disimpan sebagai reference; nama instance unik per user dan path data memakai ID yang stabil.

**Trace:** BR-002, BR-003, REQ-048, NFR-005, PRD bagian 17

- [ ] Design it (spec): `/architect data model registry`

### 6. Protokol IPC & daemon · needs a decision · GA

Daemon per user dengan protokol berversi lewat Unix domain socket privat: handshake, request ID, error terstruktur, event dengan cursor, dan otorisasi user lokal. GUI dan CLI hanya client; hanya daemon yang menulis registry dan mengendalikan lifecycle.

**Done when:** handshake menolak protocol major yang berbeda dan peer dari user lain sebelum ada efek samping; tidak ada listener TCP admin; batas ukuran pesan dan subscription berlaku; event memakai sequence, dan reconnect memakai snapshot plus cursor; fixture kontrak dipakai bersama oleh Rust dan Swift.

**Trace:** REQ-047, BR-011, VAL-017, VAL-020, NFR-011, AC-047, AC-070, AC-071, AC-079, OQ-005, PRD bagian 18

- [ ] Design it (spec): `/architect protokol IPC & daemon`

### 7. Operation engine · needs a decision · GA

Mesin operasi async yang dipakai semua mutasi: journal, idempotency key, lock per resource, state revision, progress, dan cancel. Inilah yang mencegah runtime atau instance ganda saat GUI dan CLI mengirim perintah bersamaan.

**Done when:** operasi memakai state `queued`, `running`, `succeeded`, `failed`, `cancel_requested`, `cancelled`, dan `needs_recovery`; dua client dengan idempotency key sama hanya menghasilkan satu efek samping; client yang putus tidak membatalkan pekerjaan; cancel di langkah yang tidak bisa dibalik dilaporkan apa adanya; SQLite tidak dipakai sebagai mutex selama download atau startup.

**Trace:** REQ-048, REQ-054, BR-023, NFR-005, NFR-011, AC-048, AC-054

- [ ] Design it (spec): `/architect operation engine`

### 8. CLI client & kontrak output · needs a decision

CLI `stackly` sebagai client daemon dengan help, `--json`, exit code stabil, mode noninteractive, progress ke stderr, input secret yang aman, dan `--wait` untuk operasi async. Command lain menempel ke kontrak ini seiring tiap journey dibangun.

**Done when:** `stackly daemon status` serta `stackly operation list`, `inspect`, dan `cancel` bekerja lewat daemon; output `--json` bisa diparse tanpa ANSI atau progress; exit code mengikuti tabel di PRD bagian 12; CLI tidak pernah menjalankan binary provider sendiri.

**Trace:** REQ-028, BR-011, AC-028, PRD bagian 12

- [ ] Design it (spec): `/architect CLI client & kontrak output`

### 9. Provider engine & manifest schema · needs a decision · GA

Schema manifest provider deklaratif (`stackly/v1alpha1`), loader, validasi, interpolasi terkontrol, dan primitive aman yang menghasilkan execution plan tervalidasi. Tool baru yang cukup dengan primitive yang ada tidak perlu patch core atau GUI.

**Done when:** manifest contoh NATS dan fake provider lolos validasi; field berisi shell metacharacter tidak pernah dijalankan lewat shell; field keamanan yang tidak dikenal dan capability yang tidak bisa dipenuhi ditolak; membaca catalog tidak menjalankan kode provider.

**Trace:** REQ-029, REQ-031, VAL-002, VAL-012, VAL-018, VAL-019, AC-029, AC-031, RISK-07, RISK-08, PRD bagian 21

- [ ] Design it (spec): `/architect provider engine & manifest schema`

### 10. Process supervisor · needs a decision · GA

Komponen di dalam daemon yang menjalankan executable terverifikasi dengan argv eksplisit, mencatat identitas proses (bukan PID saja), menjalankan probe readiness, dan menghentikan proses secara graceful sesuai strategi provider.

**Done when:** startup transaction di PRD bagian 19.2 berjalan untuk fake provider; process, readiness, health, dan desired state terlihat terpisah; stop normal tidak pernah memakai SIGKILL atau kill berdasarkan nama; path dengan spasi dan Unicode diteruskan utuh sebagai argumen.

**Trace:** REQ-011, REQ-012, BR-010, AC-011, AC-012, AC-058, RISK-03, RISK-14, PRD bagian 19

- [ ] Design it (spec): `/architect process supervisor`

### 11. Secret store & redaction · needs a decision · GA

Simpan secret di Keychain dan hanya reference di SQLite atau manifest, lalu terapkan masking yang sama di GUI, CLI, log, diagnostics, dan export. Reveal atau copy secret selalu butuh aksi eksplisit.

**Done when:** sentinel credential tidak muncul di output default, registry, log, atau audit; secret noninteractive masuk lewat input aman, bukan argumen CLI; Keychain yang terkunci menghasilkan status needs secret, bukan auth kosong.

**Trace:** REQ-016, BR-018, AC-016, AC-062, OQ-008

- [ ] Design it (spec): `/architect secret store & redaction`

### 12. Design system & UI foundation · needs a decision

Bahasa visual dan komponen dasar SwiftUI supaya semua layar terasa satu kesatuan dan aksesibel sejak awal: tipografi, warna light/dark, status yang tidak hanya bergantung pada warna, serta pola state loading, empty, error, stale, unsupported, dan partial.

**Done when:** `design.md` mencakup tipografi, warna, spacing, dan komponen; komponen dasar mendukung keyboard, label VoiceOver, Dynamic Type, dan konfirmasi aksi destruktif.

**Trace:** REQ-027, NFR-012, PRD bagian 11.2

- [ ] Design it (spec): `/architect design system & UI foundation`

### 13. App shell SwiftUI & Swift client · needs a decision

Shell aplikasi macOS dengan navigasi PRD bagian 11.1, status koneksi daemon, operation center, reconnect, dan Swift client yang dibangkitkan dari schema atau fixture yang sama dengan Rust. Semua mutasi lewat API daemon.

**Done when:** GUI tetap terbuka saat daemon mati dan menampilkan status koneksi, retry, dan diagnostics tanpa menyebut instance stopped; reconnect memakai snapshot plus cursor; test kontrak Swift lulus untuk enum, optional field, timestamp, dan error code.

**Trace:** REQ-024, AC-024, RISK-09, PRD bagian 11.1 dan 18.3

- [ ] Design it (spec): `/architect app shell SwiftUI & Swift client`

## Skeleton

### 14. Walking skeleton

Satu irisan tipis yang membuktikan stack sudah tersambung: fake provider dibuat, dijalankan, dan dihentikan lewat CLI, statusnya tampil di GUI, dan registry mencatatnya. Belum ada download runtime atau provider nyata.

**Done when:** `stackly instance create`, `start`, dan `stop` untuk fake provider berhasil lewat daemon; GUI menampilkan perubahan state lewat event; menutup GUI tidak menghentikan instance.

**Trace:** AC-001 (awal), AC-029 (awal), fixture dummy server di PRD bagian 32.3

- [ ] Build it: `/develop walking skeleton`
