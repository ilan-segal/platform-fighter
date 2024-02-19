use bevy::{prelude::*, sprite::Anchor};

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
        // If denominator is greater than 0, we're moving away from the collider
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

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    gravity: Gravity,
}

#[derive(Component)]
struct Terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (apply_velocity, apply_gravity, snap_transform_to_position).chain(),
        )
        .run();
}

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
            gravity: Gravity(100.0),
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

fn apply_velocity(
    mut query: Query<(&mut Position, &Velocity)>,
    colliders: Query<&Collider>,
    time: Res<Time>,
) {
    for (mut p, v) in &mut query {
        let displacement = v.0 * time.delta_seconds();
        displace(&mut p, &displacement, colliders.iter());
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

fn apply_gravity(mut query: Query<(&mut Velocity, &Gravity)>, time: Res<Time>) {
    for (mut v, g) in &mut query {
        v.0.y -= g.0 * time.delta_seconds();
    }
}
