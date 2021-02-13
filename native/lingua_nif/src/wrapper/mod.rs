use rustler::{Decoder, Error, NifResult, Term};

pub mod language;
pub mod iso_639_1;
pub mod iso_639_3;

 use self::iso_639_1::IsoCode639_1;
 use self::iso_639_3::IsoCode639_3;
 use self::language::Language;

#[derive(Debug)]
pub enum LanguageType {
    Language(Language),
    IsoCode639_1(IsoCode639_1),
    IsoCode639_3(IsoCode639_3),
}

impl<'a> Decoder<'a> for LanguageType {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if let Ok(language) = term.decode::<Language>() {
            Ok(LanguageType::Language(language))
        } else if let Ok(iso6391) = term.decode::<IsoCode639_1>() {
            Ok(LanguageType::IsoCode639_1(iso6391))
        } else if let Ok(iso6393) = term.decode::<IsoCode639_3>() {
            Ok(LanguageType::IsoCode639_3(iso6393))
        } else {
            Err(Error::BadArg)
        }
    }
}

