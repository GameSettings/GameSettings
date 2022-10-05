use std::fs;

fn make_dir() -> std::io::Result<()> {
    fs::create_dir_all("C:/Game Settings")?;
    Ok(())
}