use std::fs::File;
use std::io::{self, Write};

fn create_pokemon_file() -> io::Result<()> {
    let file_path = "pokemon_data.rs";
    let mut file = File::create(file_path)?;

    // Write the beginning of the file
    writeln!(file, "//! Auto-generated Pokemon Data\n")?;
    writeln!(file, "extern crate lazy_static;")?;
    writeln!(file, "use lazy_static::lazy_static;")?;
    writeln!(file, "use crate::game::pokemon::pokemon::Pokemon;")?;
    writeln!(file, "use crate::game::pokemon::pokemon_attacks::*;\n")?;
    writeln!(file, "lazy_static! {{")?;

    // Write entries for each Pokemon
    for pokemon_entry in get_pokemon_entries() {
        writeln!(file, get_pokemon_entrie())?;
    }

    // Write the end of the file
    writeln!(file, "}}")?;

    Ok(())
}

// Function to get Pokemon entries as a Vec of tuples
fn get_pokemon_entrie() -> String {
    // Example entry for Bulbasaur
    let default_entry = (
        "pub static ref {}: Pokemon = Pokemon::new(
            {}, 
            {}, 
            \"{}\", \"{}\",
            \"{}\", \"{}\",
            \"{}\",
            \"{}\",
            \"{}\", \"{}\", \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            \"{}\", 
            \"{}\",
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            {}, 
            \"{}\", 
            vec!(),
            vec!(), 
            vec!(),
            vec!(), 
            vec!(), 
            \"{}\", 
            \"{}\", 
            \"{}\", 
            \"{}\"
        );"
    );
    default_entry
}

fn main() {
    match create_pokemon_file() {
        Ok(()) => println!("File created successfully!"),
        Err(err) => eprintln!("Error creating file: {}", err),
    }
}
