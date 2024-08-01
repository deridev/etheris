#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum BossKind {
    Garhyan,
    Agorath,
    Orsinium,
    Ethria,
    MicrolordDiabolius,
    MacrolordVastorrant,
}

impl BossKind {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Garhyan => "Garhyan, o Senhor dos Ratos",
            Self::Agorath => "Duelista Agorath",
            Self::Orsinium => "Orsinium, o Colosso Forjado",
            Self::Ethria => "Evolucionária Ethria",
            Self::MicrolordDiabolius => "Microlorde Diabolius",
            Self::MacrolordVastorrant => "Macrolorde Vastorránt",
        }
    }

    pub const fn short_description(&self) -> &'static str {
        match self {
            Self::Garhyan => {
                "Um nobre caído que reina sobre um império de ratos nas profundezas de florestas."
            }
            Self::Agorath => "Guerreiro vagante invencível, em busca de uma derrota em um duelo.",
            Self::Orsinium => "Golem de batalha incansável, o último guardião de uma civilização esquecida no tempo.",
            Self::Ethria => "Outrora política influente, dedicou sua vida a evoluir a raça humana até o próximo patamar através do ether.",
            Self::MicrolordDiabolius => "Irmão covarde de um insano líder que pretende ascender a própria existência.",
            Self::MacrolordVastorrant => "Tirano louco que busca superar sua própria existência e ascender ao céu."  
        }
    }
}
