mod levels;
mod misc;

use crate::levels::get_level;
use crate::misc::{get_dist, update_cursor_coords};
use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
    window::PresentMode,
};

const MAX_VEL: f32 = 55.;
const MAX_DIR_LEN: f32 = 360.;
const T: f32 = 0.06;
const PLAYER_R: f32 = 15.;

const HOLE_R: f32 = 15.;

fn main() {
    App::new()
        .add_event::<LevelUpEvent>()
        .insert_resource(StrokeCount(0))
        .insert_resource(LevelCount(0))
        .insert_resource(ShrinkTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::YELLOW_GREEN))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Golf!".into(),
                present_mode: PresentMode::AutoVsync,
                resolution: (650., 650.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_systems((
            check_for_collision_with_walls,
            input,
            update_cursor_coords,
            ball_movement,
            check_for_collision_with_hole,
            on_level_up,
            update_texts,
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

pub struct LevelUpEvent;

#[derive(Component)]
struct StrokesText;

#[derive(Resource)]
struct StrokeCount(u16);

#[derive(Component)]
struct LevelText;

#[derive(Resource)]
pub struct LevelCount(u16);

#[derive(Resource)]
pub struct ShrinkTimer(Timer);

#[derive(Component)]
pub struct GameComponent;

#[derive(Component)]
struct PowerBar;

#[derive(Component)]
pub struct Hole;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CursorComponent;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player {
    vel: f32,
    dir: Vec3,
    first_mouse_pos: Vec3,
    selected: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            vel: 0.0,
            dir: Vec3::new(0., 0., 0.),
            first_mouse_pos: Vec3::new(0., 0., 0.),
            selected: false,
        }
    }
}

fn load_new_levels(
    mut commands: Commands,
    window_q: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level_count: u16,
) {
    // let hole_audio = asset_server.load("into_hole.mp3");
    //audio.play(hole_audio);
    let window = window_q.get_single().unwrap();

    let x_max = window.width() / 2.;
    let y_max = window.height() / 2.;
    let level = get_level(level_count, x_max, y_max);
    for x in 0..level.wall_translation.len() {
        let wall_translation = level.wall_translation[x];
        let wall_size = level.wall_size[x];

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(wall_size),
                    color: Color::MAROON,
                    ..default()
                },
                transform: Transform::from_translation(wall_translation),
                ..default()
            },
            Wall {},
            GameComponent {},
        ));
    }

    let player_translation = level.player_translation;
    let hole_translation = level.hole_translation;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20., 10.)),
                color: Color::INDIGO,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., -2.)),
            ..default()
        },
        PowerBar {},
        GameComponent {},
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLAYER_R).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(player_translation),
            ..default()
        },
        Player::default(),
        GameComponent {},
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(HOLE_R).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(hole_translation),
            ..default()
        },
        Hole {},
        GameComponent {},
    ));
}

fn setup(
    mut commands: Commands,
    window_q: Query<&Window>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    level_res: Res<LevelCount>,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            ..default()
        },
        CursorComponent {},
    ));

    commands.spawn((
        TextBundle::from_section(
            "STROKES:",
            TextStyle {
                font: asset_server.load("Roboto-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position: UiRect {
                left: Val::Px(15.),
                top: Val::Px(15.),
                ..default()
            },
            ..default()
        }),
        StrokesText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "LEVEL:",
            TextStyle {
                font: asset_server.load("Roboto-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position: UiRect {
                left: Val::Px(-125.),
                top: Val::Px(45.),
                ..default()
            },
            ..default()
        }),
        LevelText,
    ));

    load_new_levels(commands, window_q, meshes, materials, level_res.0);
}

fn input(
    buttons: Res<Input<MouseButton>>,
    mut player_q: Query<
        (&mut Transform, &mut Player),
        (Without<CursorComponent>, With<Player>, Without<PowerBar>),
    >,
    cursor_q: Query<&Transform, (With<CursorComponent>, Without<Player>, Without<PowerBar>)>,
    mut bar_q: Query<
        (&mut Transform, &mut Sprite),
        (With<PowerBar>, Without<CursorComponent>, Without<Player>),
    >,
    time: Res<Time>,
    mut strokes: ResMut<StrokeCount>,
    asset_server: ResMut<AssetServer>,
    audio: Res<Audio>,
) {
    let hit_audio = asset_server.load("ball_hit.mp3");
    let c_t = cursor_q.get_single().unwrap();
    let (mut bar_t, mut bar_sprite) = bar_q.get_single_mut().unwrap();
    bar_sprite.color = Color::ORANGE;
    for (player_transform, mut player) in player_q.iter_mut() {
        if player.vel > -1. && player.vel < 1. {
            player.vel = 0.;
        }
        if buttons.pressed(MouseButton::Left) && !player.selected && player.vel == 0. {
            bar_t.translation = player_transform.translation;
            bar_t.translation.x -= 40.;
            player.selected = true;
            player.first_mouse_pos = c_t.translation;
        } else if buttons.just_released(MouseButton::Left) && player.selected {
            strokes.0 += 1;
            audio.play(hit_audio.clone());
            bar_t.translation.z = -2.;
            player.selected = false;

            let dir = player.first_mouse_pos - c_t.translation;
            let dist = get_dist(player.first_mouse_pos, c_t.translation, T) * time.delta_seconds();

            if dist > MAX_VEL {
                player.vel = MAX_VEL;
            } else {
                player.vel = dist;
            }

            player.dir = dir / MAX_DIR_LEN;
        } else if player.selected == true {
            bar_t.translation.z = 2.;
            let mut dist =
                get_dist(player.first_mouse_pos, c_t.translation, T) * time.delta_seconds();
            if dist > MAX_VEL {
                dist = MAX_VEL;
            }
            bar_t.scale.y = dist / 5.;
            if bar_t.scale.y > 10. {
                bar_sprite.color = Color::LIME_GREEN;
            } else {
                bar_sprite.color = Color::DARK_GREEN;
            }
        }
    }
}

