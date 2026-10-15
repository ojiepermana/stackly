# Scope 09 · Backup & restore

[Kembali ke index](index.md)

Journey PRD JOURNEY-06: backup terverifikasi, restore as new, isi ulang secret, cek readiness dan data, lalu pengguna memilih connection baru sementara instance sumber tetap ada. Backup belum dianggap recoverable sebelum restore drill berhasil. Fase ini selesai kalau setiap provider persistent yang diklaim supported bisa dipulihkan sesuai cakupan backup yang dijanjikan.

### 46. Backup metadata registry · needs a decision · GA

Snapshot registry yang konsisten saat aplikasi aktif, dengan versi schema, checksum, dan secret reference, memakai mekanisme backup SQLite yang benar. Backup ini tidak diklaim mencakup database provider atau isi Keychain.

**Done when:** registry yang dipulihkan di profil uji konsisten, secret yang hilang ditandai, dan tidak ada instance yang diinisialisasi otomatis di path data yang sudah ada; migrasi registry yang gagal di tengah jalan membuat daemon masuk safe recovery state.

**Trace:** REQ-040, BR-022, AC-040, AC-065, PRD bagian 24.1

- [ ] Design it (spec): `/architect backup metadata registry`

### 47. Backup data per provider · needs a decision · GA

Backup native kalau capability tersedia (logical untuk PostgreSQL dan MySQL), dan cold backup hanya setelah stop bersih untuk layout yang tervalidasi. Tiap backup mencatat mode konsistensi, scope, identitas runtime, digest file, ukuran, dan status verifikasi. Termasuk halaman Backup & Restore.

**Done when:** cold backup selalu didahului quiesce dan verifikasi bahwa tidak ada proses yang menulis; instance yang gagal stop bersih tidak menghasilkan backup berlabel consistent; path eksternal yang tidak didukung membuat operasi ditolak; disk penuh di staging tidak menyentuh sumber; tidak ada auto delete backup.

**Trace:** REQ-041, PAGE-11, BR-022, VAL-016, AC-041, AC-063, AC-064, AC-077, OQ-010

- [ ] Design it (spec): `/architect backup data per provider`

### 48. Restore as new & verifikasi · needs a decision · GA

Alur restore yang secara default membuat instance baru dengan endpoint baru: preflight ruang, izin, dan kebutuhan secret, staging milik operasi, lalu start, readiness, dan verifikasi data fixture. Overwrite instance yang sudah ada adalah aksi terpisah dengan preview dan konfirmasi.

**Done when:** restore as new tidak mengubah instance sumber, dan instance baru lulus query fixture; checksum atau kompatibilitas yang gagal tidak menimpa target; cancel saat staging tidak mengubah target asli; tanggal restore terverifikasi tercatat sebagai bukti.

**Trace:** REQ-042, BR-021, AC-042, AC-054, AC-064, AC-081 (bagian restore), JOURNEY-06, PRD bagian 24.2

- [ ] Design it (spec): `/architect restore as new & verifikasi`
