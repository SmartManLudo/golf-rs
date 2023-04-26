use bevy::prelude::*;

#[derive(Debug)]
pub struct Level {
    pub wall_translation: Vec<Vec3>,
    pub wall_size: Vec<Vec2>,
    pub player_translation: Vec3,
    pub hole_translation: Vec3,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            wall_translation: Vec::new(),
            wall_size: Vec::new(),
            player_translation: Vec3::new(0., 0., 3.),
            hole_translation: Vec3::new(0., -100., 0.),
        }
    }
}

fn add_walls(mut l: Level, x_max: f32, y_max: f32) -> Level {
    l.wall_translation.push(Vec3::new(x_max, 0., 4.));
    l.wall_size.push(Vec2::new(30., 100000000.));

    l.wall_translation.push(Vec3::new(-x_max, 0., 4.));
    l.wall_size.push(Vec2::new(30., 100000000.));

    l.wall_translation.push(Vec3::new(0., y_max, 4.));
    l.wall_size.push(Vec2::new(100000000., 30.));

    l.wall_translation.push(Vec3::new(0., -y_max, 4.));
    l.wall_size.push(Vec2::new(100000000., 30.));
    l
}

pub fn get_level(lvl: u16, x_max: f32, y_max: f32) -> Level {
    match lvl {
        0 => {
            return add_walls(Level::default(), x_max, y_max);
        }

        1 => {
            let mut l = Level::default();
            l = add_walls(l, x_max, y_max);
            l.wall_translation.push(Vec3::new(0.0, -50.0, 0.));
            l.wall_size.push(Vec2::new(30., 30.));
            println!("{:?}", l);
            return l;
        }
        2 => {
            let mut l = Level::default();
            l = add_walls(l, x_max, y_max);
            l.player_translation.x = -x_max + 50.;
            l.hole_translation.x = x_max - 50.;

            l.wall_translation.push(Vec3::new(100., -100., 4.));
            l.wall_size.push(Vec2::new(30., (y_max * 2.) - 100.));

            l.wall_translation.push(Vec3::new(-100., 100., 4.));
            l.wall_size.push(Vec2::new(30., (y_max * 2.) + 100.));

            return l;
        }
        3 => {
            let mut l = Level::default();
            l = add_walls(l, x_max, y_max);
            l.player_translation.x = x_max - 100.;
            l.hole_translation.x = x_max - 100.;
            l.hole_translation.y -= 100.;

            l.wall_translation.push(Vec3::new(-20., -100., 4.));
            l.wall_size.push(Vec2::new(x_max * 2. - 90., 30.));

            return l;
        }
        4 => {
            let mut l = Level::default();
            l = add_walls(l, x_max, y_max);
            l.hole_translation.y = y_max - 50.;
            l.hole_translation.x = x_max - 50.;

            l.player_translation.y = -y_max + 50.;
            l.player_translation.x = -x_max + 50.;

            l.wall_translation.push(Vec3::new(0., 90., 4.));
            l.wall_size.push(Vec2::new(30., 50.));

            return l;
        }
        _ => {
            return add_walls(Level::default(), x_max, y_max);
        }
    };
}
