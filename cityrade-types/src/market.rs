use crate::city::City;
use crate::resources::{ResourceType, Resources};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketItem {
    pub resource: ResourceType,
    pub quantity: u32,
    pub base_price: u32,
    pub current_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub items: HashMap<ResourceType, MarketItem>,
    pub demand_factors: HashMap<ResourceType, f32>,
    pub supply_factors: HashMap<ResourceType, f32>,
}

impl Market {
    pub fn new() -> Market {
        let mut items = HashMap::new();
        let mut demand_factors = HashMap::new();
        let mut supply_factors = HashMap::new();

        for resource in [ResourceType::Gold, ResourceType::Wood, ResourceType::Stone].iter() {
            let base_price = match *resource {
                ResourceType::Gold => 100,
                ResourceType::Wood => 20,
                ResourceType::Stone => 40,
                ResourceType::Food => todo!(),
                ResourceType::Iron => todo!(),
                ResourceType::Crystal => todo!(),
                ResourceType::Population => todo!(),
                ResourceType::Energy => todo!(),
            };

            items.insert(resource.clone(), MarketItem {
                resource: resource.clone(),
                quantity: 100,
                base_price,
                current_price: base_price,
            });

            demand_factors.insert(resource.clone(), 1.0);
            supply_factors.insert(resource.clone(), 1.0);
        }

        Market {
            items,
            demand_factors,
            supply_factors,
        }
    }

    pub fn update_prices(&mut self) {
        for (resource, item) in self.items.iter_mut() {
            let demand = self.demand_factors.get(resource).unwrap_or(&1.0);
            let supply = self.supply_factors.get(resource).unwrap_or(&1.0);

            let market_factor = demand / supply;
            item.current_price = ((item.base_price as f32) * market_factor) as u32;
        }
    }

    pub fn buy(&mut self, resource: &ResourceType, quantity: u32) -> Result<u32, String> {
        if let Some(item) = self.items.get_mut(resource) {
            if item.quantity >= quantity {
                let price = item.current_price * quantity;
                item.quantity -= quantity;

                // Обновляем факторы спроса/предложения
                if let Some(factor) = self.demand_factors.get_mut(resource) {
                    *factor += 0.01; // Увеличиваем спрос
                }

                return Ok(price);
            } else {
                return Err(format!(
                    "Недостаточно ресурса {} на рынке",
                    resource_to_string(resource)
                ));
            }
        }

        Err(format!(
            "Ресурс {} не найден на рынке",
            resource_to_string(resource)
        ))
    }

    pub fn sell(&mut self, resource: &ResourceType, quantity: u32) -> Result<u32, String> {
        if let Some(item) = self.items.get_mut(resource) {
            let revenue = item.current_price * quantity;
            item.quantity += quantity;

            // Обновляем факторы спроса/предложения
            if let Some(factor) = self.supply_factors.get_mut(resource) {
                *factor += 0.01; // Увеличиваем предложение
            }

            return Ok(revenue);
        }

        Err(format!(
            "Ресурс {} не найден на рынке",
            resource_to_string(resource)
        ))
    }
}

// Вспомогательная функция для преобразования типа ресурса в строку
fn resource_to_string(resource: &ResourceType) -> &str {
    match resource {
        ResourceType::Gold => "Золото",
        ResourceType::Wood => "Древесина",
        ResourceType::Stone => "Камень",
        ResourceType::Food => todo!(),
        ResourceType::Iron => todo!(),
        ResourceType::Crystal => todo!(),
        ResourceType::Population => todo!(),
        ResourceType::Energy => todo!(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub source_city: String,
    pub target_city: String,
    pub resource: ResourceType,
    pub quantity: u32,
    pub price_per_unit: u32,
    pub duration: u32, // в игровых тиках
}

pub struct TradeManager {
    pub routes: Vec<TradeRoute>,
    pub markets: HashMap<String, Market>, // ключ - имя города
}

impl TradeManager {
    pub fn new() -> TradeManager {
        TradeManager {
            routes: Vec::new(),
            markets: HashMap::new(),
        }
    }

    pub fn create_city_market(&mut self, city_name: &str) {
        self.markets.insert(city_name.to_string(), Market::new());
    }

    pub fn establish_trade_route(
        &mut self,
        source_city: &str,
        target_city: &str,
        resource: ResourceType,
        quantity: u32,
    ) -> Result<(), String> {
        // Проверяем, существуют ли города и их рынки
        if !self.markets.contains_key(source_city) {
            return Err(format!("Город-источник {} не имеет рынка", source_city));
        }

        if !self.markets.contains_key(target_city) {
            return Err(format!("Город-получатель {} не имеет рынка", target_city));
        }

        // Получаем цену в городе-источнике
        let source_market = self.markets.get(source_city).unwrap();
        let price = if let Some(item) = source_market.items.get(&resource) {
            item.current_price
        } else {
            return Err(format!(
                "Ресурс {} не найден на рынке города {}",
                resource_to_string(&resource),
                source_city
            ));
        };

        // Рассчитываем длительность маршрута (можно доработать на основе расстояния)
        let duration = 10; // Заглушка, можно вычислять на основе расстояния между городами

        // Создаем и добавляем торговый маршрут
        let route = TradeRoute {
            source_city: source_city.to_string(),
            target_city: target_city.to_string(),
            resource,
            quantity,
            price_per_unit: price,
            duration,
        };

        self.routes.push(route);
        Ok(())
    }

    pub fn update_trade_routes(&mut self, cities: &mut HashMap<String, City>) {
        let mut completed_routes = Vec::new();

        for (index, route) in self.routes.iter_mut().enumerate() {
            // Уменьшаем оставшееся время маршрута
            if route.duration > 0 {
                route.duration -= 1;
            }

            // Если маршрут завершен
            if route.duration == 0 {
                completed_routes.push(index);

                // Передаем ресурсы между городами
                if let (Some(source_city), Some(target_city)) = (
                    cities.get_mut(&route.source_city),
                    cities.get_mut(&route.target_city),
                ) {
                    // Списываем ресурсы из исходного города
                    if source_city.subtract_resources(&route.resource, route.quantity) {
                        // Добавляем ресурсы городу-получателю
                        target_city.add_resources(&route.resource, route.quantity);

                        // Обновляем факторы спроса и предложения на рынках
                        if let Some(source_market) = self.markets.get_mut(&route.source_city) {
                            if let Some(factor) =
                                source_market.supply_factors.get_mut(&route.resource)
                            {
                                *factor -= 0.02; // Уменьшаем предложение в исходном городе
                            }
                        }

                        if let Some(target_market) = self.markets.get_mut(&route.target_city) {
                            if let Some(factor) =
                                target_market.demand_factors.get_mut(&route.resource)
                            {
                                *factor -= 0.02; // Уменьшаем спрос в городе-получателе
                            }
                        }
                    }
                }
            }
        }

        // Удаляем завершенные маршруты (в обратном порядке, чтобы индексы не сдвигались)
        for index in completed_routes.iter().rev() {
            self.routes.remove(*index);
        }

        // Обновляем цены на всех рынках
        for market in self.markets.values_mut() {
            market.update_prices();
        }
    }
}
