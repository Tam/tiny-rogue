use bevy::prelude::*;

pub const TILE_SIZE : f32 = 16.;

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
    asset_server : Res<AssetServer>,
    mut texture_atlases : ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..default()
    });
    
    // Texture
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("tilemap.png"),
        Vec2::splat(16.),
        12, 11,
        Some(Vec2::splat(1.)), None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    for x in -18..=18 {
        for y in -10..=10 {
            let i = if x == -18 {
                if y == 10 { 4 }
                else if y == -10 { 16 }
                else { 15 }
            } else if x == 18 {
                if y == 10 { 5 }
                else if y == -10 { 17 }
                else { 13 }
            } else {
                if y == 10 { 26 }
                else if y == -10 { 2 }
                else { 0 }
            };
            
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(i),
                    transform: Transform::from_xyz(TILE_SIZE * x as f32, TILE_SIZE * y as f32, 0.),
                    ..default()
                },
            ));
        }
    }
}
