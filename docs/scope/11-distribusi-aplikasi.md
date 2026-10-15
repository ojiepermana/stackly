# Scope 11 · Distribusi aplikasi

[Kembali ke index](index.md)

Journey memasang Stackly sebagai pengguna akhir: unduh aplikasi bertanda tangan di Mac bersih, pakai, update, lalu uninstall dan pasang ulang tanpa kehilangan data. Fase ini selesai kalau seluruh jalur itu lulus di profil nondeveloper tanpa Rust, Xcode, atau mematikan Gatekeeper.

### 54. Packaging, signing & notarization · needs a decision · GA

`.app` dengan helper daemon dan CLI, ditandatangani dan dinotarisasi, dengan installer, checksum, release notes, daftar OS/arch, dan third party notices. Signing hanya dilakukan dari konteks release yang tepercaya.

**Done when:** di profil macOS bersih, GUI, daemon, CLI, dan provider wajib bisa dipakai tanpa toolchain developer dan tanpa mematikan Gatekeeper; pipeline yang gagal notarization tidak mempublikasikan artefak sebagai stable; setiap artefak provider punya sumber dan keputusan distribusi yang tercatat.

**Trace:** REQ-049, AC-049, AC-075, AC-078, OQ-001, OQ-012, RISK-10, RISK-12, PRD bagian 25

- [ ] Design it (spec): `/architect packaging, signing & notarization`

### 55. Update aplikasi & kompatibilitas · needs a decision · GA

Update GUI, CLI, dan daemon yang memeriksa kompatibilitas protokol dan schema registry, menghormati operasi aktif dan jendela restart, serta membackup registry sebelum migrasi schema. Upgrade tool database tidak pernah berjalan otomatis.

**Done when:** setelah update, runtime ID dan data provider tidak berubah; operasi aktif ditangani dengan aman; schema yang tidak kompatibel mencegah rollback buta; cek update hanya mengambil metadata, dan hanya setelah ada persetujuan.

**Trace:** REQ-044, BR-013, AC-044, AC-065, PRD bagian 24.3 dan 25

- [ ] Design it (spec): `/architect update aplikasi & kompatibilitas`

### 56. Uninstall & reinstall dengan data tetap · GA

Uninstall menghentikan service secara terkontrol, melepas helper background, dan mempertahankan data serta metadata dengan penjelasan cara recovery. Reinstall versi yang kompatibel merekonsiliasi atau mengadopsi data yang tersisa.

**Done when:** menghapus app bundle tidak menghapus database pengguna; reinstall merekonsiliasi registry dan data tanpa `initdb`, dan nama instance tidak memicu overwrite; cache dan runtime hanya dihapus dengan scope yang jelas.

**Trace:** REQ-052 (bagian uninstall), AC-066, NFR-015, PRD bagian 35.3

- [ ] Build it: `/develop uninstall & reinstall dengan data tetap`
