use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#6E3E38").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tiny Rogue".into(),
                resolution: (1280., 720.).into(),
                canvas: Some("#canvas".into()),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .run();
}

fn setup (
    mut commands : Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
