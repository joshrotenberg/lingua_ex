use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct Language(pub lingua::Language);

impl<'a> Decoder<'a> for Language {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if atoms::afrikaans() == term {
            Ok(Language(lingua::Language::Afrikaans))
        } else if atoms::albanian() == term {
            Ok(Language(lingua::Language::Albanian))
        } else if atoms::arabic() == term {
            Ok(Language(lingua::Language::Arabic))
        } else if atoms::armenian() == term {
            Ok(Language(lingua::Language::Armenian))
        } else if atoms::azerbaijani() == term {
            Ok(Language(lingua::Language::Azerbaijani))
        } else if atoms::basque() == term {
            Ok(Language(lingua::Language::Basque))
        } else if atoms::belarusian() == term {
            Ok(Language(lingua::Language::Belarusian))
        } else if atoms::bengali() == term {
            Ok(Language(lingua::Language::Bengali))
        } else if atoms::bokmal() == term {
            Ok(Language(lingua::Language::Bokmal))
        } else if atoms::bosnian() == term {
            Ok(Language(lingua::Language::Bosnian))
        } else if atoms::bulgarian() == term {
            Ok(Language(lingua::Language::Bulgarian))
        } else if atoms::catalan() == term {
            Ok(Language(lingua::Language::Catalan))
        } else if atoms::chinese() == term {
            Ok(Language(lingua::Language::Chinese))
        } else if atoms::croatian() == term {
            Ok(Language(lingua::Language::Croatian))
        } else if atoms::czech() == term {
            Ok(Language(lingua::Language::Czech))
        } else if atoms::danish() == term {
            Ok(Language(lingua::Language::Danish))
        } else if atoms::dutch() == term {
            Ok(Language(lingua::Language::Dutch))
        } else if atoms::english() == term {
            Ok(Language(lingua::Language::English))
        } else if atoms::esperanto() == term {
            Ok(Language(lingua::Language::Esperanto))
        } else if atoms::estonian() == term {
            Ok(Language(lingua::Language::Estonian))
        } else if atoms::finnish() == term {
            Ok(Language(lingua::Language::Finnish))
        } else if atoms::french() == term {
            Ok(Language(lingua::Language::French))
        } else if atoms::ganda() == term {
            Ok(Language(lingua::Language::Ganda))
        } else if atoms::georgian() == term {
            Ok(Language(lingua::Language::Georgian))
        } else if atoms::german() == term {
            Ok(Language(lingua::Language::German))
        } else if atoms::greek() == term {
            Ok(Language(lingua::Language::Greek))
        } else if atoms::gujarati() == term {
            Ok(Language(lingua::Language::Gujarati))
        } else if atoms::hebrew() == term {
            Ok(Language(lingua::Language::Hebrew))
        } else if atoms::hindi() == term {
            Ok(Language(lingua::Language::Hindi))
        } else if atoms::hungarian() == term {
            Ok(Language(lingua::Language::Hungarian))
        } else if atoms::icelandic() == term {
            Ok(Language(lingua::Language::Icelandic))
        } else if atoms::indonesian() == term {
            Ok(Language(lingua::Language::Indonesian))
        } else if atoms::irish() == term {
            Ok(Language(lingua::Language::Irish))
        } else if atoms::italian() == term {
            Ok(Language(lingua::Language::Italian))
        } else if atoms::japanese() == term {
            Ok(Language(lingua::Language::Japanese))
        } else if atoms::kazakh() == term {
            Ok(Language(lingua::Language::Kazakh))
        } else if atoms::korean() == term {
            Ok(Language(lingua::Language::Korean))
        } else if atoms::latin() == term {
            Ok(Language(lingua::Language::Latin))
        } else if atoms::latvian() == term {
            Ok(Language(lingua::Language::Latvian))
        } else if atoms::macedonian() == term {
            Ok(Language(lingua::Language::Macedonian))
        } else if atoms::malay() == term {
            Ok(Language(lingua::Language::Malay))
        } else if atoms::marathi() == term {
            Ok(Language(lingua::Language::Marathi))
        } else if atoms::mongolian() == term {
            Ok(Language(lingua::Language::Mongolian))
        } else if atoms::nynorsk() == term {
            Ok(Language(lingua::Language::Nynorsk))
        } else if atoms::persian() == term {
            Ok(Language(lingua::Language::Persian))
        } else if atoms::polish() == term {
            Ok(Language(lingua::Language::Polish))
        } else if atoms::portuguese() == term {
            Ok(Language(lingua::Language::Portuguese))
        } else if atoms::punjabi() == term {
            Ok(Language(lingua::Language::Punjabi))
        } else if atoms::romanian() == term {
            Ok(Language(lingua::Language::Romanian))
        } else if atoms::russian() == term {
            Ok(Language(lingua::Language::Russian))
        } else if atoms::serbian() == term {
            Ok(Language(lingua::Language::Serbian))
        } else if atoms::shona() == term {
            Ok(Language(lingua::Language::Shona))
        } else if atoms::slovak() == term {
            Ok(Language(lingua::Language::Slovak))
        } else if atoms::slovene() == term {
            Ok(Language(lingua::Language::Slovene))
        } else if atoms::somali() == term {
            Ok(Language(lingua::Language::Somali))
        } else if atoms::sotho() == term {
            Ok(Language(lingua::Language::Sotho))
        } else if atoms::spanish() == term {
            Ok(Language(lingua::Language::Spanish))
        } else if atoms::swahili() == term {
            Ok(Language(lingua::Language::Swahili))
        } else if atoms::swedish() == term {
            Ok(Language(lingua::Language::Swedish))
        } else if atoms::tagalog() == term {
            Ok(Language(lingua::Language::Tagalog))
        } else if atoms::tamil() == term {
            Ok(Language(lingua::Language::Tamil))
        } else if atoms::telugu() == term {
            Ok(Language(lingua::Language::Telugu))
        } else if atoms::thai() == term {
            Ok(Language(lingua::Language::Thai))
        } else if atoms::tsonga() == term {
            Ok(Language(lingua::Language::Tsonga))
        } else if atoms::tswana() == term {
            Ok(Language(lingua::Language::Tswana))
        } else if atoms::turkish() == term {
            Ok(Language(lingua::Language::Turkish))
        } else if atoms::ukrainian() == term {
            Ok(Language(lingua::Language::Ukrainian))
        } else if atoms::urdu() == term {
            Ok(Language(lingua::Language::Urdu))
        } else if atoms::vietnamese() == term {
            Ok(Language(lingua::Language::Vietnamese))
        } else if atoms::welsh() == term {
            Ok(Language(lingua::Language::Welsh))
        } else if atoms::xhosa() == term {
            Ok(Language(lingua::Language::Xhosa))
        } else if atoms::yoruba() == term {
            Ok(Language(lingua::Language::Yoruba))
        } else if atoms::zulu() == term {
            Ok(Language(lingua::Language::Zulu))
        } else {
            Err(Error::BadArg)
        }
    }
}

