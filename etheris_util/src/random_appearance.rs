use etheris_data::appearance::*;
use rand::SeedableRng;

pub fn generate_random_character_appearance() -> CharacterAppearance {
    let mut rng = rand::rngs::StdRng::from_entropy();

    let body_parts = random_body_parts(&mut rng);

    CharacterAppearance {
        neck: body_parts.neck.name.to_string(),
        head: body_parts.head.name.to_string(),
        mouth: body_parts.mouth.name.to_string(),
        nose: body_parts.nose.name.to_string(),
        scars: body_parts
            .scars
            .iter()
            .map(|s| s.name.to_string())
            .collect(),
        eyes: body_parts.eyes.name.to_string(),
        pupils: body_parts.pupils.name.to_string(),
        eyebrow: body_parts.eyebrow.name.to_string(),
        hair: body_parts.hair.name.to_string(),

        face_cosmetic: body_parts.face_cosmetic.into(),
        head_cosmetic: body_parts.head_cosmetic.into(),

        skin_color: random_skin_color(&mut rng),
        eyes_color: random_eye_color(&mut rng),
        hair_color: random_hair_color(&mut rng),
    }
}
