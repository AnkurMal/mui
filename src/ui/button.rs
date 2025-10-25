use macroquad::prelude::*;

use crate::ui::{text_params, widgets::Widget};

/// ```button_new! (label: String/&str, font: &Font, scale = 1 (u16), color = DARKGRAY (Color))```
#[macro_export]
macro_rules! button_new {
    ($label: expr, $font: expr $(, scale = $scale: expr)? $(, color = $color: expr)?) => {{
        let scale = [$($scale,)? 1][0];
        let color = [$($color,)? DARKGRAY][0];

        Button::new($label.into(), $font, scale, color)
    }};
}

pub struct Button<'a> {
    prop: Vec2,
    pos: Vec2,
    scale: u16,
    offset_y: f32,
    label: String,
    font: &'a Font,
    color: Color,
}

impl<'a> Button<'a> {
    const PADDING: f32 = 7.;
    const FONT_SIZE: u16 = 28;
    const THICKNESS: f32 = 3.;

    pub fn new(label: String, font: &'a Font, scale: u16, color: Color) -> Self {
        let text_prop = measure_text(&label, Some(font), Button::FONT_SIZE, 1.);
        let prop = vec2(
            text_prop.width + Button::PADDING * 2. * scale as f32,
            text_prop.height + Button::PADDING * 2. * scale as f32,
        );

        Button {
            prop,
            pos: Vec2::default(),
            scale,
            offset_y: text_prop.offset_y,
            label,
            font,
            color,
        }
    }

    pub fn released(&self) -> bool {
        let rec = Rect::new(self.pos.x, self.pos.y, self.prop.x, self.prop.y);
        rec.contains(mouse_position().into()) && is_mouse_button_released(MouseButton::Left)
    }

    pub fn set_text(&mut self, text: String) {
        let text_prop = measure_text(&text, Some(self.font), Button::FONT_SIZE, 1.);

        self.prop = vec2(
            text_prop.width + Button::PADDING * 2.,
            text_prop.height + Button::PADDING * 2.,
        );
        self.offset_y = text_prop.offset_y;
        self.label = text;
    }
}

impl Widget for Button<'_> {
    fn draw(&self) {
        let mut color = self.color;

        let rec = Rect::new(self.pos.x, self.pos.y, self.prop.x, self.prop.y);
        if rec.contains(mouse_position().into()) {
            color.a = 0.75;
            if is_mouse_button_down(MouseButton::Left) {
                color.a = 0.5;
            }
        }

        draw_rectangle(rec.x, rec.y, rec.w, rec.h, color);
        draw_rectangle_lines(rec.x, rec.y, rec.w, rec.h, Self::THICKNESS, BLACK);

        draw_text_ex(
            &self.label,
            self.pos.x + Self::PADDING * self.scale as f32,
            self.pos.y + (self.offset_y + Self::PADDING) * self.scale as f32,
            text_params(self.font, Self::FONT_SIZE * self.scale, BLACK),
        );
    }

    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    fn get_size(&self) -> Vec2 {
        self.prop
    }
}
