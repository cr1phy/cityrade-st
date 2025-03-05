use crate::city::City;

#[derive(Debug, Default)]
pub struct World {
    seed: u64,
    map: WorldMap,
    cities: Vec<City>,
}

impl World {
    pub fn new() -> World {
        World {
            map: WorldMap::default(),
            cities: Vec::new(),
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.cities.push(city);
    }

    pub fn get_cities(&self) -> &Vec<City> {
        &self.cities
    }
}

#[derive(Debug)]
pub struct WorldMap {
    width: u64,
    height: u64,
    terrain: Vec<u8>,
}

impl Default for WorldMap {
    fn default() -> Self {
        WorldMap {
            width: 100,
            height: 100,
            terrain: vec![],
        }
    }
}