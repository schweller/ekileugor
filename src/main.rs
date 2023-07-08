use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
use components::*;
mod map;
use map::*;
mod game;
use game::*;
mod gui;
mod systems;

pub struct State {
    pub ecs: World,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput, PreRun, PlayerTurn, MonsterTurn 
}

impl State {
    fn run_systems(&mut self) {
        let mut fov = systems::VisibilitySystem{};
        fov.run_now(&self.ecs);
        let mut map_index = systems::MapIndexingSystem{};
        map_index.run_now(&self.ecs);
        let mut melee_combat = systems::MeleeCombatSystem{};
        melee_combat.run_now(&self.ecs);
        let mut damage = systems::DamageSystem{};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();

        draw_map(&self.ecs, ctx);
        
        {
            let map = self.ecs.fetch::<Map>();
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            for (pos, render) in (&positions, &renderables).join() {
                let idx = map.xy_idx(pos.x, pos.y);
                if map.visible_tiles[idx] {
                    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
                }
            }
            
            gui::draw_ui(&self.ecs, ctx);
        }

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }            
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        ctx.print(1, 49, &format!("FPS: {}", ctx.fps));

        systems::delete_the_dead(&mut self.ecs);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("GMTK2023 - Ekileugor")
        .with_tile_dimensions(16, 16)
        .build()?;
    context.with_post_scanlines(true);

    let mut gamestate = State{
        ecs: World::new()
    };

    let rng = bracket_lib::random::RandomNumberGenerator::new();

    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<Player>();
    gamestate.ecs.register::<Mob>();
    gamestate.ecs.register::<Controllable>();
    gamestate.ecs.register::<Name>();
    gamestate.ecs.register::<Viewshed>();

    // Stats components
    gamestate.ecs.register::<SinglePoolStat>();
    gamestate.ecs.register::<SingleStat>();
    gamestate.ecs.register::<CombatStats>();
    gamestate.ecs.register::<PoolStats>();
    
    // Map meta components
    gamestate.ecs.register::<BlocksTile>();

    // Combat components
    gamestate.ecs.register::<MeleeIntent>();
    gamestate.ecs.register::<Damage>();

    let map = map::basic_map();

    let player_entity = gamestate
        .ecs
        .create_entity()
        .with(Player{})
        .with(Controllable{ current: true})
        .with(Position{ x: 20, y: 40})
        .with(Name{name: "Player".to_string() })
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
        .with(Renderable{
            glyph: bracket_lib::terminal::to_cp437('@'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
            render_order: 0
        })
        .with(CombatStats{
            attack: 10,
            defense: 10,
            evade: 0
        })
        .with(PoolStats{
            hp: SinglePoolStat { current: 25, max: 25 },
            xp: 0,
            level: 1,
            gold: 0
        })
        .build();

    let active_camera = ActiveEntity{
        target: player_entity
    };

    gamestate.ecs
        .create_entity()
        .with(Mob{})
        .with(Controllable{ current: false })
        .with(Position{ x: 21, y: 41})
        .with(BlocksTile{})
        .with(Name{name: "Mob 1".to_string() })
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
        .with(Renderable{
            glyph: bracket_lib::terminal::to_cp437('G'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
            render_order: 1
        })
        .with(CombatStats{
            attack: 5,
            defense: 5,
            evade: 0
        })
        .with(PoolStats{
            hp: SinglePoolStat { current: 10, max: 10 },
            xp: 0,
            level: 1,
            gold: 0
        })        
        .build();
    
    gamestate.ecs.insert(active_camera);
    gamestate.ecs.insert(map);
    gamestate.ecs.insert(rng);
    gamestate.ecs.insert(RunState::PreRun);

    main_loop(context, gamestate)
}

