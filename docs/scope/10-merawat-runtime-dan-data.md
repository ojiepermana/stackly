# Scope 10 · Merawat runtime & data

[Kembali ke index](index.md)

Journey pemeliharaan: hapus instance tanpa kehilangan data, bersihkan runtime yang tidak terpakai, daftarkan runtime Homebrew yang sudah ada, temukan data yatim, dan perbarui runtime tanpa merusak data. Fase ini selesai kalau tidak ada satu pun aksi pembersihan yang bisa menghapus data di luar scope atau tanpa konfirmasi eksplisit.

### 49. Remove, retain & purge · needs a decision · GA

Remove instance mempertahankan data sebagai retained record; purge adalah aksi terpisah dengan preview path, ukuran, dampak, dan konfirmasi eksplisit.

**Done when:** remove tanpa purge menyisakan data dan tercatat sebagai retained; purge di luar owned root ditolak walau lewat symlink, termasuk race yang bisa direproduksi; purge memeriksa ulang revision, operasi aktif, reference environment, dan path saat commit.

**Trace:** REQ-052, BR-006, VAL-007, VAL-015, AC-052, AC-057

- [ ] Design it (spec): `/architect remove, retain & purge`

### 50. Hapus runtime & garbage collection

Penghapusan runtime managed ditolak selama masih ada reference dari instance (termasuk yang stopped), dependency, operasi, atau snapshot recovery. GC hanya membersihkan cache dan runtime tak terpakai milik manager.

**Done when:** runtime yang dipakai instance stopped ditahan beserta daftar reference yang menahannya, dan data tidak berubah; GC tidak menyentuh runtime external atau dependency aktif; `stackly runtime remove` dan GUI menampilkan alasan yang sama.

**Trace:** REQ-007, BR-007, AC-007, PRD bagian 35.2

- [ ] Build it: `/develop hapus runtime & garbage collection`

### 51. Runtime external (Homebrew) · needs a decision

Daftarkan instalasi yang sudah ada dan terverifikasi sebagai runtime external, dengan ownership dan batas reproduksibilitas yang terlihat. Stackly tidak menjalankan `brew upgrade` atau `brew cleanup`, tidak mengganti symlink, dan tidak mengambil alih service atau data yang sudah ada.

**Done when:** binary external yang diubah atau dihapus dari luar terdeteksi sebagai drift atau missing; Stackly tidak beralih ke binary lain dan tidak menghapus instalasi itu; `stackly runtime register` dan halaman Runtimes menandai sumbernya sebagai external.

**Trace:** REQ-006, BR-019, VAL-009, AC-006, PRD bagian 20.2

- [ ] Design it (spec): `/architect runtime external (Homebrew)`

### 52. Scan data orphan & adoption · needs a decision · GA

Temukan direktori yang sudah ada atau orphan dan inventarisasi secara read only. Adoption hanya lewat preflight versi, config, dan ownership proses, lalu tindakan eksplisit pengguna; tidak pernah ada `initdb`, reset, atau upgrade otomatis.

**Done when:** scan data PostgreSQL orphan hanya menemukan metadata; adoption tidak berjalan sebelum kompatibilitas dan ownership terverifikasi dan pengguna setuju; `stackly instance adopt` dan GUI setara.

**Trace:** REQ-053, BR-008, VAL-009, AC-053, OQ-018

- [ ] Design it (spec): `/architect scan data orphan & adoption`

### 53. Update runtime & provider revision · needs a decision · GA

Empat operasi yang selalu terpisah: install runtime baru, rebind ke runtime yang kompatibel, update provider revision, dan migrasi data mayor (manual). Provider revision baru tidak mengubah instance lama sebelum pengguna memilih apply setelah melihat diff.

**Done when:** rebind hanya diizinkan untuk pasangan kompatibel yang terverifikasi, dengan stop terkontrol dan verifikasi start; update provider revision menampilkan diff capability, template, dan izin; migrasi mayor diarahkan ke backup dan restore, tidak pernah sekadar mengganti binary.

**Trace:** REQ-043, BR-013, BR-017, AC-043, PRD bagian 24.3

- [ ] Design it (spec): `/architect update runtime & provider revision`
