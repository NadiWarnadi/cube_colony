// rust/src/core/worker.rs

#[derive(Clone, Copy, PartialEq)]
pub enum WorkerStatus {
    Idle,
    MovingToTree,
    ChippingTree,
    ReturningToStorage,
}

pub struct CubeWorker {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub status: WorkerStatus,
    pub color_hex: u32,
    pub speed: f32,
}

impl CubeWorker {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        CubeWorker {
            id,
            x,
            y,
            target_x: x,
            target_y: y,
            status: WorkerStatus::Idle,
            color_hex: 0xFFFFD54F, // Warna Kuning Amber khas warga
            speed: 50.0,            // Piksel per detik
        }
    }

    // Update posisi warga menuju target
    pub fn update(&mut self, delta_time: f32) {
        let dx = self.target_x - self.x;
        let dy = self.target_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 1.0 {
            // Bergerak mendekati target
            self.x += (dx / distance) * self.speed * delta_time;
            self.y += (dy / distance) * self.speed * delta_time;
        } else {
            // Sudah sampai di target
            self.x = self.target_x;
            self.y = self.target_y;
            // Di sini nanti kita ubah statusnya (misal mulai nebang pohon)
        }
    }
}