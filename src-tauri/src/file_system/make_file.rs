use std::fs;

fn make_file() -> std::io::Result<()> {
    fs::write("foo.txt", b"Lorem ipsum")?;
    fs::write("bar.txt", "dolor sit")?;
    Ok(())
}