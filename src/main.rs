use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprites)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("arby.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(100.0, 100.0, 1.0),
            ..Default::default()
        })
        .insert(Timer::from_seconds(1.0, true));
}

fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut texture_atlas_sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if texture_atlas_sprite.index < texture_atlas.textures.len() - 1 {
                texture_atlas_sprite.index += 1;
            } else {
                texture_atlas_sprite.index = 0;
            }
        }
    }
}
