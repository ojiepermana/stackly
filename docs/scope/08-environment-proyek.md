# Scope 08 · Environment proyek

[Kembali ke index](index.md)

Journey PRD JOURNEY-04 (contoh environment ERP): import `stackly.yaml`, review trust provider, plan, lock versi exact, up dengan progress dan readiness, lalu down tanpa menghapus data. Fase ini datang setelah stack enam service karena environment ERP memakai keenamnya. Fase ini selesai kalau tim bisa berbagi satu file environment tanpa ikut membagikan data, path pribadi, atau password.

### 42. Manifest environment: validate & plan · needs a decision

Schema `stackly.yaml` (versi, namespace, selector runtime, endpoint, settings, secret reference, dependency, ownership) dan plan yang menampilkan diff serta dampak tanpa efek samping. Termasuk halaman Environments untuk import, edit, validasi, dan preview plan.

**Done when:** plan untuk manifest baru menampilkan install, create, dan start tanpa membuat file data, proses, perubahan trust, atau runtime; cycle, reference yang tidak ada, secret literal, dan konflik endpoint fixed ditolak sebelum ada mutasi.

**Trace:** REQ-020, VAL-013, VAL-014, PAGE-10, AC-020, AC-080 (bagian plan), JOURNEY-04

- [ ] Design it (spec): `/architect manifest environment: validate & plan`

### 43. Lockfile & override lokal · needs a decision

Pisahkan manifest yang dibagikan, `stackly.lock` yang menyimpan versi provider, runtime, dan digest per platform, dan `.stackly.local.yaml` untuk override lokal. `stackly env up --locked` tidak pernah melakukan resolusi ulang.

**Done when:** lockfile tidak memuat password, home directory, atau PID; mesin tanpa artefak yang kompatibel gagal dengan jelas tanpa substitusi; override port dan storage tidak mengubah manifest milik tim; `stackly env export` tidak membawa secret.

**Trace:** REQ-023, BR-018, AC-023, AC-080, PRD bagian 13.3

- [ ] Design it (spec): `/architect lockfile & override lokal`

### 44. Env up & dependency graph · needs a decision

Up memasang yang dibutuhkan, membuat instance dedicated, mengambil lease, lalu menjalankan dependency graph berdasarkan readiness dan timeout. Partial failure menghasilkan state yang jelas, dan kompensasi hanya menyentuh resource milik operasi itu.

**Done when:** dependency yang tidak ready membuat dependent tidak ditandai sukses; instance yang sudah ada dan sehat tidak dihapus; operasi menunjukkan tahap yang gagal dan langkah recovery; graph diuji dengan provider fixture tanpa dependency palsu antar database.

**Trace:** REQ-021, BR-023, AC-021, PRD bagian 13.2

- [ ] Design it (spec): `/architect env up & dependency graph`

### 45. Env down, shared instance & lease · needs a decision

Down melepas lease environment dan hanya menghentikan instance yang tidak dibutuhkan lagi. `instanceRef` meminjam instance yang ada tanpa mengubah config, auth, port, atau autostart miliknya. Stop manual yang mengganggu environment lain ditolak atau meminta override dengan dampak yang jelas.

**Done when:** saat environment A down, lease B tetap ada dan instance tetap ready; up A dan down B yang bersamaan menghasilkan lease konsisten tanpa create ganda; down tidak pernah menghapus data atau runtime.

**Trace:** REQ-022, BR-006, BR-014, BR-015, VAL-014, AC-022, AC-073

- [ ] Design it (spec): `/architect env down, shared instance & lease`
