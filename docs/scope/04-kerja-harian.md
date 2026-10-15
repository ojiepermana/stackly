# Scope 04 · Kerja harian

[Kembali ke index](index.md)

Journey yang paling sering dipakai setelah instance ada: stop, restart, lihat log, pantau metrik, ubah config, dan percaya bahwa instance tetap benar walau GUI ditutup, daemon restart, atau Mac login ulang. Fase ini selesai kalau seluruh pekerjaan harian bisa dilakukan dari GUI, CLI, dan menu bar dengan state yang jujur.

### 24. Stop, restart & reload

Lifecycle lengkap sesuai capability provider: stop graceful dengan timeout, restart yang diserialisasi, reload hanya kalau provider mendukung, dan opsi force dengan risiko yang jelas setelah timeout.

**Done when:** stop normal menjalankan strategi provider dan melaporkan hasilnya; stop dan start yang bersamaan diserialisasi; provider tanpa hot reload meminta restart terkontrol, bukan klaim reload palsu; force kill tidak pernah menyasar proses yang identitasnya belum terverifikasi.

**Trace:** REQ-011, BR-016, AC-011, AC-026, AC-067

- [ ] Build it: `/develop stop, restart & reload`

### 25. Log per instance · needs a decision

Log terpisah untuk tiap instance dengan follow, cursor, filter dasar, timestamp, pemisahan stdout dan stderr, batas ukuran, dan rotasi. Subscriber yang lambat tidak boleh menahan pipe proses.

**Done when:** log besar dengan client lambat tidak membuat daemon lambat atau memori tumbuh tanpa batas; rotasi berjalan (default usulan 5 file × 20 MiB); segmen yang hilang ditandai sebagai truncation; `stackly instance logs --follow` dan layar Logs setara; secret tidak pernah muncul.

**Trace:** REQ-017, PAGE-07, NFR-004, AC-017, PRD bagian 35.2

- [ ] Design it (spec): `/architect log per instance`

### 26. Metrics & health · needs a decision

Status, readiness, process tree, uptime, restart count, kegagalan terakhir, CPU, memori, dan pemakaian disk terjadwal, dengan label cakupan yang jelas antara proses utama dan agregat child.

**Done when:** metrik yang tidak tersedia tampil unavailable, bukan nol; label menjelaskan cakupan agregasi untuk provider multiprocess; health punya timestamp dan alasan; setelah Mac bangun dari sleep, endpoint dicek ulang tanpa mengganti port fixed.

**Trace:** REQ-018, PAGE-07, AC-018, PRD bagian 19.5

- [ ] Design it (spec): `/architect metrics & health`

### 27. Edit config & drift · needs a decision

Form typed dan advanced view dari schema provider, dengan diff, validasi, revision, dampak reload atau restart, histori minimal last known good, dan deteksi drift akibat edit dari luar.

**Done when:** config yang tidak valid tidak mengganti config aktif, dan error menunjuk field atau baris yang relevan; field managed penting tidak bisa ditimpa override tanpa validasi; drift tampil sebagai peringatan; `stackly instance config validate` dan `apply` setara dengan GUI.

**Trace:** REQ-015, PAGE-06, VAL-012, BR-017, AC-015, AC-026

- [ ] Design it (spec): `/architect edit config & drift`

### 28. Reconciliation setelah daemon restart · needs a decision · GA

Setelah daemon restart atau crash, rekonsiliasi proses, runtime, endpoint, storage, operasi, dan desired state. Proses yang masih hidup hanya diadopsi setelah identitasnya terbukti.

**Done when:** menutup dan membuka GUI tidak mengubah PID atau identitas instance; setelah daemon crash tidak ada instance ganda atau data yang terhapus; PID yang kini milik executable lain tidak dibunuh, dan instance dilaporkan unknown beserta buktinya; inventory tersedia dalam target NFR-007.

**Trace:** REQ-001, REQ-013, AC-001, AC-013, AC-067, NFR-007, OQ-007, PRD bagian 19.4

- [ ] Design it (spec): `/architect reconciliation setelah daemon restart`

### 29. Autostart instance & login · needs a decision

Autostart daemon dan autostart instance sebagai dua setting berbeda, dengan model demand (manual, startup) dan suppression: stop eksplisit tidak dibatalkan reconciler di sesi yang sama.

**Done when:** setelah login dan izin tersedia, daemon serta instance terpilih ready; izin yang dicabut terlihat di UI; stop manual tidak langsung dinyalakan lagi; Keychain yang terkunci menghasilkan needs secret, bukan auth kosong.

**Trace:** REQ-014, BR-024, AC-014, AC-062, AC-069, PRD bagian 13.2

- [ ] Design it (spec): `/architect autostart instance & login`

### 30. Menu bar & Settings

Status ringkas di menu bar dengan aksi buka aplikasi, **Quit App** yang hanya menutup GUI, dan **Stop Services** sebagai aksi terpisah dengan impact preview. Halaman Settings untuk background startup, default autostart, path, retention, dan privasi.

**Done when:** Quit App tidak menghentikan service; Stop Services menampilkan dampak sebelum berjalan; Settings menjelaskan perilaku autostart pada login berikutnya; telemetry mati secara default.

**Trace:** REQ-027 (menu bar), REQ-046 (privasi), PAGE-14, NFR-016

- [ ] Build it: `/develop menu bar & Settings`
