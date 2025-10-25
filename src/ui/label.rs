use macroquad::prelude::*;

use crate::ui::{text_params, widgets::Widget};

pub struct Label<'a> {
    prop: Vec2,
    pos: Vec2,
    offset_y: f32,
    label: String,
    font: &'a Font,
    scale: u16,
    color: Color,
}

/// ```label_new! (label: String/&str, font: &Font, scale = 1 (u16), color = BLACK (Color))```
#[macro_export]
macro_rules! label_new {
    ($label: expr, $font: expr $(, scale = $scale: expr)? $(, color = $color: expr)?) => {{
        let scale = [$($scale,)? 1][0];
        let color = [$($color,)? BLACK][0];

        Label::new($label.into(), $font, scale, color)
    }};
}

impl<'a> Label<'a> {
    const FONT_SIZE: u16 = 28;

    pub fn new(label: String, font: &'a Font, scale: u16, color: Color) -> Self {
        let text_prop = measure_text(&label, Some(font), Label::FONT_SIZE, scale as f32);

        Label {
            prop: vec2(text_prop.width, text_prop.height),
            pos: Vec2::default(),
            offset_y: text_prop.offset_y,
            label,
            font,
            scale,
            color,
        }
    }

    pub fn set_text(&mut self, text: String) {
        let text_prop = measure_text(&text, Some(self.font), Label::FONT_SIZE, self.scale as f32);

        self.prop = vec2(text_prop.width, text_prop.height);
        self.offset_y = text_prop.offset_y;
        self.label = text;
    }
}

impl Widget for Label<'_> {
    fn draw(&self) {
        draw_text_ex(
            &self.label,
            self.pos.x,
            self.pos.y + self.offset_y * self.scale as f32,
            text_params(self.font, Self::FONT_SIZE * self.scale, self.color),
        );
    }

    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    fn get_size(&self) -> Vec2 {
        self.prop
    }
}
