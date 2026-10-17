# 0003. Rationale: sumber dan packaging runtime provider

[Kembali ke spec](index.md)

## Context

Stackly menjanjikan banyak versi dan banyak instance enam provider (PostgreSQL, MySQL, NATS, ClickHouse, Valkey, Traefik) di satu Mac Apple Silicon, tanpa Docker dan tanpa Homebrew global (basis: PRD bagian 20.2, RISK-01). Runtime managed harus punya versi dan artefak yang pasti, dependency closure (semua library yang ikut dimuat) yang terverifikasi, dan harus jalan dari folder Stackly sendiri dengan path absolut, tanpa PATH shell atau symlink versi global (basis: REQ-005). Fitur 18 (install runtime managed) tidak bisa dibangun sebelum jelas dari mana byte artefak itu datang, dan gate G0 butuh bukti bahwa setiap executable benar benar jalan di Mac uji.

Gayanya berbeda per provider. Sebagian upstream menerbitkan binary macOS arm64 resmi, sebagian hanya source. PostgreSQL bergantung pada `lib`, `share`, extension, serta OpenSSL dan ICU, dan tidak otomatis bisa dipindah ke folder lain. Homebrew memakai keg berversi dan symlink `opt` yang bisa berubah saat `brew upgrade` atau `brew cleanup`, jadi tidak cocok sebagai sumber pin (basis: PRD [S16]). Binary yang diunduh tidak ikut tercakup signing dan notarization app manager, dan Stackly tidak boleh menyuruh pengguna mematikan Gatekeeper atau menghapus quarantine secara global (basis: PRD bagian 23 dan 25, [S14]). Identity Developer ID (OQ-012) belum ada.

Batasan lain: tiap artefak butuh sumber dan keputusan distribusi yang tercatat, dan paket dengan izin atau sumber yang tidak jelas tetap `BLOCKED` (basis: AC-078, RISK-12). Repo sudah punya kontrak evidence native (spec 0002) dan satu Mac self hosted macOS 27 yang belum terdaftar. Workspace memakai `cargo xtask` sebagai task runner dan Rust untuk semua tooling (basis: AGENTS.md, spec 0001). Kalau keputusan ini ditunda, fitur 18 akan dibangun di atas URL dan digest karangan, atau diam diam bergantung pada Homebrew developer.

## Options considered

### Option 1: Sumber campuran, upstream resmi bila ada, pipeline maintainer untuk sisanya

Unduh langsung dari rilis resmi upstream dengan digest yang dipin bila upstream menerbitkan binary macOS arm64 (MySQL, NATS, ClickHouse, Traefik). Bangun sendiri PostgreSQL dan Valkey di pipeline dengan provenance (basis: OQ-003 default, "sumber resmi atau maintainer built dengan provenance").

**Pros**:

- Binary upstream dipakai apa adanya, termasuk signature Developer ID upstream bila ada; Stackly tidak mendistribusikan ulang binary GPL MySQL.
- Pipeline hanya untuk provider yang memang tidak punya binary relocatable, sehingga closure PostgreSQL (OpenSSL, ICU, lz4, zstd) dikendalikan penuh.

**Cons**:

- Dua jalur verifikasi (checksum upstream dan attestation sendiri) yang harus dirawat.
- URL upstream bisa pindah atau hilang (misalnya arsip MySQL); hanya digest dan URL kandidat yang menolong.

### Option 2: Semua dibangun maintainer

Pipeline membangun keenam provider dari source dan menerbitkan semuanya di hosting Stackly (basis: PRD bagian 20.2, mode build from source sebagai pipeline maintainer).

**Pros**:

- Satu cara verifikasi, satu format, satu kebijakan signing.
- Tidak bergantung pada ketersediaan URL upstream.

**Cons**:

- Build ClickHouse sangat berat (jam per build, toolchain LLVM tertentu) dan MySQL besar; biaya pipeline melonjak.
- Stackly mendistribusikan ulang semuanya, termasuk binary GPL MySQL dengan kewajiban menyediakan source.

### Option 3: Build pihak ketiga untuk yang tidak punya binary resmi

