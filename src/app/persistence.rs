use std::{fs::File, io::Write};

use serde_json::to_string_pretty;

use super::Clock;

pub fn save(app: &Clock) -> std::io::Result<()> {
    let mut file = File::create("data.json")?;
    let data = to_string_pretty(app).expect("Failed to serialize data");
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn restore() -> std::io::Result<Clock> {
    let file = File::open("data.json")?;
    let app = serde_json::from_reader(file)?;
    Ok(app)
}
