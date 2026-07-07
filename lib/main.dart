import 'package:flutter/material.dart';
import 'package:flame/game.dart';
import 'src/game/game_root.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  
  final game = CubeColonyGame();
  
  runApp(
    MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        // GameWidget adalah wadah dari Flame untuk menjalankan game loop
        body: GameWidget(game: game),
      ),
    ),
  );
}