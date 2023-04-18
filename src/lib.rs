use bevy::prelude::*;

pub struct PointCloudBevyPlugin;

impl Plugin for PointCloudBevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

#[derive(Debug, Component, Default)]
pub struct PointCloudBevyComponent;

impl PointCloudBevyPlugin {
    fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
        // use std::f32::consts::TAU;
        // commands
        //     .spawn(PointCloudBevyComponent)
        //     .insert(Name::new("PointCloudBevyPlugin Root"))
        //     .insert(SpatialBundle::default())
        //     .with_children(|commands| {
        //         commands
        //             .spawn(SceneBundle {
        //                 scene: asset_server.load("cube.glb#Scene0"),
        //                 transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
        //                     Quat::from_euler(
        //                         EulerRot::XYZ,
        //                         22.5 * TAU / 360.0,
        //                         45.0 * TAU / 360.0,
        //                         0.0,
        //                     ),
        //                 ),
        //                 ..default()
        //             })
        //             .insert(Name::new("PointCloudBevyPlugin Scene"));
        //     });
    }

    // fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<PointCloudBevyComponent>>) {
    //     use std::f32::consts::TAU;
    //     for mut transform in &mut transforms {
    //         transform.rotate_z(45.0 * TAU / 360.0 * time.delta_seconds());
    //     }
    // }
}
