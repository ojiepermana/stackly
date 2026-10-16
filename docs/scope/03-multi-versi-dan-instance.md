# Scope 03 · Multi versi & instance

[Kembali ke index](index.md)

Journey PRD JOURNEY-02 dan JOURNEY-03, nilai inti produk (GOAL-01): `pg17-dev`, `pg17-test`, dan `pg18-dev` jalan bersamaan dengan data, config, log, dan port yang terpisah. Fase ini selesai kalau data sentinel tiap instance tidak pernah bocor ke instance lain, termasuk setelah tiap instance dihentikan lalu dijalankan lagi (restart sebagai satu operasi datang di fitur 26).

### 24. Instance kedua dari runtime yang sama

Buat instance kedua yang memakai runtime ID identik, lewat create biasa atau duplicate config. Duplicate config membuat storage baru, bukan clone data.

**Done when:** dua instance PG17 dengan runtime ID sama jalan bersamaan dengan port dan data berbeda; duplicate config menghasilkan ID, path data, kebijakan secret, dan port baru yang tervalidasi; data lama tidak pernah disalin.

**Trace:** REQ-009, REQ-025 (duplicate config), AC-009, AC-025, JOURNEY-02

- [ ] Build it: `/develop instance kedua dari runtime yang sama`

### 25. PostgreSQL 17 & 18 berdampingan

Pasang runtime mayor kedua dan buat instance baru di sampingnya tanpa menyentuh data instance lama. Binary mayor yang salah terhadap data yang sudah ada selalu ditolak.

**Done when:** tiga cluster (dua PG17, satu PG18) menyimpan sentinel berbeda dan tidak saling terbaca setelah stop dan start ulang; menjalankan binary PG18 pada data PG17, atau rebind biasa, ditolak oleh preflight kompatibilitas dari fitur 19 dengan alasan butuh migrasi; install runtime baru tidak mengubah instance lama.

**Trace:** REQ-035 (multi versi), REQ-043 (penolakan rebind), BR-001, BR-009, AC-035 (tanpa bagian backup dan restore, yang menyusul di fase 09), AC-043, JOURNEY-03

- [ ] Build it: `/develop PostgreSQL 17 & 18 berdampingan`
