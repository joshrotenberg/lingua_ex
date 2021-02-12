use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

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

impl Encoder for IsoCode639_3 {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            IsoCode639_3(lingua::IsoCode639_3::AFR) => atoms::afr().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SQI) => atoms::sqi().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ARA) => atoms::ara().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::HYE) => atoms::hye().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::AZE) => atoms::aze().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::EUS) => atoms::eus().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::BEL) => atoms::bel().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::BEN) => atoms::ben().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::NOB) => atoms::nob().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::BOS) => atoms::bos().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::BUL) => atoms::bul().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::CAT) => atoms::cat().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ZHO) => atoms::zho().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::HRV) => atoms::hrv().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::CES) => atoms::ces().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::DAN) => atoms::dan().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::NLD) => atoms::nld().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ENG) => atoms::eng().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::EPO) => atoms::epo().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::EST) => atoms::est().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::FIN) => atoms::fin().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::FRA) => atoms::fra().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::LUG) => atoms::lug().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::KAT) => atoms::kat().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::DEU) => atoms::deu().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ELL) => atoms::ell().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::GUJ) => atoms::guj().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::HEB) => atoms::heb().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::HIN) => atoms::hin().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::HUN) => atoms::hun().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ISL) => atoms::isl().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::IND) => atoms::ind().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::GLE) => atoms::gle().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ITA) => atoms::ita().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::JPN) => atoms::jpn().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::KAZ) => atoms::kaz().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::KOR) => atoms::kor().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::LAT) => atoms::lat().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::LAV) => atoms::lav().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::LIT) => atoms::lit().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::MKD) => atoms::mkd().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::MSA) => atoms::msa().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::MRI) => atoms::mri().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::MAR) => atoms::mar().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::MON) => atoms::mon().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::NNO) => atoms::nno().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::FAS) => atoms::fas().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::POL) => atoms::pol().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::POR) => atoms::por().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::PAN) => atoms::pan().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::RON) => atoms::ron().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::RUS) => atoms::rus().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SRP) => atoms::srp().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SNA) => atoms::sna().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SLK) => atoms::slk().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SLV) => atoms::slv().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SOM) => atoms::som().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SOT) => atoms::sot().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SPA) => atoms::spa().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SWA) => atoms::swa().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::SWE) => atoms::swe().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TGL) => atoms::tgl().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TAM) => atoms::tam().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TEL) => atoms::tel().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::THA) => atoms::tha().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TSO) => atoms::tso().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TSN) => atoms::tsn().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::TUR) => atoms::tur().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::UKR) => atoms::ukr().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::URD) => atoms::urd().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::VIE) => atoms::vie().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::CYM) => atoms::cym().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::XHO) => atoms::xho().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::YOR) => atoms::yor().encode(env),
            IsoCode639_3(lingua::IsoCode639_3::ZUL) => atoms::zul().encode(env),
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
