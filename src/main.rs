use bevy::prelude::*;


trait Location {
    fn x(&self) -> f64;

    fn y(&self) -> f64;
}

#[derive(Default)]
struct Snake {
}

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(load_cameras.system())
        .add_startup_system(load_snake.system());

    app.run();
}

fn load_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_snake(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    
    
    commands.spawn_bundle(SpriteBundle {
            material:   materials.add(Color::rgba(0.5, 0.75, 0.5, 1.0).into()),
            transform:  Transform::from_xyz(1.0, -1.0, 0.0),
            sprite:     Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Snake::default());
}

fn hello_world() {
    println!("Hello, world!");

}
