use bevy::{
    prelude::*,
    ecs::system::QuerySet
};


trait Location {
    fn x(&self) -> f64;

    fn y(&self) -> f64;
}

#[derive(Default)]
struct Snake {
    body: Vec<Entity>
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
        .id()
    ];

    let snek = Snake {
        body
    };

    commands.spawn()
        .insert(snek);

    
}

fn snek_movement_system(
    _key_input:         Res<Input<KeyCode>>,
    // mut query_set:      QuerySet<(
    //     Query<&mut Snake>,
    //     Query<&mut Transform>
    // )>,
    mut snake_query:    Query<& Snake>,
    mut body_query:     Query<&mut Transform>
) {
    if let Ok(snake) = snake_query.single_mut() {
        // transform.translation.x += 1.0;

        let snake_head = snake.body.first();

        if let Some(head) = snake_head {
            if let Ok(mut tail_trans) = body_query.get_mut(*head) {
                tail_trans.translation.x += 0.9;
            }
        }
    }
}
