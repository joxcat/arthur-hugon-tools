use headless_chrome::protocol::page::ScreenshotFormat;
use headless_chrome::Tab;
use std::io::Write;

pub fn take_screenshot(tab: &Tab, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}.png", path);
    let mut screen = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(path.as_str()))?)
        .write_all(screen.as_mut_slice())?;
    Ok(())
}
