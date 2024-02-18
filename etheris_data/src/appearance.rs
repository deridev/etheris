use etheris_common::{Color, Probability};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorWithProbability {
    pub color: Color,
    pub probability: Probability,
}

impl ColorWithProbability {
    pub const fn new(color: Color, probability: Probability) -> Self {
        Self { color, probability }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BodyPart {
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub probability: Probability,
}

impl BodyPart {
    pub const fn new(name: &'static str, bytes: &'static [u8], probability: Probability) -> Self {
        Self {
            name,
            bytes,
            probability,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BodyCosmetic {
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub colors: &'static [(Color, Probability)],
    pub probability: Probability,
}

impl BodyCosmetic {
    pub const fn new(
        name: &'static str,
        bytes: &'static [u8],
        colors: &'static [(Color, Probability)],
        probability: Probability,
    ) -> Self {
        Self {
            name,
            bytes,
            colors,
            probability,
        }
    }
}

pub const NECKS: [BodyPart; 1] = [BodyPart::new(
    "1",
    include_bytes!("../../assets/appearance/necks/1.png"),
    Probability::ALWAYS,
)];

pub const HEADS: [BodyPart; 1] = [BodyPart::new(
    "default",
    include_bytes!("../../assets/appearance/heads/1.png"),
    Probability::ALWAYS,
)];

pub const NOSES: [BodyPart; 1] = [BodyPart::new(
    "1",
    include_bytes!("../../assets/appearance/noses/1.png"),
    Probability::ALWAYS,
)];

pub const SCARS: [BodyPart; 4] = [
    BodyPart::new(
        "tiny_1",
        include_bytes!("../../assets/appearance/scars/tiny_1.png"),
        Probability::new(10),
    ),
    BodyPart::new(
        "tiny_2",
        include_bytes!("../../assets/appearance/scars/tiny_2.png"),
        Probability::new(10),
    ),
    BodyPart::new(
        "medium_1",
        include_bytes!("../../assets/appearance/scars/medium_1.png"),
        Probability::new(5),
    ),
    BodyPart::new(
        "big_eye_1",
        include_bytes!("../../assets/appearance/scars/big_eye_1.png"),
        Probability::new(5),
    ),
];

pub const MOUTHS: [BodyPart; 1] = [BodyPart::new(
    "1",
    include_bytes!("../../assets/appearance/mouths/1.png"),
    Probability::ALWAYS,
)];

pub const EYES: [BodyPart; 2] = [
    BodyPart::new(
        "1",
        include_bytes!("../../assets/appearance/eyes/1.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "2",
        include_bytes!("../../assets/appearance/eyes/2.png"),
        Probability::new(70),
    ),
];

pub const PUPILS: [BodyPart; 1] = [BodyPart::new(
    "1",
    include_bytes!("../../assets/appearance/pupils/1.png"),
    Probability::ALWAYS,
)];

pub const HAIRS: [BodyPart; 15] = [
    BodyPart::new(
        "short_1",
        include_bytes!("../../assets/appearance/hairs/short_1.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_2",
        include_bytes!("../../assets/appearance/hairs/short_2.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_3",
        include_bytes!("../../assets/appearance/hairs/short_3.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_4",
        include_bytes!("../../assets/appearance/hairs/short_4.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_5",
        include_bytes!("../../assets/appearance/hairs/short_5.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_6",
        include_bytes!("../../assets/appearance/hairs/short_6.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_7",
        include_bytes!("../../assets/appearance/hairs/short_7.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "short_7_variant",
        include_bytes!("../../assets/appearance/hairs/short_7_variant.png"),
        Probability::new(0),
    ),
    BodyPart::new(
        "short_8",
        include_bytes!("../../assets/appearance/hairs/short_8.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "medium_1",
        include_bytes!("../../assets/appearance/hairs/medium_1.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "long_1",
        include_bytes!("../../assets/appearance/hairs/long_1.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "long_2",
        include_bytes!("../../assets/appearance/hairs/long_2.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "long_3",
        include_bytes!("../../assets/appearance/hairs/long_3.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "long_4",
        include_bytes!("../../assets/appearance/hairs/long_4.png"),
        Probability::new(5), // Easter Egg: Suguru Geto's hair
    ),
    BodyPart::new(
        "long_5",
        include_bytes!("../../assets/appearance/hairs/long_5.png"),
        Probability::ALWAYS,
    ),
];

pub const EYEBROWS: [BodyPart; 2] = [
    BodyPart::new(
        "default",
        include_bytes!("../../assets/appearance/eyebrow/default.png"),
        Probability::ALWAYS,
    ),
    BodyPart::new(
        "angry",
        include_bytes!("../../assets/appearance/eyebrow/angry.png"),
        Probability::ALMOST_NEVER,
    ),
];

pub const FACE_COSMETICS: [BodyCosmetic; 3] = [
    BodyCosmetic::new(
        "empty",
        include_bytes!("../../assets/appearance/faceCosmetics/empty.png"),
        &[],
        Probability::ALWAYS,
    ),
    BodyCosmetic::new(
        "glasses_1",
        include_bytes!("../../assets/appearance/faceCosmetics/glasses_1.png"),
        &[
            (Color::Rgb(10, 110, 150), Probability::new(10)),
            (Color::Rgb(10, 70, 30), Probability::new(10)),
            (Color::Rgb(110, 30, 10), Probability::new(10)),
            (Color::Rgb(50, 50, 50), Probability::new(90)),
        ],
        Probability::new(20),
    ),
    BodyCosmetic::new(
        "eye_bandana",
        include_bytes!("../../assets/appearance/faceCosmetics/eye_bandana.png"),
        &[],
        Probability::ALMOST_NEVER,
    ),
];

pub const HEAD_COSMETICS: [BodyCosmetic; 2] = [
    BodyCosmetic::new(
        "empty",
        include_bytes!("../../assets/appearance/headCosmetics/empty.png"),
        &[],
        Probability::ALWAYS,
    ),
    BodyCosmetic::new(
        "strawhat",
        include_bytes!("../../assets/appearance/headCosmetics/strawhat.png"),
        &[],
        Probability::new(1),
    ),
];

pub const SKIN_COLORS: [Color; 6] = [
    Color::Rgb(235, 187, 164),
    Color::Rgb(240, 171, 149),
    Color::Rgb(194, 113, 85),
    Color::Rgb(158, 95, 74),
    Color::Rgb(74, 31, 21),
    Color::Rgb(38, 7, 0),
];

pub const EYE_COLORS: [ColorWithProbability; 12] = [
    ColorWithProbability::new(Color::Rgb(54, 33, 28), Probability::ALWAYS),
    ColorWithProbability::new(Color::Rgb(89, 89, 88), Probability::new(90)),
    ColorWithProbability::new(Color::Rgb(63, 133, 232), Probability::new(70)),
    ColorWithProbability::new(Color::Rgb(21, 232, 63), Probability::new(50)),
    ColorWithProbability::new(Color::Rgb(0, 255, 94), Probability::new(40)),
    ColorWithProbability::new(Color::Rgb(34, 211, 242), Probability::new(20)),
    ColorWithProbability::new(Color::Rgb(245, 87, 24), Probability::new(15)),
    ColorWithProbability::new(Color::Rgb(240, 200, 43), Probability::new(15)),
    ColorWithProbability::new(Color::Rgb(242, 19, 34), Probability::new(10)),
    ColorWithProbability::new(Color::Rgb(237, 47, 209), Probability::new(10)),
    ColorWithProbability::new(Color::Rgb(158, 48, 242), Probability::new(10)),
    ColorWithProbability::new(Color::Rgb(245, 245, 245), Probability::ALMOST_NEVER),
];

pub const HAIR_COLORS: [ColorWithProbability; 11] = [
    ColorWithProbability::new(Color::Rgb(7, 12, 8), Probability::ALWAYS),
    ColorWithProbability::new(Color::Rgb(23, 22, 21), Probability::ALWAYS),
    ColorWithProbability::new(Color::Rgb(51, 22, 13), Probability::ALWAYS),
    ColorWithProbability::new(Color::Rgb(158, 66, 38), Probability::ALWAYS),
    ColorWithProbability::new(Color::Rgb(245, 195, 47), Probability::new(50)),
    ColorWithProbability::new(Color::Rgb(245, 87, 24), Probability::new(20)),
    ColorWithProbability::new(Color::Rgb(252, 85, 76), Probability::new(20)),
    ColorWithProbability::new(Color::Rgb(245, 47, 37), Probability::new(10)),
    ColorWithProbability::new(Color::Rgb(77, 235, 161), Probability::new(2)),
    ColorWithProbability::new(Color::Rgb(52, 133, 247), Probability::new(1)),
    ColorWithProbability::new(Color::Rgb(165, 53, 230), Probability::ALMOST_NEVER),
];

pub fn random_skin_color<T: Rng>(rng: &mut T) -> Color {
    *SKIN_COLORS.choose(rng).unwrap()
}

pub fn random_eye_color<T: Rng>(rng: &mut T) -> Color {
    EYE_COLORS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap()
        .color
}

pub fn random_hair_color<T: Rng>(rng: &mut T) -> Color {
    HAIR_COLORS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap()
        .color
}

#[derive(Debug)]
pub struct BodyPartBundle {
    pub neck: BodyPart,
    pub head: BodyPart,
    pub mouth: BodyPart,
    pub nose: BodyPart,
    pub scars: Vec<BodyPart>,
    pub eyes: BodyPart,
    pub pupils: BodyPart,
    pub eyebrow: BodyPart,
    pub face_cosmetic: BodyCosmetic,
    pub head_cosmetic: BodyCosmetic,
    pub hair: BodyPart,
}

pub fn random_body_parts<T: Rng>(rng: &mut T) -> BodyPartBundle {
    let neck = *NECKS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let head = *HEADS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let mouth = *MOUTHS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let nose = *NOSES
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let eyes = *EYES
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let pupils = *PUPILS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let eyebrow = *EYEBROWS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let hair = *HAIRS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();

    let face_cosmetic = *FACE_COSMETICS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();
    let head_cosmetic = *HEAD_COSMETICS
        .choose_weighted(rng, |item| item.probability.value() as usize)
        .unwrap();

    let mut scars = Vec::new();

    while rng.gen_bool(0.05) {
        let scar = *SCARS
            .choose_weighted(rng, |item| item.probability.value() as usize)
            .unwrap();
        if !scars.iter().any(|s: &BodyPart| s.name == scar.name) {
            scars.push(scar);
        }
    }

    BodyPartBundle {
        neck,
        head,
        mouth,
        nose,
        scars,
        eyes,
        pupils,
        eyebrow,
        face_cosmetic,
        head_cosmetic,
        hair,
    }
}

////////////////////////////////////////////////////////////////
/// CHARACTER APPEARANCE
////////////////////////////////////////////////////////////////
fn _default_body_part() -> String {
    "1".into()
}

fn _default_head() -> String {
    "default".into()
}

// should probably never be used
const fn _default_color() -> Color {
    Color::BLUE
}

fn _default_eyebrow() -> String {
    "default".into()
}

fn _default_hair() -> String {
    "short_1".into()
}

fn _default_cosmetic() -> Cosmetic {
    Cosmetic::default()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Cosmetic {
    pub identifier: String,
    pub color: Color,
}

impl Default for Cosmetic {
    fn default() -> Self {
        Self {
            identifier: "empty".to_string(),
            color: Color::WHITE,
        }
    }
}

impl From<BodyCosmetic> for Cosmetic {
    fn from(value: BodyCosmetic) -> Self {
        Self {
            identifier: value.name.to_string(),
            color: *value
                .colors
                .choose_weighted(&mut StdRng::from_entropy(), |item| item.1.value() as usize)
                .map(|c| &c.0)
                .unwrap_or(&Color::WHITE),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Deserialize, Serialize)]
pub struct CharacterAppearance {
    #[serde(default = "_default_body_part")]
    pub neck: String,
    #[serde(default = "_default_head")]
    pub head: String,
    #[serde(default = "_default_body_part")]
    pub mouth: String,
    #[serde(default = "_default_body_part")]
    pub nose: String,
    #[serde(default = "Vec::new")]
    pub scars: Vec<String>,
    #[serde(default = "_default_body_part")]
    pub eyes: String,
    #[serde(default = "_default_body_part")]
    pub pupils: String,
    #[serde(default = "_default_eyebrow")]
    pub eyebrow: String,
    #[serde(default = "_default_hair")]
    pub hair: String,
    #[serde(default = "_default_cosmetic")]
    pub face_cosmetic: Cosmetic,
    #[serde(default = "_default_cosmetic")]
    pub head_cosmetic: Cosmetic,

    #[serde(default = "_default_color")]
    pub skin_color: Color,
    #[serde(default = "_default_color")]
    pub eyes_color: Color,
    #[serde(default = "_default_color")]
    pub hair_color: Color,
}

impl Default for CharacterAppearance {
    fn default() -> CharacterAppearance {
        CharacterAppearance {
            neck: _default_body_part(),
            head: _default_head(),
            mouth: _default_body_part(),
            nose: _default_body_part(),
            scars: Vec::new(),
            eyes: _default_body_part(),
            pupils: _default_body_part(),
            eyebrow: _default_eyebrow(),
            hair: _default_hair(),

            face_cosmetic: _default_cosmetic(),
            head_cosmetic: _default_cosmetic(),

            skin_color: _default_color(),
            eyes_color: _default_color(),
            hair_color: _default_color(),
        }
    }
}
