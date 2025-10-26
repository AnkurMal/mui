use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet},
    f32,
};

use macroquad::{math::vec2, window::screen_width};

use crate::ui::widgets::Widget;

pub enum Orientation {
    Vertical,
}

pub struct Layout<'a> {
    widgets: Vec<&'a dyn Widget>,
    orientation: Orientation,
    gap: f32,
}

impl<'a> Layout<'a> {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            widgets: Vec::new(),
            orientation,
            gap: 10.,
        }
    }

    pub fn add_widget<T: Widget>(&mut self, widget: &'a T) {
        self.widgets.push(widget);

        // if !success {
        //     panic!("Attempted to insert an already present widget.")
        // }
    }

    pub fn draw(&self) {
        let mut gap = self.gap;

        for widget in &self.widgets {
            let size = widget.get_size();
            let x = screen_width() - size.x;
            widget.set_pos(vec2(x / 2., gap));

            gap += size.y + self.gap;
            widget.draw();
        }
    }
}
