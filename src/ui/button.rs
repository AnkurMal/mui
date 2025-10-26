use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::ui::{text_params, widgets::Widget};

/// ```button_new! (label: String/&str, font: &Font, scale = 1 (u16), color = DARKGRAY (Color))```
#[macro_export]
macro_rules! button_new {
    ($label: expr, $font: expr $(, scale = $scale: expr)? $(, color = $color: expr)?) => {{
        let scale = [$($scale,)? 1][0];
        let color = [$($color,)? DARKGRAY][0];

        Button::new($label.into(), &*$font, scale, color)
    }};
}

pub struct Button {
    inner: RefCell<ButtonInner>,
}

pub struct ButtonInner {
    prop: Vec2,
    pos: Vec2,
    scale: u16,
    offset_y: f32,
    label: String,
    font: &'static Font,
    color: Color,
}

impl Button {
    const PADDING: f32 = 7.;
    const FONT_SIZE: u16 = 28;
    const THICKNESS: f32 = 3.;

    pub fn new(label: String, font: &'static Font, scale: u16, color: Color) -> Self {
        let text_prop = measure_text(&label, Some(font), Button::FONT_SIZE, 1.);
        let prop = vec2(
            text_prop.width + Button::PADDING * 2. * scale as f32,
            text_prop.height + Button::PADDING * 2. * scale as f32,
        );

        Self {
            inner: RefCell::new(ButtonInner {
                prop,
                pos: Vec2::default(),
                scale,
                offset_y: text_prop.offset_y,
                label,
                font,
                color,
            }),
        }
    }

    pub fn released(&self) -> bool {
        let button = self.inner.borrow();

        let rec = Rect::new(button.pos.x, button.pos.y, button.prop.x, button.prop.y);
        rec.contains(mouse_position().into()) && is_mouse_button_released(MouseButton::Left)
    }

    pub fn set_text(&mut self, text: String) {
        let mut button = self.inner.borrow_mut();
        let text_prop = measure_text(&text, Some(button.font), Button::FONT_SIZE, 1.);

        button.prop = vec2(
            text_prop.width + Button::PADDING * 2.,
            text_prop.height + Button::PADDING * 2.,
        );
        button.offset_y = text_prop.offset_y;
        button.label = text;
    }
}

impl Widget for Button {
    fn draw(&self) {
        let button = self.inner.borrow();
        let mut color = button.color;

        let rec = Rect::new(button.pos.x, button.pos.y, button.prop.x, button.prop.y);
        if rec.contains(mouse_position().into()) {
            color.a = 0.75;
            if is_mouse_button_down(MouseButton::Left) {
                color.a = 0.5;
            }
        }

        draw_rectangle(rec.x, rec.y, rec.w, rec.h, color);
        draw_rectangle_lines(rec.x, rec.y, rec.w, rec.h, Self::THICKNESS, BLACK);

        draw_text_ex(
            &button.label,
            button.pos.x + Self::PADDING * button.scale as f32,
            button.pos.y + (button.offset_y + Self::PADDING) * button.scale as f32,
            text_params(button.font, Self::FONT_SIZE * button.scale, BLACK),
        );
    }

    fn set_pos(&self, new_pos: Vec2) {
        self.inner.borrow_mut().pos = new_pos;
    }

    fn get_size(&self) -> Vec2 {
        self.inner.borrow().prop
    }
}
