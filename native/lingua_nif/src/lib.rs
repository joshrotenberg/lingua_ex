#[macro_use]
extern crate rustler;

use builder::BuilderOption;
use lingua::Language as linguaLanguage;
use lingua::{LanguageDetectorBuilder};
use rustler::{Encoder, Env, NifResult, Term};
use std::collections::hash_set::HashSet;

use crate::wrapper::iso_639_1::IsoCode639_1;
use crate::wrapper::iso_639_3::IsoCode639_3;
use crate::wrapper::language::Language;
use crate::wrapper::LanguageType;

mod atoms;
mod builder;
mod wrapper;

rustler::init! {
    "Elixir.Lingua.Nif",
    [
        // language detection
        // http://erlang.org/pipermail/erlang-questions/2018-October/096531.html
        init,
        run_detection,

        // language utility functions
        all_languages,
        all_spoken_languages,
        all_languages_with_arabic_script,
        all_languages_with_cyrillic_script,
        all_languages_with_devanagari_script,
        all_languages_with_latin_script,
        language_for_iso_code,
        language_for_iso_code_639_1,
        language_for_iso_code_639_3,
        iso_code_639_1_for_language,
        iso_code_639_3_for_language,
    ]
}

// language detection
#[rustler::nif(schedule = "DirtyCpu")]
fn init<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    LanguageDetectorBuilder::from_all_languages()
        .with_preloaded_language_models()
        .build();

    Ok((atoms::ok()).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn run_detection<'a>(
    env: Env<'a>,
    text: String,
    option: BuilderOption,
    language_types: Vec<LanguageType>,
    compute_language_confidence_values: bool,
    minimum_relative_distance: f64,
    preload_language_models: bool,
    low_accuracy_mode: bool
) -> NifResult<Term<'a>> {
    let languages = decode_languages(language_types);

    if languages.len() < 2 && option == BuilderOption::WithLanguages {
        return Ok((atoms::error(), atoms::insufficient_languages()).encode(env));
    }

    if minimum_relative_distance < 0.0 || minimum_relative_distance > 0.99 {
        return Ok((
            atoms::error(),
            atoms::out_of_range_minimum_relative_distance(),
        )
            .encode(env));
    }

    let mut builder = match option {
        BuilderOption::AllLanguages => LanguageDetectorBuilder::from_all_languages(),
        BuilderOption::AllSpokenLanguages => LanguageDetectorBuilder::from_all_spoken_languages(),
        BuilderOption::AllLanguagesWithArabicScript => {
            LanguageDetectorBuilder::from_all_languages_with_arabic_script()
        }
        BuilderOption::AllLanguagesWithCyrillicScript => {
            LanguageDetectorBuilder::from_all_languages_with_cyrillic_script()
        }
        BuilderOption::AllLanguagesWithDevanagariScript => {
            LanguageDetectorBuilder::from_all_languages_with_devanagari_script()
        }
        BuilderOption::AllLanguagesWithLatinScript => {
            LanguageDetectorBuilder::from_all_languages_with_latin_script()
        }

        BuilderOption::WithLanguages => LanguageDetectorBuilder::from_languages(&languages),
        BuilderOption::WithoutLanguages => {
            LanguageDetectorBuilder::from_all_languages_without(&languages)
        }
    };

    builder.with_minimum_relative_distance(minimum_relative_distance);
    if preload_language_models {
        builder.with_preloaded_language_models();
    }
    if low_accuracy_mode {
        builder.with_low_accuracy_mode();
    }

    let detector = builder.build();
    if compute_language_confidence_values == true {
        let confidence_values: Vec<(linguaLanguage, f64)> =
            detector.compute_language_confidence_values(text);
        let result: Vec<(Language, f64)> = confidence_values
            .into_iter()
            .map(|(language, value)| (Language(language.clone()), value))
            .collect();

        match result.len() {
            0 => Ok((atoms::ok(), atoms::no_match()).encode(env)),
            _ => Ok((atoms::ok(), result).encode(env)),
        }
    } else {
        let detected_language: Option<linguaLanguage> = detector.detect_language_of(text);

        match detected_language {
            Some(language) => Ok((atoms::ok(), Language(language)).encode(env)),
            None => Ok((atoms::ok(), atoms::no_match()).encode(env)),
        }
    }
}

