// rust/src/core/grid.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Tree,
    Storage,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
    pub resource_amount: u32, // Jumlah SDA yang tersisa di petak ini
}

pub struct GameMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::new();
        for y in 0..height {
            for x in 0..width {
                // Taruh pohon secara acak (misal tiap kelipatan grid tertentu)
                let is_tree = (x * 3 + y * 7) % 11 == 0;
                
                let (tile_type, resource_amount) = if is_tree {
                    (TileType::Tree, 5) // Setiap pohon punya 5 pasokan kayu
                } else {
                    (TileType::Empty, 0)
                };
                
                tiles.push(Tile { x, y, tile_type, resource_amount });
            }
        }
        GameMap { width, height, tiles }
    }
}