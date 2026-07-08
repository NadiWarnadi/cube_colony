import 'dart:ui';
import 'package:flame/game.dart';
import 'package:flutter/material.dart' as dart_colors;
// Import hasil generate jembatan Rust kamu 
import 'package:cube_colony/src/rust/frb_generated.dart';

// FRB v2 otomatis membuat pemetaan struktur Rust ke Dart di sub-berkas ini:
import 'package:cube_colony/src/rust/api/api.dart'; 

class CubeColonyGame extends FlameGame {
  //  PERBAIKAN: Hapus prefix 'rust_api.', panggil WorkerDto langsung
  List<WorkerDto> workers = [
    WorkerDto(x: 50, y: 50, colorHex: 0xFFFFD54F),
    WorkerDto(x: 100, y: 80, colorHex: 0xFFFFD54F),
  ];

  @override
  void update(double dt) {
    super.update(dt);
    
    // TIAP DETIK, MINTA RUST UNTUK MENGHITUNG PERGERAKAN BARU
    //  PERBAIKAN: Hapus 'rust_api.', panggil fungsi updateGameState langsung.
    // Jika fungsi di Rust mengembalikan Future (async), tambahkan keyword 'await' dan ubah method update menjadi async.
    _fetchNextState(dt);
  }

  // Helper method untuk memproses update state dari Rust
  void _fetchNextState(double dt) async {
    // Nama fungsi dari Rust otomatis berubah menjadi camelCase di Dart (updateGameState)
    workers = await updateGameState(deltaTime: dt, currentWorkers: workers);
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
