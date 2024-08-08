use etheris_data::PactKind;
use once_cell::sync::Lazy;

pub mod prelude;

macro_rules! use_pact {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

use_pact!(solidity);
use_pact!(alchemist);
use_pact!(hercules);
use_pact!(athena);
use_pact!(metis);
use_pact!(consistency);
use_pact!(phoenix);
use_pact!(hunter);
use_pact!(fallen_angel);
use_pact!(coward);
use_pact!(ares);
use_pact!(insatiable);
use_pact!(rock);
use_pact!(courage);
use_pact!(thoth);
use_pact!(unshakable);
use_pact!(apollo);

pub static ALL_PACTS: Lazy<Vec<Box<dyn super::Pact + Send + Sync>>> = Lazy::new(|| {
    PactKind::list()
        .into_iter()
        .map(get_boxed_pact_from_kind)
        .collect()
});

pub fn get_boxed_pact_from_kind(kind: PactKind) -> Box<dyn super::Pact + Send + Sync> {
    // PS: for this function to work properly, PactKind::list() must be CORRECT.

    match kind {
        PactKind::Solidity => Box::<SolidityPact>::default(),
        PactKind::Alchemist => Box::<AlchemistPact>::default(),
        PactKind::Hercules => Box::<HerculesPact>::default(),
        PactKind::Athena => Box::<AthenaPact>::default(),
        PactKind::Metis => Box::<MetisPact>::default(),
        PactKind::Consistency => Box::<ConsistencyPact>::default(),
        PactKind::Phoenix => Box::<PhoenixPact>::default(),
        PactKind::Hunter => Box::<HunterPact>::default(),
        PactKind::FallenAngel => Box::<FallenAngelPact>::default(),
        PactKind::Coward => Box::<CowardPact>::default(),
        PactKind::Ares => Box::<AresPact>::default(),
        PactKind::Insatiable => Box::<InsatiablePact>::default(),
        PactKind::Rock => Box::<RockPact>::default(),
        PactKind::Courage => Box::<CouragePact>::default(),
        PactKind::Thoth => Box::<ThothPact>::default(),
        PactKind::Unshakable => Box::<UnshakablePact>::default(),
        PactKind::Apollo => Box::<ApolloPact>::default(),
    }
}
