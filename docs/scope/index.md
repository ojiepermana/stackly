# Scope: Stackly

Stackly adalah aplikasi open source untuk memasang, membuat instance, menjalankan, mengatur, dan memantau developer tools native di satu Mac, dengan core, daemon, dan CLI berbasis Rust serta GUI SwiftUI. Penggunanya developer dan tim internal yang butuh banyak versi dan banyak instance database secara lokal, tanpa Docker dan tanpa akun cloud.

**Build approach:** Journey (satu user journey dibangun lengkap, termasuk semua state, error, dan polish, sebelum journey berikutnya dimulai).

**Workflow:** Beta (setelah `/develop`: `/check verify`, lalu `/test`). Tingkat rigor default proyek. Fitur bertag `· GA` juga menjalani `/check review` dengan model baru, lalu `/document`. `/architect` disarankan sebagai langkah pertama untuk fitur yang punya keputusan nyata, tapi boleh kamu lewati kalau cara membangunnya sudah jelas.

**Cakupan:** baseline rilis stabil macOS (macOS 27, Apple Silicon). Windows, Linux, public registry, dan katalog tambahan ada di [Deferred](#deferred).

**Sumber:** `prd.md` versi 1.0.0 (15 Oktober 2026). ID `REQ`, `BR`, `VAL`, `NFR`, `AC`, `PAGE`, `JOURNEY`, `RISK`, dan `OQ` dari PRD dipertahankan di baris **Trace** tiap fitur supaya traceability tetap utuh.

_Ini rekomendasi supaya build kamu tetap tertib, bukan kewajiban. Lewati apa pun yang tidak cocok: kalau kamu sudah tahu cara membangun sebuah fitur, langsung pakai `/develop` dan lewati `/architect`. Kamu yang menentukan kapan sebuah fitur `done`._

## Urutan file

Bangun dari atas ke bawah. Nomor file adalah urutan development yang disarankan, dan nomor fitur berlanjut lintas file.

| File | Fase | Fitur | Status |
| --- | --- | --- | --- |
| [01-foundation.md](01-foundation.md) | Foundation dan Skeleton | 1 sampai 14 | 14 planned |
| [02-instalasi-pertama.md](02-instalasi-pertama.md) | Instalasi pertama (JOURNEY-01, PostgreSQL) | 15 sampai 21 | 7 planned |
| [03-multi-versi-dan-instance.md](03-multi-versi-dan-instance.md) | Multi versi & instance (JOURNEY-02, JOURNEY-03) | 22 sampai 23 | 2 planned |
| [04-kerja-harian.md](04-kerja-harian.md) | Kerja harian | 24 sampai 30 | 7 planned |
| [05-saat-instance-gagal.md](05-saat-instance-gagal.md) | Saat instance gagal (JOURNEY-05) | 31 sampai 33 | 3 planned |
| [06-menambah-provider.md](06-menambah-provider.md) | Menambah provider (JOURNEY-07) | 34 sampai 36 | 3 planned |
| [07-stack-enam-service.md](07-stack-enam-service.md) | Stack enam service | 37 sampai 41 | 5 planned |
| [08-environment-proyek.md](08-environment-proyek.md) | Environment proyek (JOURNEY-04) | 42 sampai 45 | 4 planned |
| [09-backup-dan-restore.md](09-backup-dan-restore.md) | Backup & restore (JOURNEY-06) | 46 sampai 48 | 3 planned |
| [10-merawat-runtime-dan-data.md](10-merawat-runtime-dan-data.md) | Merawat runtime & data | 49 sampai 53 | 5 planned |
| [11-distribusi-aplikasi.md](11-distribusi-aplikasi.md) | Distribusi aplikasi | 54 sampai 56 | 3 planned |
| [12-siap-rilis-stabil.md](12-siap-rilis-stabil.md) | Siap rilis stabil | 57 sampai 61 | 5 planned |

## At a glance

| # | Feature | Phase | Status |
| --- | --- | --- | --- |
| 1 | Stack & arsitektur | Foundation | planned |
| 2 | Coding standards & tooling | Foundation | planned |
| 3 | Sumber & packaging runtime | Foundation | planned |
| 4 | Domain core & platform abstraction | Foundation | planned |
| 5 | Data model registry | Foundation | planned |
| 6 | Protokol IPC & daemon | Foundation | planned |
| 7 | Operation engine | Foundation | planned |
| 8 | CLI client & kontrak output | Foundation | planned |
| 9 | Provider engine & manifest schema | Foundation | planned |
| 10 | Process supervisor | Foundation | planned |
| 11 | Secret store & redaction | Foundation | planned |
| 12 | Design system & UI foundation | Foundation | planned |
| 13 | App shell SwiftUI & Swift client | Foundation | planned |
| 14 | Walking skeleton | Skeleton | planned |
| 15 | Onboarding & background agent | Instalasi pertama | planned |
| 16 | Catalog & provider detail | Instalasi pertama | planned |
| 17 | Install runtime managed | Instalasi pertama | planned |
| 18 | Provider PostgreSQL | Instalasi pertama | planned |
| 19 | Create instance | Instalasi pertama | planned |
| 20 | Start, readiness & connection | Instalasi pertama | planned |
| 21 | Overview & daftar instance | Instalasi pertama | planned |
| 22 | Instance kedua dari runtime yang sama | Multi versi & instance | planned |
| 23 | PostgreSQL 17 & 18 berdampingan | Multi versi & instance | planned |
| 24 | Stop, restart & reload | Kerja harian | planned |
| 25 | Log per instance | Kerja harian | planned |
| 26 | Metrics & health | Kerja harian | planned |
| 27 | Edit config & drift | Kerja harian | planned |
| 28 | Reconciliation setelah daemon restart | Kerja harian | planned |
| 29 | Autostart instance & login | Kerja harian | planned |
| 30 | Menu bar & Settings | Kerja harian | planned |
| 31 | Error aman & crash loop | Saat instance gagal | planned |
| 32 | Doctor & diagnostic bundle | Saat instance gagal | planned |
| 33 | Operations, activity & audit | Saat instance gagal | planned |
| 34 | Provider SDK & test harness | Menambah provider | planned |
| 35 | Import provider lokal & trust | Menambah provider | planned |
| 36 | Panduan kontributor & governance | Menambah provider | planned |
| 37 | Provider NATS & JetStream | Stack enam service | planned |
| 38 | Provider Valkey | Stack enam service | planned |
| 39 | Provider Traefik & routing lokal | Stack enam service | planned |
| 40 | Provider MySQL | Stack enam service | planned |
| 41 | Provider ClickHouse | Stack enam service | planned |
| 42 | Manifest environment: validate & plan | Environment proyek | planned |
| 43 | Lockfile & override lokal | Environment proyek | planned |
| 44 | Env up & dependency graph | Environment proyek | planned |
| 45 | Env down, shared instance & lease | Environment proyek | planned |
| 46 | Backup metadata registry | Backup & restore | planned |
| 47 | Backup data per provider | Backup & restore | planned |
| 48 | Restore as new & verifikasi | Backup & restore | planned |
| 49 | Remove, retain & purge | Merawat runtime & data | planned |
| 50 | Hapus runtime & garbage collection | Merawat runtime & data | planned |
| 51 | Runtime external (Homebrew) | Merawat runtime & data | planned |
| 52 | Scan data orphan & adoption | Merawat runtime & data | planned |
| 53 | Update runtime & provider revision | Merawat runtime & data | planned |
| 54 | Packaging, signing & notarization | Distribusi aplikasi | planned |
| 55 | Update aplikasi & kompatibilitas | Distribusi aplikasi | planned |
| 56 | Uninstall & reinstall dengan data tetap | Distribusi aplikasi | planned |
| 57 | Security review & fuzzing | Siap rilis stabil | planned |
| 58 | Fault injection, recovery & soak | Siap rilis stabil | planned |
| 59 | Audit aksesibilitas journey kritis | Siap rilis stabil | planned |
| 60 | Quality gates & release report | Siap rilis stabil | planned |
| 61 | Runbook & dokumentasi pengguna | Siap rilis stabil | planned |

## Catatan urutan

- **Mulai risiko besar lebih awal.** Fitur 3 (sumber runtime) bisa kamu kerjakan bersamaan dengan fitur 1 dan 2, karena hasilnya menentukan fitur 17. Kalau mau menurunkan RISK-10 lebih cepat, jalankan `/architect` untuk fitur 15 (background agent) dan 54 (signing) di minggu awal; build keduanya tetap mengikuti urutan.
- **Jalur SwiftUI paralel.** Fitur 12 dan 13 bisa dikerjakan engineer SwiftUI segera setelah kontrak IPC (fitur 6) stabil, memakai contract fixtures. Setelah itu setiap journey membawa layarnya sendiri, jadi GUI dan CLI tetap setara per fase.
- **Jalur kontributor sebelum lima provider.** Fase 06 sengaja datang sebelum fase 07, supaya NATS, Valkey, Traefik, MySQL, dan ClickHouse dibangun lewat SDK dan harness yang sama dengan kontributor (menjawab RISK-07).
- **Enam provider sebelum environment.** Environment ERP di fase 08 memakai keenam provider, jadi fase 07 harus selesai lebih dulu.
- **Pilot bukan fitur.** Pilot internal (gate G7) dijalankan setelah fitur 61, dengan backup teruji, dan hasilnya dicatat lewat `/scope` berikutnya.

## Deferred

Di luar baseline macOS, dicatat supaya rencana tetap jujur.

- **Community beta**: publikasi SDK dan docs, kontribusi source, alur review catalog · needs a decision
- **Public registry**: discovery, publishing, trust roots, revocation, laporan kompatibilitas · needs a decision · GA
- **Windows foundation**: IPC named pipe, adapter proses, secret, path, dan startup, runner dan packaging spike (OQ-014, OQ-015) · needs a decision
- **WinUI 3 beta**: client C#, UI native, parity lifecycle dan recovery untuk provider yang tersedia · needs a decision
- **Windows stable**: gate distribusi, keamanan, dan recovery untuk Windows · needs a decision · GA
- **Linux dan plugin lanjutan**: distribusi Linux, mode headless, WASM atau custom hooks · needs a decision
- **Dukungan Intel macOS**: setelah hardware dan matrix lulus (OQ-002)
- **Katalog tambahan**: MariaDB, MongoDB, RabbitMQ, Kafka, Redpanda, Redis, Memcached, Caddy, Nginx, HAProxy, MinIO, Meilisearch, Typesense, OpenSearch, OpenTelemetry Collector, Prometheus, Grafana, Mailpit, Temporal, LocalStack, dan runtime bahasa (PRD bagian 5.3) · needs a decision
- **Backup lanjutan**: PITR, WAL archive, backup terjadwal atau incremental, retention otomatis, S3 · needs a decision
- **Upgrade mayor otomatis**: otomasi `pg_upgrade` penuh dengan preflight dan rollback · needs a decision · GA
- **Jaringan lanjutan**: bind LAN, port privileged, TLS trust otomatis, perubahan DNS atau `/etc/hosts` (OQ-017) · needs a decision · GA
- **Storage eksternal**: custom data volume dan external storage (OQ-018) · needs a decision

## Legend

**Kotak keputusan.** Tiap fitur punya tepat satu kotak yang labelnya berakhir dengan `(spec)`. Bunyinya bisa berbeda (`Design it (spec)` biasanya, `Decide the stack (spec)` untuk Stack & arsitektur), jadi skill lain mencarinya lewat akhiran `(spec)`, bukan label persis. Kotak lain adalah kotak eksekusi, dan `/architect` tidak pernah mencentangnya.

**Siklus fitur:**

| State | Diatur oleh | Yang terlihat di fitur |
| --- | --- | --- |
| `planned` · needs a decision | `/scope` | satu kotak: `Design it (spec): /architect <feature>` |
| `in-progress` (sudah didesain) | `/architect` saat spec ditangkap | `Design it` dicentang, spec ditautkan, `Build it: /develop <feature>` dengan 2 sampai 5 milestone, lalu kotak penutup sesuai tier (`Verify it` mulai Alpha, `Test it` mulai Beta, `Review it` dan `Document it` untuk GA) |
| `in-progress` (sedang dibangun) | `/develop` | milestone dicentang satu per satu, pointer kode terisi |
| `in-progress` (sudah diverifikasi) | `/check verify` | `Build it` dan milestone dicentang, `Verify it` dicentang |
| `done` | kamu, saat kamu memutuskan; `/sync` merekonsiliasi | kotak yang dijalankan dicentang, yang dilewati ditandai skipped; tahap terakhir tier (Beta: setelah `/test`) adalah titik yang disarankan |

- **Langkah berikutnya** adalah kotak pertama yang belum dicentang, selalu berupa command atau milestone.
- **needs a decision**: jalankan `/architect` dulu; tanpa tag ini langsung `/develop` (atau `/audit` untuk standards & tooling). Tag ini hilang setelah spec ditangkap.
- **Tugas build yang rinci** tinggal di `## Build plan` milik spec, bukan di sini. Scope hanya membawa ringkasan milestone.
- **Status**: `planned`, `in-progress`, `done`, plus `existing` (sudah ada sebelum workflow) dan `dropped` (dikeluarkan dari scope, tetap disimpan sebagai riwayat).
- **Tag tier** di samping judul (misalnya `· GA`) menaikkan rigor fitur itu di atas default Beta. Tanpa tag berarti ikut default.
- **Workflow** (baris header) adalah tahap setelah `/develop`: **Prototype** tidak ada; **Alpha** `/check verify`; **Beta** `/check verify` lalu `/test`; **GA** ditambah `/check review` dengan model baru lalu `/document`. Fitur yang dibangun di atas keputusan yang belum diratifikasi (spec `Assumed`) tetap ditandai, tapi tidak menghalangi `done`.
- **Trace**: ID dari `prd.md` yang dicakup fitur itu. `/architect` menurunkan requirement dan acceptance criteria lengkap ke spec dari ID ini, supaya PRD tidak perlu dibaca langsung saat implementasi.
- **Pointer** (`spec <n> · code in <path>`): link spec ditambahkan `/architect`, path kode ditambahkan `/develop`.