fn decode_languages(language_types: Vec<LanguageType>) -> Vec<linguaLanguage> {
    language_types
        .into_iter()
        .map(|language_type| match language_type {
            LanguageType::IsoCode639_1(l) => linguaLanguage::from_iso_code_639_1(&*l),
            LanguageType::IsoCode639_3(l) => linguaLanguage::from_iso_code_639_3(&*l),
            LanguageType::Language(l) => l.clone(),
        })
        .collect()
}

// language utility functions
#[rustler::nif]
fn all_languages<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all)
}

#[rustler::nif]
fn all_spoken_languages<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all_spoken_ones)
}

#[rustler::nif]
fn all_languages_with_arabic_script<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all_with_arabic_script)
}

#[rustler::nif]
fn all_languages_with_cyrillic_script<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all_with_cyrillic_script)
}

#[rustler::nif]
fn all_languages_with_devanagari_script<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all_with_devanagari_script)
}

#[rustler::nif]
fn all_languages_with_latin_script<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    all(env, linguaLanguage::all_with_latin_script)
}

#[rustler::nif]
fn language_for_iso_code<'a>(env: Env<'a>, iso_code: Term<'a>) -> NifResult<Term<'a>> {
    match iso_code.decode() {
        Ok(LanguageType::IsoCode639_1(code)) => Ok((
            atoms::ok(),
            Language(linguaLanguage::from_iso_code_639_1(&*code)),
        )
            .encode(env)),
        Ok(LanguageType::IsoCode639_3(code)) => Ok((
            atoms::ok(),
            Language(linguaLanguage::from_iso_code_639_3(&*code)),
        )
            .encode(env)),
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

#[rustler::nif]
fn language_for_iso_code_639_1<'a>(env: Env<'a>, iso_code: Term<'a>) -> NifResult<Term<'a>> {
    match iso_code.decode() {
        Ok(LanguageType::IsoCode639_1(code)) => Ok((
            atoms::ok(),
            Language(linguaLanguage::from_iso_code_639_1(&*code)),
        )
            .encode(env)),
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

#[rustler::nif]
fn language_for_iso_code_639_3<'a>(env: Env<'a>, iso_code: Term<'a>) -> NifResult<Term<'a>> {
    match iso_code.decode() {
        Ok(LanguageType::IsoCode639_3(code)) => Ok((
            atoms::ok(),
            Language(linguaLanguage::from_iso_code_639_3(&*code)),
        )
            .encode(env)),
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

#[rustler::nif]
fn iso_code_639_1_for_language<'a>(env: Env<'a>, language: Term<'a>) -> NifResult<Term<'a>> {
    match language.decode() {
        Ok(Language(language)) => {
            let code = language.iso_code_639_1();
            Ok((atoms::ok(), IsoCode639_1(code)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_language()).encode(env)),
    }
}

#[rustler::nif]
fn iso_code_639_3_for_language<'a>(env: Env<'a>, language: Term<'a>) -> NifResult<Term<'a>> {
    match language.decode() {
        Ok(Language(language)) => {
            let code = language.iso_code_639_3();
            Ok((atoms::ok(), IsoCode639_3(code)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_language()).encode(env)),
    }
}

fn all<'a>(env: Env<'a>, f: fn() -> HashSet<linguaLanguage>) -> NifResult<Term<'a>> {
    let mut languages = f()
        .into_iter()
        .map(|language| Language(language))
        .collect::<Vec<Language>>();
    languages.sort_by(|a, b| a.cmp(b));
    Ok((languages).encode(env))
}
