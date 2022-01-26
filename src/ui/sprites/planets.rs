use bevy::prelude::*;

pub fn planet_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("planets/fire.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 60, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(2560.0 / 2.0 - 300.0, -1440.0 / 2.0 + 300.0, 0.0),
                scale: Vec3::splat(8.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(1.0 / 15.0, true));
}

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
