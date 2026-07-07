// rust/src/core/grid.rs

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Tree,
    Storage,
}

pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
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
                // Beri beberapa pohon secara acak sebagai simulasi awal
                let tile_type = if (x + y) % 7 == 0 {
                    TileType::Tree
                } else {
                    TileType::Empty
                };
                
                tiles.push(Tile { x, y, tile_type });
            }
        }
        GameMap { width, height, tiles }
    }
}