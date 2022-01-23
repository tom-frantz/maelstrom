use crate::entities::Name;
use crate::ui::interface::resource::UiResource;
use crate::ui::interface::sidebar::SidebarQuery;
use crate::ui::interface::text::{FontResource, MaelstromFont};
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub(crate) type SpaceshipQuery<'w, 's, 't0> = Query<'w, 's, &'t0 Name>;

pub(super) fn render_spaceship_info(
    // spaceship: Entity,
    spaceship: (&Name),
    sidebar_commands: &mut EntityCommands,
    fonts: Res<FontResource>,
) {
    // let sidebar_node = q_sidebar.iter_mut().next().unwrap();
    // let mut commands = commands.entity(sidebar);
    // let spaceship = q_spaceship.get(spaceship).unwrap();

    sidebar_commands.despawn_descendants();
    sidebar_commands.with_children(|sidebar| {
        sidebar.spawn_bundle(MaelstromFont::Title.with_text(spaceship.0.clone(), &fonts));
        sidebar.spawn_bundle({
            let mut text = MaelstromFont::Subtitle.with_text("Amariex Class Spaceship", &fonts);
            text.style.margin = Rect {
                left: Px(0.0),
                right: Px(0.0),
                top: Px(0.0),
                bottom: Px(12.0),
            };
            text
        });

        sidebar.spawn_bundle(MaelstromFont::Heading.with_text(spaceship.0.clone(), &fonts));
        sidebar
            .spawn_bundle(MaelstromFont::Subheading.with_text("Amariex Class Spaceship", &fonts));
        sidebar.spawn_bundle(MaelstromFont::Paragraph.with_text("This is a testing paragraph. It's a bit longer to highlight how exactly this sort of thing works.", &fonts));
    });
}
