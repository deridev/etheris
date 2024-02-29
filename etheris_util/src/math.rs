use etheris_data::items::ConsumptionProperties;

fn item_scale_factor_by_pl(consumption_properties: ConsumptionProperties, pl: i64) -> f32 {
    let pl = 1 + (pl / 25);
    (consumption_properties.scale_factor as f32 / 100.0) * pl as f32
}

pub fn calculate_health_regeneration(
    consumption_properties: ConsumptionProperties,
    amount: i32,
    pl: i64,
) -> i32 {
    let health_regeneration = consumption_properties.health_regenation * amount;

    let scale_factor = item_scale_factor_by_pl(consumption_properties, pl);

    (health_regeneration as f32 * scale_factor) as i32
}

pub fn calculate_ether_regeneration(
    consumption_properties: ConsumptionProperties,
    amount: i32,
    pl: i64,
) -> i32 {
    let ether_regeneration = consumption_properties.ether_regeneration * amount;

    let scale_factor = item_scale_factor_by_pl(consumption_properties, pl);

    (ether_regeneration as f32 * scale_factor) as i32
}