impl Encoder for Language {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            Language(lingua::Language::Afrikaans) => atoms::afrikaans().encode(env),
            Language(lingua::Language::Albanian) => atoms::albanian().encode(env),
            Language(lingua::Language::Arabic) => atoms::arabic().encode(env),
            Language(lingua::Language::Armenian) => atoms::armenian().encode(env),
            Language(lingua::Language::Azerbaijani) => atoms::azerbaijani().encode(env),
            Language(lingua::Language::Basque) => atoms::basque().encode(env),
            Language(lingua::Language::Belarusian) => atoms::belarusian().encode(env),
            Language(lingua::Language::Bengali) => atoms::bengali().encode(env),
            Language(lingua::Language::Bokmal) => atoms::bokmal().encode(env),
            Language(lingua::Language::Bosnian) => atoms::bosnian().encode(env),
            Language(lingua::Language::Bulgarian) => atoms::bulgarian().encode(env),
            Language(lingua::Language::Catalan) => atoms::catalan().encode(env),
            Language(lingua::Language::Chinese) => atoms::chinese().encode(env),
            Language(lingua::Language::Croatian) => atoms::croatian().encode(env),
            Language(lingua::Language::Czech) => atoms::czech().encode(env),
            Language(lingua::Language::Danish) => atoms::danish().encode(env),
            Language(lingua::Language::Dutch) => atoms::dutch().encode(env),
            Language(lingua::Language::English) => atoms::english().encode(env),
            Language(lingua::Language::Esperanto) => atoms::esperanto().encode(env),
            Language(lingua::Language::Estonian) => atoms::estonian().encode(env),
            Language(lingua::Language::Finnish) => atoms::finnish().encode(env),
            Language(lingua::Language::French) => atoms::french().encode(env),
            Language(lingua::Language::Ganda) => atoms::ganda().encode(env),
            Language(lingua::Language::Georgian) => atoms::georgian().encode(env),
            Language(lingua::Language::German) => atoms::german().encode(env),
            Language(lingua::Language::Greek) => atoms::greek().encode(env),
            Language(lingua::Language::Gujarati) => atoms::gujarati().encode(env),
            Language(lingua::Language::Hebrew) => atoms::hebrew().encode(env),
            Language(lingua::Language::Hindi) => atoms::hindi().encode(env),
            Language(lingua::Language::Hungarian) => atoms::hungarian().encode(env),
            Language(lingua::Language::Icelandic) => atoms::icelandic().encode(env),
            Language(lingua::Language::Indonesian) => atoms::indonesian().encode(env),
            Language(lingua::Language::Irish) => atoms::irish().encode(env),
            Language(lingua::Language::Italian) => atoms::italian().encode(env),
            Language(lingua::Language::Japanese) => atoms::japanese().encode(env),
            Language(lingua::Language::Kazakh) => atoms::kazakh().encode(env),
            Language(lingua::Language::Korean) => atoms::korean().encode(env),
            Language(lingua::Language::Latin) => atoms::latin().encode(env),
            Language(lingua::Language::Latvian) => atoms::latvian().encode(env),
            Language(lingua::Language::Lithuanian) => atoms::lithuanian().encode(env),
            Language(lingua::Language::Macedonian) => atoms::macedonian().encode(env),
            Language(lingua::Language::Malay) => atoms::malay().encode(env),
            Language(lingua::Language::Marathi) => atoms::marathi().encode(env),
            Language(lingua::Language::Mongolian) => atoms::mongolian().encode(env),
            Language(lingua::Language::Nynorsk) => atoms::nynorsk().encode(env),
            Language(lingua::Language::Persian) => atoms::persian().encode(env),
            Language(lingua::Language::Polish) => atoms::polish().encode(env),
            Language(lingua::Language::Portuguese) => atoms::portuguese().encode(env),
            Language(lingua::Language::Punjabi) => atoms::punjabi().encode(env),
            Language(lingua::Language::Romanian) => atoms::romanian().encode(env),
            Language(lingua::Language::Russian) => atoms::russian().encode(env),
            Language(lingua::Language::Serbian) => atoms::serbian().encode(env),
            Language(lingua::Language::Shona) => atoms::shona().encode(env),
            Language(lingua::Language::Slovak) => atoms::slovak().encode(env),
            Language(lingua::Language::Slovene) => atoms::slovene().encode(env),
            Language(lingua::Language::Somali) => atoms::somali().encode(env),
            Language(lingua::Language::Sotho) => atoms::sotho().encode(env),
            Language(lingua::Language::Spanish) => atoms::spanish().encode(env),
            Language(lingua::Language::Swahili) => atoms::swahili().encode(env),
            Language(lingua::Language::Swedish) => atoms::swedish().encode(env),
            Language(lingua::Language::Tagalog) => atoms::tagalog().encode(env),
            Language(lingua::Language::Tamil) => atoms::tamil().encode(env),
            Language(lingua::Language::Telugu) => atoms::telugu().encode(env),
            Language(lingua::Language::Thai) => atoms::thai().encode(env),
            Language(lingua::Language::Tsonga) => atoms::tsonga().encode(env),
            Language(lingua::Language::Tswana) => atoms::tswana().encode(env),
            Language(lingua::Language::Turkish) => atoms::turkish().encode(env),
            Language(lingua::Language::Ukrainian) => atoms::ukrainian().encode(env),
            Language(lingua::Language::Urdu) => atoms::urdu().encode(env),
            Language(lingua::Language::Vietnamese) => atoms::vietnamese().encode(env),
            Language(lingua::Language::Welsh) => atoms::welsh().encode(env),
            Language(lingua::Language::Xhosa) => atoms::xhosa().encode(env),
            Language(lingua::Language::Yoruba) => atoms::yoruba().encode(env),
            Language(lingua::Language::Zulu) => atoms::zulu().encode(env),
        }
    }
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
