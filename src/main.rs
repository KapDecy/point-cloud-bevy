#![doc = include_str!("../README.md")]
use bevy::prelude::*;
use ply::PlyPlugin;
use point_cloud_bevy::*;

mod debug;
mod ply;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!(
                    "{} - v{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ),
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugins(debug::DebugPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PointCloudBevyPlugin)
        .add_plugin(PlyPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_resizer::Plugin);

    app.run();
}

fn spawn_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.color = Color::WHITE;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -20., 30.0)
            .looking_at(Vec3::new(-3.6, 3.11, 5.6), Vec3::Y),
        ..default()
    });
}
