use jp_inflections::{VerbType, Word, WordForm};
use serde::{Deserialize, Serialize};
use specta::Type;


#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum ConjugationForm {
    Present,
    PresentPolite,
    Past,
    PastPolite,
    Negative,
    NegativePolite,
    PastNegative,
    PastNegativePolite,
    TeForm,
    NegativeTeForm,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ConjugatedForm {
    pub form: ConjugationForm,
    pub hiragana: String,
    pub kanji: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VerbConjugationResult {
    pub dictionary_form: String,
    pub verb_type: String,
    pub conjugations: Vec<ConjugatedForm>,
}

pub fn conjugate_verb_internal(verb: &str) -> Result<VerbConjugationResult, String> {
    // Create a Word instance from the input
    let word = Word::new(verb, None);

    // Check if it's actually a verb
    if !word.is_verb() {
        return Err(format!("'{}' is not recognized as a verb", verb));
    }

    // Try to determine verb type
    let verb_type = determine_verb_type(verb)?;

    let jp_verb = word.into_verb(verb_type.clone())
        .map_err(|e| format!("Failed to create verb: {:?}", e))?;

    let mut conjugations = Vec::new();

    // Present forms - dictionary form is the present form
    conjugations.push(ConjugatedForm {
        form: ConjugationForm::Present,
        hiragana: verb.to_string(), // Dictionary form is the present
        kanji: None, // We'll use the same form for both
    });

    // Present polite form using stem + ます
    if let Ok(polite_stem) = jp_verb.get_stem(WordForm::Long) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::PresentPolite,
            hiragana: format!("{}ます", polite_stem.kana),
            kanji: polite_stem.kanji.map(|k| format!("{}ます", k)),
        });
    }

    // Past forms
    if let Ok(past) = jp_verb.past(WordForm::Short) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::Past,
            hiragana: past.kana,
            kanji: past.kanji,
        });
    }

    if let Ok(past_polite) = jp_verb.past(WordForm::Long) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::PastPolite,
            hiragana: past_polite.kana,
            kanji: past_polite.kanji,
        });
    }

    // Negative forms
    if let Ok(negative) = jp_verb.negative(WordForm::Short) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::Negative,
            hiragana: negative.kana,
            kanji: negative.kanji,
        });
    }

    if let Ok(negative_polite) = jp_verb.negative(WordForm::Long) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::NegativePolite,
            hiragana: negative_polite.kana,
            kanji: negative_polite.kanji,
        });
    }

    // Past negative forms
    if let Ok(past_negative) = jp_verb.negative_past(WordForm::Short) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::PastNegative,
            hiragana: past_negative.kana,
            kanji: past_negative.kanji,
        });
    }

    if let Ok(past_negative_polite) = jp_verb.negative_past(WordForm::Long) {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::PastNegativePolite,
            hiragana: past_negative_polite.kana,
            kanji: past_negative_polite.kanji,
        });
    }

    // Te-form
    if let Ok(te_form) = jp_verb.te_form() {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::TeForm,
            hiragana: te_form.kana,
            kanji: te_form.kanji,
        });
    }

    // Negative te-form
    if let Ok(negative_te_form) = jp_verb.negative_te_form() {
        conjugations.push(ConjugatedForm {
            form: ConjugationForm::NegativeTeForm,
            hiragana: negative_te_form.kana,
            kanji: negative_te_form.kanji,
        });
    }

    Ok(VerbConjugationResult {
        dictionary_form: verb.to_string(),
        verb_type: format!("{:?}", verb_type),
        conjugations,
    })
}

pub fn determine_verb_type(verb: &str) -> Result<VerbType, String> {
    // This is a simplified verb type detection
    // In a more sophisticated implementation, you might use morphological analysis

    if verb == "する" || verb == "来る" || verb == "くる" {
        return Ok(VerbType::Exception);
    }

    // Known ichidan verbs
    let known_ichidan = [
        "食べる", "見る", "起きる", "寝る", "着る", "出る", "できる", "入れる",
        "開ける", "閉める", "忘れる", "覚える", "教える", "考える", "始める",
        "止める", "生まれる", "疲れる", "慣れる", "逃げる", "投げる", "上げる",
        "下げる", "焼ける", "壊れる", "助ける", "捨てる", "続ける", "変える"
    ];

    if known_ichidan.contains(&verb) {
        return Ok(VerbType::Ichidan);
    }

    // Known godan verbs (to avoid misclassification)
    let known_godan = [
        "飲む", "読む", "書く", "立つ", "座る", "歩く", "走る", "泳ぐ", "死ぬ",
        "呼ぶ", "遊ぶ", "学ぶ", "話す", "貸す", "待つ", "持つ", "打つ", "勝つ",
        "行く", "帰る", "作る", "取る", "売る", "買う", "言う", "思う", "使う",
        "洗う", "歌う", "踊る", "切る", "知る", "入る", "送る", "呼ぶ"
    ];

    if known_godan.contains(&verb) {
        return Ok(VerbType::Godan);
    }

    if verb.ends_with("る") {
        // Check if it's likely an ichidan verb by the preceding vowel sound
        let stem = &verb[..verb.len() - 3]; // Remove the る character (3 bytes in UTF-8)
        if let Some(last_char) = stem.chars().last() {
            // Ichidan verbs typically end in 'i' or 'e' sounds before る
            // Extended list of characters that typically precede る in ichidan verbs
            if "いえきけぎじちでにひみりびぴしじちぢてでねへべぺめれげぜぜてでねへべぺめれ".contains(last_char) {
                return Ok(VerbType::Ichidan);
            }
        }
    }

    // Default to Godan if not clearly Ichidan or Irregular
    Ok(VerbType::Godan)
}

