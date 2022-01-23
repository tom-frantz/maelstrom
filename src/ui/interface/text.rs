use crate::ui::interface::sidebar::SIDEBAR_WINDOW_WIDTH;
use bevy::prelude::*;

pub(super) struct FontResource {
    pub(super) primary: Handle<Font>,
}

impl FromWorld for FontResource {
    fn from_world(world: &mut World) -> Self {
        let asset = world.get_resource::<AssetServer>().unwrap();

        FontResource {
            primary: asset.load("VT323-Regular.ttf"),
        }
    }
}

pub(super) enum MaelstromFont {
    Title,
    Subtitle,
    Heading,
    Subheading,
    Paragraph,
}

impl MaelstromFont {
    fn size(&self) -> f32 {
        match self {
            MaelstromFont::Title => 40.0,
            MaelstromFont::Subtitle => 30.0,

            MaelstromFont::Heading => 34.0,
            MaelstromFont::Subheading => 24.0,

            MaelstromFont::Paragraph => 20.0,
        }
    }

    fn font(&self, fonts: &Res<FontResource>) -> Handle<Font> {
        return fonts.primary.clone();
    }

    fn colour(&self) -> Color {
        match self {
            MaelstromFont::Title | MaelstromFont::Heading => Color::hex("fff").unwrap(),
            MaelstromFont::Subtitle | MaelstromFont::Subheading => Color::hex("ccc").unwrap(),
            MaelstromFont::Paragraph => Color::hex("999").unwrap(),
        }
    }

    pub fn with_text<S>(&self, text: S, fonts: &Res<FontResource>) -> TextBundle
    where
        S: Into<String>,
    {
        TextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: self.font(fonts),
                    font_size: self.size(),
                    color: self.colour(),
                },
                Default::default(),
            ),
            style: Style {
                // flex_wrap: FlexWrap::Wrap,
                max_size: Size {
                    width: SIDEBAR_WINDOW_WIDTH,
                    height: Val::Auto,
                },
                // align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