PostgreSQL dari `theseus-rs/postgresql-binaries` (17.11 dan 18.6 untuk `aarch64-apple-darwin`, ada `.sha256`), Valkey dan sisanya dari upstream atau build pihak ketiga lain (basis: cek lanskap 17 Oktober 2026).

**Pros**:

- Tanpa pipeline sendiri; artefak PostgreSQL langsung tersedia.

**Cons**:

- Flag build, closure, dan provenance di luar kendali; tanpa signature. Untuk Valkey tidak ada build macOS pihak ketiga yang setara, jadi pipeline tetap dibutuhkan.

### Option 4: Bottle Homebrew sebagai sumber

Ambil bottle dari registry Homebrew lalu relokasi sendiri (basis: PRD [S16], keg dan symlink `opt` Homebrew).

**Pros**:

- Hampir semua provider tersedia, dibangun rutin oleh proyek besar.

**Cons**:

- Bottle memakai placeholder prefix dan dependency antar bottle yang harus direlokasi dan diaudit sendiri; artefak lama dibersihkan dari registry, dan ketergantungan pada graf dependency Homebrew bertentangan dengan tujuan lepas dari Homebrew.

## Rationale

Option 1 dipilih karena memisahkan dua masalah yang memang berbeda. Untuk provider dengan binary resmi, risiko utamanya adalah supply chain dan URL yang hilang, dan digest yang dipin di inventaris yang direview menjawab keduanya tanpa menambah pekerjaan build. Untuk PostgreSQL, risiko utamanya relokasi dan closure (RISK-01), dan hanya build sendiri yang memberi kendali atas install name, dependency bundel, dan tanda tangan setelah `install_name_tool`. Valkey tidak punya binary macOS resmi sama sekali, jadi pipeline diperlukan apa pun pilihannya; memasukkan PostgreSQL ke pipeline yang sama hampir gratis.

Option 2 membayar biaya build ClickHouse dan kewajiban redistribusi GPL tanpa mengurangi risiko yang nyata. Option 3 menghemat waktu sekarang tapi meninggalkan pertanyaan provenance yang justru ditanyakan AC-078. Option 4 mempertahankan ketergantungan pada Homebrew yang ingin dihilangkan PRD.

Bukti dibuat lewat test native spec 0002, bukan format sendiri, karena fitur 65 membaca satu format evidence untuk semua bukti rilis. Worker dijalankan launchd lewat LaunchAgent sementara karena Terminal bisa punya izin Developer Tools yang melewati pemeriksaan Gatekeeper, sehingga run dari Terminal tidak mewakili cara stacklyd (agent SMAppService) menjalankan provider.

### Keputusan turunan

Keputusan yang kamu pilih saat wawancara:

| Keputusan | Pilihan | Runner up |
| --- | --- | --- |
| Kedalaman spike | start, ready, stop di temp root | hanya `--version` |
| Konteks launch | LaunchAgent sementara | runner CI self hosted |
| Bukti tanpa Homebrew | audit library statis dan runtime | VM macOS bersih |
| Lokasi inventaris | `runtimes/inventory.toml` dicek xtask | tabel Markdown di spec |
| Lokasi kode spike | subcommand xtask, dipanggil test native | skrip shell |
| Perilaku quarantine | direkam per artefak | diteliti di fitur 18 |
| Lingkup pipeline di fitur 3 | pipeline CI penuh dengan prerelease | resep lokal saja |
| Distribusi upstream | unduh langsung, digest dipin | mirror semua |
| Hosting artefak maintainer | repo terpisah `stackly-runtimes` | Releases di monorepo |
| Runner build | hosted `macos-26` arm64 | Mac self hosted macOS 27 |
| Provenance | GitHub attestation dan digest | kunci minisign sendiri |
| Sumber PostgreSQL | build maintainer | `theseus-rs` |
| Closure PostgreSQL | OpenSSL, ICU, lz4, zstd | OpenSSL dan ICU saja |
| MySQL | 8.4 LTS dan 9.7 LTS | 8.4 LTS saja |
| ClickHouse | LTS terbaru, fallback stable | stable terbaru |
| Signing upstream | pertahankan, ad hoc bila perlu | selalu ad hoc |
| Signing maintainer | ad hoc sekarang, Developer ID di fitur 60 | tunggu Developer ID |
| Valkey | 9.1, tanpa TLS | 8.1, atau dengan TLS |
| NATS, Traefik | 2.15 dan 3.7 terbaru | lini sebelumnya |
| Format artefak maintainer | `tar.gz` | `tar.zst` |
| Bump versi | cek mingguan, PR manual | manual sepenuhnya |
| Satuan inventaris | per artefak | per provider |
| Status inventaris | `available` atau `blocked`, hasil uji di evidence | status PRD 5.1 |
| Evidence | test native spec 0002, salinan run bersih di `evidence/runtime/` | perintah xtask lewat wrapper |
| Closure dicatat | bundel dan sistem | bundel saja |
| Trigger spike CI | setiap trigger `native.yml` | PR inventaris dan manual |
| Publish | tag terlindungi di `main` saja | dispatch dari branch mana pun |
| Aturan evidence | `available` butuh `PASS` | hanya dicatat |
| Watch | provider dan dependency bundel | provider saja |
| Extension PostgreSQL | contrib saja | contrib dan pgvector |
| Cache spike | per sha256, dicek ulang | selalu unduh baru |

