use anyhow::Result;
use bevy::prelude::*;
use point_cloud_bevy::*;

mod common;

#[test]
fn spawns_entity_with_name() -> Result<()> {
    let mut app = common::bevy_test_app();
    app.add_plugin(PointCloudBevyPlugin);

    app.update();

    let e = app
        .world
        .query_filtered::<Entity, With<PointCloudBevyComponent>>()
        .iter(&app.world)
        .next()
        .unwrap();

    assert_eq!(app.world.query::<&Name>().get(&app.world, e)?.as_str(), "PointCloudBevyPlugin Root");

    Ok(())
}
