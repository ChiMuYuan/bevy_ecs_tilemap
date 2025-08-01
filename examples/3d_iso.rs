use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let map_handle = helpers::tiled::TiledMapHandle(asset_server.load("iso_map.tmx"));

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        render_settings: TilemapRenderSettings {
            // Enable y-sort
            y_sort: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("3D Isometric Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((TilemapPlugin, helpers::tiled::TiledMapPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, helpers::camera::movement)
        .run();
}