Trigger spike CI awalnya kamu pilih "PR inventaris dan manual". Setelah spec 0002 ternyata sudah menetapkan `native.yml`, kamu memilih test native yang ikut semua trigger `native.yml`; pilihan pertama itu sudah tidak berlaku.

Keputusan yang saya ambil sendiri (rekomendasi, dengan runner up):

- **Unduhan lewat `/usr/bin/curl`**, bukan crate HTTP, supaya tidak mendahului pilihan HTTP client daemon di fitur 18, dan karena `curl` tidak memberi atribut quarantine, sama seperti unduhan daemon nanti. Runner up: `ureq`.
- **Digest dengan crate `sha2`** supaya `pin`, `verify`, dan `check` jalan juga di Linux CI. Runner up: `shasum -a 256`.
- **Audit library runtime dengan `lsof`**, bukan `DYLD_PRINT_LIBRARIES`, karena variabel `DYLD_*` diabaikan pada binary dengan hardened runtime (misalnya binary Developer ID upstream), sedangkan `lsof` melihat file yang benar benar dipetakan proses. Ini menyesuaikan pilihan "audit library" yang kamu ambil, yang deskripsinya menyebut `DYLD_PRINT_LIBRARIES`. Runner up: `DYLD_PRINT_LIBRARIES`.
- **Ekstraksi spike dengan `/usr/bin/tar`** setelah gerbang digest; extractor aman yang diuji fuzzing adalah milik fitur 18. Spike tetap melaporkan symlink, setuid, dan file device sebagai data untuk fitur 18. Runner up: crate `tar` dan `flate2` sekarang.
- **Root spike berisi spasi dan `Ü`**, dan socket di `/tmp/sk-*` yang pendek, karena path Application Support asli berisi spasi (PRD 32.2 meminta uji path spasi dan Unicode) dan path socket Unix dibatasi panjangnya.
- **Tanpa credential di spike** (PostgreSQL `trust` hanya lewat socket di folder 0700 dengan user database `stackly`, MySQL tanpa jaringan), supaya tidak ada secret yang bisa bocor ke evidence dan nama user OS tidak masuk ke role atau log. Peer auth akan memaksa nama user OS menjadi nama role. Runner up: `FixtureCredential` spec 0002 dengan `scram-sha-256`.
- **Batas waktu** ready 120 s (start pertama ClickHouse dan init MySQL lambat), stop 60 s, quarantine 30 s (dialog Gatekeeper bisa menahan proses), worker 12 menit, driver 15 menit.
- **Meson untuk PostgreSQL**, karena itu sistem build masa depan PostgreSQL dan didukung penuh di 17 dan 18. Runner up: `configure`.
- **Tool build dari PyPI dengan hash** (meson, ninja, pkgconf) dan `PATH` tanpa Homebrew, supaya library Homebrew bawaan runner tidak bisa ikut tertaut. Runner up: tool Homebrew runner plus audit saja.
- **OpenSSL lini 3.5 LTS** untuk umur dukungan panjang. Runner up: lini 3.x terbaru non LTS.
- **Fitur PostgreSQL yang dimatikan** (GSSAPI, LDAP, PAM, Bonjour, NLS, JIT LLVM, PL/Perl, PL/Python, PL/Tcl): jarang dipakai untuk dev lokal, dan PL/Perl atau PL/Python akan menautkan runtime bahasa sistem dengan ABI yang bisa berubah antar versi macOS.
- **Archive maintainer tanpa symlink**, supaya extractor fitur 18 cukup menangani file biasa untuk artefak buatan sendiri.
- **Satu test native per lini**, supaya bump versi tidak butuh perubahan kode dan satu lini yang gagal tidak menyeret yang lain. Runner up: satu test untuk semua artefak.
- **`covers = ["AC-078", "REQ-005"]`**, karena hanya ID PRD tiga digit yang diterima spec 0002, dan keduanya yang dibuktikan spike (inventaris distribusi dan runtime tanpa PATH atau symlink global).
- **Tipe inventaris di xtask**, bukan di `stackly-domain`, karena model runtime domain adalah keputusan fitur 4 dan 18. Runner up: tipe domain sekarang.
- **Owner default `@ojiepermana`** untuk entri `blocked`, sebagai runtime maintainer (owner OQ-003).
- **Server dijalankan langsung sebagai child** (`bin/postgres`, bukan `pg_ctl start`), karena `pg_ctl` memanggil `setsid` sehingga postmaster lepas dari process group dan tidak ikut dimatikan. Runner up: `pg_ctl` plus pembersihan lewat file PID.
- **Unduhan di driver, sebelum bootstrap**, supaya unduhan pertama ratusan MB tidak memakan batas waktu worker. Runner up: batas waktu worker yang jauh lebih panjang.
- **Lock file di folder cache dan label LaunchAgent tetap**, bukan `--test-threads=1`, supaya perintah native di spec 0002 tidak berubah dan daftar Login Items tidak bertambah setiap run. Runner up: `--test-threads=1` dan label per run.
- **Probe quarantine sekali per `sha256`**, karena dialog Gatekeeper bisa tertinggal di layar Mac CI setiap malam. Runner up: setiap run, atau hanya dengan flag manual.
- **Hanya domain `gui`**, tanpa fallback `user/<uid>`: runner self hosted sendiri berjalan di sesi GUI, dan agent SMAppService yang ingin ditiru juga hidup di sana. Tanpa sesi GUI, test `BLOCKED`. Runner up: fallback ke `user/<uid>`.
- **Dependency `own`** untuk library dari source provider sendiri (libpq, libmysqlclient), supaya aturan "inventaris sama dengan audit" tidak memaksa versi dan lisensi palsu untuk library yang sudah tercakup lisensi artefak. Runner up: mengecualikan library itu dari perbandingan.
- **Prefix build PostgreSQL tanpa kata `postgres` atau `pgsql`**, supaya PostgreSQL tetap memakai `lib/postgresql/` dan `share/postgresql/`, sesuai aturan rpath. Runner up: prefix yang memuat `postgresql` dengan layout datar.
- **OpenSSL `--openssldir=/etc/ssl`**, supaya OPENSSLDIR menunjuk lokasi yang ada di setiap Mac, bukan path build yang hilang setelah relokasi. Runner up: biarkan default dan tidak memakai sertifikat sistem.

