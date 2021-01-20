#[macro_use]
extern crate rustler;

use builder::BuilderOption;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use rustler::{Atom, Encoder, Env, NifResult, SchedulerFlags, Term};
use std::ops::Deref;

mod atoms;
mod builder;
mod language;

#[derive(Debug, Clone)]
struct ArgumentError {
    error: Atom,
}

rustler::rustler_export_nifs! {
    "Elixir.Lingua.Nif",
    [
        ("detect_language_of", 4, detect_language_of, SchedulerFlags::DirtyIo),
    ],
    None
}

fn detect_language_of<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;

    let opt: BuilderOption = args[1].decode()?;

    let language_list: Vec<language::Language> = args[2].decode()?;
    let languages: Vec<lingua::Language> = language_list
        .into_iter()
        .map(|x| x.deref().clone())
        .collect();

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
