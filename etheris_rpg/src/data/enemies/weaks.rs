use etheris_data::items;
use weaklings_plus::frost_wolf;

use crate::ImmunityKind;

use super::*;
make_enemy!(
    conscious_bear,
    Enemy {
        identifier: "conscious_bear",
        name: "Urso Consciente",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Ethergrove, 6),
            (WorldRegion::Midgrass, 1),
            (WorldRegion::Sunreach, 2)
        ],
        personalities: &[
            Personality::Aggressiveness,
            Personality::Insanity,
            Personality::Arrogance
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.25)
            .with_resistance(ImmunityKind::Cut, 0.8)
            .with_resistance(ImmunityKind::Bleeding, 0.5),
        strength: 40,
        intelligence: 15,
        resistance: 400,
        vitality: 185,
        ether: 25,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Bite,
            SkillKind::Charge,
            SkillKind::ImbuedPunch,
            SkillKind::MirrorDamage,
        ],
        drop: EnemyReward {
            orbs: (20, 60),
            xp: (30, 85),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INVIGORATING_CRYSTAL,
                probability: Probability::new(1),
            },],
        },
    }
);

make_enemy!(
    serpentoid_weak,
    Enemy {
        identifier: "serpentoid_weak",
        name: "Serpentóide",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Tenypt, 8),
            (WorldRegion::Sandywater, 1),
            (WorldRegion::Sunreach, 1)
        ],
        personalities: &[Personality::Insanity, Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Bleeding, 0.5)
            .with_resistance(ImmunityKind::Cut, 0.85),
        strength: 40,
        intelligence: 10,
        resistance: 550,
        vitality: 230,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Bite,
            SkillKind::Charge,
            SkillKind::ElectricSlap,
            SkillKind::SimpleCut,
        ],
        drop: EnemyReward {
            orbs: (50, 70),
            xp: (30, 70),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::KATANA,
                probability: Probability::new(50),
            },],
        },
    }
);

make_enemy!(
    cactoid,
    Enemy {
        identifier: "cactoid",
        name: "Cactóide",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 5)],
        personalities: &[Personality::Insanity, Personality::Courage],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 29,
        intelligence: 8,
        resistance: 480,
        vitality: 150,
        ether: 40,
        weapon: Some(WeaponKind::Bat),
        allies: None,
        skills: vec![
            SkillKind::FlamingBall,
            SkillKind::FirePunch,
            SkillKind::MirrorDamage,
        ],
        drop: EnemyReward {
            orbs: (30, 60),
            xp: (30, 70),
            items: vec![],
        },
    }
);

make_enemy!(
    desert_nomad,
    Enemy {
        identifier: "desert_nomad",
        name: "Nômade do Deserto",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Tenypt, 7), (WorldRegion::Sandywater, 1)],
        personalities: &[
            Personality::Calm,
            Personality::Courage,
            Personality::Intelligence,
        ],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        strength: 8,
        intelligence: 14,
        resistance: 285,
        vitality: 92,
        ether: 30,
        weapon: Some(WeaponKind::Umbrella),
        allies: None,
        skills: vec![
            SkillKind::FlamingBall,
            SkillKind::Refresh,
            SkillKind::TornadoKick,
            SkillKind::WaterBlessing,
        ],
        drop: EnemyReward {
            orbs: (30, 60),
            xp: (50, 65),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::UMBRELLA,
                probability: Probability::new(50),
            },],
        },
    }
);

make_enemy!(
    midgrass_centaur,
    Enemy {
        identifier: "midgrass_centaur",
        name: "Centauro de Midgrass",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Midgrass, 7), (WorldRegion::Emerelis, 1)],
        personalities: &[Personality::Courage, Personality::Arrogance],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_little_resistance(ImmunityKind::Physical)
            .with_resistance(ImmunityKind::Cut, 0.7),
        strength: 44,
        intelligence: 20,
        resistance: 400,
        vitality: 160,
        ether: 40,
        weapon: Some(WeaponKind::Spear),
        allies: None,
        skills: vec![
            SkillKind::Charge,
            SkillKind::SimpleCut,
            SkillKind::TornadoKick,
            SkillKind::DefensiveJump,
            SkillKind::ImbuedPunch,
        ],
        drop: EnemyReward {
            orbs: (60, 120),
            xp: (70, 130),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::SPEAR,
                    probability: Probability::new(30),
                },
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::consumable::WATER,
                    probability: Probability::new(60),
                },
            ],
        },
    }
);

make_enemy!(
    thornbeast,
    Enemy {
        identifier: "thornbeast",
        name: "Fera de Espinhos",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Midgrass, 6), (WorldRegion::Gloomwood, 1)],
        personalities: &[Personality::Aggressiveness, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Cut, 0.8)
            .with_little_weakness(ImmunityKind::Fire),
        strength: 30,
        intelligence: 10,
        resistance: 350,
        vitality: 140,
        ether: 30,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Bite,
            SkillKind::Charge,
            SkillKind::SimpleCut,
            SkillKind::MirrorDamage,
        ],
        drop: EnemyReward {
            orbs: (50, 100),
            xp: (60, 110),
            items: vec![
                EnemyRewardItem {
                    amount: (2, 5),
                    item: items::material::STICK,
                    probability: Probability::new(80),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::KNIFE,
                    probability: Probability::new(20),
                },
            ],
        },
    }
);

