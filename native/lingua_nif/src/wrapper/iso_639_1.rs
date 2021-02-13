use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

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

impl Encoder for IsoCode639_1 {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            IsoCode639_1(lingua::IsoCode639_1::AF) => atoms::af().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SQ) => atoms::sq().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::AR) => atoms::ar().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::HY) => atoms::hy().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::AZ) => atoms::az().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::EU) => atoms::eu().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::BE) => atoms::be().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::BN) => atoms::bn().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::NB) => atoms::nb().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::BS) => atoms::bs().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::BG) => atoms::bg().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::CA) => atoms::ca().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ZH) => atoms::zh().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::HR) => atoms::hr().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::CS) => atoms::cs().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::DA) => atoms::da().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::NL) => atoms::nl().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::EN) => atoms::en().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::EO) => atoms::eo().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ET) => atoms::et().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::FI) => atoms::fi().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::FR) => atoms::fr().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::LG) => atoms::lg().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::KA) => atoms::ka().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::DE) => atoms::de().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::EL) => atoms::el().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::GU) => atoms::gu().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::HE) => atoms::he().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::HI) => atoms::hi().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::HU) => atoms::hu().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::IS) => atoms::is().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ID) => atoms::id().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::GA) => atoms::ga().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::IT) => atoms::it().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::JA) => atoms::ja().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::KK) => atoms::kk().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::KO) => atoms::ko().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::LA) => atoms::la().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::LV) => atoms::lv().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::LT) => atoms::lt().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::MK) => atoms::mk().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::MS) => atoms::ms().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::MI) => atoms::mi().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::MR) => atoms::mr().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::MN) => atoms::mn().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::NN) => atoms::nn().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::FA) => atoms::fa().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::PL) => atoms::pl().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::PT) => atoms::pt().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::PA) => atoms::pa().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::RO) => atoms::ro().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::RU) => atoms::ru().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SR) => atoms::sr().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SN) => atoms::sn().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SK) => atoms::sk().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SL) => atoms::sl().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SO) => atoms::so().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ST) => atoms::st().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ES) => atoms::es().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SW) => atoms::sw().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::SV) => atoms::sv().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TL) => atoms::tl().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TA) => atoms::ta().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TE) => atoms::te().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TH) => atoms::th().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TS) => atoms::ts().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TN) => atoms::tn().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::TR) => atoms::tr().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::UK) => atoms::uk().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::UR) => atoms::ur().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::VI) => atoms::vi().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::CY) => atoms::cy().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::XH) => atoms::xh().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::YO) => atoms::yo().encode(env),
            IsoCode639_1(lingua::IsoCode639_1::ZU) => atoms::zu().encode(env),
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
