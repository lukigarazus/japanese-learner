use lindera::LinderaResult;
use lindera::dictionary::{DictionaryKind, load_embedded_dictionary};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;

fn katakana_to_hiragana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if ('ァ'..='ン').contains(&c) {
                // Shift Unicode value from Katakana to Hiragana block
                std::char::from_u32(c as u32 - 0x60).unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub fn convert(text: &String) -> LinderaResult<String> {
    let dictionary = load_embedded_dictionary(DictionaryKind::IPADIC)?;
    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
    let tokenizer = Tokenizer::new(segmenter);

    let mut tokens = tokenizer.tokenize(text)?;

    let mut hiragana_output = String::new();
    for token in tokens.iter_mut() {
        let details = token.details();
        if details.len() > 7 {
            let katakana = &details[7];
            let hira = katakana_to_hiragana(katakana);
            hiragana_output.push_str(&hira);
        } else {
            // fallback to surface form if no reading
            hiragana_output.push_str(&token.text);
        }
    }

    Ok(hiragana_output)
}