make_enemy!(
    frost_shaman,
    Enemy {
        identifier: "frost_shaman",
        name: "Xamã do Gelo",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Icefields, 2), (WorldRegion::Wornpeaks, 1)],
        personalities: &[Personality::Intelligence, Personality::Calm],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Ice, 0.9)
            .with_resistance(ImmunityKind::Water, 0.7)
            .with_weakness(ImmunityKind::Fire, 0.3),
        strength: 15,
        intelligence: 40,
        resistance: 350,
        vitality: 120,
        ether: 100,
        weapon: Some(WeaponKind::Bat),
        allies: Some(vec![(Probability::new(30), Box::new(frost_wolf()))]),
        skills: vec![
            SkillKind::IcyBreath,
            SkillKind::WaterBlessing,
            SkillKind::IcyShot,
            SkillKind::Refresh,
            SkillKind::WoundHealing,
        ],
        drop: EnemyReward {
            orbs: (60, 120),
            xp: (80, 150),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::BAT,
                    probability: Probability::new(20),
                },
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::consumable::WATER,
                    probability: Probability::new(70),
                },
            ],
        },
    }
);

make_enemy!(
    rock_thrower,
    Enemy {
        identifier: "rock_thrower",
        name: "Arremessador de Pedras",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Wornpeaks, 6), (WorldRegion::Icefields, 2)],
        personalities: &[Personality::Intelligence, Personality::Calm],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Physical),
        strength: 15,
        intelligence: 12,
        resistance: 250,
        vitality: 85,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::CyclonePush,
            SkillKind::TornadoKick,
        ],
        drop: EnemyReward {
            orbs: (35, 65),
            xp: (45, 75),
            items: vec![
                EnemyRewardItem {
                    amount: (3, 6),
                    item: items::material::STONE,
                    probability: Probability::new(80),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::BAT,
                    probability: Probability::new(20),
                },
            ],
        },
    }
);

make_enemy!(
    ethereal_hunter,
    Enemy {
        identifier: "ethereal_hunter",
        name: "Caçador Etéreo",
        base_probability: Probability::ALWAYS,
        regions: &[
            (WorldRegion::Ethergrove, 1),
            (WorldRegion::Wornpeaks, 2),
            (WorldRegion::Starbreeze, 2)
        ],
        brain: BrainKind::Simple,
        boss: None,
        personalities: &[
            Personality::Calm,
            Personality::Insanity,
            Personality::Intelligence,
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Cut, 0.3)
            .with_little_weakness(ImmunityKind::Ice),
        strength: 20,
        intelligence: 60,
        resistance: 400,
        vitality: 150,
        ether: 100,
        weapon: Some(WeaponKind::Bat),
        skills: vec![
            SkillKind::MirrorDamage,
            SkillKind::YinYang,
            SkillKind::IcyBreath,
            SkillKind::IcyShot,
            SkillKind::CyclonePush,
            SkillKind::Bite,
            SkillKind::ParalyzingBet,
        ],
        allies: None,
        drop: EnemyReward {
            orbs: (40, 100),
            xp: (80, 130),
            items: vec![],
        },
    }
);

make_enemy!(
    corrupt_monk,
    Enemy {
        identifier: "corrupt_monk",
        name: "Monge Corrupto",
        base_probability: Probability::ALWAYS,
        boss: None,
        brain: BrainKind::Simple,
        regions: &[
            (WorldRegion::Ethergrove, 1),
            (WorldRegion::Starbreeze, 1),
            (WorldRegion::Wornpeaks, 3)
        ],
        personalities: &[
            Personality::Calm,
            Personality::Insanity,
            Personality::Arrogance,
        ],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Poison),
        strength: 15,
        intelligence: 50,
        resistance: 540,
        vitality: 80,
        ether: 100,
        weapon: Some(WeaponKind::Bat),
        skills: vec![
            SkillKind::FlamingBall,
            SkillKind::ThermalFists,
            SkillKind::IcyBreath,
            SkillKind::IcyShot,
            SkillKind::CyclonePush,
            SkillKind::WoundHealing,
        ],
        allies: None,
        drop: EnemyReward {
            orbs: (30, 90),
            xp: (50, 150),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INTELLIGENCE_CRYSTAL,
                probability: Probability::new(1),
            }],
        },
    }
);

make_enemy!(
    etherking,
    Enemy {
        identifier: "etherking",
        name: "Etherking",
        base_probability: Probability::new(50),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Ethergrove, 1),
            (WorldRegion::Starbreeze, 1),
            (WorldRegion::Sunreach, 1),
            (WorldRegion::Wornpeaks, 2)
        ],
        personalities: &[
            Personality::Calm,
            Personality::Insanity,
            Personality::Arrogance,
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.8)
            .with_resistance(ImmunityKind::Cut, 0.6)
            .with_resistance(ImmunityKind::Poison, 1.0)
            .with_resistance(ImmunityKind::Water, 0.5)
            .with_resistance(ImmunityKind::Bleeding, 1.0)
            .with_resistance(ImmunityKind::Ice, 1.0),
        strength: 50,
        intelligence: 70,
        resistance: 900,
        vitality: 400,
        ether: 175,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::WoundHealing,
            SkillKind::YinYang,
            SkillKind::IcyBreath,
            SkillKind::IcyShot,
            SkillKind::CyclonePush,
            SkillKind::FirePunch,
            SkillKind::FlamingBall,
            SkillKind::ElectricSlap,
            SkillKind::Refresh,
            SkillKind::InstinctiveReaction,
            SkillKind::AtomicHollow,
        ],
        drop: EnemyReward {
            orbs: (150, 300),
            xp: (150, 350),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INVIGORATING_CRYSTAL,
                probability: Probability::new(1),
            }],
        },
    }
);
