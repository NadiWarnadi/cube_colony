// rust/src/utils/pathfinding.rs

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    f_score: i32,
}

// Urutkan BinaryHeap agar Node dengan f_score terkecil keluar duluan
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Fungsi Utama A* Pathfinding
pub fn find_path(start: Point, target: Point, width: i32, height: i32, blocked_tiles: &[Point]) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, i32> = HashMap::new();
    let mut f_score: HashMap<Point, i32> = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, (start.x - target.x).abs() + (start.y - target.y).abs());

    open_set.push(Node { point: start, f_score: f_score[&start] });

    while let Some(Node { point: current, .. }) = open_set.pop() {
        if current == target {
            // Rekonstruksi jalur dari akhir ke awal
            let mut path = vec![current];
            let mut curr = current;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(prev);
                curr = prev;
            }
            path.reverse();
            return Some(path);
        }

        // Cek 4 arah tetangga (Atas, Bawah, Kiri, Kanan)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &(dx, dy) in &directions {
            let neighbor = Point { x: current.x + dx, y: current.y + dy };

            // Pastikan tetangga masih di dalam batas map
            if neighbor.x < 0 || neighbor.x >= width || neighbor.y < 0 || neighbor.y >= height {
                continue;
            }

            // Pastikan tetangga tidak diblokir (misal ada tembok/bangunan lain)
            if blocked_tiles.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap_or(&i32::MAX) + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                
                // Heuristic: Manhattan Distance
                let h = (neighbor.x - target.x).abs() + (neighbor.y - target.y).abs();
                f_score.insert(neighbor, tentative_g_score + h);

                open_set.push(Node { point: neighbor, f_score: tentative_g_score + h });
            }
        }
    }

    None // Jalur tidak ditemukan
}