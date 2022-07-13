use rltk::Point;
use specs::{Join, ReadExpect, ReadStorage, System};

use crate::components::{Enemy, Viewshed, Name};

pub struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, enemy, name) = data;

        for (viewshed, enemy, name) in (&viewshed, &enemy, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("{} thinks", name.name);
            }
        }
    }
}
