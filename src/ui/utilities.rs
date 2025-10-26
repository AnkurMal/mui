use lazy_static::lazy_static;
use macroquad::prelude::*;

#[macro_export]
/// ```load_font! (name: variable name, path: &str)```
macro_rules! load_font {
    ($name: ident, $path: expr) => {
        lazy_static! {
            static ref FONT: Font = {
                let mut f = load_ttf_font_from_bytes(include_bytes!($path)).unwrap();
                f.set_filter(FilterMode::Nearest);
                f
            };
        }
    };
}

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
