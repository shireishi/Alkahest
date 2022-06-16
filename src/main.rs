#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{
    io, fmt, borrow::Borrow
};

mod world;
mod object;
mod entity;
mod debug;
mod game;

use world::*;
use object::*;
use entity::*;
use debug::*;
use game::*;

struct Health(u32);
struct Name(&'static str);

const STEP: f32 = 5.; // the amount the player moves on each key press

#[derive(Debug, Eq, Component, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Default, Clone, Debug, Eq, PartialEq, Component, Hash)]
struct BackgroundHandle {
    handle: Handle<Image>,
    height: u32,
    width: u32
}
impl BackgroundHandle {
    fn new(asset: Handle<Image>) -> BackgroundHandle {
        BackgroundHandle {
            handle: asset,
            ..default()
        }
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Component, Hash)]
pub struct BackgroundSize {
    height: u32,
    width: u32
}

// Game Structs //
#[derive(Default, Debug, Clone)]
pub struct Location {
    x: f32,
    y: f32,
    z: f32
}
impl Location {
    fn new() -> Location {
        Location {
            ..default()
        }
    }
}

// Master Events //
fn resize_window(
    resize_event: Res<Events<WindowResized>>
) {
    let mut reader = resize_event.get_reader();
    for edit in reader.iter(&resize_event) {
        info!("width = {} height = {}", edit.width, edit.height);
    }
}

// Game Events //
fn keyboard_input_system(
    _commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
    mut _game_state: ResMut<State<GameState>>,
    mut transforms: Query<&mut Transform>
) { 
    let mut moved: bool = false;

    if keyboard_input.pressed(KeyCode::W) {
        player.location.y += STEP;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player.location.x -= STEP;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player.location.y -= STEP;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player.location.x += STEP;
        moved = true;
    }
    
    if moved {
        *transforms.get_mut(player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(
                player.location.x,
                player.location.y,
                0f32
            ),
            ..default()
        }
    }
}

fn check_collision(
    _commands: Commands,
    mut players: Query<&mut Player>,
    _objects: Res<GameObject>,
    _entities: Res<GameEntity<Player>>
) {
    for player in players.iter_mut() {
        info!("{:?}", player.location);
    }
}

fn menu_key_check(
    _commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    if keyboard_input.pressed(KeyCode::Return) {
        game_state.set(GameState::Playing).unwrap();
    }
}

// Main Methods //
fn main() {
    let mut world: World = World::new();
    let player: usize = world.new_entity();

    world.add_comp(player, Health(100));
    world.add_comp(player, Name("Helena"));

    let inventory: Vec<Item> = Vec::with_capacity(27usize);
    let mut player: GameEntity<Player> = GameEntity::new(Some(Player::new(inventory)));

    let zip = health.iter_mut().zip(name.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    App::new() // Bevy window creation
    
    .add_startup_system(setup)
    .add_state(GameState::BackgroundInit)
    
    .insert_resource(WindowDescriptor {
        title: app_name,
        width: WIDTH,
        height: HEIGHT,
        present_mode: PresentMode::Fifo,
        ..default()
    })
        .insert_resource(game)
        .insert_resource(player)

        .add_plugins(DefaultPlugins)

        .add_system(resize_window)
        .add_system_set(
            SystemSet::on_update(GameState::BackgroundInit)
            .with_system(setup_bounds)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::BackgroundInit)
                .with_system(update_background)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(create_objects)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu_key_check)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                // .with_system(player_tracker)
                .with_system(keyboard_input_system)
                // .with_system(check_collision)
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<GameEntity<Player>>
) {
    // let (r, g, b) = from_hex("00cc00".to_string()); //debug color

    let background_texture: Handle<Image> = asset_server.load("background.png");
    let player_texture: Handle<Image> = asset_server.load("sex_idle.png");

    commands.spawn();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(BackgroundHandle::new(background_texture));
    commands.insert_resource(player.add_handle(player_texture.clone()));

    player.entity = Some(
        commands
            .spawn_bundle(TransformBundle::from(Transform {
                translation: Vec3::new(
                    player.location.x,
                    player.location.y,
                    1f32
                ),
                ..default()
            }))
            .with_children(|player| {
                    player.spawn_bundle(SpriteBundle {
                    texture: player_texture,
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                });
            })
            .id()
    );
}

fn setup_bounds(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
    mut event_asset: EventReader<AssetEvent<Image>>,
    assets: Res<Assets<Image>>,
    bg: ResMut<BackgroundHandle>
) {
    for event in event_asset.iter() {
        match event {
            AssetEvent::Created { handle } => {
                info!("Asset created: {:?}", *handle);

                if *handle == bg.handle {
                    let img = assets.get(bg.handle.clone()).unwrap();
                    let bg_size = BackgroundSize {
                        height: img.texture_descriptor.size.height,
                        width: img.texture_descriptor.size.width
                    };

                    commands.insert_resource(bg_size);
                    app_state.set(GameState::Menu).unwrap();
                }
            },
            _ => {
                info!("{:?}", event);
            }
        }
    }
}

fn update_background(
    mut commands: Commands,
    mut bg: ResMut<BackgroundHandle>,
    game: Res<Game>,
    bg_size: Res<BackgroundSize>
) {
    bg.height = bg_size.height;
    bg.width = bg_size.width;

    let scale = game.height / bg.height as f32;

    info!("Bg width = {} Bg height = {}", bg.width, bg.height);

    commands.spawn_bundle(SpriteBundle {
        texture: bg.handle.clone().into(),
        transform: Transform::from_scale(Vec3::new(scale, scale, 0.)),
        ..default()
    });
}

fn create_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _game: ResMut<Game>
) {
    // let rng = thread_rng();
    // for _ in 0 .. 10 {
    //     // future entity generation based on different features
    // }

    let position_x = 0.;
    let position_y = -(HEIGHT/2.);
    
    // ! Define the ground object //
    let _ground_object = Some(commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ground.png"),
        transform: Transform::from_xyz(position_x, position_y, 1.),
        ..default()
    })).unwrap();

    // game.add_object(ground_object, position_x, position_y);
}