pub fn check_for_collision_with_walls(
    time: Res<Time>,
    mut player_q: Query<(&Transform, &mut Player), With<Player>>,
    wall_q: Query<(&Transform, &Sprite), (Without<Player>, With<Wall>)>,
) {
    let (transform, mut player) = player_q.get_single_mut().unwrap();
    let assumed_player_pos =
        transform.translation + (player.vel * player.dir) * time.delta_seconds(); // position of player in the next frame with respect to the current position
    let player_size = Vec2::new(PLAYER_R, PLAYER_R);
    for (wall_transform, wall_sprite) in wall_q.iter() {
        let wall_shape = wall_sprite.custom_size.unwrap();

        let collision = collide(
            wall_transform.translation,
            wall_shape,
            assumed_player_pos,
            player_size,
        );
        if collision.is_some() {
            match collision.unwrap() {
                Collision::Right => {
                    player.dir.x = -player.dir.x;
                }
                Collision::Left => {
                    player.dir.x = -player.dir.x;
                }
                Collision::Top => {
                    player.dir.y = -player.dir.y;
                }
                Collision::Bottom => {
                    player.dir.y = -player.dir.y;
                }
                _ => {
                    player.dir.x = -player.dir.x;
                    player.dir.y = -player.dir.y;
                }
            }
        }
    }
}

pub fn ball_movement(mut player_q: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    for (mut transform, mut player) in player_q.iter_mut() {
        transform.translation += (player.vel * player.dir) * time.delta_seconds() * 20.;

        if player.vel > 0. {
            player.vel -= 1.;
        } else if player.vel < 0. {
            player.vel += 1.;
        }
    }
}

fn on_level_up(
    mut commands: Commands,
    window_q: Query<&Window>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    lvl_up_ev: EventReader<LevelUpEvent>,
    lvl_res: Res<LevelCount>,
    all_entities: Query<Entity, With<GameComponent>>,
) {
    if !lvl_up_ev.is_empty() {
        for e in all_entities.iter() {
            commands.entity(e).despawn();
        }
        load_new_levels(commands, window_q, meshes, materials, lvl_res.0);
    }
}

pub fn check_for_collision_with_hole(
    hole_query: Query<&Transform, (Without<Player>, With<Hole>)>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
    mut shrink_timer: ResMut<ShrinkTimer>,
    mut lvl_ctr: ResMut<LevelCount>,
    mut lvl_up_ev: EventWriter<LevelUpEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for hole_transform in hole_query.iter() {
        for (mut player_transform, mut player) in player_query.iter_mut() {
            let player_size = Vec2::new(PLAYER_R, PLAYER_R);
            let hole_size = Vec2::new(HOLE_R, HOLE_R);
            if collide(
                player_transform.translation,
                player_size,
                hole_transform.translation,
                hole_size,
            )
            .is_some()
            {
                player.vel = 0.;
                let timer = &mut shrink_timer.0;

                player_transform.translation = hole_transform.translation;
                player_transform.translation.z = 3.;

                if timer.tick(time.delta()).finished()
                    && player_transform.scale.x > 0.
                    && player_transform.scale.y > 0.
                {
                    player_transform.scale -= Vec3::new(0.1, 0.1, 0.);
                }
            }

            if player_transform.scale.x == 0.9 {
                audio.play(asset_server.load("into_hole.mp3"));
            }

            if (player_transform.scale.x <= 0.1) && (player_transform.scale.y <= 0.1) {
                player_transform.translation.x = -10000000000000.;
                player_transform.scale.x = 1.;
                lvl_ctr.0 += 1;
                lvl_up_ev.send(LevelUpEvent);
                return;
            }
        }
    }
}

fn update_texts(
    mut stroke_text_q: Query<&mut Text, (With<StrokesText>, Without<LevelText>)>,
    strokes_res: Res<StrokeCount>,

    mut level_text_q: Query<&mut Text, (With<LevelText>, Without<StrokesText>)>,
    level_res: Res<LevelCount>,
) {
    if let Ok(mut text) = stroke_text_q.get_single_mut() {
        text.sections[0].value = format!("STROKES: {}", strokes_res.0);
    }

    if let Ok(mut text) = level_text_q.get_single_mut() {
        text.sections[0].value = format!("LEVEL: {}", level_res.0);
    }
}
