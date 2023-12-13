use std::{fs, io::Result};

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

pub fn second_p1() -> Result<()> {
    let inp = fs::read_to_string("input/second.txt")?
        .replace(' ', "")
        .lines()
        .map(|line| line);
    Ok(())
}
fn spilt_input(s: &str) {
    s.split(pat)
}
