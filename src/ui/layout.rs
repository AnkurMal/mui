use std::f32;

use macroquad::{math::vec2, window::screen_width};

use crate::ui::widgets::Widget;

pub enum Orientation {
    Vertical,
}

pub struct Layout {
    widgets: Vec<*mut dyn Widget>,
    orientation: Orientation,
    gap: f32,
}

impl Layout {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            widgets: Vec::new(),
            orientation,
            gap: 10.,
        }
    }

    pub fn add_widget<T: Widget>(&mut self, widget: &mut T) {
        self.widgets.push(widget as *mut T as *mut dyn Widget);
    }

    pub fn draw(&self) {
        let mut gap = self.gap;

        for widget in &self.widgets {
            unsafe {
                let x = screen_width() - (**widget).get_size().x;
                (**widget).set_pos(vec2(x / 2., gap));

                gap += (**widget).get_size().y + self.gap;
                (**widget).draw();
            }
        }
    }
}
