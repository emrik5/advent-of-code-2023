use std::{
    fs::{self, read_to_string},
    io::{Result, Write},
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

pub fn first_p2() -> Result<u32> {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter();
    let inp: Vec<_> = read_to_string("input/first.txt")?
        .lines()
        .map(|line| {
            let mut temp = String::from(line);
            nums.clone().enumerate().for_each(|(i, num)| {
                temp = temp.replacen(num, &i.to_string(), 1).to_owned();
            });
            nums.clone().enumerate().for_each(|(i, num)| {
                temp = temp
                    .chars()
                    .rev()
                    .collect::<String>()
                    .replacen(&num.chars().rev().collect::<String>(), &i.to_string(), 1)
                    .chars()
                    .rev()
                    .collect::<String>()
            });
            temp.chars()
                .filter(|&char| char.is_ascii_digit())
                .collect::<String>()
        })
        .collect();
    let inp: Vec<_> = inp
        .into_iter()
        .map(|line| {
            let len = line.len();
            let mut line = line.chars();
            print!("{:?}", line);
            let ret = if len == 1 {
                String::from_iter([line.next().unwrap(); 2])
            } else {
                String::from_iter([line.next().unwrap(), line.last().unwrap()])
            }
            .parse::<u32>()
            .unwrap();
            println!("{:?}", ret);
            ret
        })
        .collect();
    println!("AAAAAAAAAAAAaaa");
    println!("e : {:?}", inp);
    println!("sum : {:?}", inp.iter().sum::<u32>());
    // .sum();

    Ok(1)
}
