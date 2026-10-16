# Scope 11 · Distribusi aplikasi

[Kembali ke index](index.md)

Journey memasang Stackly sebagai pengguna akhir: unduh aplikasi bertanda tangan di Mac bersih, pakai, update, lalu uninstall dan pasang ulang tanpa kehilangan data. Fase ini selesai kalau seluruh jalur itu lulus di profil nondeveloper tanpa Rust, Xcode, atau mematikan Gatekeeper.

### 60. Packaging, signing & notarization · needs a decision · GA

Pipeline rilis untuk bundle yang sudah dipakai sejak fitur 15: `.app` dengan helper daemon dan CLI, ditandatangani dan dinotarisasi, dengan installer, checksum, release notes, daftar OS/arch, dan third party notices. Signing hanya dilakukan dari konteks release yang tepercaya.

**Done when:** di profil macOS bersih, GUI, daemon, CLI, dan provider wajib bisa dipakai tanpa toolchain developer dan tanpa mematikan Gatekeeper; pipeline yang gagal notarization tidak mempublikasikan artefak sebagai stable; setiap artefak provider punya sumber dan keputusan distribusi yang tercatat.

**Trace:** REQ-049, AC-049, AC-075, AC-078, OQ-001, OQ-012, RISK-10, RISK-12, PRD bagian 25

- [ ] Design it (spec): `/architect packaging, signing & notarization`

### 61. Update aplikasi & kompatibilitas · needs a decision · GA

Update GUI, CLI, dan daemon yang memeriksa kompatibilitas protokol dan schema registry, menghormati operasi aktif dan jendela restart, serta membackup registry sebelum migrasi schema. Upgrade tool database tidak pernah berjalan otomatis.

**Done when:** setelah update, runtime ID dan data provider tidak berubah; operasi aktif ditangani dengan aman; schema yang tidak kompatibel mencegah rollback buta; cek update hanya mengambil metadata, dan hanya setelah ada persetujuan.

**Trace:** REQ-044, BR-013, AC-044, AC-065, RISK-09, PAGE-14 (update channel), PRD bagian 24.3 dan 25

- [ ] Design it (spec): `/architect update aplikasi & kompatibilitas`

### 62. Uninstall & reinstall dengan data tetap · needs a decision · GA

Uninstall menghentikan service secara terkontrol, melepas helper background, dan mempertahankan data serta metadata dengan penjelasan cara recovery. Keputusannya: mekanisme uninstall (perintah GUI dan CLI, atau uninstaller dalam artefak fitur 60), urutan unregister helper (cermin fitur 15), dan scope cleanup cache serta runtime; reinstall memakai rekonsiliasi fitur 30, kompatibilitas schema fitur 61, dan adoption fitur 57.

**Done when:** menghapus app bundle tidak menghapus database pengguna; uninstall menjelaskan apa yang tetap (registry, data, secret) dan apa yang dihapus (bundle, registrasi helper), dan tidak meninggalkan background item yatim; reinstall merekonsiliasi registry dan data tanpa `initdb`, dan nama instance tidak memicu overwrite; cache dan runtime hanya dihapus dengan scope yang jelas.

**Trace:** REQ-049 (bagian uninstaller), REQ-052 (bagian uninstall), AC-066, NFR-015, PRD bagian 20.1 dan 35.3

- [ ] Design it (spec): `/architect uninstall & reinstall dengan data tetap`
