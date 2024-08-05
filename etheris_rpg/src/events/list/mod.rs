pub mod common;
pub mod prelude;

use super::Event;
use etheris_database::character_model::CharacterModel;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub struct EventBuildState {
    pub character: CharacterModel,
}

impl EventBuildState {
    pub fn new(character: CharacterModel) -> Self {
        Self { character }
    }
}

pub type EventBuilder = fn(EventBuildState) -> Event;

pub mod basic;
pub mod bosses;
pub mod general_special;
pub mod shredder_basic;

// Lore events
pub mod corlyn;
pub mod vinizi;
pub static ALL_EVENTS: Lazy<Vec<EventBuilder>> = Lazy::new(|| {
    [
        // Bosses
        bosses::garhyan::garhyan_shredder_first_invitation,
        bosses::garhyan::garhyan_rematch,
        bosses::agorath::agorath_first_encounter,
        bosses::agorath::agorath_rematch,
        bosses::orsinium::orsinium_first_encounter,
        bosses::orsinium::orsinium_rematch,
        // General
        basic::general::basic_general_rock_mining,
        basic::general::basic_general_big_rock_mining,
        basic::general::basic_general_place_to_meditate,
        basic::general::basic_general_mysterious_chest,
        basic::general::basic_general_traveller_riddle,
        basic::general::basic_general_gambler_encounter,
        basic::general::basic_general_lost_pet,
        basic::general::basic_general_broken_cart,
        basic::general::basic_general_item_hole,
        basic::general::basic_general_old_well,
        general_special::special_track_miniorbs,
        general_special::creative_general_mysterious_portal,
        general_special::special_ether_fountain,
        general_special::special_adventurer_soul,
        general_special::special_general_blind_seer,
        general_special::special_general_mysterious_merchant,
        // Plains
        basic::plains::basic_plains_exploration,
        basic::plains::basic_plains_digging,
        basic::plains::basic_plains_person_wanting_materials,
        basic::plains::basic_plains_begger,
        basic::plains::basic_plains_water_well,
        basic::plains::basic_plains_lost_traveler,
        basic::plains::basic_plains_person_in_danger,
        basic::plains::basic_plains_trapped,
        basic::plains::basic_plains_old_man_help,
        basic::plains::basic_plains_abandoned_campsite,
        basic::plains::basic_plains_fork_in_the_road,
        basic::plains::basic_plains_wounded_traveler,
        basic::plains::basic_plains_abandoned_picnic,
        basic::plains::basic_plains_fallen_tree,
        basic::plains::basic_plains_small_stream,
        basic::plains::basic_plains_training_person,
        basic::plains::basic_plains_fruit_tree,
        // Forest
        basic::forest::basic_forest_exploration,
        basic::forest::basic_forest_digging,
        basic::forest::basic_forest_dangerous_button,
        basic::forest::basic_forest_feet_stuck_in_vines,
        basic::forest::basic_forest_knowledge_books_pedestal,
        basic::forest::basic_forest_strange_shrine,
        basic::forest::basic_forest_animal_tracks,
        basic::forest::basic_forest_suspicious_tree,
        basic::forest::basic_swamp_murky_waters,
        basic::forest::basic_swamp_quicksand,
        basic::forest::basic_forest_ancient_tree_library,
        basic::forest::basic_forest_abandoned_picnic,
        basic::forest::basic_forest_apple_tree,
        basic::forest::basic_forest_fallen_tree,
        basic::forest::basic_forest_unusual_rock,
        basic::forest::basic_forest_house,
        basic::forest::basic_gloomwood_mysterious_fog,
        basic::forest::basic_mudland_trapped,
        // Desert
        basic::desert::basic_desert_exploration,
        basic::desert::basic_desert_digging,
        basic::desert::basic_desert_beginner_nomad_merchant,
        basic::desert::basic_desert_lost_traveler,
        basic::desert::basic_desert_oasis,
        basic::desert::basic_desert_sandstorm,
        basic::desert::basic_desert_abandoned_campsite,
        basic::desert::basic_desert_ancient_ruins,
        basic::desert::basic_desert_mirage_merchant,
        basic::desert::basic_desert_scorpion_nest,
        basic::desert::basic_desert_training_person,
        // Ethereal Forest
        basic::ethereal_forest::basic_ethereal_forest_digging,
        basic::ethereal_forest::basic_ethereal_forest_whispering_trees,
        basic::ethereal_forest::basic_ethereal_forest_glowing_pond,
        basic::ethereal_forest::basic_ethereal_forest_wishing_tree,
        basic::ethereal_forest::basic_ethereal_forest_strange_tree,
        // Ice Fields
        basic::ice_fields::basic_icefields_exploration,
        basic::ice_fields::basic_icefields_frozen_lake,
        basic::ice_fields::basic_icefields_frost_wolf_pack,
        basic::ice_fields::basic_icefields_snow_storm,
        basic::ice_fields::basic_icefields_aurora_borealis,
        basic::ice_fields::basic_icefields_frozen_waterfall,
        basic::ice_fields::basic_icefields_snow_sculpture,
        basic::ice_fields::basic_icefields_person_wanting_materials,
        // Mountains
        basic::mountains::basic_mountain_exploration,
        basic::mountains::basic_mountain_abandoned_campsite,
        basic::mountains::basic_mountain_person_in_danger,
        basic::mountains::basic_montain_unstable_path,
        basic::mountains::basic_mountain_mysterious_cave,
        basic::mountains::basic_mountain_avalanche_event,
        basic::mountains::basic_mountain_climber,
        basic::mountains::basic_mountain_abandoned_cabin,
        // Specific - Shredder
        shredder_basic::basic_shredder_first_encounter,
        shredder_basic::basic_shredder_robbery,
        shredder_basic::shredder_ambush,
        shredder_basic::shredder_ambush_for_payment,
        shredder_basic::shredder_training,
        shredder_basic::shredder_recruitment,
        shredder_basic::shredder_heist,
        // Lore - Vinizi
        vinizi::vinizi_first_encounter,
        vinizi::vinizi_first_stage,
        // Lore - Corlyn
        corlyn::corlyn_first_encounter,
        corlyn::corlyn_quest_icefields,
    ]
    .to_vec()
});

#[macro_export]
macro_rules! make_event {
    ($identifier:ident, $event:expr) => {
        #[allow(unused)]
        pub fn $identifier(state: $crate::events::list::EventBuildState) -> Event {
            $event
        }
    };
}

pub use make_event;

#[test]
fn count_events() {
    use etheris_data::world::regions::WorldRegion;
    use etheris_discord::twilight_model::id::Id;
    use etheris_util::generate_random_character_appearance;
    use std::collections::HashMap;

    let mut region_counter = HashMap::<WorldRegion, usize>::new();

    for event in ALL_EVENTS.iter() {
        let event = event(EventBuildState::new(CharacterModel::new(
            Id::new(12345678),
            "Dummy".to_string(),
            vec![],
            vec![],
            generate_random_character_appearance(),
        )));
        for (region, ..) in event.spawn.weighted_regions.iter() {
            *region_counter.entry(*region).or_insert(0) += 1;
        }
    }

    let mut region_counter_sorted = region_counter.into_iter().collect::<Vec<_>>();
    region_counter_sorted.sort_by_key(|(region, _)| *region);

    for (region, count) in region_counter_sorted {
        println!("{region} has {count} events");
    }
}
