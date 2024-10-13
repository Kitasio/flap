use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        // Bevy built-ins
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        // User configured
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
