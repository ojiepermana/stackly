# Scope 02 · Instalasi pertama

[Kembali ke index](index.md)

Journey PRD JOURNEY-01 dengan PostgreSQL, dibangun lengkap: buka aplikasi, onboarding, izin background, catalog, install runtime, create instance, readiness, lalu copy connection. Fase ini selesai kalau seluruh jalur itu bekerja dari GUI dan CLI, termasuk state empty, error, unsupported, dan konfirmasi, bukan hanya jalur sukses.

### 15. Onboarding & background agent · needs a decision

Halaman pertama yang menjelaskan status daemon dan izin background, lokasi storage, model isolasi yang jujur, trust catalog, dan pilihan memasang CLI, tanpa akun cloud. Daemon didaftarkan sebagai background agent per user.

**Done when:** status `requiresApproval`, `disabled`, dan `notRegistered` terlihat sebagai state produk; izin yang ditolak tidak diubah tanpa sepengetahuan pengguna; teks onboarding menjelaskan isolasi logis dan tidak menyebut provider disandbox; memasang CLI tidak mengubah PATH global tanpa persetujuan.

**Trace:** PAGE-01, REQ-014 (bagian daemon), AC-061, OQ-006, PRD bagian 25

- [ ] Design it (spec): `/architect onboarding & background agent`

### 16. Catalog & provider detail · needs a decision

Catalog bawaan (official) yang menampilkan publisher, revision, kategori, OS/arch, versi, trust, status kompatibilitas, dependency, kebutuhan disk, dan capability. Kombinasi yang unsupported terlihat jujur, bukan disembunyikan.

**Done when:** provider untuk platform lain berlabel unsupported dengan alasan dan install ditolak oleh daemon, bukan hanya oleh UI; catalog yang stale menampilkan waktu pembaruan terakhir; `stackly provider list` dan `inspect` setara dengan GUI.

**Trace:** REQ-002, REQ-033 (catalog bawaan), PAGE-08, BR-016, BR-025, NFR-014, AC-002

- [ ] Design it (spec): `/architect catalog & provider detail`

### 17. Install runtime managed · needs a decision · GA

Resolusi selector (`@17`, `@stable`) ke exact version dan digest, download ke staging, verifikasi digest dan signature, ekstrak aman, validasi binary dan dependency, lalu commit atomik. Termasuk halaman Runtimes untuk melihat hasilnya.

**Done when:** download yang putus lalu diulang hanya menghasilkan runtime utuh yang terverifikasi; artefak dengan digest yang tidak cocok tidak pernah dijalankan; traversal, symlink escape, dan archive bomb ditolak; artefak x86_64 di ARM64 tanpa dukungan ditolak dengan diagnosis; tanpa internet dan tanpa cache muncul status offline yang benar.

**Trace:** REQ-003, REQ-004, REQ-005, REQ-046, VAL-003, VAL-010, VAL-011, PAGE-09, AC-003, AC-004, AC-005, AC-057, AC-059, AC-060

- [ ] Design it (spec): `/architect install runtime managed`

### 18. Provider PostgreSQL · needs a decision

Manifest dan adapter PostgreSQL resmi: `initdb` dari runtime yang cocok, auth lokal yang aman, locale dan encoding eksplisit, endpoint dan Unix socket per instance, readiness lewat query terautentikasi dengan role berhak minimum, dan shutdown terkontrol.

**Done when:** cluster baru terinisialisasi tanpa default `trust` yang tidak disengaja; readiness gagal kalau query gagal walau TCP terbuka; stop normal memakai mode shutdown provider; tool admin memakai runtime yang cocok, bukan `psql` dari PATH.

**Trace:** REQ-035 (dasar), JOURNEY-01, PRD bagian 22.1

- [ ] Design it (spec): `/architect provider PostgreSQL`

### 19. Create instance · needs a decision · GA

Wizard dan command untuk membuat instance: ID stabil, nama, runtime, endpoint, auth, persistence, storage, dan autostart, dengan preflight dan error per field. Inisialisasi hanya terjadi di direktori baru milik operasi itu.

**Done when:** direktori yang sudah berisi data ditolak sebelum `initdb` berjalan; nama mengikuti VAL-001; port fixed yang konflik menghasilkan error, dan auto port dipersistenkan setelah bind berhasil; overlap IPv4/IPv6 dan wildcard terdeteksi; bind default ke loopback; path tidak keluar dari owned root.

**Trace:** REQ-008, REQ-010, PAGE-04, BR-002, BR-004, BR-005, BR-008, VAL-001, VAL-004 sampai VAL-008, AC-008, AC-055, AC-056

- [ ] Design it (spec): `/architect create instance`

### 20. Start, readiness & connection

Menjalankan instance, menunggu readiness, dan menampilkan ringkasan instance: ID, versi dan digest, path dan ukuran data, endpoint, connection string yang dimasking, serta tombol copy. Inilah titik akhir JOURNEY-01.

**Done when:** status process dan readiness tampil terpisah; `stackly instance connection --format uri` menyembunyikan password secara default; copy endpoint dan reveal secret butuh aksi eksplisit; GUI dan CLI menunjukkan hasil yang sama.

**Trace:** REQ-012, PAGE-05, BR-010, AC-012, JOURNEY-01

- [ ] Build it: `/develop start, readiness & connection`

### 21. Overview & daftar instance

Halaman Overview (ringkasan ready, stopped, failed, operasi aktif, peringatan kapasitas) dan Instances (cari, filter, urutkan; nama, provider, exact version, endpoint, lifecycle, readiness, ownership).

**Done when:** keduanya menangani state loading, empty, error, stale, dan partial; error di satu instance tidak membuat seluruh layar kosong; ringkasan tidak dihitung dari PID saja.

**Trace:** PAGE-02, PAGE-03, NFR-002

- [ ] Build it: `/develop overview & daftar instance`
