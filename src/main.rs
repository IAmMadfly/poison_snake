use std::time::Duration;

use bevy::prelude::*;

use rand::prelude::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 400.0;
const BLOCK_SIZE: u32 = 10;
const WINDOW_WIDTH_BLOCKS: i32 = WINDOW_WIDTH as i32/ BLOCK_SIZE as i32;
const WINDOW_HEIGHT_BLOCKS: i32 = WINDOW_HEIGHT as i32/ BLOCK_SIZE as i32;

#[derive(Clone, PartialEq, Eq)]
enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Default for MoveDirection {
    fn default() -> Self {
        MoveDirection::Right
    }
}

struct Position {
    x:      i32,
    y:      i32
}

struct PositionCouple {
    next_pos:   Position,
    prev_pos:   Position,
}

struct Materials {
    mouse_material: Handle<ColorMaterial>,
}

#[derive(Default)]
struct Snake {
    body: Vec<Entity>,
    current_direction: MoveDirection,
    head_colour: Handle<ColorMaterial>,
    body_colour: Handle<ColorMaterial>
}

enum LivingState {
    Dead,
    Alive,
}

#[derive(Default)]
struct SnakeBody {}

#[derive(Default)]
struct SnakeHead {}

#[derive(Default)]
struct Mouse {}

#[derive(Default)]
struct Wall {}

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title:  "Poison Snake!".to_string(),
        width:  WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });

    app.insert_resource(
            Timer::from_seconds(0.5, true)
        );

    app.add_plugins(DefaultPlugins);

    app
        .add_startup_system(load_cameras.system())
        .add_startup_system(load_snake.system())
        .add_startup_system(make_walls.system())
        .add_startup_system(make_mouse_resources.system());

    app
        .add_system(snek_movement_system.system())
        .add_system(game_input_listening_system.system())
        .add_system(mouse_generating_system.system())
        .add_system(snake_mouse_collision_system.system())
        .add_system(snake_wall_collision_system.system());

    app.run();
}

fn make_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::GRAY.into());

    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .insert(Wall::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .insert(Wall::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, 10.0)),
            ..Default::default()
        })
        .insert(Wall::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, 10.0)),
            ..Default::default()
        })
        .insert(Wall::default());
}

fn make_mouse_resources(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        mouse_material: materials.add(Color::BEIGE.into()),
    });
}

fn load_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Snake head provided by the awesome designer:
// <div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

fn load_snake(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>
) {
    let head_colour = materials.add(Color::rgba(0.45, 0.75, 0.45, 1.0).into());
    let body_colour = materials.add(Color::rgba(0.4, 0.6, 0.4, 1.0).into());

    // let head_texture = materials.add(
    //     asset_server.load("images/snake_head.png").into()
    // );

    let body = vec![
        commands
            .spawn_bundle(SpriteBundle {
                material: head_colour.clone(),
                transform: Transform::from_xyz(10.0, 0.0, 1.0),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .insert(SnakeHead {})
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                material: body_colour.clone(),
                transform: Transform::from_xyz(0.0, 10.0, 0.0),
                sprite: Sprite::new(Vec2::new(8.0, 8.0)),
                ..Default::default()
            })
            .insert(SnakeBody {})
            .id(),
    ];

    let snek = Snake {
        body,
        current_direction: MoveDirection::default(),
        head_colour,
        body_colour,
    };

    commands
        .spawn()
        .insert(snek)
        .insert(LivingState::Alive)
        .insert(MoveDirection::Right);
}

fn mouse_generating_system(
    mut commands: Commands, 
    materials: Res<Materials>,
    mouse_query: Query<&Mouse>
) {
    let mut rnd_gen = thread_rng();

    if mouse_query.iter().len() < 1 {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.mouse_material.clone(),
                transform: Transform::from_xyz(
                    (rnd_gen.gen_range(
                        (-WINDOW_WIDTH_BLOCKS/2)+1..(WINDOW_WIDTH_BLOCKS/2)
                    ) * (BLOCK_SIZE as i32)) as f32,
                    (rnd_gen.gen_range(
                        (-WINDOW_HEIGHT_BLOCKS/2)+1..(WINDOW_HEIGHT_BLOCKS/2)
                    ) * (BLOCK_SIZE as i32)) as f32,
                    0.0,
                ),
                sprite: Sprite::new(Vec2::new(8.0, 8.0)),
                ..Default::default()
            })
            .insert(Mouse {});
    }
}

fn snake_wall_collision_system(
    snake_head_query: Query<(&SnakeHead, &Transform)>,
    mut snake_state_query: Query<(&Snake, &mut LivingState)>,
) {
    if let Ok((_snake, mut living_state)) = snake_state_query.single_mut() {
        if let Ok((_snake_head, head_trans)) = snake_head_query.single() {
            if head_trans.translation.x >= WINDOW_WIDTH / 2.0
                || head_trans.translation.x <= -WINDOW_WIDTH / 2.0
                || head_trans.translation.y >= WINDOW_HEIGHT / 2.0
                || head_trans.translation.y <= -WINDOW_HEIGHT / 2.0
            {
                *living_state = LivingState::Dead;
            }
        }
    }
}

