use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    name: String,
    building_type: BuildingType,
    level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingType {
    Residential,
    Industrial,
    Commercial,
    Military,
}

impl Building {
    pub fn new(name: String, building_type: BuildingType) -> Building {
        Building {
            name,
            building_type,
            level: 1,
        }
    }

    // Метод для улучшения здания
    pub fn upgrade(&mut self) {
        self.level += 1;
    }

    // Метод для получения информации о здании
    pub fn get_info(&self) -> String {
        format!("Building: {}, Type: {:?}, Level: {}", self.name, self.building_type, self.level)
    }
}