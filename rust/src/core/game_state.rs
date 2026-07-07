// rust/src/core/game_state.rs

use crate::core::grid::GameMap;
use crate::core::worker::CubeWorker;

pub struct GameState {
    pub map: GameMap,
    pub workers: Vec<CubeWorker>,
    pub wood_count: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            map: GameMap::new(20, 20), // Map ukuran 20x20 tile
            workers: Vec::new(),
            wood_count: 0,
        };

        // Spawn 3 warga kotak pertama di tengah map
        state.workers.push(CubeWorker::new(1, 100.0, 100.0));
        state.workers.push(CubeWorker::new(2, 120.0, 100.0));
        state.workers.push(CubeWorker::new(3, 140.0, 100.0));

        state
    }

    // Fungsi krusial yang dipanggil setiap frame game berjalan
    pub fn update(&mut self, delta_time: f32) {
        for worker in &mut self.workers {
            worker.update(delta_time);
            
            // Logika AI sederhana: Jika idle, beri target acak biar jalan-jalan
            if worker.status == crate::core::worker::WorkerStatus::Idle && worker.x == worker.target_x {
                // Hanya simulasi random walk sederhana dulu
                worker.target_x = (worker.x + 50.0) % 500.0;
                worker.target_y = (worker.y + 30.0) % 500.0;
            }
        }
    }
}