fn snake_mouse_collision_system(
    mut snake_query: Query<(&mut Snake, &mut LivingState)>,
    trans_query: Query<&Transform>,
    mouse_query: Query<(Entity, &Mouse, &Transform)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((mut snake, mut living_state)) = snake_query.single_mut() {
        if let LivingState::Dead = *living_state {
            return;
        }

        let snake_head = snake.body.first().unwrap();

        let snake_head_trans = trans_query
            .get(*snake_head)
            .expect("Failed to get snake head transform");

        for body_element in &snake.body[1..] {
            if let Ok(trans) = trans_query.get(*body_element) {
                if snake_head_trans.translation.x == trans.translation.x
                    && snake_head_trans.translation.y == trans.translation.y
                {
                    *living_state = LivingState::Dead;

                    let head_colour = materials
                        .get_mut(snake.head_colour.clone())
                        .expect("Failed to get head colour!");

                    head_colour.color = Color::RED;

                    let body_colour = materials
                        .get_mut(snake.body_colour.clone())
                        .expect("Failed to get head colour!");

                    body_colour.color = Color::rgba(0.6, 0.4, 0.4, 1.0);
                    return;
                }
            }
        }

        let tail_pos_result = trans_query.get(*snake.body.last().unwrap());

        if let Ok(tail_pos) = tail_pos_result {
            for (ent, _mouse, trans) in mouse_query.iter() {
                if trans.translation.x == snake_head_trans.translation.x
                    && trans.translation.y == snake_head_trans.translation.y
                {
                    commands.entity(ent).despawn();

                    let body_colour_handle = snake.body_colour.clone();
                    snake.body.push(
                        commands
                            .spawn_bundle(SpriteBundle {
                                material: body_colour_handle,
                                transform: tail_pos.clone(),
                                sprite: Sprite::new(Vec2::new(8.0, 8.0)),
                                ..Default::default()
                            })
                            .insert(SnakeBody {})
                            .id(),
                    );

                    break;
                }
            }
        } else {
            println!("Failed to get tail position!");
        }
    }
}

fn game_input_listening_system(
    keycode: Res<Input<KeyCode>>,
    snek_query: Query<&Snake>,
    mut direction_query: Query<&mut MoveDirection>,
) {
    if let Ok(mut direction) = direction_query.single_mut() {
        if let Ok(snek) = snek_query.single() {
            if keycode.pressed(KeyCode::Left) && snek.current_direction != MoveDirection::Right {
                *direction = MoveDirection::Left;
            } else if keycode.pressed(KeyCode::Right)
                && snek.current_direction != MoveDirection::Left
            {
                *direction = MoveDirection::Right;
            } else if keycode.pressed(KeyCode::Up) && snek.current_direction != MoveDirection::Down
            {
                *direction = MoveDirection::Up;
            } else if keycode.pressed(KeyCode::Down) && snek.current_direction != MoveDirection::Up
            {
                *direction = MoveDirection::Down;
            }
        }
    }
}

fn snek_movement_system(
    time: Res<Time>,
    mut snake_query: Query<(&mut Snake, &MoveDirection, &LivingState)>,
    mut body_query: Query<&mut Transform>,
    move_timer: Res<Timer>
) {
    if let Ok((mut snake, direction, living_state)) = snake_query.single_mut() {
        // transform.translation.x += 1.0;

        if let LivingState::Dead = living_state {
            return;
        }

        // snake.last_move += time.delta_seconds();
        println!("Time on timer: {}, Paused? {}", move_timer.percent(), move_timer.paused());

        if move_timer.finished() {
            // snake.last_move = 0.0;

            let snake_head = snake.body.first().unwrap().clone();
            let snake_tail = snake.body.last().unwrap().clone();

            if snake_head != snake_tail {
                let head_trans = body_query
                    .get_mut(snake_head)
                    .expect("Failed to get head transform")
                    .clone();

                let mut tail_trans = body_query
                    .get_mut(snake_tail)
                    .expect("Failed to get tail transform");

                tail_trans.translation.x = head_trans.translation.x;
                tail_trans.translation.y = head_trans.translation.y;

                let new_pos = snake.body.pop().unwrap().clone();

                snake.body.insert(1, new_pos);
            }

            if let Ok(mut head_trans) = body_query.get_mut(snake_head) {
                snake.current_direction = direction.clone();
                match direction {
                    MoveDirection::Left => {
                        head_trans.translation.x -= BLOCK_SIZE as f32;
                    }
                    MoveDirection::Right => {
                        head_trans.translation.x += BLOCK_SIZE as f32;
                    }
                    MoveDirection::Up => {
                        head_trans.translation.y += BLOCK_SIZE as f32;
                    }
                    MoveDirection::Down => {
                        head_trans.translation.y -= BLOCK_SIZE as f32;
                    }
                }
            }
        }
    }
}
