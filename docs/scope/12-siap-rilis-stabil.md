# Scope 12 · Siap rilis stabil

[Kembali ke index](index.md)

Bukan journey pengguna, tapi pekerjaan yang membuat baseline layak disebut stabil: review keamanan, uji kegagalan dan performa, audit aksesibilitas, bukti per acceptance criteria, dan runbook. Release gate G1 sampai G6 menunggu fase ini. Pilot internal (G7, usulan minimal 3 pengguna selama 10 hari kerja) dijalankan setelah fitur 61 selesai dan bukan fitur build.

### 57. Security review & fuzzing · needs a decision · GA

Review keamanan untuk archive parser, template engine, eksekusi proses native, purge dan adoption, framing IPC, credential, izin, update dan revocation, serta diagnostic export, plus fuzzing untuk manifest, parser, framing, dan path.

**Done when:** tidak ada temuan critical atau high yang belum selesai di distribusi, IPC, eksekusi provider, atau operasi destruktif; residual risk di PRD bagian 23 tertulis di docs pengguna dan developer.

**Trace:** NFR-009, AC-061, AC-070, gate G4, PRD bagian 23

- [ ] Design it (spec): `/architect security review & fuzzing`

### 58. Fault injection, recovery & soak · needs a decision

Harness untuk crash daemon dan provider, PID reuse, race port, disk penuh, download korup, operasi yang dibatalkan, sleep dan wake, dua user di satu Mac, serta benchmark dan soak pada hardware baseline yang tercatat.

**Done when:** skenario lintas fitur AC-055 sampai AC-080 punya test yang bisa dijalankan; target NFR-001 sampai NFR-007 diukur dan disimpan, termasuk outlier; process crash, OS restart, dan simulasi power loss dibedakan.

**Trace:** NFR-001 sampai NFR-008, NFR-013, AC-055 sampai AC-080, OQ-011, PRD bagian 32

- [ ] Design it (spec): `/architect fault injection, recovery & soak`

### 59. Audit aksesibilitas journey kritis

Review keyboard dan VoiceOver untuk journey kritis: create, stop, dan inspect instance, environment, serta restore.

**Done when:** create, stop, dan inspect instance bisa diselesaikan dengan keyboard dan VoiceOver tanpa informasi yang hanya tersedia lewat warna atau hover; kontras serta status error tidak bergantung pada warna.

**Trace:** REQ-027, NFR-012, AC-027

- [ ] Build it: `/develop audit aksesibilitas journey kritis`

### 60. Quality gates & release report · needs a decision

Evidence record per AC (commit, versi provider, OS/arch, langkah test, hasil, waktu, reviewer), traceability dari REQ ke AC ke test, compatibility matrix, dan release report yang menahan rilis kalau test wajib gagal atau `BLOCKED`.

**Done when:** satu restore test wajib yang `BLOCKED` membuat report menyatakan belum stable ready tanpa mengubah statusnya jadi `PASS`; versi di luar matrix tetap `UNVERIFIED`; laporan fixture dibedakan dari laporan integrasi runtime nyata.

**Trace:** REQ-051, BR-025, AC-051, gate G1 sampai G6, PRD bagian 28, 31, 32.2, dan 34

- [ ] Design it (spec): `/architect quality gates & release report`

### 61. Runbook & dokumentasi pengguna

Runbook untuk kondisi di PRD bagian 35.3 (daemon tidak terhubung, konflik port, runtime hilang, start gagal, disk penuh, crash loop, backup atau restore gagal, update gagal, provider revoked, uninstall dan reinstall), plus getting started, troubleshooting, dan known limitations.

**Done when:** tiap kondisi punya langkah aman yang tidak menghapus data atau registry; dokumentasi menjelaskan isolasi logis dengan jujur beserta batas dukungan per platform.

**Trace:** gate G6, AC-061, PRD bagian 35.3 dan 36

- [ ] Build it: `/develop runbook & dokumentasi pengguna`
