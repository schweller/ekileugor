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
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub width: i32,
    pub height: i32
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

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 { return false };
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }    
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
      Point::new(self.width, self.height)
    }
  }
  
  impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
      self.tiles[idx as usize] == TileType::Wall
    }
  
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
      let w = self.width as usize;
      let p1 = Point::new(idx1 % w, idx1 / w);
      let p2 = Point::new(idx2 % w, idx2 / w);
      bracket_lib::pathfinding::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
  
    fn get_available_exits(&self, idx: usize) -> bracket_lib::pathfinding::SmallVec<[(usize, f32); 10]> {
      let mut exits = bracket_lib::pathfinding::SmallVec::new();
      let x = idx as i32 % self.width;
      let y = idx as i32 / self.width;
      let w = self.width as usize;
  
      if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
      if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
      if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
      if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };
  
      exits
    }
  }

pub fn basic_map() -> Map {
    let mut new_map = Map{
        tiles: vec![TileType::Floor; 80*50],
        blocked: vec![false; 80*50],
        tile_content: vec![Vec::new(); 80*50],
        visible_tiles: vec![false; 80*50],
        revealed_tiles: vec![false; 80*50],
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
  
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
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
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, bg, glyph);
        }
  
        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}