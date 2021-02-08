#[macro_use]
extern crate rustler;

use crate::language::LanguageType;
use builder::BuilderOption;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use rustler::{Encoder, Env, NifResult, SchedulerFlags, Term};
use std::collections::hash_set::HashSet;

mod atoms;
mod builder;
mod language;

rustler::rustler_export_nifs! {
    "Elixir.Lingua.Nif",
    [
        // language utility function
        ("all_languages", 0, all_languages),
        ("all_spoken_languages", 0, all_spoken_languages),
        ("all_with_arabic_script", 0, all_with_arabic_script),
        ("all_with_cyrillic_script", 0, all_with_cyrillic_script),
        ("all_with_devanagari_script", 0, all_with_devanagari_script),
        ("all_with_latin_script", 0, all_with_latin_script),
        ("language_for_iso_code", 1, language_for_iso_code),
        ("language_for_iso_code_639_1", 1, language_for_iso_code_639_1),
        ("language_for_iso_code_639_3", 1, language_for_iso_code_639_3),
        ("iso_code_639_1_for_language", 1, iso_code_639_1_for_language),
        ("iso_code_639_3_for_language", 1, iso_code_639_3_for_language),

        // language detection
        ("init", 0, init, SchedulerFlags::DirtyIo),
        ("detect_language_of", 4, detect_language_of, SchedulerFlags::DirtyIo),
        ("compute_language_confidence_values",  4, compute_language_confidence_values, SchedulerFlags::DirtyIo),
    ],
    None
}

fn _all_languages<'a>(env: Env<'a>, f: fn() -> HashSet<lingua::Language>) -> NifResult<Term<'a>> {
    Ok((f()
        .into_iter()
        .map(|language| language::Language(language))
        .collect::<Vec<language::Language>>())
    .encode(env))
}

fn all_languages<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all)
}

fn all_spoken_languages<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all_spoken_ones)
}

fn all_with_arabic_script<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all_with_arabic_script)
}

fn all_with_cyrillic_script<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all_with_cyrillic_script)
}

fn all_with_devanagari_script<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all_with_devanagari_script)
}

fn all_with_latin_script<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    _all_languages(env, lingua::Language::all_with_latin_script)
}

fn language_for_iso_code<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match args[0].decode() {
        Ok(LanguageType::IsoCode639_1(code)) => {
            let language = Language::from_iso_code_639_1(&*code);
            Ok((atoms::ok(), language::Language(language)).encode(env))
        }
        Ok(LanguageType::IsoCode639_3(code)) => {
            let language = Language::from_iso_code_639_3(&*code);
            Ok((atoms::ok(), language::Language(language)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

fn language_for_iso_code_639_1<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match args[0].decode() {
        Ok(LanguageType::IsoCode639_1(code)) => {
            let language = Language::from_iso_code_639_1(&*code);
            Ok((atoms::ok(), language::Language(language)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

fn language_for_iso_code_639_3<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match args[0].decode() {
        Ok(LanguageType::IsoCode639_3(code)) => {
            let language = Language::from_iso_code_639_3(&*code);
            Ok((atoms::ok(), language::Language(language)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_iso_code()).encode(env)),
    }
}

fn iso_code_639_1_for_language<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match args[0].decode() {
        Ok(language::Language(lang)) => {
            let code = lang.iso_code_639_1();
            Ok((atoms::ok(), language::IsoCode639_1(code)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_language()).encode(env)),
    }
}

fn iso_code_639_3_for_language<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match args[0].decode() {
        Ok(language::Language(lang)) => {
            let code = lang.iso_code_639_3();
            Ok((atoms::ok(), language::IsoCode639_3(code)).encode(env))
        }
        _ => Ok((atoms::error(), atoms::unrecognized_language()).encode(env)),
    }
}

fn init<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    LanguageDetectorBuilder::from_all_languages().build();

    Ok((atoms::ok()).encode(env))
}

fn detect_language_of<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;

    let opt: BuilderOption = args[1].decode()?;
    let languages = decode_languages(args[2])?;
    let mut builder = create_builder(opt, languages);
    let minimum_relative_distance: f64 = args[3].decode()?;

    builder.with_minimum_relative_distance(minimum_relative_distance);
    let detector: LanguageDetector = builder.build();

    let detected_language: Option<Language> = detector.detect_language_of(text);

    match detected_language {
        Some(language) => Ok((atoms::ok(), language::Language(language)).encode(env)),
        None => Ok((atoms::ok(), atoms::no_match()).encode(env)),
    }
}

fn compute_language_confidence_values<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;

    let opt: BuilderOption = args[1].decode()?;
    let languages = decode_languages(args[2])?;
    let mut builder = create_builder(opt, languages);
    let minimum_relative_distance: f64 = args[3].decode()?;

    builder.with_minimum_relative_distance(minimum_relative_distance);
    let detector: LanguageDetector = builder.build();

    let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(text);
    let result: Vec<(language::Language, f64)> = confidence_values
        .into_iter()
        .map(|(language, value)| (language::Language(language.clone()), value))
        .collect();

    match result.len() {
        0 => Ok((atoms::ok(), atoms::no_match()).encode(env)),
        _ => Ok((atoms::ok(), result).encode(env)),
    }
}

fn decode_languages<'a>(arg: Term<'a>) -> NifResult<Vec<lingua::Language>> {
    let language_types: Vec<language::LanguageType> = arg.decode()?;

    Ok(language_types
        .into_iter()
        .map(|language_type| match language_type {
            LanguageType::IsoCode639_1(l) => lingua::Language::from_iso_code_639_1(&*l),
            LanguageType::IsoCode639_3(l) => lingua::Language::from_iso_code_639_3(&*l),
            LanguageType::Language(l) => l.clone(),
        })
        .collect())
}

fn create_builder(option: BuilderOption, languages: Vec<Language>) -> LanguageDetectorBuilder {
    match option {
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
    }
}
