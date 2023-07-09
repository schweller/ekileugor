use std::cmp::{min, max};

use bracket_lib::prelude::*;
use serde::{Serialize, Deserialize};
use specs::prelude::*;

pub const MAPWIDTH : usize = 32;
pub const MAPHEIGHT : usize = 21;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall, Floor, Exit
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct RoomRect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}
  
impl RoomRect {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> RoomRect {
        RoomRect{x1:x, y1:y, x2:x+w, y2:y+h}
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other:&RoomRect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub blocked: Vec<bool>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub rooms : Vec<RoomRect>,
    pub width: i32,
    pub height: i32,
    pub depth: i32
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room : &RoomRect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
      
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
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

    pub fn map_with_rooms_and_corridors(depth: i32) -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; MAPCOUNT],
            rooms : Vec::new(),
            width : MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles : vec![false; MAPCOUNT],
            visible_tiles : vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content : vec![Vec::new(); MAPCOUNT],
            depth
        };

        let MAX_ROOMS : i32 = 7;
        let MIN_SIZE : i32 = 3;
        let MAX_SIZE : i32 = 5;
      
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, (MAPWIDTH as i32) - w - 1) - 1;
            let y = rng.roll_dice(1, (MAPHEIGHT as i32) - h - 1) - 1;
            let new_room = RoomRect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);            
            }
        }

        let exit_position = map.rooms[map.rooms.len() -1 ].center();
        let exit_idx = map.xy_idx(exit_position.0, exit_position.1);
        map.tiles[exit_idx] = TileType::Exit;
    
        map
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

// @Deprecated
// pub fn basic_map() -> Map {
//     let mut new_map = Map{
//         tiles: vec![TileType::Floor; 80*50],
//         blocked: vec![false; 80*50],
//         rooms : Vec::new(),
//         tile_content: vec![Vec::new(); 80*50],
//         visible_tiles: vec![false; 80*50],
//         revealed_tiles: vec![false; 80*50],
//         width: 80 as i32,
//         height: 50 as i32
//     };

//     // Make the boundaries walls
//     for x in 0..80 {
//         let foo = new_map.xy_idx(x, 0);
//         let bar = new_map.xy_idx(x, 49);
//         new_map.tiles[foo] = TileType::Wall;
//         new_map.tiles[bar] = TileType::Wall;
//         new_map.blocked[foo] = true;
//         new_map.blocked[bar] = true;
//     }

//     for y in 0..50 {
//         let foo = new_map.xy_idx(0, y);
//         let bar = new_map.xy_idx(79, y);
//         new_map.tiles[foo] = TileType::Wall;
//         new_map.tiles[bar] = TileType::Wall;
//         new_map.blocked[foo] = true;
//         new_map.blocked[bar] = true;        
//     }

//     let mut rng = bracket_lib::random::RandomNumberGenerator::new();
//     for _i in 0..400 {
//         let x = rng.roll_dice(1, 79);
//         let y = rng.roll_dice(1, 49);
//         let idx = new_map.xy_idx(x, y);
//         if idx != new_map.xy_idx(40, 25) {
//             new_map.tiles[idx] = TileType::Wall;
//             new_map.blocked[idx] = true;
//         }        
//     }
//   new_map
// }

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
                    glyph = to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.2, 0.2);
                    bg = RGB::from_f32(0., 0., 0.);
                }
                TileType::Wall => {
                    glyph = wall_glyph(&*map, x, y);
                    fg = RGB::from_f32(0., 1.0, 0.);
                    bg = RGB::named(bracket_lib::color::ROYALBLUE3);
                }
                TileType::Exit => {
                    glyph = to_cp437('☼');
                    fg = RGB::from_f32(0., 1.0, 1.0);
                    bg = RGB::from_f32(0., 0., 0.);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, bg, glyph);
        }
  
        // Move the coordinates
        x += 1;
        if x > (MAPWIDTH as i32) - 1 {
            x = 0;
            y += 1;
        }
    }
}

fn wall_glyph(map : &Map, x: i32, y:i32) -> bracket_lib::terminal::FontCharType {
    if x < 1 || x > map.width-2 || y < 1 || y > map.height-2 as i32 { return 35; }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}
