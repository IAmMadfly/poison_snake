use bevy::{
    prelude::*
};


trait Location {
    fn x(&self) -> f64;

    fn y(&self) -> f64;
}

#[derive(Default)]
struct Snake {
    body:       Vec<Entity>,
    last_move:  f32
}

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_cameras.system())
        .add_startup_system(load_snake.system());
    
    app.add_system(snek_movement_system.system());

    app.run();
}

fn load_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_snake(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    
    let body = vec![
        commands.spawn_bundle(SpriteBundle {
            material:   materials.add(Color::rgba(0.45, 0.75, 0.45, 1.0).into()),
            transform:  Transform::from_xyz(10.0, 0.0, 0.0),
            sprite:     Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .id(),
        commands.spawn_bundle(SpriteBundle {
            material:   materials.add(Color::rgba(0.4, 0.6, 0.4, 1.0).into()),
            transform:  Transform::from_xyz(10.0, 0.0, 0.0),
            sprite:     Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .id(),
        commands.spawn_bundle(SpriteBundle {
            material:   materials.add(Color::rgba(0.4, 0.6, 0.4, 1.0).into()),
            transform:  Transform::from_xyz(20.0, 0.0, 0.0),
            sprite:     Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .id()
    ];

    let snek = Snake {
        body,
        last_move: 0.0
    };

    commands.spawn()
        .insert(snek);

    
}

fn snek_movement_system(
    _key_input:         Res<Input<KeyCode>>,
    time:               Res<Time>,
    mut snake_query:    Query<&mut Snake>,
    mut body_query:     Query<&mut Transform>
) {
    if let Ok(mut snake) = snake_query.single_mut() {
        // transform.translation.x += 1.0;

        snake.last_move += time.delta_seconds();
        
        if snake.last_move >= 1.0 {
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
                tail_trans.translation.x += 10.0;
            }
        }
    }
}
