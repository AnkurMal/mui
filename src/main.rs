mod ui;

use anyhow::Result;
use lazy_static::lazy_static;
use macroquad::prelude::*;
use ui::*;

load_font!(FONT, "../assets/fonts/Alegreya-Regular.ttf");

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let mut count = 0;

    let btn = button_new!("Count++", FONT, color = BLUE);
    let btn2 = button_new!("Count--", FONT, color = BLUE);

    let label = label_new!(format!("Count is: {count}"), FONT);

    let mut layout = Layout::new(Orientation::Vertical);
    layout.add_widget(&btn);
    layout.add_widget(&btn2);
    layout.add_widget(&label);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if btn.released() {
            count += 1;
            label.set_text(format!("Count is: {count}"));
        } else if btn2.released() {
            count -= 1;
            label.set_text(format!("Count is: {count}"));
        }

        clear_background(RED);
        layout.draw();

        next_frame().await;
    }

    Ok(())
}
