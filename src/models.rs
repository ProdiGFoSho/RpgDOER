use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub mp: i32,
    pub atk: i32,
    pub def: i32,
    pub spc: i32,
    pub spd: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub name: String,
    pub stats: Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassModifiers {
    pub hp: i32,
    pub mp: i32,
    pub atk: i32,
    pub def: i32,
    pub spc: i32,
    pub spd: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub modifiers: ClassModifiers,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub race: Race,
    pub class: Class,
}

impl Character {
    pub fn calculate_stats(&self) -> Stats {
        Stats {
            hp: self.race.stats.hp + self.class.modifiers.hp,
            mp: self.race.stats.mp + self.class.modifiers.mp,
            atk: self.race.stats.atk + self.class.modifiers.atk,
            def: self.race.stats.def + self.class.modifiers.def,
            spc: self.race.stats.spc + self.class.modifiers.spc,
            spd: self.race.stats.spd + self.class.modifiers.spd,
        }
    }
}