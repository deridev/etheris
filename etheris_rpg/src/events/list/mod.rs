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
pub mod plains_basic;
pub static ALL_EVENTS: Lazy<Vec<EventBuilder>> = Lazy::new(|| {
    [
        general_basic::basic_rock_mining,
        plains_basic::basic_plains_exploration,
        plains_basic::basic_plains_digging,
        plains_basic::basic_plains_water_well,
        forest_basic::basic_forest_exploration,
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
