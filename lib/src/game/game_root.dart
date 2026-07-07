import 'dart:ui';
import 'package:flame/game.dart';
import 'package:flutter/material.dart' as dart_colors;
// Import hasil generate jembatan Rust kamu (sesuaikan nama package jika berbeda)
import 'package:cube_colony/src/rust/api.rs' as rust_api; 

class CubeColonyGame extends FlameGame {
  // Kita simpan data warga lokal di Flutter yang datanya selalu diperbarui dari Rust
  List<rust_api.WorkerDto> workers = [
    rust_api.WorkerDto(x: 50, y: 50, colorHex: 0xFFFFD54F),
    rust_api.WorkerDto(x: 100, y: 80, colorHex: 0xFFFFD54F),
  ];

  @override
  void update(double dt) {
    super.update(dt);
    
    // TIAP DETIK, MINTA RUST UNTUK MENGHITUNG PERGERAKAN BARU
    // Fungsi 'updateGameState' ini berasal dari file rust/src/api.rs kita kemarin
    workers = rust_api.updateGameState(deltaTime: dt, currentWorkers: workers);
  }

  @override
  void render(Canvas canvas) {
    super.render(canvas);

    // 1. Gambar Background Tanah Semen/Rumput Minimalis
    canvas.drawColor(const Color(0xFF2E7D32), BlendMode.src); // Hijau Tua

    // 2. Gambar Semua Warga Kotak dari Data Rust
    for (var worker in workers) {
      final paint = Paint()
        ..color = Color(worker.colorHex)
        ..style = PaintingStyle.fill;

      // Gambar tubuh kotak (Ukuran 20x20 piksel)
      final rect = Rect.fromLTWH(worker.x, worker.y, 20.0, 20.0);
      canvas.drawRect(rect, paint);

      // Beri garis tepi hitam biar rapi ala Achikaps
      final strokePaint = Paint()
        ..color = const Color(0xFF000000)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2.0;
      canvas.drawRect(rect, strokePaint);
    }
  }
}