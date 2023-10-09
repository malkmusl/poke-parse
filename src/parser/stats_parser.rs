use select::document::Document;
use select::predicate::Name;
use select::predicate::Predicate;

use std::sync::Mutex;
use std::collections::HashMap;

extern crate lazy_static;
use lazy_static::lazy_static;

use reqwest::Client;

use crate::r#struct::pokemon::pokemon_stats::*;

lazy_static! {
    // Use a Mutex to make it mutable
    pub static ref STATS: Mutex<HashMap<String, BaseStats>> = Mutex::new(HashMap::new());
    pub static ref EVS: Mutex<HashMap<String, EVStats>> = Mutex::new(HashMap::new());
}

pub async fn parse_stats() {
    // Make an HTTP request to the URL containing the HTML
    let url = "https://pokemondb.net/pokedex/all"; // Replace with your actual URL
    // Vector to store the entries and their corresponding numbers
    // Send the request and handle the result
    match Client::new().get(url).send().await {
        Ok(response) => {
            // Check if the response was successful
            if response.status().is_success() {
                // Try to get the response body as text
                match response.text().await {
                    Ok(body) => {
                        // Now you can use the body
                        // Parse the HTML document
                        let document = Document::from(body.as_str());
                        // Find all tables on the page
                        for table in document.find(Name("table")) {
                            // Find the <tbody> within each table
                            if let Some(tbody) = table.find(Name("tbody")).next() {
                                // Extract every <tr> element within the <tbody>
                                for row in tbody.find(Name("tr")) {

                                    let mut name = "?";
                                    let mut id = 0;
                                    let mut _total = 0;
                                    let mut hp = 0;
                                    let mut attack = 0;
                                    let mut defense = 0;
                                    let mut sp_attack = 0;
                                    let mut sp_defense = 0;
                                    let mut speed = 0;

                                    // Extract specific elements within each <tr> element

                                    let mut cell_number = 0;
                                    if let Some(img_alt) = row.find(Name("img")).next().and_then(|img| img.attr("alt")) {
                                        name = img_alt;
                                    }
                                    for cell in row.find(Name("td").and(select::predicate::Class("cell-num"))) {
                                        let parsed_number: Result<i32, _> = cell.text().parse();
                                        let mut result_u32: u32 = 0;

                                        match parsed_number {
                                            Ok(parsed_value) => {
                                                // Converting the parsed integer to u8
                                                result_u32 = parsed_value as u32;
                                            }
                                            Err(_) => {
                                                println!("Failed to parse the string as an integer");
                                            }
                                        }

                                        // Extract and print the text content of each <td> element
                                        if cell_number == 0 {
                                            id = result_u32;
                                        }
                                        if cell_number == 1 {
                                            _total = result_u32;
                                        }
                                        if cell_number == 2 {
                                            hp = result_u32;
                                        }
                                        if cell_number == 3 {
                                            attack = result_u32;
                                        }
                                        if cell_number == 4 {
                                            defense = result_u32;
                                        }
                                        if cell_number == 5 {
                                            sp_attack = result_u32;
                                        }
                                        if cell_number == 6 {
                                            sp_defense = result_u32;
                                        }
                                        if cell_number == 7 {
                                            speed = result_u32;
                                        }
                                        cell_number += 1;
                                    }
                                    
                                println!(
                                    "Name: {} ID: {}, HP: {}, Attack: {}, Defense: {}, SP-Attack: {}, SP-Defense: {}, Speed: {}",
                                    name, id, hp, attack, defense, sp_attack, sp_defense, speed
                                );

                                STATS.lock().unwrap().insert(name.to_string(), BaseStats::new(hp, attack, defense, sp_attack, sp_defense, speed));
                                }
                            }
                        }
                    }
                    Err(err) => eprintln!("Failed to get response body: {}", err),
                }
            } else {
                eprintln!("Request failed with status code: {}", response.status());
            }
        }
        Err(err) => eprintln!("Failed to send request: {}", err),
    }
}


pub async fn parse_ev() {
    // Make an HTTP request to the URL containing the HTML
    let url = "https://pokemondb.net/ev/all"; // Replace with your actual URL

    // Send the request and handle the result
    match Client::new().get(url).send().await {
        Ok(response) => {
            // Check if the response was successful
            if response.status().is_success() {
                // Try to get the response body as text
                match response.text().await {
                    Ok(body) => {
                        // Now you can use the body
                        // Parse the HTML document
                        let document = Document::from(body.as_str());

                        // Find all tables on the page
                        for table in document.find(Name("table")) {
                            // Find the <tbody> within each table
                            if let Some(tbody) = table.find(Name("tbody")).next() {
                                // Extract every <tr> element within the <tbody>
                                for row in tbody.find(Name("tr")) {
                                    // Process each cell in the row
                                    let mut cells: Vec<String> = Vec::new();

                                    for cell in row.find(Name("td")) {
                                        cells.push(cell.text());
                                    }

                                    // Use the extracted cell values as needed
                                    if cells.len() >= 8 {
                                        let id: u32 = cells[0].parse().unwrap_or_default();
                                        let name: String = cells[1].parse().unwrap_or_default();
                                        let hp: u32 = cells[2].parse().unwrap_or_default();
                                        let attack: u32 = cells[3].parse().unwrap_or_default();
                                        let defense: u32 = cells[4].parse().unwrap_or_default();
                                        let sp_attack: u32 = cells[5].parse().unwrap_or_default();
                                        let sp_defense: u32 = cells[6].parse().unwrap_or_default();
                                        let speed: u32 = cells[7].parse().unwrap_or_default();

                                        println!(
                                            "Name: {} ID: {}, HP: {}, Attack: {}, Defense: {}, SP-Attack: {}, SP-Defense: {}, Speed: {}",
                                            name, id, hp, attack, defense, sp_attack, sp_defense, speed
                                        );

                                        // You can insert into STATS.lock().unwrap() here if needed
                                        EVS.lock().unwrap().insert(name, EVStats::new(hp, attack, defense, sp_attack, sp_defense, speed));
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => eprintln!("Failed to get response body: {}", err),
                }
            } else {
                eprintln!("Request failed with status code: {}", response.status());
            }
        }
        Err(err) => eprintln!("Failed to send request: {}", err),
    }
}
