use std::error::Error;

use first::first_p1;

pub mod first;
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", first_p1()?);
    Ok(())
}
