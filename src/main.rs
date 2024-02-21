use bevy::time::Fixed;
use bevy::time::Time;
use bevy::{prelude::*, sprite::Anchor};
use std::time::Duration;

const UPDATES_PER_SECOND: u8 = 60;
const SECONDS_PER_UPDATE: f32 = 1.0 / (UPDATES_PER_SECOND as f32);

fn main() {
    let fixed_update_systems = (
        apply_velocity,
        apply_gravity,
        snap_transform_to_position,
        increment_frame_counter,
    );
    App::new()
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
            SECONDS_PER_UPDATE,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fixed_update_systems.chain())
        .run();
}

#[derive(Component)]
struct Collider {
    centre: Vec2,
    normal: Vec2,
    breadth: f32,
}

impl Collider {
    fn get_pushback(&self, p: &Vec2, d: &Vec2) -> Option<Vec2> {
        let denominator = self.normal.dot(*d);
        // If denominator is 0, velocity is parallel to collider
        // If denominator is positive, we're moving away from the collider
        if denominator >= 0.0 {
            return None;
        }
        let numerator = self.normal.dot(self.centre - *p);
        let t = numerator / denominator;
        if t < 0.0 || t > 1.0 {
            return None;
        }
        let b_0 = *p + t * *d;
        let distance_from_centre = (b_0 - self.centre).length();
        if distance_from_centre > self.breadth * 0.5 {
            return None;
        }
        Some((t - 1.0) * d.dot(self.normal) * self.normal)
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Gravity(f32);

#[derive(Component)]
struct FallSpeed(f32);

#[derive(Component)]
struct FrameCounter(u32);

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    gravity: Gravity,
    fall_speed: FallSpeed,
    frame_counter: FrameCounter,
}

#[derive(Component)]
struct Terrain;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprite/character/robot/walk0.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
        PlayerBundle {
            marker: Player,
            position: Position(Vec2::new(0.0, 0.0)),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            gravity: Gravity(1.0),
            fall_speed: FallSpeed(100.0),
            frame_counter: FrameCounter(0),
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 0.0),
                scale: Vec3::new(400.0, 1.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Terrain,
        Collider {
            centre: Vec2::new(0.0, -200.0),
            normal: Vec2::new(0.0, 1.0),
            breadth: 400.0,
        },
    ));
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, colliders: Query<&Collider>) {
    for (mut p, v) in &mut query {
        displace(&mut p, &v.0, colliders.iter());
    }
}

fn displace<'a>(
    position: &mut Position,
    displacement: &Vec2,
    colliders: impl Iterator<Item = &'a Collider>,
) {
    let pushback = colliders
        .into_iter()
        .filter_map(|c| c.get_pushback(&position.0, displacement))
        .next()
        .unwrap_or_default();
    position.0 += *displacement + pushback;
}

fn snap_transform_to_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut t, p) in &mut query {
        t.translation.x = p.0.x;
        t.translation.y = p.0.y;
    }
}

fn apply_gravity(mut query: Query<(&mut Velocity, &Gravity, &FallSpeed)>) {
    for (mut v, g, f) in &mut query {
        v.0.y -= g.0;
        if v.0.y < -f.0 {
            v.0.y = -f.0;
        }
    }
}

fn increment_frame_counter(mut query: Query<&mut FrameCounter>) {
    for mut c in &mut query {
        c.0 += 1;
    }
}
