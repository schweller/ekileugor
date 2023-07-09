use specs::prelude::*;
use bracket_lib::pathfinding::field_of_view;
use bracket_lib::geometry::Point;
use crate::components::{Position, Viewshed, ActiveEntity};
use crate::map::Map;

pub struct VisibilitySystem {}

impl<'a> System <'a> for VisibilitySystem {
  type SystemData = ( 
                        ReadExpect<'a, ActiveEntity>,
                        WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (active_entity, mut map, entities, mut viewshed, pos) = data;
        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            
                if ent.eq(&active_entity.target) {
                    for t in map.visible_tiles.iter_mut() { *t = false };
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                } 
            }
        }
    }
}



