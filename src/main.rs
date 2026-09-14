mod models;

use models::{Character, Class, Race};
use std::fs;

fn load_races(path: &str) -> Result<Vec<Race>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let races: Vec<Race> = serde_json::from_str(&contents)?;

    Ok(races)
}

fn load_classes(path: &str) -> Result<Vec<Class>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let classes: Vec<Class> = serde_json::from_str(&contents)?;

    Ok(classes)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===============================");
    println!("   RPG Character Designer POC");
    println!("===============================\n");

    // Load RPG data from JSON files.
    let races = load_races("data/races.json")?;
    let classes = load_classes("data/classes.json")?;

    println!("Available Races:");
    for race in &races {
        println!("  - {}", race.name);
    }

    println!("\nAvailable Classes:");
    for class in &classes {
        println!("  - {}", class.name);
    }

    // For the first POC, we'll use a predefined character.
    let human = races
        .iter()
        .find(|race| race.name == "Human")
        .expect("Human race not found")
        .clone();

    let mage = classes
        .iter()
        .find(|class| class.name == "Mage")
        .expect("Mage class not found")
        .clone();

    let character = Character {
        name: String::from("Aria"),
        level: 5,
        race: human,
        class: mage,
    };

    let final_stats = character.calculate_stats();

    println!("\n-------------------------------");
    println!("Character");
    println!("-------------------------------");
    println!("Name:  {}", character.name);
    println!("Race:  {}", character.race.name);
    println!("Class: {}", character.class.name);
    println!("Level: {}", character.level);

    println!("\nFinal Stats");
    println!("-------------------------------");
    println!("HP:  {}", final_stats.hp);
    println!("MP:  {}", final_stats.mp);
    println!("ATK: {}", final_stats.atk);
    println!("DEF: {}", final_stats.def);
    println!("SPC: {}", final_stats.spc);
    println!("SPD: {}", final_stats.spd);

    Ok(())
}