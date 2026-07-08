# Cube Colony 🟥🟩

Sebuah game simulasi manajemen koloni dan pertahanan minimalis 2D yang dibangun menggunakan **Flutter** untuk rendering visual dan **Rust** untuk core game loop yang performan. Terinspirasi dari mekanik rantai pasok *Achikaps* dan simulasi otonom *WorldBox*.

## 🚀 Fitur Utama
- **Murni Kode (Procedural Rendering):** Tidak menggunakan aset gambar eksternal, semua objek digambar langsung via Flutter Canvas.
- **High-Performance AI (Rust):** Logika pencarian jalan (*pathfinding*) ratusan unit warga dihitung dengan cepat di sisi Rust menggunakan `flutter_rust_bridge`.
- **Manajemen Otomatis:** Warga kotak akan bekerja secara mandiri mengatur rantai pasok kerajaan.

## 🛠️ Tech Stack
- **Frontend:** Flutter & Flame Engine (2D Game Loop)
- **Logic Engine:** Rust
- **Bridge:** `flutter_rust_bridge` (FRB v2)

## 📦 Cara Menjalankan (Development)
*(Akan diperbarui setelah konfigurasi bridge selesai)*

```

---

## 3. Struktur File Projek (Arsitektur Flutter + Rust)

Ini adalah struktur folder ideal untuk projek *hybrid* Flutter-Rust menggunakan `flutter_rust_bridge`. Struktur ini memisahkan kode UI dan kode simulasi Rust dengan sangat rapi.

```text
cube_colony/
│
├── rust/                       <-- SELURUH LOGIKA GAME DI SINI (RUST)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              <-- Entry point untuk Flutter Bridge
│       ├── api.rs              <-- Fungsi & Data yang diekspos ke Flutter
│       ├── core/
│       │   ├── game_state.rs   <-- Pengatur ticks, resource, & loop utama
│       │   ├── worker.rs       <-- Perilaku & AI Warga Kotak
│       │   └── grid.rs         <-- Logika map & koordinat heksagon/kotak
│       └── utils/
│           └── pathfinding.rs  <-- Algoritma pencarian rute (A*)
│
├── lib/                        <-- SELURUH TAMPILAN DI SINI (FLUTTER)
│   ├── main.dart               <-- Entry point aplikasi
│   ├── src/
│   │   ├── rust/               <-- Kode otomatis hasil generate dari Rust (Jangan diedit)
│   │   │   └── frb_generated.dart
│   │   ├── game/
│   │   │   ├── game_root.dart  <-- Mengatur inisialisasi Flame/Game Widget
│   │   │   ├── painters/
│   │   │   │   ├── map_painter.dart    <-- Menggambar grid tanah
│   │   │   │   └── entity_painter.dart <-- Menggambar warga & bangunan kotak
│   │   │   └── components/     <-- Logika input pengetukan (tap) layar
│   │   └── ui/
│   │       └── widgets/        <-- UI Overlay (Teks Wood, Food, Tombol Pause)
│   └── pubspec.yaml
│
└── README.md

```

---


## 📦 Panduan Pengembangan

### 1. Prasyarat
- Flutter SDK & Rust Toolchain
- `cargo install flutter_rust_bridge_codegen` [1.3]

### 2. Generator Kode
Jalankan perintah ini setiap kali mengubah kode `rust/src/` [1.3]:
```bash
flutter_rust_bridge_codegen generate
```

### 3. Menjalankan
```bash
flutter run
```

---

### Catatan: Inisialisasi `main.dart`
Pastikan `lib/main.dart` memanggil `await RustLib.init();` sebelum `runApp()` untuk menginisialisasi jembatan Rust [1.3].