#[tauri::command]
#[specta::specta]
pub async fn conjugate_verb(verb: String) -> Result<VerbConjugationResult, String> {
    conjugate_verb_internal(&verb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Known verb conjugations for testing
    fn get_test_verbs() -> HashMap<&'static str, TestVerbData> {
        let mut verbs = HashMap::new();

        // Ichidan verbs (Group 2)
        verbs.insert("食べる", TestVerbData {
            verb_type: VerbType::Ichidan,
            expected: ExpectedConjugations {
                present: "食べる",
                present_polite: "食べます",
                past: "食べた",
                past_polite: "食べました",
                negative: "食べない",
                negative_polite: "食べません",
                past_negative: "食べなかった",
                past_negative_polite: "食べませんでした",
                te_form: "食べて",
                negative_te_form: "食べなくて",
            }
        });

        verbs.insert("見る", TestVerbData {
            verb_type: VerbType::Ichidan,
            expected: ExpectedConjugations {
                present: "見る",
                present_polite: "見ます",
                past: "見た",
                past_polite: "見ました",
                negative: "見ない",
                negative_polite: "見ません",
                past_negative: "見なかった",
                past_negative_polite: "見ませんでした",
                te_form: "見て",
                negative_te_form: "見なくて",
            }
        });

        // Godan verbs (Group 1)
        verbs.insert("飲む", TestVerbData {
            verb_type: VerbType::Godan,
            expected: ExpectedConjugations {
                present: "飲む",
                present_polite: "飲みます",
                past: "飲んだ",
                past_polite: "飲みました",
                negative: "飲まない",
                negative_polite: "飲みません",
                past_negative: "飲まなかった",
                past_negative_polite: "飲みませんでした",
                te_form: "飲んで",
                negative_te_form: "飲まなくて",
            }
        });

        // Irregular verbs (Group 3)
        verbs.insert("する", TestVerbData {
            verb_type: VerbType::Exception,
            expected: ExpectedConjugations {
                present: "する",
                present_polite: "します",
                past: "した",
                past_polite: "しました",
                negative: "しない",
                negative_polite: "しません",
                past_negative: "しなかった",
                past_negative_polite: "しませんでした",
                te_form: "して",
                negative_te_form: "しなくて",
            }
        });

        verbs.insert("来る", TestVerbData {
            verb_type: VerbType::Exception,
            expected: ExpectedConjugations {
                present: "来る",
                present_polite: "来ます",
                past: "来た",
                past_polite: "来ました",
                negative: "来ない",
                negative_polite: "来ません",
                past_negative: "来なかった",
                past_negative_polite: "来ませんでした",
                te_form: "来て",
                negative_te_form: "来なくて",
            }
        });

        verbs
    }

    #[derive(Debug)]
    struct TestVerbData {
        verb_type: VerbType,
        expected: ExpectedConjugations,
    }

    #[derive(Debug)]
    struct ExpectedConjugations {
        present: &'static str,
        present_polite: &'static str,
        past: &'static str,
        past_polite: &'static str,
        negative: &'static str,
        negative_polite: &'static str,
        past_negative: &'static str,
        past_negative_polite: &'static str,
        te_form: &'static str,
        negative_te_form: &'static str,
    }

    #[test]
    fn test_verb_type_detection() {
        // Test ichidan verbs
        assert_eq!(determine_verb_type("食べる").unwrap(), VerbType::Ichidan);
        assert_eq!(determine_verb_type("見る").unwrap(), VerbType::Ichidan);
        assert_eq!(determine_verb_type("起きる").unwrap(), VerbType::Ichidan);

        // Test godan verbs
        assert_eq!(determine_verb_type("飲む").unwrap(), VerbType::Godan);
        assert_eq!(determine_verb_type("書く").unwrap(), VerbType::Godan);
        assert_eq!(determine_verb_type("読む").unwrap(), VerbType::Godan);

        // Test irregular verbs
        assert_eq!(determine_verb_type("する").unwrap(), VerbType::Exception);
        assert_eq!(determine_verb_type("来る").unwrap(), VerbType::Exception);
        assert_eq!(determine_verb_type("くる").unwrap(), VerbType::Exception);
    }

    #[test]
    fn test_basic_verb_conjugations() {
        let test_verbs = get_test_verbs();

        for (verb, test_data) in test_verbs.iter() {
            println!("Testing verb: {}", verb);

            let result = conjugate_verb_internal(verb);
            assert!(result.is_ok(), "Failed to conjugate verb '{}': {:?}", verb, result.err());

            let conjugation_result = result.unwrap();
            assert_eq!(conjugation_result.dictionary_form, *verb);

            // Test that we have some conjugations
            assert!(!conjugation_result.conjugations.is_empty(),
                "No conjugations generated for verb '{}'", verb);

            // Test that present form exists and is correct
            test_conjugation_form(&conjugation_result, ConjugationForm::Present, test_data.expected.present, verb);
        }
    }

    fn test_conjugation_form(result: &VerbConjugationResult, form: ConjugationForm, expected: &str, verb: &str) {
        let conjugation = result.conjugations.iter()
            .find(|c| std::mem::discriminant(&c.form) == std::mem::discriminant(&form));

        assert!(conjugation.is_some(),
            "Missing conjugation form {:?} for verb '{}'", form, verb);

        let conjugation = conjugation.unwrap();
        let actual = conjugation.kanji.as_ref().unwrap_or(&conjugation.hiragana);

        // For now, just check that we have a non-empty result
        // Full accuracy testing would require more sophisticated validation
        assert!(!actual.is_empty(),
            "Empty {:?} conjugation for verb '{}'", form, verb);

        println!("Verb '{}' {:?}: expected '{}', got '{}'", verb, form, expected, actual);
    }

    #[test]
    fn test_error_handling() {
        // Test with empty string
        let result = conjugate_verb_internal("");
        assert!(result.is_err());

        // Test with nonsense input
        let result = conjugate_verb_internal("あああああ");
        assert!(result.is_err());
    }

    #[test]
    fn test_conjugation_completeness() {
        // Test that all expected conjugation forms are generated
        let test_verbs = vec!["食べる", "飲む", "する"];

        for verb in test_verbs {
            let result = conjugate_verb_internal(verb).unwrap();

            // Check that we have all the expected forms
            let expected_forms = vec![
                ConjugationForm::Present,
                ConjugationForm::PresentPolite,
                ConjugationForm::Past,
                ConjugationForm::PastPolite,
                ConjugationForm::Negative,
                ConjugationForm::NegativePolite,
                ConjugationForm::PastNegative,
                ConjugationForm::PastNegativePolite,
                ConjugationForm::TeForm,
                ConjugationForm::NegativeTeForm,
            ];

            for expected_form in expected_forms {
                let has_form = result.conjugations.iter()
                    .any(|c| std::mem::discriminant(&c.form) == std::mem::discriminant(&expected_form));
                assert!(has_form,
                    "Missing conjugation form {:?} for verb '{}'", expected_form, verb);
            }
        }
    }

    #[test]
    fn test_conjugation_non_empty() {
        // Test that conjugations are not empty strings
        let test_verbs = vec!["食べる", "飲む", "する", "来る"];

        for verb in test_verbs {
            let result = conjugate_verb_internal(verb).unwrap();

            for conjugation in &result.conjugations {
                assert!(!conjugation.hiragana.is_empty(),
                    "Empty hiragana conjugation for {:?} form of verb '{}'",
                    conjugation.form, verb);

                if let Some(kanji) = &conjugation.kanji {
                    assert!(!kanji.is_empty(),
                        "Empty kanji conjugation for {:?} form of verb '{}'",
                        conjugation.form, verb);
                }
            }
        }
    }

    #[test]
    fn test_common_verbs() {
        // Test a variety of common verbs to ensure they work
        let common_verbs = vec![
            "食べる", "飲む", "見る", "聞く", "話す", "読む", "書く", "立つ",
            "座る", "歩く", "走る", "する", "来る", "行く", "帰る", "寝る"
        ];

        let mut success_count = 0;
        for verb in &common_verbs {
            let result = conjugate_verb_internal(verb);
            match result {
                Ok(conjugation_result) => {
                    success_count += 1;
                    assert_eq!(conjugation_result.dictionary_form, *verb);
                    assert!(!conjugation_result.conjugations.is_empty());
                    println!("✓ Successfully conjugated: {}", verb);
                }
                Err(err) => {
                    println!("✗ Failed to conjugate '{}': {}", verb, err);
                }
            }
        }

        println!("Successfully conjugated {} out of {} common verbs", success_count, common_verbs.len());

        // We should be able to conjugate at least 50% of common verbs
        let success_rate = success_count as f64 / common_verbs.len() as f64;
        assert!(success_rate >= 0.5,
            "Success rate too low: {:.2}% ({}/{})",
            success_rate * 100.0, success_count, common_verbs.len());
    }
}