import 'package:flame/game.dart';
import 'package:flutter/material.dart'; 
// PANGGIL SEMUANYA DARI SINI: FRB v2 default menyatukan semua struct & fungsi di file ini
import 'package:cube_colony/src/rust/api.dart'; 

class CubeColonyGame extends FlameGame {
  // Perbaikan Sintaks List: Menghapus keyword 'final workerX =' di dalam deklarasi isi array.
  List<WorkerDto> workers = [
    WorkerDto(
      id: 1,
      name: "Pekerja 1",
      tileTypeId: 0,
      resourceAmount: 0,
      x: 10.0,
      y: 20.0,
      colorHex: "0xFF4CAF50", 
    ),
    WorkerDto(
      id: 2,
      name: "Pekerja 2",
      tileTypeId: 0,
      resourceAmount: 0,
      x: 50.0,
      y: 60.0,
      colorHex: "0xFFFF9800", 
    ),
  ];

  // Flag untuk mencegah tabrakan frame asinkron Rust
  bool _isUpdating = false;

  @override
  void update(double dt) {
    super.update(dt);
    if (!_isUpdating) {
      _fetchNextState(dt);
    }
  }

  void _fetchNextState(double dt) async {
    _isUpdating = true;
    try {
      // updateGameState juga otomatis terbaca langsung dari frb_generated.dart
      final nextState = await updateGameState(deltaTime: dt, currentWorkers: workers);
      workers = nextState;
    } catch (e) {
      debugPrint("Gagal memperbarui state dari Rust: $e");
    } finally {
      _isUpdating = false;
    }
  }

  // Fungsi Helper untuk mengubah data String Hex dari Rust menjadi Objek Color Flutter
  Color _parseHexColor(String hexStr) {
    try {
      // Menghapus prefix bawaan jika ada dan membaca string radix 16 (Hexadecimal)
      final cleanHex = hexStr.replaceAll('#', '').replaceAll('0x', '');
      return Color(int.parse(cleanHex, radix: 16));
    } catch (e) {
      return const Color(0xFFFFFFFF); // Fallback ke warna putih jika format string salah
    }
  }

  @override
  void render(Canvas canvas) {
    super.render(canvas);

    canvas.drawColor(const Color(0xFF2E7D32), BlendMode.src); // Latar Hijau Tua

    for (var worker in workers) {
      // Perbaikan Baris 45: Menggunakan parser helper untuk mengubah String menjadi Integer warna yang valid
      final paint = Paint()
        ..color = _parseHexColor(worker.colorHex)
        ..style = PaintingStyle.fill;

      final rect = Rect.fromLTWH(worker.x, worker.y, 20.0, 20.0);
      canvas.drawRect(rect, paint);

      final strokePaint = Paint()
        ..color = const Color(0xFF000000)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2.0;
      canvas.drawRect(rect, strokePaint);
    }
  }
}
