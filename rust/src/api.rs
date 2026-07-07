// rust/src/api.rs

// Struktur data yang ringkas untuk dikirim ke Flutter setiap frame
pub struct WorkerDto {
    pub x: f32,
    pub y: f32,
    pub color_hex: u32,
}

// State global sementara di sisi Rust (menggunakan Mutex/Lazy static aman)
// Untuk prototipe awal, kita buat fungsi murni yang mengembalikan data update
pub fn update_game_state(delta_time: f32, current_workers: Vec<WorkerDto>) -> Vec<WorkerDto> {
    // Di langkah berikutnya, ini akan membaca dari static GameState kita.
    // Sementara kita buat simulasi pergerakan langsung di sini agar Flutter bisa ngetes bridge-nya.
    current_workers.into_iter().map(|mut w| {
        let mut x = w.x + 20.0 * delta_time; // Bergerak ke kanan perlahan
        if x > 400.0 { x = 10.0; } // Reset kalau mentok
        WorkerDto { x, y: w.y, color_hex: w.color_hex }
    }).collect()
}