use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Resources {
    gold: u32,
    wood: u32,
    stone: u32,
    gold_rate: f32,
    wood_rate: f32,
    stone_rate: f32,
    last_update: DateTime<Utc>,
}

impl Resources {
    pub fn new(gold: u32, wood: u32, stone: u32) -> Self {
        Resources {
            gold,
            wood,
            stone,
            last_update: Utc::now(),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_update);
        let hours = duration.num_milliseconds() as f32 / 3_600_000.0;

        self.gold += (self.gold_rate * hours) as u32;
        self.wood += (self.wood_rate * hours) as u32;
        self.stone += (self.stone_rate * hours) as u32;

        self.last_update = now;
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
