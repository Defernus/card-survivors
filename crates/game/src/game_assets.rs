use macroquad::prelude::*;

pub struct GameAssets {
    pub font_regular: Font,
    pub font_bold: Font,
    pub font_italic: Font,
    pub font_bold_italic: Font,
}

impl GameAssets {
    pub async fn load() -> Self {
        let font_regular =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/Cousine-Bold.ttf"))
                .expect("Failed to load regular font");
        let font_bold = load_ttf_font_from_bytes(include_bytes!("assets/fonts/Cousine-Bold.ttf"))
            .expect("Failed to load bold font");
        let font_italic =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/Cousine-BoldItalic.ttf"))
                .expect("Failed to load italic font");
        let font_bold_italic =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/Cousine-BoldItalic.ttf"))
                .expect("Failed to load bold italic font");

        Self {
            font_regular,
            font_bold,
            font_italic,
            font_bold_italic,
        }
    }
}
