use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct Language(pub lingua::Language);

macro_rules! impl_language_codec {
    ($($atom:ident => $variant:ident),* $(,)?) => {
        impl<'a> Decoder<'a> for Language {
            fn decode(term: Term<'a>) -> NifResult<Self> {
                $(
                    if atoms::$atom() == term {
                        return Ok(Language(lingua::Language::$variant));
                    }
                )*
                Err(Error::BadArg)
            }
        }

        impl Encoder for Language {
            fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
                match self.0 {
                    $(
                        lingua::Language::$variant => atoms::$atom().encode(env),
                    )*
                }
            }
        }
    };
}

impl_language_codec! {
    afrikaans => Afrikaans,
    albanian => Albanian,
    arabic => Arabic,
    armenian => Armenian,
    azerbaijani => Azerbaijani,
    basque => Basque,
    belarusian => Belarusian,
    bengali => Bengali,
    bokmal => Bokmal,
    bosnian => Bosnian,
    bulgarian => Bulgarian,
    catalan => Catalan,
    chinese => Chinese,
    croatian => Croatian,
    czech => Czech,
    danish => Danish,
    dutch => Dutch,
    english => English,
    esperanto => Esperanto,
    estonian => Estonian,
    finnish => Finnish,
    french => French,
    ganda => Ganda,
    georgian => Georgian,
    german => German,
    greek => Greek,
    gujarati => Gujarati,
    hebrew => Hebrew,
    hindi => Hindi,
    hungarian => Hungarian,
    icelandic => Icelandic,
    indonesian => Indonesian,
    irish => Irish,
    italian => Italian,
    japanese => Japanese,
    kazakh => Kazakh,
    korean => Korean,
    latin => Latin,
    latvian => Latvian,
    lithuanian => Lithuanian,
    macedonian => Macedonian,
    malay => Malay,
    maori => Maori,
    marathi => Marathi,
    mongolian => Mongolian,
    nynorsk => Nynorsk,
    persian => Persian,
    polish => Polish,
    portuguese => Portuguese,
    punjabi => Punjabi,
    romanian => Romanian,
    russian => Russian,
    serbian => Serbian,
    shona => Shona,
    slovak => Slovak,
    slovene => Slovene,
    somali => Somali,
    sotho => Sotho,
    spanish => Spanish,
    swahili => Swahili,
    swedish => Swedish,
    tagalog => Tagalog,
    tamil => Tamil,
    telugu => Telugu,
    thai => Thai,
    tsonga => Tsonga,
    tswana => Tswana,
    turkish => Turkish,
    ukrainian => Ukrainian,
    urdu => Urdu,
    vietnamese => Vietnamese,
    welsh => Welsh,
    xhosa => Xhosa,
    yoruba => Yoruba,
    zulu => Zulu,
}

impl Deref for Language {
    type Target = lingua::Language;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
