use std::{
    fs::{self, read_to_string},
    io::Result,
};

pub fn first_p1() -> Result<u32> {
    let inp: Vec<_> = read_to_string("input/first.txt")?
        .lines()
        .map(|line| {
            line.chars()
                .filter(|&char| char.is_ascii_digit())
                .collect::<String>()
        })
        .collect();
    let inp: u32 = inp
        .into_iter()
        .map(|line| {
            let mut line = line.chars();
            if line.size_hint().1 == Some(1) {
                String::from_iter([line.next().unwrap(); 2])
            } else {
                String::from_iter([line.next().unwrap(), line.last().unwrap()])
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum();
    Ok(inp)
}
