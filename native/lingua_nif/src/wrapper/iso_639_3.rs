use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct IsoCode639_3(pub lingua::IsoCode639_3);

macro_rules! impl_iso_639_3_codec {
    ($($atom:ident => $variant:ident),* $(,)?) => {
        impl<'a> Decoder<'a> for IsoCode639_3 {
            fn decode(term: Term<'a>) -> NifResult<Self> {
                $(
                    if atoms::$atom() == term {
                        return Ok(IsoCode639_3(lingua::IsoCode639_3::$variant));
                    }
                )*
                Err(Error::BadArg)
            }
        }

        impl Encoder for IsoCode639_3 {
            fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
                match self.0 {
                    $(
                        lingua::IsoCode639_3::$variant => atoms::$atom().encode(env),
                    )*
                }
            }
        }
    };
}

impl_iso_639_3_codec! {
    afr => AFR,
    sqi => SQI,
    ara => ARA,
    hye => HYE,
    aze => AZE,
    eus => EUS,
    bel => BEL,
    ben => BEN,
    nob => NOB,
    bos => BOS,
    bul => BUL,
    cat => CAT,
    zho => ZHO,
    hrv => HRV,
    ces => CES,
    dan => DAN,
    nld => NLD,
    eng => ENG,
    epo => EPO,
    est => EST,
    fin => FIN,
    fra => FRA,
    lug => LUG,
    kat => KAT,
    deu => DEU,
    ell => ELL,
    guj => GUJ,
    heb => HEB,
    hin => HIN,
    hun => HUN,
    isl => ISL,
    ind => IND,
    gle => GLE,
    ita => ITA,
    jpn => JPN,
    kaz => KAZ,
    kor => KOR,
    lat => LAT,
    lav => LAV,
    lit => LIT,
    mkd => MKD,
    msa => MSA,
    mri => MRI,
    mar => MAR,
    mon => MON,
    nno => NNO,
    fas => FAS,
    pol => POL,
    por => POR,
    pan => PAN,
    ron => RON,
    rus => RUS,
    srp => SRP,
    sna => SNA,
    slk => SLK,
    slv => SLV,
    som => SOM,
    sot => SOT,
    spa => SPA,
    swa => SWA,
    swe => SWE,
    tgl => TGL,
    tam => TAM,
    tel => TEL,
    tha => THA,
    tso => TSO,
    tsn => TSN,
    tur => TUR,
    ukr => UKR,
    urd => URD,
    vie => VIE,
    cym => CYM,
    xho => XHO,
    yor => YOR,
    zul => ZUL,
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
