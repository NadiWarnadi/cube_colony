// rust/src/api.rs

#[derive(Debug, Clone)]
pub struct TileDto {
    pub id: i32,
    pub tile_type_id: i32,
    pub resource_amount: i32,
}

#[derive(Debug, Clone)]
pub struct WorkerDto {
    pub id: i32,
    pub name: String,
    pub tile_type_id: i32,
    pub resource_amount: i32,
    pub x: f64,             // Ditambahkan untuk koordinat horizontal
    pub y: f64,             // Ditambahkan untuk koordinat vertikal
    pub color_hex: String,  // Ditambahkan untuk kode warna, otomatis menjadi colorHex di Dart
}

// Periksa juga fungsi pembuat objek WorkerDto di bawahnya jika ada (misal baris 36)
// Pastikan semua field baru di atas sudah diisi nilai awalnya, contoh:
pub fn create_worker_sample() -> WorkerDto {
    WorkerDto {
        id: 1,
        name: "Warga Kotak".to_string(),
        tile_type_id: 0,
        resource_amount: 0,
        x: 0.0,
        y: 0.0,
        color_hex: "#FF0000".to_string(),
    }
}

// Tambahkan fungsi ini di dalam rust/src/api.rs jika belum ada, atau pastikan ada keyword 'pub'
pub fn get_initial_worker() -> WorkerDto {
    WorkerDto {
        id: 1,
        name: "Warga Kotak".to_string(),
        tile_type_id: 0,
        resource_amount: 0,
        x: 0.0,
        y: 0.0,
        color_hex: "#00FF00".to_string(),
    }
}


// 1. Modul internal harus di-import ke api.rs
use crate::core::worker::{CubeWorker, WorkerStatus};

// 2. Fungsi update_game_state yang diekspos ke FLUTTER (Dipanggil oleh FRB v2)
pub fn update_game_state(delta_time: f64, current_workers: Vec<WorkerDto>) -> Vec<WorkerDto> {
    let mut next_workers = Vec::new();

    for dto in current_workers {
        // A. Ubah data dari Flutter (WorkerDto) menjadi data internal Rust (CubeWorker)
        let mut internal_worker = CubeWorker {
            id: dto.id as u32,
            x: dto.x as f32,
            y: dto.y as f32,
            // Sementara target disamakan dulu agar tidak langsung berpindah tanpa tujuan
            target_x: dto.x as f32, 
            target_y: dto.y as f32,
            status: WorkerStatus::Idle, // Nanti sesuaikan statusnya jika sudah kompleks
            color_hex: u32::from_str_radix(&dto.color_hex.replace("0x", "").replace("#", ""), 16).unwrap_or(0xFFFFD54F),
            speed: 50.0,
        };

        // B. JALANKAN LOGIKA JALAN WARGA yang Anda buat di worker.rs!
        internal_worker.update(delta_time as f32);

        // C. Ubah kembali hasil kalkulasi Rust ke WorkerDto untuk dikirim balik ke Flutter
        next_workers.push(WorkerDto {
            id: internal_worker.id as i32,
            name: dto.name, // Pertahankan nama asli dari Flutter
            tile_type_id: dto.tile_type_id,
            resource_amount: dto.resource_amount,
            x: internal_worker.x as f64,
            y: internal_worker.y as f64,
            color_hex: format!("0x{:X}", internal_worker.color_hex),
        });
    }

    next_workers
}
