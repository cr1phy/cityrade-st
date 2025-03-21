use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechnologyType {
    // Экономические
    Agriculture,
    Mining,
    Forestry,
    Trade,
    Banking,

    // Строительные
    BasicConstruction,
    AdvancedConstruction,
    StoneWorks,

    // Военные
    BasicMilitary,
    AdvancedMilitary,
    Fortification,

    // Социальные
    Education,
    Culture,
    Administration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Technology {
    pub tech_type: TechnologyType,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub prerequisites: Vec<TechnologyType>,
    pub unlock_effects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyTree {
    technologies: HashMap<TechnologyType, Technology>,
}

impl TechnologyTree {
    pub fn new() -> Self {
        let mut technologies = HashMap::new();

        // Базовые технологии (без предпосылок)
        technologies.insert(TechnologyType::Agriculture, Technology {
            tech_type: TechnologyType::Agriculture,
            name: "Сельское хозяйство".to_string(),
            description: "Улучшает производство пищи для населения".to_string(),
            cost: 100,
            prerequisites: Vec::new(),
            unlock_effects: vec![
                "Увеличивает производство пищи на 20%".to_string(),
                "Позволяет строить фермы".to_string(),
            ],
        });

        technologies.insert(TechnologyType::Mining, Technology {
            tech_type: TechnologyType::Mining,
            name: "Горное дело".to_string(),
            description: "Улучшает добычу камня и других минералов".to_string(),
            cost: 100,
            prerequisites: Vec::new(),
            unlock_effects: vec![
                "Увеличивает добычу камня на 20%".to_string(),
                "Позволяет строить шахты".to_string(),
            ],
        });

        technologies.insert(TechnologyType::Forestry, Technology {
            tech_type: TechnologyType::Forestry,
            name: "Лесное хозяйство".to_string(),
            description: "Улучшает заготовку древесины".to_string(),
            cost: 100,
            prerequisites: Vec::new(),
            unlock_effects: vec![
                "Увеличивает производство дерева на 20%".to_string(),
                "Позволяет строить лесопилки".to_string(),
            ],
        });

        technologies.insert(TechnologyType::BasicConstruction, Technology {
            tech_type: TechnologyType::BasicConstruction,
            name: "Основы строительства".to_string(),
            description: "Базовые принципы строительства зданий".to_string(),
            cost: 100,
            prerequisites: Vec::new(),
            unlock_effects: vec![
                "Позволяет строить базовые здания".to_string(),
                "Снижает стоимость строительства на 10%".to_string(),
            ],
        });

        // Технологии второго уровня
        technologies.insert(TechnologyType::Trade, Technology {
            tech_type: TechnologyType::Trade,
            name: "Торговля".to_string(),
            description: "Развивает торговые отношения с другими городами".to_string(),
            cost: 200,
            prerequisites: vec![TechnologyType::Agriculture],
            unlock_effects: vec![
                "Позволяет строить рынки".to_string(),
                "Позволяет устанавливать торговые маршруты".to_string(),
            ],
        });

        technologies.insert(TechnologyType::AdvancedConstruction, Technology {
            tech_type: TechnologyType::AdvancedConstruction,
            name: "Продвинутое строительство".to_string(),
            description: "Усовершенствованные методы строительства".to_string(),
            cost: 200,
            prerequisites: vec![TechnologyType::BasicConstruction, TechnologyType::Mining],
            unlock_effects: vec![
                "Позволяет строить продвинутые здания".to_string(),
                "Снижает стоимость строительства на дополнительные 15%".to_string(),
            ],
        });

        TechnologyTree { technologies }
    }

    pub fn get_all_technologies(&self) -> &HashMap<TechnologyType, Technology> {
        &self.technologies
    }

    pub fn get_technology(&self, tech_type: &TechnologyType) -> Option<&Technology> {
        self.technologies.get(tech_type)
    }
}
