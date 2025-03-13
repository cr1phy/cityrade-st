use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::{building::Building, resources::{ResourceType, Resources}};

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    name: String,
    population: u32,
    buildings: Vec<Building>,
    resources: Resources,
    created_at: DateTime<Utc>,
}

impl City {
    /// Создает новый город с указанными параметрами.
    pub fn new(name: String, population: u32, resources: Resources) -> City {
        City {
            name,
            population,
            resources,
            buildings: vec![],
            created_at: Utc::now(),
        }
    }
    
    /// Возвращает название города.
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Возвращает текущее население города.
    pub fn get_population(&self) -> u32 {
        self.population
    }
    
    /// Возвращает дату создания города.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    /// Возвращает ссылку на ресурсы города.
    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
    
    /// Добавляет ресурсы в город.
    pub fn add_resources(&mut self, resource: &ResourceType, amount: u32) {
        self.resources.add_resources(resource, amount);
    }
    
    /// Списывает ресурсы города, возвращает true, если операция успешна.
    pub fn subtract_resources(&mut self, resource: &ResourceType, amount: u32) -> bool {
        self.resources.subtract_resources(resource, amount)
    }
    
    /// Увеличивает население города.
    pub fn increase_population(&mut self, amount: u32) {
        self.population += amount;
    }
    
    /// Уменьшает население города.
    pub fn decrease_population(&mut self, amount: u32) {
        if self.population >= amount {
            self.population -= amount;
        }
    }

    /// Добавляет здание в город.
    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    /// Возвращает список зданий города.
    pub fn get_buildings(&self) -> &Vec<Building> {
        &self.buildings
    }
}
