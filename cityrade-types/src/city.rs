use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    name: String,
    population: u32,
}

impl City {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_population(&self) -> u32 {
        self.population
    }

    fn set_population(&mut self, population: u32) {
        self.population = population;
    }

    fn new(name: String, population: u32) -> City {
        City {
            name,
            population,
        }
    }
}