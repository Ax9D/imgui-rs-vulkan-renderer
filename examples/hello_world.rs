<<<<<<< HEAD
mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "hello world";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;
    System::new(APP_NAME)?.run((), |_, ui, _| {
        ui.window("Hello world")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello world!");
                ui.text("こんにちは世界！");
                ui.text("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    })?;

    Ok(())
}
=======
mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "hello world";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    System::new(APP_NAME)?.run((), move |_, ui, _| {
        Window::new("Hello world")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    })?;

    Ok(())
}
>>>>>>> 7cbc5c699c381aa738a13055f87f04985a61f3be
