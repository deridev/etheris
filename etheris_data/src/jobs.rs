use std::fmt::Display;

use etheris_discord::Emoji;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum JobExhaustionLevel {
    Low,
    Medium,
    High,
}

impl Display for JobExhaustionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => f.write_str("Baixo"),
            Self::Medium => f.write_str("Médio"),
            Self::High => f.write_str("Elevado"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Job {
    pub emoji: Emoji<'static>,
    pub identifier: &'static str,
    pub display_name: &'static str,
    pub wage: (i64, i64),
    pub iq_required: f32,
    pub exhaustion_level: JobExhaustionLevel,
}

pub const JANITOR: Job = Job {
    emoji: Emoji::from_unicode("🧹"),
    identifier: "janitor",
    display_name: "Faxineiro",
    exhaustion_level: JobExhaustionLevel::Medium,
    iq_required: 80.0,
    wage: (50, 100),
};

pub const DELIVERY: Job = Job {
    emoji: Emoji::from_unicode("🛵"),
    identifier: "delivery",
    display_name: "Entregador",
    exhaustion_level: JobExhaustionLevel::Low,
    iq_required: 81.0,
    wage: (90, 100),
};

pub const PACKER: Job = Job {
    emoji: Emoji::from_unicode("📦"),
    identifier: "packer",
    display_name: "Empacotador",
    exhaustion_level: JobExhaustionLevel::Medium,
    iq_required: 82.0,
    wage: (100, 150),
};

pub const DRIVER: Job = Job {
    emoji: Emoji::from_unicode("🚗"),
    identifier: "driver",
    display_name: "Motorista",
    exhaustion_level: JobExhaustionLevel::Low,
    iq_required: 84.0,
    wage: (150, 200),
};

pub const ALL_JOBS: &[Job] = &[JANITOR, DELIVERY, PACKER, DRIVER];
