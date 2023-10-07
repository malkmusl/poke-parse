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
        writeln!(file, "pub static ref {}: Pokemon = {};", pokemon_entry.0, pokemon_entry.1)?;
    }

    // Write the end of the file
    writeln!(file, "}}")?;

    Ok(())
}
/* 
// Function to get Pokemon entries as a Vec of tuples
fn get_pokemon_entries() -> Vec<(&'static str, &'static str)> {
    let mut entries = Vec::new();

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
            \"Bulbasaur\", 
            \"フシギダネ (Fushigidane)\", 
            \"Bisasam\", 
            \"Bulbizarre\", 
            \"Bulbasaur\", 
            \"Bulbasaur\", 
            \"이상해씨 (isanghaessi)\", 
            \"妙蛙种子\", 
            \"妙蛙種子\", 
            \"A strange seed was planted on its back at birth. The plant sprouts and grows with this Pokémon.\", 
            \"生まれたときから 背中に 不思議な タネが 植えられている。 この ポケモンと ともに 育つ。\", 
            \"Schon bei seiner Geburt wurde ein seltsamer Samen auf seinen Rücken gepflanzt. Die Pflanze keimt und wächst mit diesem Pokémon.\", 
            \"Une graine étrange a été plantée sur son dos à sa naissance. La plante germe et grandit avec ce Pokémon.\", 
            \"Un seme strano è stato piantato sulla sua schiena alla nascita. La pianta germoglia e cresce con questo Pokémon.\", 
            \"Desde su nacimiento, lleva en la espalda una planta extraña. La planta crece y se desarrolla con este Pokémon.\", 
            \"태어났을 때부터 등에 이상한 씨앗이 심어져 있다. 이 포켓몬과 함께 자란다。\", 
            \"从出生时起，背上就有一颗奇怪的种子。这个种子随着这只宝可梦一起生长。\", 
            \"從出生時起，背上就有一顆奇怪的種子。這個種子隨著這隻寶可夢一起生長。\", 
            \"Seed Pokémon\", 
            \"たねポケモン\", 
            \"Samen-Pokémon\", 
            \"Pokémon Graine\", 
            \"Pokémon Seme\", 
            \"Pokémon Semilla\", 
            \"씨앗 포켓몬\", 
            \"种子宝可梦\", 
            \"種子寶可夢\", 
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
            "grass", 
            "monster",
            20, 
            87.5, 
            12.5, 
            0.0, 
            45, 
            50, 
            64, 
            "MEDIUM_SLOW", 
            vec!(
                LevelUpAttack::new("growl", 1),
                LevelUpAttack::new("tackle", 1),
                LevelUpAttack::new("vine_whip", 3),
                LevelUpAttack::new("growth", 6),
                LevelUpAttack::new("leech_seed", 9),
                LevelUpAttack::new("razor_leaf", 12),
                LevelUpAttack::new("poison_powder", 15),
                LevelUpAttack::new("sleep_powder", 15),
                LevelUpAttack::new("seed_bomb", 18),
                LevelUpAttack::new("take_down", 21),
                LevelUpAttack::new("sweet_scent", 24),
                LevelUpAttack::new("synthesis", 27),
                LevelUpAttack::new("worry_seed", 30),
                LevelUpAttack::new("double_edge", 33),
                LevelUpAttack::new("solar_beam", 36),
            ),
            vec!(
                HMTMAttack::new("toxic"),
                HMTMAttack::new("bullet_seed"),
                HMTMAttack::new("work_up"),
                HMTMAttack::new("sunny_day"),
                HMTMAttack::new("light_screen"),
                HMTMAttack::new("protect"),
                HMTMAttack::new("giga_drain"),
                HMTMAttack::new("safeguard"),
                HMTMAttack::new("solar_beam"),
                HMTMAttack::new("double_team"),
                HMTMAttack::new("sludge_bomb"),
                HMTMAttack::new("facade"),
                HMTMAttack::new("rest"),
                HMTMAttack::new("attract"),
                HMTMAttack::new("energy_ball"),
                HMTMAttack::new("false_swipe"),
                HMTMAttack::new("endure"),
                HMTMAttack::new("flash"),
                HMTMAttack::new("swords_dance"),
                HMTMAttack::new("sleep_talk"),
                HMTMAttack::new("grass_knot"),
                HMTMAttack::new("swagger"),
                HMTMAttack::new("substitute"),
                HMTMAttack::new("cut"),
                HMTMAttack::new("strength"),
                HMTMAttack::new("rock_smash"),
            ), 
            vec!(
                EggAttack::new("amnesia"),
                EggAttack::new("charm"),
                EggAttack::new("curse"),
                EggAttack::new("grassy_terrain"),
                EggAttack::new("ingrain"),
                EggAttack::new("leaf_storm"),
                EggAttack::new("magical_leaf"),
                EggAttack::new("nature_power"),
                EggAttack::new("petal_dance"),
                EggAttack::new("power_whip"),
                EggAttack::new("skull_bash"),
                EggAttack::new("sludge"),
    
            ),
            vec!(), 
            vec!(), 
            "front_sprite", 
            "back_sprite", 
            "front_shiny_sprite", 
            "back_shiny_sprite"
        );"
    );
    entries.push(bulbasaur_entry);

    // Add entries for other Pokemon here...

    entries
}

fn main() {
    match create_pokemon_file() {
        Ok(()) => println!("File created successfully!"),
        Err(err) => eprintln!("Error creating file: {}", err),
    }
}
*/