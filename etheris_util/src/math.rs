use etheris_data::items::ConsumptionProperties;

pub fn calculate_health_regeneration(
    consumption_properties: ConsumptionProperties,
    amount: i32,
    health: i32,
) -> i32 {
    let mul = (health as f32 / 400.0).max(1.0).powf(0.7);
    let regeneration = (consumption_properties.health_regenation as f32 * mul) as i32;
    regeneration * amount
}

pub fn calculate_ether_regeneration(
    consumption_properties: ConsumptionProperties,
    amount: i32,
    ether: i32,
) -> i32 {
    let mul = (ether as f32 / 30.0).max(1.0).powf(0.6);
    let regeneration = (consumption_properties.ether_regeneration as f32 * mul) as i32;
    regeneration * amount
}
