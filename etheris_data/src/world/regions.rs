use std::fmt::Display;

use etheris_macros::List;
use serde::{Deserialize, Serialize};

#[derive(
    List, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum WorldRegion {
    Greenagis,
    Emerelis,
    Gloomwood,
    Metrolis,
    Mudland,
    Murkswamp,
    Tenypt,
    Ethergrove,
    SwordTown,
    Sandywater,
    Midgrass,
    Wornpeaks,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RegionKind {
    Plains,
    Forest,
    Swamp,
    Desert,
    EtherealForest,
    City,
    Mountains,
}

impl Display for RegionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plains => f.write_str("Planície"),
            Self::Forest => f.write_str("Floresta"),
            Self::Swamp => f.write_str("Pântano"),
            Self::Desert => f.write_str("Deserto"),
            Self::EtherealForest => f.write_str("Floresta Etérea"),
            Self::City => f.write_str("Cidade"),
            Self::Mountains => f.write_str("Montanhas"),
        }
    }
}

impl RegionKind {
    pub const fn emoji_str(&self) -> &'static str {
        match self {
            Self::Plains => "🍃",
            Self::Forest => "🌳",
            Self::Swamp => "☘️",
            Self::Desert => "🏜️",
            Self::EtherealForest => "🎄",
            Self::City => "🏙️",
            Self::Mountains => "⛰️",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionNeighbors {
    pub up: Option<WorldRegion>,
    pub left: Option<WorldRegion>,
    pub right: Option<WorldRegion>,
    pub down: Option<WorldRegion>,
}

impl Display for WorldRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greenagis => f.write_str("Greenagis"),
            Self::Emerelis => f.write_str("Emerelis"),
            Self::Gloomwood => f.write_str("Gloomwood"),
            Self::Metrolis => f.write_str("Metrolis"),
            Self::Mudland => f.write_str("Mudland"),
            Self::Murkswamp => f.write_str("Murkswamp"),
            Self::Tenypt => f.write_str("Tenypt"),
            Self::Ethergrove => f.write_str("Ethergrove"),
            Self::SwordTown => f.write_str("Sword Town"),
            Self::Sandywater => f.write_str("Sandywater"),
            Self::Midgrass => f.write_str("Midgrass"),
            Self::Wornpeaks => f.write_str("Wornpeaks"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionData {
    pub kind: RegionKind,
    pub travel_price: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionCity {
    pub wage: (i64, i64),
    pub work_strength_xp_gain: (u32, u32),
    pub work_health_xp_gain: (u32, u32),
    pub work_intelligence_xp_gain: (u32, u32),
    pub work_ap_cost: u32,
}

impl WorldRegion {
    pub const fn data(&self) -> RegionData {
        match self {
            Self::Greenagis => RegionData {
                kind: RegionKind::Plains,
                travel_price: 0,
            },
            Self::Emerelis => RegionData {
                kind: RegionKind::Plains,
                travel_price: 150,
            },
            Self::Gloomwood => RegionData {
                kind: RegionKind::Forest,
                travel_price: 200,
            },
            Self::Metrolis => RegionData {
                kind: RegionKind::City,
                travel_price: 300,
            },
            Self::Mudland => RegionData {
                kind: RegionKind::Forest,
                travel_price: 400,
            },
            Self::Murkswamp => RegionData {
                kind: RegionKind::Swamp,
                travel_price: 550,
            },
            Self::Tenypt => RegionData {
                kind: RegionKind::Desert,
                travel_price: 700,
            },
            Self::Ethergrove => RegionData {
                kind: RegionKind::EtherealForest,
                travel_price: 1000,
            },
            Self::SwordTown => RegionData {
                kind: RegionKind::City,
                travel_price: 1500,
            },
            Self::Sandywater => RegionData {
                kind: RegionKind::Desert,
                travel_price: 2000,
            },
            Self::Midgrass => RegionData {
                kind: RegionKind::Plains,
                travel_price: 1250,
            },
            Self::Wornpeaks => RegionData {
                kind: RegionKind::Mountains,
                travel_price: 3000,
            },
        }
    }

    pub const fn kind(&self) -> RegionKind {
        self.data().kind
    }

    pub const fn city(&self) -> Option<RegionCity> {
        match self {
            Self::Metrolis => Some(RegionCity {
                wage: (10, 30),
                work_ap_cost: 1,
                work_health_xp_gain: (2, 6),
                work_strength_xp_gain: (3, 7),
                work_intelligence_xp_gain: (1, 4),
            }),
            Self::SwordTown => Some(RegionCity {
                wage: (25, 60),
                work_ap_cost: 2,
                work_health_xp_gain: (6, 10),
                work_strength_xp_gain: (8, 12),
                work_intelligence_xp_gain: (5, 8),
            }),
            _ => None,
        }
    }

    pub fn neighbors(&self) -> RegionNeighbors {
        use WorldRegion::*;

        match self {
            Greenagis => RegionNeighbors {
                up: Some(Mudland),
                left: Some(Murkswamp),
                right: Some(Gloomwood),
                down: Some(Emerelis),
            },

            Emerelis => RegionNeighbors {
                up: Some(Greenagis),
                left: Some(Ethergrove),
                right: Some(Metrolis),
                down: None,
            },

            Gloomwood => RegionNeighbors {
                up: Some(Tenypt),
                left: Some(Greenagis),
                right: None,
                down: Some(Metrolis),
            },

            Metrolis => RegionNeighbors {
                up: Some(Gloomwood),
                left: Some(Emerelis),
                right: None,
                down: None,
            },

            Mudland => RegionNeighbors {
                up: None,
                left: Some(SwordTown),
                right: Some(Tenypt),
                down: Some(Greenagis),
            },

            Murkswamp => RegionNeighbors {
                up: Some(SwordTown),
                left: Some(Sandywater),
                right: Some(Greenagis),
                down: Some(Ethergrove),
            },

            Tenypt => RegionNeighbors {
                up: None,
                left: Some(Mudland),
                right: None,
                down: Some(Gloomwood),
            },

            Ethergrove => RegionNeighbors {
                up: Some(Murkswamp),
                left: Some(Wornpeaks),
                right: Some(Emerelis),
                down: None,
            },

            SwordTown => RegionNeighbors {
                up: None,
                left: Some(Midgrass),
                right: Some(Mudland),
                down: Some(Murkswamp),
            },

            Sandywater => RegionNeighbors {
                up: Some(Midgrass),
                left: None,
                right: Some(Murkswamp),
                down: Some(Wornpeaks),
            },

            Midgrass => RegionNeighbors {
                up: None,
                left: None,
                right: Some(SwordTown),
                down: Some(Sandywater),
            },

            Wornpeaks => RegionNeighbors {
                up: Some(Sandywater),
                left: None,
                right: Some(Ethergrove),
                down: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_neighbors_consistency(
        region: WorldRegion,
        direction_fn: fn(&RegionNeighbors) -> Option<WorldRegion>,
        opposite_direction_fn: fn(&RegionNeighbors) -> Option<WorldRegion>,
        direction_str: &str,
        opposite_direction_str: &str,
    ) {
        if let Some(neighbor_region) = direction_fn(&region.neighbors()) {
            if let Some(neighbor_opposite_region) =
                opposite_direction_fn(&neighbor_region.neighbors())
            {
                assert_eq!(
                    region,
                    neighbor_opposite_region,
                    "Invalid neighbor relationship: {} {} is {}, but {} {} is not {}",
                    region,
                    direction_str,
                    neighbor_region,
                    neighbor_region,
                    opposite_direction_str,
                    region
                );
            }
        }
    }

    #[test]
    fn test_region_neighbors() {
        // Iterate through each WorldRegion
        for &region in WorldRegion::LIST.iter() {
            // Check left and right consistency
            check_neighbors_consistency(region, |n| n.left, |n| n.right, "left", "right");
            check_neighbors_consistency(region, |n| n.right, |n| n.left, "right", "left");

            // Check up and down consistency
            check_neighbors_consistency(region, |n| n.up, |n| n.down, "up", "down");
            check_neighbors_consistency(region, |n| n.down, |n| n.up, "down", "up");
        }
    }
}
