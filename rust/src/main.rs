mod input;
use anyhow::Result;

fn main() -> Result<()> {
    let input = input::get_input(2015, 1)?;
    println!("{}", input);

    Ok(())
}
