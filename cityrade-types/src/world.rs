use std::collections::HashMap;

#[derive(Debug)]
pub struct WorldMap {
    width: u64,
    height: u64,
    terrain: HashMap<(i32, i32), TerrainTile>, // Маппинг (x, y) на плитку
}

#[derive(Debug)]
pub enum TerrainTile {
    Land,
    Water,
    Building,  // Местоположение зданий
    Unknown,   // Неизвестная область
}

impl WorldMap {
    pub fn new(width: u64, height: u64) -> WorldMap {
        let mut terrain = HashMap::new();
        
        // Заполнение карты начальными значениями (например, водой или землёй)
        for x in 0..width {
            for y in 0..height {
                terrain.insert((x as i32, y as i32), TerrainTile::Land); // Заполняем землю
            }
        }
        
        WorldMap {
            width,
            height,
            terrain,
        }
    }
    
    // Функция для расширения карты
    pub fn expand_map(&mut self, new_width: u64, new_height: u64) {
        // Обновляем карту, добавляя новые участки
        for x in self.width..new_width {
            for y in self.height..new_height {
                self.terrain.insert((x as i32, y as i32), TerrainTile::Unknown); // Новая территория
            }
        }
        
        self.width = new_width;
        self.height = new_height;
    }
    
    // Функция для добавления зданий
    pub fn add_building(&mut self, x: i32, y: i32) {
        if let Some(tile) = self.terrain.get_mut(&(x, y)) {
            *tile = TerrainTile::Building; // Местоположение здания
        }
    }

    // Пример получения состояния плитки
    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TerrainTile> {
        self.terrain.get(&(x, y))
    }
}
