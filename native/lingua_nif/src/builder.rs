use crate::atoms;
use rustler::{Decoder, Error, NifResult, Term};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum BuilderOption {
    AllLanguages,
    AllSpokenLanguages,
    AllLanguagesWithArabicScript,
    AllLanguagesWithCyrillicScript,
    AllLanguagesWithDevanagariScript,
    AllLanguagesWithLatinScript,

    WithLanguages,
    WithoutLanguages,
}

impl<'a> Decoder<'a> for BuilderOption {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if atoms::all_languages() == term {
            Ok(Self::AllLanguages)
        } else if atoms::all_spoken_languages() == term {
            Ok(Self::AllSpokenLanguages)
        } else if atoms::all_languages_with_arabic_script() == term {
            Ok(Self::AllLanguagesWithArabicScript)
        } else if atoms::all_languages_with_cyrillic_script() == term {
            Ok(Self::AllLanguagesWithCyrillicScript)
        } else if atoms::all_languages_with_devanagari_script() == term {
            Ok(Self::AllLanguagesWithDevanagariScript)
        } else if atoms::all_languages_with_latin_script() == term {
            Ok(Self::AllLanguagesWithLatinScript)
        } else if atoms::with_languages() == term {
            Ok(Self::WithLanguages)
        } else if atoms::without_languages() == term {
            Ok(Self::WithoutLanguages)
        } else {
            Err(Error::BadArg)
        }
    }
}

