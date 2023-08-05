use anyhow::{Context, Result};
use reqwest::header::COOKIE;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get_input(year: u32, day: u32) -> Result<String> {
    let file_path = format!("input/{}/day_{}", year, day);
    let path = std::path::Path::new(&file_path);

    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        println!("Downloading input for Year: {}, Day: {}", year, day);
        Ok(download(year, day, path)?)
    }
}

fn download(year: u32, day: u32, path: &Path) -> Result<String> {
    // Advent of Code parameters
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let session_id = get_session_id().context("Retrieving session cookie from input/session")?;
    let session = format!("session={}", session_id);

    let client = reqwest::blocking::Client::new();
    let res = client.get(url).header(COOKIE, session).send()?;
    let mut input = res.text()?;
    input.truncate(input.len() - 1);

    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path)?;
    file.write_all(input.as_bytes())?;

    Ok(input)
}

fn get_session_id() -> Result<String> {
    let mut id = fs::read_to_string("input/session")?;
    // Remove trailing new line
    id.truncate(id.len() - 1);

    Ok(id)
}
