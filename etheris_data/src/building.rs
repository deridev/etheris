use etheris_discord::Emoji;
use serde::{Deserialize, Serialize};

use crate::ShopItem;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum BuildingKind {
    JobAgency,
    Hospital,
    Shop,
}

impl BuildingKind {
    pub const fn display_name(&self) -> &str {
        match self {
            Self::JobAgency => "Agência de Empregos",
            Self::Hospital => "Hospital",
            Self::Shop => "Loja",
        }
    }

    pub const fn emoji(&self) -> Emoji<'static> {
        match self {
            Self::JobAgency => Emoji::from_unicode("🏤"),
            Self::Hospital => Emoji::from_unicode("🏥"),
            Self::Shop => Emoji::from_unicode("🏪"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JobAgencyData {
    pub stats_total_employments: usize,
    pub stats_total_payouts: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShopBuildingData {
    pub items: Vec<ShopItem>,
    pub stats_total_sells: i64,
    pub stats_total_profit: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum BuildingData {
    JobAgency(JobAgencyData),
    Hospital,
    Shop(ShopBuildingData),
}

impl BuildingData {
    pub const fn kind(&self) -> BuildingKind {
        match self {
            Self::JobAgency(..) => BuildingKind::JobAgency,
            Self::Hospital => BuildingKind::Hospital,
            Self::Shop(..) => BuildingKind::Shop,
        }
    }
}
