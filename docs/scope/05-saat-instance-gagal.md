# Scope 05 · Saat instance gagal

[Kembali ke index](index.md)

Journey PRD JOURNEY-05: start gagal, pesan yang aman, doctor dan log, perbaikan config atau dependency, coba lagi, dan semuanya tercatat sebagai operasi. Data tidak pernah direset otomatis. Fase ini selesai kalau pengguna bisa sampai ke akar masalah dan pulih tanpa menebak.

### 31. Error aman & crash loop

Error terstruktur dengan kode stabil, pesan aman, konteks, dan petunjuk recovery di GUI dan CLI, plus restart policy dengan backoff dan budget supaya crash loop berhenti sendiri.

**Done when:** provider yang terus crash berhenti di retry budget (usulan 5 kali dalam 5 menit) tanpa busy loop atau child ganda; GUI dan CLI menampilkan backoff dan error terakhir; tiap error membawa correlation ID.

**Trace:** VAL-020, REQ-013 (restart policy), NFR-011, AC-068, PRD bagian 35.2

- [ ] Build it: `/develop error aman & crash loop`

### 32. Doctor & diagnostic bundle · needs a decision

`stackly doctor` dan halaman Diagnostics yang memeriksa izin, runtime atau dependency yang hilang, konflik endpoint, data lock, ruang disk, registry, background agent, protocol mismatch, dan config drift, plus paket diagnosis yang disanitasi.

**Done when:** PID record yang menunjuk proses lain dilaporkan beserta bukti tanpa menghapus data atau menghentikan proses itu; bundle diagnosis bisa dipreview sebelum export dan tidak memuat secret; diagnosis tidak pernah menghapus lock secara otomatis.

**Trace:** REQ-019, PAGE-13, NFR-016, AC-019, PRD bagian 35.3

- [ ] Design it (spec): `/architect doctor & diagnostic bundle`

### 33. Operations, activity & audit

Halaman Operations dan tab Activity: stage, progress, cancel, error, retry atau resume bila aman, serta audit lokal (actor, operation ID, instance, aksi, hasil) tanpa secret.

**Done when:** create, start, dan ganti port tercatat dengan correlation ID dan hasil tanpa credential; cancel di langkah yang tidak bisa dibalik menampilkan continuing safe shutdown atau needs recovery; retention audit mengikuti setting.

**Trace:** REQ-045, REQ-054, PAGE-12, AC-045, AC-054, JOURNEY-05

- [ ] Build it: `/develop operations, activity & audit`
