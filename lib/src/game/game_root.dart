import 'package:flame/game.dart';
import 'package:flutter/material.dart'; 
// PANGGIL SEMUANYA DARI SINI: FRB v2 default menyatukan semua struct & fungsi di file ini
import 'package:cube_colony/src/rust/api.dart'; 

class CubeColonyGame extends FlameGame {
  // WorkerDto kini otomatis terbaca langsung dari frb_generated.dart
  List<WorkerDto> workers = [
    WorkerDto(x: 50.0, y: 50.0, colorHex: 0xFFFFD54F),
    WorkerDto(x: 100.0, y: 80.0, colorHex: 0xFFFFD54F),
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

  @override
  void render(Canvas canvas) {
    super.render(canvas);

    canvas.drawColor(const Color(0xFF2E7D32), BlendMode.src); // Latar Hijau Tua

    for (var worker in workers) {
      final paint = Paint()
        ..color = Color(worker.colorHex)
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
