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

pub mod forest_basic;
pub mod general_basic;
pub mod general_special;
pub mod plains_basic;
pub mod shredder_basic;

// Lore events
pub mod vinizi;
pub static ALL_EVENTS: Lazy<Vec<EventBuilder>> = Lazy::new(|| {
    [
        // General
        general_special::special_track_miniorbs,
        general_basic::basic_rock_mining,
        general_basic::general_basic_place_to_meditate,
        // Plains
        plains_basic::basic_plains_exploration,
        plains_basic::basic_plains_digging,
        plains_basic::basic_plains_person_wanting_materials,
        plains_basic::basic_plains_begger,
        plains_basic::basic_plains_water_well,
        plains_basic::basic_plains_person_in_danger,
        plains_basic::basic_plains_trapped,
        plains_basic::basic_plains_old_man_help,
        plains_basic::basic_plains_abandoned_campsite,
        plains_basic::basic_plains_fork_in_the_road,
        plains_basic::basic_plains_wounded_traveler,
        // Forest
        forest_basic::basic_forest_exploration,
        forest_basic::basic_forest_digging,
        forest_basic::basic_forest_dangerous_button,
        forest_basic::basic_forest_feet_stuck_in_vines,
        forest_basic::basic_forest_knowledge_books_pedestal,
        forest_basic::basic_forest_strange_shrine,
        forest_basic::basic_forest_animal_tracks,
        forest_basic::basic_forest_suspicious_tree,
        // Specific - Shredder
        shredder_basic::basic_shredder_first_encounter,
        shredder_basic::basic_shredder_robbery,
        shredder_basic::shredder_ambush,
        shredder_basic::shredder_ambush_for_payment,
        shredder_basic::shredder_training,
        // Lore - Vinizi
        vinizi::vinizi_first_encounter,
        vinizi::vinizi_first_stage,
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
