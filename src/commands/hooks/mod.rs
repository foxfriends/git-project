use std::error::Error;

pub fn hooks() -> Result<(), Box<dyn Error>> {
    println!("Hello this is hooks");
    Ok(())
}
