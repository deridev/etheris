use etheris_common::clear_string;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzç-";
const LANGUAGE_CHARSET: &str = "ηλhjircμyzłjwtωsvkaσpdqmxnψ↣";

pub fn translate(text: &str) -> String {
    let mut output = String::with_capacity(text.len());

    for char in text.split("") {
        let is_uppercase = char == char.to_uppercase();
        let c = clear_string(char);

        if let Some((char_index, _)) = CHARSET.split("").enumerate().find(|(_, lc)| lc == &c) {
            let character = LANGUAGE_CHARSET.split("").nth(char_index).unwrap_or(&c);
            output.push_str(&if is_uppercase {
                character.to_uppercase().to_string()
            } else {
                character.to_string()
            });
        } else {
            output.push_str(char);
        }
    }

    output
}

pub fn untranslate(text: &str) -> String {
    let mut output = String::with_capacity(text.len());

    for char in text.split("") {
        let is_uppercase = char == char.to_uppercase();
        let c = char.to_lowercase();

        if let Some((char_index, _)) = LANGUAGE_CHARSET
            .split("")
            .enumerate()
            .find(|(_, lc)| lc == &c)
        {
            let character = CHARSET.split("").nth(char_index).unwrap_or(&c);
            output.push_str(&if is_uppercase {
                character.to_uppercase().to_string()
            } else {
                character.to_string()
            });
        } else {
            output.push_str(char);
        }
    }

    output
}

#[test]
fn test_translation() {
    let strs = [
        "Etheris is very interesting and beautiful.",
        "The quick brown fox jumps over the lazy dog",
        "The quick brown fox jumps over a lazy dog is the most famous pangram in English, that is the most short sentence in which all the 26 letters of the alphabet are used. For this reason, it is useful to test the graphic aspect of the fonts, because all the letters are immediately on hand."
    ];

    for str in strs {
        assert_eq!(translate(str), translate(&untranslate(&translate(str))));
    }
}
