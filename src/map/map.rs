use bracket_lib::prelude::*;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

pub fn basic_map() -> Map {
    let mut new_map = Map{
        tiles: vec![TileType::Floor; 80*50],
        blocked: vec![false; 80*50],
        tile_content: vec![Vec::new(); 80*50],
        width: 80 as i32,
        height: 50 as i32
    };

    // Make the boundaries walls
    for x in 0..80 {
        let foo = new_map.xy_idx(x, 0);
        let bar = new_map.xy_idx(x, 49);
        new_map.tiles[foo] = TileType::Wall;
        new_map.tiles[bar] = TileType::Wall;
        new_map.blocked[foo] = true;
        new_map.blocked[bar] = true;
    }

    for y in 0..50 {
        let foo = new_map.xy_idx(0, y);
        let bar = new_map.xy_idx(79, y);
        new_map.tiles[foo] = TileType::Wall;
        new_map.tiles[bar] = TileType::Wall;
        new_map.blocked[foo] = true;
        new_map.blocked[bar] = true;        
    }

    let mut rng = bracket_lib::random::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = new_map.xy_idx(x, y);
        if idx != new_map.xy_idx(40, 25) {
            new_map.tiles[idx] = TileType::Wall;
            new_map.blocked[idx] = true;
        }        
    }
  new_map
}

pub fn draw_map(ecs: &World, ctx : &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;
  
    for (_idx, tile) in map.tiles.iter().enumerate() {
        let glyph;
        let fg;
        let bg;
        match tile {
        TileType::Floor => {
            glyph = bracket_lib::terminal::to_cp437('.');
            fg = RGB::from_f32(0.0, 0.2, 0.2);
            bg = RGB::from_f32(0., 0., 0.);
        }
        TileType::Wall => {
            glyph = bracket_lib::terminal::to_cp437('#');
            fg = RGB::from_f32(0., 1.0, 0.);
            bg = RGB::named(bracket_lib::color::ROYALBLUE3);
        }
        }
        ctx.set(x, y, fg, bg, glyph);
  
        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}