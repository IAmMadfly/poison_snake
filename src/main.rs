use bevy::{
    prelude::*
};

use rand::prelude::*;

trait Location {
    fn x(&self) -> f64;

    fn y(&self) -> f64;
}

enum MoveDirection {
    Left,
    Right,
    Up,
    Down
}

#[derive(Default)]
struct Snake {
    body:           Vec<Entity>,
    head_colour:    Handle<ColorMaterial>,
    body_colour:    Handle<ColorMaterial>,
    last_move:      f32,
    dead:           bool
}

#[derive(Default)]
struct Body {}

#[derive(Default)]
struct Mouse {}

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_cameras.system())
        .add_startup_system(load_snake.system());
    
    app.add_system(snek_movement_system.system());
    app.add_system(game_input_listening_system.system());
    app.add_system(mouse_generating_system.system());
    app.add_system(snake_collision_system.system());

    app.run();
}

fn load_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_snake(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    
    let head_colour = materials.add(Color::rgba(0.45, 0.75, 0.45, 1.0).into());
    let body_colour = materials.add(Color::rgba(0.4, 0.6, 0.4, 1.0).into());

    let body = vec![
        commands.spawn_bundle(SpriteBundle {
            material:   head_colour.clone(),
            transform:  Transform::from_xyz(10.0, 0.0, 0.0),
            sprite:     Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Body{})
        .id(),
        commands.spawn_bundle(SpriteBundle {
            material:   body_colour.clone(),
            transform:  Transform::from_xyz(0.0, 10.0, 0.0),
            sprite:     Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .insert(Body{})
        .id()
    ];

    let snek = Snake {
        body,
        last_move:      0.0,
        dead:           false,
        head_colour,
        body_colour
    };

    commands.spawn()
        .insert(snek)
        .insert(MoveDirection::Right);
}

fn mouse_generating_system(
    mut commands:           Commands,
    mut materials:          ResMut<Assets<ColorMaterial>>
    // mice_query:             Query<&Mouse>
) {
    let mut rnd_gen = thread_rng();

    if rnd_gen.gen_bool(0.005) {
        commands.spawn_bundle(SpriteBundle {
            material:       materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            transform:      Transform::from_xyz(
                rnd_gen.gen_range(-5.0f32..5.0f32).round()*10.0,
                rnd_gen.gen_range(-5.0f32..5.0f32).round()*10.0,
                0.0
            ),
            sprite:         Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .insert(Mouse{});
    }
}

fn snake_collision_system(
    mut snake_query:        Query<&mut Snake>,
    trans_query:            Query<&Transform>,
    mouse_query:            Query<(Entity, &Mouse, &Transform)>,
    mut commands:           Commands,
    mut materials:          ResMut<Assets<ColorMaterial>>
) {
    if let Ok(mut snake) = snake_query.single_mut() {

        if snake.dead {
            return;
        }

        let snake_head = snake.body.first().unwrap();

        let snake_head_trans = trans_query
            .get(*snake_head)
            .expect("Failed to get snake head transform");

        for body_element in &snake.body[1..] {
            if let Ok(trans) = trans_query.get(*body_element) {
                if snake_head_trans.translation.x == trans.translation.x && 
                snake_head_trans.translation.y == trans.translation.y {
                    snake.dead = true;

                    let head_colour = materials
                        .get_mut(
                            snake.head_colour.clone()
                        ).expect("Failed to get head colour!");
                    
                    head_colour.color = Color::RED;

                    let body_colour = materials
                        .get_mut(
                            snake.body_colour.clone()
                        ).expect("Failed to get head colour!");
                    
                    body_colour.color = Color::rgba(0.6, 0.4, 0.4, 1.0);
                    return;
                }
            }
        }

        let tail_pos_result = trans_query
            .get(
                *snake.body.last().unwrap()
            );
        
        if let Ok(tail_pos) = tail_pos_result {
            for (ent, _mouse, trans) in mouse_query.iter() {
                if trans.translation.x == snake_head_trans.translation.x && 
                trans.translation.y == snake_head_trans.translation.y {
                    
                    commands.entity(ent).despawn();
                    
                    let body_colour_handle = snake.body_colour.clone();
                    snake.body.push(
                        commands.spawn_bundle(SpriteBundle {
                            material:   body_colour_handle,
                            transform:  tail_pos.clone(),
                            sprite:     Sprite::new(Vec2::new(8.0, 8.0)),
                            ..Default::default()
                        })
                        .insert(Body{})
                        .id()
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
    keycode:                Res<Input<KeyCode>>,
    mut direction_query:    Query<&mut MoveDirection>
) {
    if let Ok(mut direction) = direction_query.single_mut() {
        if keycode.pressed(KeyCode::Left) {
            *direction = MoveDirection::Left;
        } else if keycode.pressed(KeyCode::Right) {
            *direction = MoveDirection::Right;
        } else if keycode.pressed(KeyCode::Up) {
            *direction = MoveDirection::Up;
        } else if keycode.pressed(KeyCode::Down) {
            *direction = MoveDirection::Down;
        }
    }
}

fn snek_movement_system(
    time:               Res<Time>,
    mut snake_query:    Query<(&mut Snake, &MoveDirection)>,
    mut body_query:     Query<&mut Transform>
) {
    if let Ok((mut snake, direction)) = snake_query.single_mut() {
        // transform.translation.x += 1.0;

        if snake.dead {
            return;
        }

        snake.last_move += time.delta_seconds();
        
        if snake.last_move >= 0.05 {
            snake.last_move = 0.0;

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

                snake.body.insert(
                    1, 
                    new_pos
                );
            }

            if let Ok(mut tail_trans) = body_query.get_mut(snake_head) {
                match direction {
                    MoveDirection::Left => {
                        tail_trans.translation.x -= 10.0;
                    },
                    MoveDirection::Right => {
                        tail_trans.translation.x += 10.0;
                    },
                    MoveDirection::Up => {
                        tail_trans.translation.y += 10.0;
                    },
                    MoveDirection::Down => {
                        tail_trans.translation.y -= 10.0;
                    },
                }
            }
        }
    }
}
