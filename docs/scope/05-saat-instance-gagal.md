# Scope 05 · Saat instance gagal

[Kembali ke index](index.md)

Journey PRD JOURNEY-05: start gagal, pesan yang aman, doctor dan log, perbaikan config atau dependency, coba lagi, dan semuanya tercatat sebagai operasi. Data tidak pernah direset otomatis. Fase ini selesai kalau pengguna bisa sampai ke akar masalah dan pulih tanpa menebak, dan seluruh jalur itu bekerja dari GUI dan CLI.

### 33. Crash loop & error start yang aman

Hasil JOURNEY-05 di GUI dan CLI: start yang gagal tampil dengan pesan aman, recovery hint, dan tautan ke log dan doctor, plus backoff dan retry budget dari spec supervisor (fitur 10) yang terlihat oleh pengguna.

**Done when:** provider yang terus crash berhenti di retry budget (usulan 5 kali dalam 5 menit) tanpa busy loop atau child ganda; GUI dan CLI menampilkan backoff dan error terakhir; tiap error membawa correlation ID dan memakai envelope dari fitur 6.

**Trace:** AC-068, JOURNEY-05, NFR-011, VAL-020 (tampilan; taksonomi di fitur 6), PRD bagian 19.4 dan 35.2

- [ ] Build it: `/develop crash loop & error start yang aman`

### 34. Doctor & diagnostic bundle · needs a decision

`stackly doctor` dan halaman Diagnostics yang memeriksa izin, runtime atau dependency yang hilang, konflik endpoint, data lock, ruang disk, registry, background agent, protocol mismatch, dan config drift, plus paket diagnosis yang disanitasi.

**Done when:** PID record yang menunjuk proses lain dilaporkan beserta bukti tanpa menghapus data atau menghentikan proses itu; bundle diagnosis bisa dipreview sebelum export dan tidak memuat secret, baik lewat GUI maupun `stackly diagnostics export`; diagnosis tidak pernah menghapus lock secara otomatis.

**Trace:** REQ-019, REQ-028, PAGE-13, NFR-016, AC-019, PRD bagian 35.3

- [ ] Design it (spec): `/architect doctor & diagnostic bundle`

### 35. Operations, activity & audit · needs a decision

Halaman Operations dan tab Activity: stage, progress, cancel, error, retry atau resume bila aman, serta audit lokal (actor, operation ID, instance, aksi, config revision, hasil) tanpa secret.

**Done when:** create, start, dan ganti port tercatat dengan correlation ID dan hasil tanpa credential; cancel di langkah yang tidak bisa dibalik menampilkan continuing safe shutdown atau needs recovery; resume hanya ditawarkan untuk operasi yang menyatakan bisa dilanjutkan; retention audit mengikuti setting.

**Trace:** REQ-045, REQ-054, PAGE-12, AC-045, AC-054, JOURNEY-05

- [ ] Design it (spec): `/architect operations, activity & audit`
