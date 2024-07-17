#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum BossKind {
    Garhyan,
}

impl BossKind {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Garhyan => "Garhyan, o Senhor dos Ratos",
        }
    }

    pub const fn short_description(&self) -> &'static str {
        match self {
            Self::Garhyan => {
                "Um nobre caído que reina sobre um império de ratos nas profundezas de Murkswamp."
            }
        }
    }
}
