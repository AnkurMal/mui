mod ui;

use anyhow::Result;
use macroquad::prelude::*;
use ui::*;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let mut count = 0;

    let mut font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Alegreya-Regular.ttf"))?;
    font.set_filter(FilterMode::Nearest);

    let mut btn = button_new!("Count++", &font, color = BLUE);
    let mut btn2 = button_new!("Count--", &font, color = BLUE);

    let mut label = label_new!(format!("Count is: {count}"), &font);

    let mut layout = Layout::new(Orientation::Vertical);
    layout.add_widget(&mut btn);
    layout.add_widget(&mut btn2);
    layout.add_widget(&mut label);

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