### Cross check

Spec ditinjau sekali oleh model lain (read only) sebelum diterima. Temuan yang diterapkan: helper port sendiri di xtask (testkit hanya dependency dev), allowlist audit runtime untuk shared cache dyld di cryptex, server PostgreSQL sebagai child langsung, unduhan dipindah ke driver, `archive_root` opsional (Traefik), mekanisme plist dan pembersihan proses lewat `pgrep`, alur dua commit untuk evidence, normalisasi versi, aturan `BLOCKED` dan `FAIL` di test, dependency `own`, opsi configure resep, alur release immutable, MySQL tanpa checksum polos, quarantine lewat clone APFS sekali per digest, privasi user dan hostname, lock untuk test paralel, listener provider yang eksplisit, aturan 9 untuk delapan lini, dan `timeout-minutes` 90.

## Landscape scan 2026-10-17

Cek web sekali saat wawancara (5 pencarian, 8 halaman). Yang tertulis "belum terkonfirmasi" dikonfirmasi saat `runtime pin`.

| Provider | Temuan |
| --- | --- |
| PostgreSQL | Rilis minor terbaru 17.11 dan 18.6. Tidak ada archive relocatable resmi untuk macOS arm64. Zip macOS EDB belum terkonfirmasi; panduan terbaru menyebut installer `.dmg`. `theseus-rs/postgresql-binaries` punya `postgresql-<v>-aarch64-apple-darwin.tar.gz` dan `.sha256` untuk 17.11.0 dan 18.6.0 (terbit 15 Agustus 2026), tanpa signature. |
| MySQL | Halaman unduhan mendaftar 9.7.2 LTS dan 8.4.11 LTS, serta 26.7.0 sebagai GA teratas tanpa label LTS. Nama file tarball macOS ARM64, pola URL, signature untuk file macOS, dan perpindahan URL ke arsip belum terkonfirmasi. |
| NATS | v2.15.1 (9 Oktober 2026), aset `nats-server-v2.15.1-darwin-arm64.tar.gz`, `SHA256SUMS`, SBOM per aset, tanpa signature. |
| ClickHouse | Stable terbaru v26.9.20.23 (16 Oktober 2026) membawa `clickhouse-macos-aarch64` dan versi `.zip` aset itu; file `.sha512` hanya untuk paket Linux. Lini LTS 26.3; aset macOS di rilis LTS belum terkonfirmasi. |
| Valkey | 9.1.2 (1 September 2026), rilis GitHub tanpa aset. Binary macOS resmi belum terkonfirmasi ada di tempat lain. |
| Traefik | v3.7.15 (15 Oktober 2026), aset `traefik_v3.7.15_darwin_arm64.tar.gz`, `traefik_v3.7.15_checksums.txt`, tanpa signature. |

