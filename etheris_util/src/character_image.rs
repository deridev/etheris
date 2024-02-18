use std::io::Cursor;

use etheris_common::Color;
use etheris_data::appearance::*;
use image::imageops::{overlay, resize, FilterType};
use image::ImageFormat;

use crate::image_util::*;
use cached::proc_macro::cached;
use cached::SizedCache;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

pub fn generate_character_image_buffer(appearance: &CharacterAppearance) -> Vec<u8> {
    internal_generate_character_image_buffer(appearance).expect("Error generating character image buffer, probably generated at one of the image::load_from_memory's.")
}

#[cached(
    type = "SizedCache<CharacterAppearance, Option<Vec<u8>>>",
    create = "{ SizedCache::with_size(256) }"
)]
fn internal_generate_character_image_buffer(appearance: &CharacterAppearance) -> Option<Vec<u8>> {
    // Colors
    let skin_color = appearance.skin_color;
    let eyes_color = appearance.eyes_color;
    let mut hair_color = appearance.hair_color;

    let face_cosmetic_color = appearance.face_cosmetic.color;
    let head_cosmetic_color = appearance.head_cosmetic.color;

    // Load images
    let neck = NECKS
        .into_iter()
        .find(|v| v.name == appearance.neck)
        .unwrap()
        .bytes;
    let head = HEADS
        .into_iter()
        .find(|v| v.name == appearance.head)
        .unwrap()
        .bytes;
    let mouth = MOUTHS
        .into_iter()
        .find(|v| v.name == appearance.mouth)
        .unwrap()
        .bytes;
    let nose = NOSES
        .into_iter()
        .find(|v| v.name == appearance.nose)
        .unwrap()
        .bytes;
    let eyes = EYES
        .into_iter()
        .find(|v| v.name == appearance.eyes)
        .unwrap()
        .bytes;
    let pupils = PUPILS
        .into_iter()
        .find(|v| v.name == appearance.pupils)
        .unwrap()
        .bytes;
    let eyebrow = EYEBROWS
        .into_iter()
        .find(|v| v.name == appearance.eyebrow)
        .unwrap()
        .bytes;

    // If the hair is short_7 and the equipped face cosmetic is eye_bandana, then draw the Gojo's hair easter egg.
    // Else, just draw the character hair
    let hair_name =
        if appearance.face_cosmetic.identifier == "eye_bandana" && appearance.hair == "short_7" {
            hair_color = Color::WHITE;
            "short_7_variant".to_string()
        } else {
            appearance.hair.clone()
        };

    let hair = HAIRS
        .into_iter()
        .find(|v| v.name == hair_name)
        .unwrap()
        .bytes;

    let face_cosmetic = FACE_COSMETICS
        .into_iter()
        .find(|v| v.name == appearance.face_cosmetic.identifier)
        .unwrap()
        .bytes;

    let head_cosmetic = HEAD_COSMETICS
        .into_iter()
        .find(|v| v.name == appearance.head_cosmetic.identifier)
        .unwrap()
        .bytes;

    // Create Images
    let neck = image::load_from_memory(neck).ok()?;
    let head = image::load_from_memory(head).ok()?;
    let mouth = image::load_from_memory(mouth).ok()?;
    let nose = image::load_from_memory(nose).ok()?;
    let eyes = image::load_from_memory(eyes).ok()?;
    let pupils = image::load_from_memory(pupils).ok()?;
    let eyebrow = image::load_from_memory(eyebrow).ok()?;
    let hair = image::load_from_memory(hair).ok()?;

    let face_cosmetic = image::load_from_memory(face_cosmetic).ok()?;
    let head_cosmetic = image::load_from_memory(head_cosmetic).ok()?;

    let mut neck = resize(&neck, WIDTH, HEIGHT, FilterType::Nearest);

    // Composite head, mouse and nose
    let images = vec![&head, &mouth, &nose];
    for image in images.into_iter() {
        let image = resize(image, WIDTH, HEIGHT, FilterType::Nearest);
        overlay(&mut neck, &image, 0, 0);
    }

    // And composite scars, if any
    for scar in appearance.scars.iter() {
        let scar = SCARS.into_iter().find(|s| s.name == scar).unwrap().bytes;
        let scar_image = image::load_from_memory(scar).ok()?;
        let scar_image = resize(&scar_image, WIDTH, HEIGHT, FilterType::Nearest);
        overlay(&mut neck, &scar_image, 0, 0);
    }

    // Change the skin color
    tint_image(
        &mut neck,
        Tint {
            color: skin_color,
            factor: 0.8,
            brightness_threshold: None,
        },
    );

    // Eyes and pupils
    let mut eyes = resize(&eyes, WIDTH, HEIGHT, FilterType::Nearest);
    let mut pupils = resize(&pupils, WIDTH, HEIGHT, FilterType::Nearest);

    // Change the pupils color
    tint_image(
        &mut pupils,
        Tint {
            color: eyes_color,
            factor: 0.7,
            brightness_threshold: Some(170),
        },
    );

    // Overlay pupil with eyes with the base image (neck)
    overlay(&mut eyes, &pupils, 0, 0);
    overlay(&mut neck, &eyes, 0, 0);

    // Face cosmetic
    let mut face_cosmetic = resize(&face_cosmetic, WIDTH, HEIGHT, FilterType::Nearest);
    if face_cosmetic_color != Color::WHITE {
        tint_image(
            &mut face_cosmetic,
            Tint {
                color: face_cosmetic_color,
                factor: 0.7,
                brightness_threshold: Some(170),
            },
        );
    }
    overlay(&mut neck, &face_cosmetic, 0, 0);

    // Tint hair and eyebrow and composite it
    let mut eyebrow = resize(&eyebrow, WIDTH, HEIGHT, FilterType::Nearest);
    let mut hair = resize(&hair, WIDTH, HEIGHT, FilterType::Nearest);
    tint_image(
        &mut eyebrow,
        Tint {
            color: hair_color,
            factor: 0.6,
            brightness_threshold: Some(200),
        },
    );

    tint_image(
        &mut hair,
        Tint {
            color: hair_color,
            factor: 0.7,
            brightness_threshold: Some(220),
        },
    );

    overlay(&mut neck, &eyebrow, 0, 0);
    overlay(&mut neck, &hair, 0, 0);

    // Head cosmetic
    let mut head_cosmetic = resize(&head_cosmetic, WIDTH, HEIGHT, FilterType::Nearest);
    if head_cosmetic_color != Color::WHITE {
        tint_image(
            &mut head_cosmetic,
            Tint {
                color: head_cosmetic_color,
                factor: 0.7,
                brightness_threshold: Some(170),
            },
        );
    }
    overlay(&mut neck, &head_cosmetic, 0, 0);
    remove_chroma_key(&mut neck); // head_cosmetic can have a "chroma key" that should be removed

    // Return buffer
    let base_image = trim_transparent_pixels(neck.into());

    let mut data = Cursor::new(Vec::new());
    base_image.write_to(&mut data, ImageFormat::Png).unwrap();

    let result = Some(data.get_ref().clone()); // Clone the Vec to return it
    data.get_mut().clear(); // Clear the Vec to free its memory
    result
}
