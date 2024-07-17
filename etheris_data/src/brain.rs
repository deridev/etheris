#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BrainKind {
    #[default]
    Simple,

    /// A insane brain will always try to kill the target. Never giving up.
    Insane,

    /// A boss brain thinks more.
    Boss,
}
