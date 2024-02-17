use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Gravity(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_velocity, apply_gravity).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprite/character/robot/walk0.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player,
        Velocity(Vec2::new(0.0, 0.0)),
        Gravity(100.0),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut d, v) in &mut query {
        d.translation.x += v.0.x * time.delta_seconds();
        d.translation.y += v.0.y * time.delta_seconds();
    }
}

fn apply_gravity(mut query: Query<(&mut Velocity, &Gravity)>, time: Res<Time>) {
    for (mut v, g) in &mut query {
        v.0.y -= g.0 * time.delta_seconds();
    }
}
