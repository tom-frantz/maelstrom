use bevy::prelude::*;

use crate::ui::interface::UiInterfacePlugin;
use crate::ui::sprites::UiSpritesPlugin;
use crate::ui::utils::interaction_generator;

pub mod interface;
pub mod sprites;

pub(super) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiSpritesPlugin)
            .add_plugin(UiInterfacePlugin)
            .add_system(interaction_generator);
    }
}

pub(crate) mod utils {
    use bevy::prelude::*;
    use bevy::render::render_resource::Texture;

    use crate::ui::sprites::spaceships::SpaceshipSprite;
    use crate::ui::sprites::MainCamera;

    #[derive(Component)]
    pub struct Clickable;

    /// Stolen from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    fn get_world_pos_from_cursor(
        // need to get window dimensions
        windows: Res<Windows>,
        // query to get camera transform
        q_camera: Query<&Transform, With<MainCamera>>,
    ) -> Option<(f32, f32)> {
        // get the primary window
        let wnd = windows.get_primary().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {
            // get the size of the window
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single();

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            // eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
            return Some((pos_wld.x, pos_wld.y));
        }
        None
    }

    fn is_interaction(
        cursor: (f32, f32),
        target_transform: &Transform,
        mut target_size: (f32, f32),
    ) -> bool {
        target_size = (target_size.0 / 2.0, target_size.1 / 2.0);
        let lower_bounds = (
            target_transform.translation.x - target_size.0,
            target_transform.translation.y - target_size.1,
        );
        let upper_bounds = (
            target_transform.translation.x + target_size.0,
            target_transform.translation.y + target_size.1,
        );

        (lower_bounds.0 <= cursor.0 && cursor.0 <= upper_bounds.0)
            && (lower_bounds.1 <= cursor.1 && cursor.1 <= upper_bounds.1)
    }

    pub(super) fn interaction_generator(
        windows: Res<Windows>,
        mouse_buttons: Res<Input<MouseButton>>,
        q_camera: Query<&Transform, With<MainCamera>>,
        q_clickable: Query<(&Transform, &SpaceshipSprite), With<Clickable>>,
    ) {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            if let Some(cursor_pos) = get_world_pos_from_cursor(windows, q_camera) {
                for (transform, texture) in q_clickable.iter() {
                    let size = texture.size();

                    if is_interaction(cursor_pos, transform, (size.0 as f32, size.1 as f32)) {
                        println!("clicked on clickable: {:?}", transform)
                    }
                }
            }
        }
    }
}
