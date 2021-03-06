use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".into(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(
            snake_movement_input
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead {
            direction: Direction::Up,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = Direction::from_keyboard_or(keyboard_input, head.direction);
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn snake_movement(mut head_positions: Query<(&mut Position, &SnakeHead)>) {
    for (mut pos, head) in head_positions.iter_mut() {
        match &head.direction {
            Direction::Left => {
                pos.x -= 1;
                if pos.x < 0 {
                    pos.x += ARENA_WIDTH as i32;
                }
            }
            Direction::Right => {
                pos.x += 1;
                if pos.x >= ARENA_WIDTH as i32 {
                    pos.x -= ARENA_WIDTH as i32;
                }
            }
            Direction::Down => {
                pos.y -= 1;
                if pos.y < 0 {
                    pos.y += ARENA_HEIGHT as i32;
                }
            }
            Direction::Up => {
                pos.y += 1;
                if pos.y >= ARENA_HEIGHT as i32 {
                    pos.y -= ARENA_HEIGHT as i32;
                }
            }
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: i32, window: f32, game: u32) -> f32 {
        let tile_size = window / game as f32;
        pos as f32 / game as f32 * window - (window / 2.0) + (tile_size / 2.0)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        let x = convert(pos.x, window.width(), ARENA_WIDTH);
        let y = convert(pos.y, window.height(), ARENA_HEIGHT);
        transform.translation = Vec3::new(x, y, 0.0)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.6));
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    fn from_keyboard_or(keyboard: Res<Input<KeyCode>>, default: Self) -> Self {
        if keyboard.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            default
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum SnakeMovement {
    Input,
    Movement,
}
