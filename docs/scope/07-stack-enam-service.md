# Scope 07 · Stack enam service

[Kembali ke index](index.md)

Lima provider wajib berikutnya, tiap provider dibangun lewat SDK dari fase 06 dan menjalani journey yang sama dengan PostgreSQL: catalog, install, create, start, readiness, connection, dan dua instance yang terisolasi. Urutannya dimulai dari yang paling sederhana dan paling berbeda dari database (supaya terbukti engine tidak hanya cocok untuk PostgreSQL) ke yang paling berat. Fase ini selesai kalau setiap provider lulus matrix minimum di PRD bagian 32.2 tanpa label supported palsu.

### 37. Provider NATS & JetStream · needs a decision

Client endpoint, monitoring di loopback, credential, JetStream on/off, server name unik, `store_dir` per instance, dan batas resource.

**Done when:** dua instance NATS dengan JetStream dan `store_dir` berbeda hanya mengakses stream miliknya setelah restart; monitoring ada di endpoint milik instance itu dan tidak terbuka ke LAN; port monitoring yang bentrok menunjuk endpoint yang tepat tanpa menghentikan proses luar.

**Trace:** REQ-036, AC-010, AC-036, AC-077 (sebagian), PRD bagian 22.2

- [ ] Design it (spec): `/architect provider NATS & JetStream`

### 38. Provider Valkey · needs a decision

Config, bind dan auth, PING, direktori persistent, kebijakan RDB/AOF, log, dan stop, dengan penjelasan jujur soal mode volatile versus durable.

**Done when:** dengan persistence aktif, fixture pulih setelah shutdown normal; mode volatile menampilkan peringatan bahwa data bisa hilang dan tidak menjanjikan restore; Valkey tidak pernah diganti Redis atau container tanpa sepengetahuan pengguna.

**Trace:** REQ-038, AC-038, AC-077 (sebagian), PRD bagian 22.4

- [ ] Design it (spec): `/architect provider Valkey`

### 39. Provider Traefik & routing lokal · needs a decision

Entrypoint loopback, routing lewat file provider dengan namespace per instance, health endpoint terpisah, validasi, dan apply yang aman. HTTPS hanya aktif dengan certificate yang diberikan pengguna.

**Done when:** mengubah route di instance A hanya mengubah request di A sementara B tetap jalan; config yang tidak valid tidak mengganti last known good; dashboard insecure mati secara default; tidak ada perubahan root CA, DNS, atau `/etc/hosts`.

**Trace:** REQ-039, AC-039, PRD bagian 22.5

- [ ] Design it (spec): `/architect provider Traefik & routing lokal`

### 40. Provider MySQL · needs a decision

Path data, config, temp, dan log yang terisolasi, bind dan auth eksplisit, readiness lewat query terautentikasi berhak minimum, serta shutdown terkontrol sesuai versi MySQL yang dipilih.

**Done when:** dua instance MySQL dengan data dan port berbeda hanya membaca sentinel miliknya setelah restart; runtime tidak bisa direbind ke data yang tidak kompatibel; adopsi data yang sudah ada butuh preflight dan persetujuan eksplisit.

**Trace:** REQ-055, AC-081 (tanpa bagian restore, yang menyusul di fase 09), PRD bagian 22.6

- [ ] Design it (spec): `/architect provider MySQL`

### 41. Provider ClickHouse · needs a decision

Binary native yang dipilih, path storage, temp, log, user files, dan config per instance, port HTTP dan native TCP, auth, serta readiness lewat query. Cluster, replikasi, dan Keeper tidak diaktifkan.

**Done when:** dua instance dengan tabel bernama sama berisi data berbeda tidak tercampur setelah restart; seluruh listener aktif cocok dengan registry; jalur Windows lewat WSL tidak pernah diberi label native.

**Trace:** REQ-037, AC-037, AC-074, PRD bagian 22.3

- [ ] Design it (spec): `/architect provider ClickHouse`
