use std::cell::RefCell;

use macroquad::prelude::*;

use crate::ui::{label, text_params, widgets::Widget};

pub struct Label {
    inner: RefCell<LabelInner>,
}

pub struct LabelInner {
    prop: Vec2,
    pos: Vec2,
    offset_y: f32,
    label: String,
    font: &'static Font,
    scale: u16,
    color: Color,
}

/// ```label_new! (label: String/&str, font: &Font, scale = 1 (u16), color = BLACK (Color))```
#[macro_export]
macro_rules! label_new {
    ($label: expr, $font: expr $(, scale = $scale: expr)? $(, color = $color: expr)?) => {{
        let scale = [$($scale,)? 1][0];
        let color = [$($color,)? BLACK][0];

        Label::new($label.into(), &*$font, scale, color)
    }};
}

impl Label {
    const FONT_SIZE: u16 = 28;

    pub fn new(label: String, font: &'static Font, scale: u16, color: Color) -> Self {
        let text_prop = measure_text(&label, Some(font), Label::FONT_SIZE, scale as f32);

        Self {
            inner: RefCell::new(LabelInner {
                prop: vec2(text_prop.width, text_prop.height),
                pos: Vec2::default(),
                offset_y: text_prop.offset_y,
                label,
                font,
                scale,
                color,
            }),
        }
    }

    pub fn set_text(&self, text: String) {
        let mut label = self.inner.borrow_mut();
        let text_prop = measure_text(
            &text,
            Some(label.font),
            Label::FONT_SIZE,
            label.scale as f32,
        );

        label.prop = vec2(text_prop.width, text_prop.height);
        label.offset_y = text_prop.offset_y;
        label.label = text;
    }
}

impl Widget for Label {
    fn draw(&self) {
        let label = self.inner.borrow();
        draw_text_ex(
            &label.label,
            label.pos.x,
            label.pos.y + label.offset_y * label.scale as f32,
            text_params(label.font, Self::FONT_SIZE * label.scale, label.color),
        );
    }

    fn set_pos(&self, new_pos: Vec2) {
        self.inner.borrow_mut().pos = new_pos;
    }

    fn get_size(&self) -> Vec2 {
        self.inner.borrow().prop
    }
}
