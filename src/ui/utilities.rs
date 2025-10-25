use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "sample".to_string(),
        window_width: 800,
        window_height: 600,
        icon: None,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}


pub fn text_params(font: &'_ Font, font_size: u16, color: Color) -> TextParams<'_> {
    TextParams {
        font: Some(font),
        font_size,
        color,
        ..Default::default()
    }
}
