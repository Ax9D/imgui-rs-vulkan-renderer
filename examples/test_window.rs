mod common;

use common::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "test window";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().without_timestamps().init()?;
    System::new(APP_NAME)?.run((), |run, ui, _| ui.show_demo_window(run))?;

    Ok(())
}
