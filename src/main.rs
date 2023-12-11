use std::error::Error;

use first::first_p2;

pub mod first;
pub mod second;
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", first_p2()?);
    Ok(())
}
