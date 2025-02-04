mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "disablement";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    let mut edit_mode = true;
    let mut safe_mode = true;

    let mut click_count = 0;

    System::new(APP_NAME)?.run((), move |_, ui, _| {
        Window::new("Disabling widgets")
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.checkbox("Edit mode", &mut edit_mode);
                ui.checkbox("Safe mode", &mut safe_mode);

                ui.separator();

                // Disable entire rest of widget unless in edit mode
                let _d = ui.begin_enabled(edit_mode);

                if ui.button("Button 1") {
                    click_count += 1;
                }
                if ui.button("Button 2") {
                    click_count += 1;
                }

                // Disable dangerous buttons when in safe mode
                ui.disabled(safe_mode, || {
                    let _red = ui.push_style_color(StyleColor::Button, [1.0, 0.0, 0.0, 1.0]);
                    if ui.button("Dangerous button!") {
                        click_count -= 1;
                    }
                });

                // Can also create a token in a specific scope
                {
                    let _danger_token = ui.begin_disabled(safe_mode);
                    if ui.button("Button 3") {
                        click_count += 1;
                    }
                    // _danger_token implicitly dropped here
                }

                // Or manually drop the token
                let danger_token2 = ui.begin_disabled(safe_mode);
                if ui.button("Button 4") {
                    click_count += 1;
                }
                danger_token2.end();

                // Note the `_d` token is dropped here automatically
            });
    })?;

    Ok(())
}
