use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    gold: u32,
    wood: u32,
    stone: u32,
}

impl Resources {
    pub fn new(gold: u32, wood: u32, stone: u32) -> Self {
        Resources { gold, wood, stone }
    }
    
    // Метод для добавления ресурсов
    pub fn add_resources(&mut self, resource: &ResourceType, amount: u32) {
        match resource {
            ResourceType::Gold => self.gold += amount,
            ResourceType::Wood => self.wood += amount,
            ResourceType::Stone => self.stone += amount,
        }
    }
    
    // Метод для списания ресурсов
    pub fn subtract_resources(&mut self, resource: &ResourceType, amount: u32) -> bool {
        match resource {
            ResourceType::Gold if self.gold >= amount => {
                self.gold -= amount;
                true
            }
            ResourceType::Wood if self.wood >= amount => {
                self.wood -= amount;
                true
            }
            ResourceType::Stone if self.stone >= amount => {
                self.stone -= amount;
                true
            }
            _ => false, // Если недостаточно ресурса
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    Gold,
    Wood,
    Stone,
}