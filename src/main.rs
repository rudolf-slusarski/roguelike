mod components;
mod enemy_ai_system;
mod map;
mod player;
mod rect;
mod visibility_system;

use rltk::{GameState, Point, RandomNumberGenerator, Rltk, RGB};
use specs::{Builder, Join, RunNow, World, WorldExt};

use crate::{
    components::{Enemy, Name, Player, Position, Renderable, Viewshed},
    enemy_ai_system::EnemyAI,
    map::{draw_map, Map},
    player::player_input,
    visibility_system::VisibilitySystem,
};
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = EnemyAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused
        } else {
            self.runstate = player_input(self, ctx)
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.get_index(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("gra w komputer")
        .build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Enemy>();
    gs.ecs.register::<Name>();

    let map: Map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => {
                glyph = rltk::to_cp437('w');
                name = "weak enemy".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('s');
                name = "strong enemy".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Enemy {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    // resources
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    rltk::main_loop(context, gs)
}
