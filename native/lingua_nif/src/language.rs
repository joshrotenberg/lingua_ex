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
        } else if atoms::maori() == term {
            Ok(Language(lingua::Language::Maori))
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
            Language(lingua::Language::Maori) => atoms::maori().encode(env),
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

#[derive(Debug)]
pub struct IsoCode639_1(pub lingua::IsoCode639_1);

impl<'a> Decoder<'a> for IsoCode639_1 {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if atoms::af() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::AF))
        } else if atoms::sq() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SQ))
        } else if atoms::ar() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::AR))
        } else if atoms::hy() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::HY))
        } else if atoms::az() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::AZ))
        } else if atoms::eu() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::EU))
        } else if atoms::be() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::BE))
        } else if atoms::bn() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::BN))
        } else if atoms::nb() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::NB))
        } else if atoms::bs() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::BS))
        } else if atoms::bg() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::BG))
        } else if atoms::ca() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::CA))
        } else if atoms::zh() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ZH))
        } else if atoms::hr() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::HR))
        } else if atoms::cs() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::CS))
        } else if atoms::da() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::DA))
        } else if atoms::nl() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::NL))
        } else if atoms::en() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::EN))
        } else if atoms::eo() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::EO))
        } else if atoms::et() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ET))
        } else if atoms::fi() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::FI))
        } else if atoms::fr() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::FR))
        } else if atoms::lg() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::LG))
        } else if atoms::ka() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::KA))
        } else if atoms::de() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::DE))
        } else if atoms::el() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::EL))
        } else if atoms::gu() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::GU))
        } else if atoms::he() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::HE))
        } else if atoms::hi() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::HI))
        } else if atoms::hu() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::HU))
        } else if atoms::is() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::IS))
        } else if atoms::id() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ID))
        } else if atoms::ga() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::GA))
        } else if atoms::it() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::IT))
        } else if atoms::ja() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::JA))
        } else if atoms::kk() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::KK))
        } else if atoms::ko() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::KO))
        } else if atoms::la() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::LA))
        } else if atoms::lv() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::LV))
        } else if atoms::lt() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::LT))
        } else if atoms::mk() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::MK))
        } else if atoms::ms() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::MS))
        } else if atoms::mi() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::MI))
        } else if atoms::mr() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::MR))
        } else if atoms::mn() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::MN))
        } else if atoms::nn() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::NN))
        } else if atoms::fa() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::FA))
        } else if atoms::pl() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::PL))
        } else if atoms::pt() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::PT))
        } else if atoms::pa() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::PA))
        } else if atoms::ro() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::RO))
        } else if atoms::ru() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::RU))
        } else if atoms::sr() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SR))
        } else if atoms::sn() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SN))
        } else if atoms::sk() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SK))
        } else if atoms::sl() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SL))
        } else if atoms::so() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SO))
        } else if atoms::st() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ST))
        } else if atoms::es() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ES))
        } else if atoms::sw() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SW))
        } else if atoms::sv() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::SV))
        } else if atoms::tl() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TL))
        } else if atoms::ta() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TA))
        } else if atoms::te() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TE))
        } else if atoms::th() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TH))
        } else if atoms::ts() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TS))
        } else if atoms::tn() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TN))
        } else if atoms::tr() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::TR))
        } else if atoms::uk() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::UK))
        } else if atoms::ur() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::UR))
        } else if atoms::vi() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::VI))
        } else if atoms::cy() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::CY))
        } else if atoms::xh() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::XH))
        } else if atoms::yo() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::YO))
        } else if atoms::zu() == term {
            Ok(IsoCode639_1(lingua::IsoCode639_1::ZU))
        } else {
            Err(Error::BadArg)
        }
    }
}

impl Deref for IsoCode639_1 {
    type Target = lingua::IsoCode639_1;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for IsoCode639_1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug)]
pub struct IsoCode639_3(pub lingua::IsoCode639_3);

impl<'a> Decoder<'a> for IsoCode639_3 {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if atoms::afr() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::AFR))
        } else if atoms::sqi() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SQI))
        } else if atoms::ara() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ARA))
        } else if atoms::hye() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::HYE))
        } else if atoms::aze() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::AZE))
        } else if atoms::eus() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::EUS))
        } else if atoms::bel() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::BEL))
        } else if atoms::ben() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::BEN))
        } else if atoms::nob() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::NOB))
        } else if atoms::bos() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::BOS))
        } else if atoms::bul() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::BUL))
        } else if atoms::cat() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::CAT))
        } else if atoms::zho() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ZHO))
        } else if atoms::hrv() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::HRV))
        } else if atoms::ces() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::CES))
        } else if atoms::dan() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::DAN))
        } else if atoms::nld() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::NLD))
        } else if atoms::eng() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ENG))
        } else if atoms::epo() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::EPO))
        } else if atoms::est() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::EST))
        } else if atoms::fin() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::FIN))
        } else if atoms::fra() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::FRA))
        } else if atoms::lug() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::LUG))
        } else if atoms::kat() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::KAT))
        } else if atoms::deu() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::DEU))
        } else if atoms::ell() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ELL))
        } else if atoms::guj() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::GUJ))
        } else if atoms::heb() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::HEB))
        } else if atoms::hin() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::HIN))
        } else if atoms::hun() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::HUN))
        } else if atoms::isl() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ISL))
        } else if atoms::ind() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::IND))
        } else if atoms::gle() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::GLE))
        } else if atoms::ita() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ITA))
        } else if atoms::jpn() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::JPN))
        } else if atoms::kaz() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::KAZ))
        } else if atoms::kor() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::KOR))
        } else if atoms::lat() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::LAT))
        } else if atoms::lav() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::LAV))
        } else if atoms::lit() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::LIT))
        } else if atoms::mkd() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::MKD))
        } else if atoms::msa() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::MSA))
        } else if atoms::mri() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::MRI))
        } else if atoms::mar() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::MAR))
        } else if atoms::mon() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::MON))
        } else if atoms::nno() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::NNO))
        } else if atoms::fas() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::FAS))
        } else if atoms::pol() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::POL))
        } else if atoms::por() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::POR))
        } else if atoms::pan() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::PAN))
        } else if atoms::ron() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::RON))
        } else if atoms::rus() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::RUS))
        } else if atoms::srp() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SRP))
        } else if atoms::sna() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SNA))
        } else if atoms::slk() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SLK))
        } else if atoms::slv() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SLV))
        } else if atoms::som() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SOM))
        } else if atoms::sot() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SOT))
        } else if atoms::spa() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SPA))
        } else if atoms::swa() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SWA))
        } else if atoms::swe() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::SWE))
        } else if atoms::tgl() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TGL))
        } else if atoms::tam() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TAM))
        } else if atoms::tel() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TEL))
        } else if atoms::tha() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::THA))
        } else if atoms::tso() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TSO))
        } else if atoms::tsn() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TSN))
        } else if atoms::tur() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::TUR))
        } else if atoms::ukr() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::UKR))
        } else if atoms::urd() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::URD))
        } else if atoms::vie() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::VIE))
        } else if atoms::cym() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::CYM))
        } else if atoms::xho() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::XHO))
        } else if atoms::yor() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::YOR))
        } else if atoms::zul() == term {
            Ok(IsoCode639_3(lingua::IsoCode639_3::ZUL))
        } else {
            Err(Error::BadArg)
        }
    }
}

impl Deref for IsoCode639_3 {
    type Target = lingua::IsoCode639_3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for IsoCode639_3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}