## References

**Project sources**:

- `prd.md`: bagian 5.2, 20.1, 20.2, 23, 25, 27, 28, 32.2, 33.2, 34 (G0), 37 (RISK-01, RISK-12), 38 (OQ-003, OQ-004, OQ-012), 39; REQ-004, REQ-005, REQ-049, AC-078; sumber [S14] (notarization Apple) dan [S16] (manpage Homebrew).
- `docs/scope/01-foundation.md` fitur 3, serta fitur 15, 18, 55, 56, dan 60 di scope lain.
- Spec 0001 (stack, `cargo xtask`, kebijakan `unsafe`, Mac self hosted) dan spec 0002 (evidence `stackly.evidence/v1`, `native.yml`, follow up fitur 3).
- `AGENTS.md` root (aturan workspace, testkit, CI).
- Skill terpasang: `rust-best-practices`, `meson`, `signing-entitlements`.

**Practices & standards**:

- Pin artefak dengan digest kriptografis, URL hanya sebagai lokasi.
- Provenance build dari runner ephemeral (gaya SLSA lewat GitHub artifact attestation).
- Relokasi Mach-O dengan `@rpath` dan `@loader_path`, lalu tanda tangan ulang setelah mengubah binary.
- Least privilege token CI: PR fork read only, OIDC hanya di job publish.

**Links** (terverifikasi saat cek lanskap):

- PostgreSQL versioning policy: https://www.postgresql.org/support/versioning/
- theseus-rs postgresql-binaries, rilis terbaru: https://api.github.com/repos/theseus-rs/postgresql-binaries/releases/latest
- MySQL Community Server downloads: https://dev.mysql.com/downloads/mysql/
- NATS server, rilis terbaru: https://api.github.com/repos/nats-io/nats-server/releases/latest
- ClickHouse, rilis terbaru: https://api.github.com/repos/ClickHouse/ClickHouse/releases/latest
- Valkey, rilis terbaru: https://api.github.com/repos/valkey-io/valkey/releases/latest
- Traefik, rilis terbaru: https://api.github.com/repos/traefik/traefik/releases/latest
