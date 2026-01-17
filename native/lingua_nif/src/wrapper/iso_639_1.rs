use crate::atoms;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct IsoCode639_1(pub lingua::IsoCode639_1);

macro_rules! impl_iso_639_1_codec {
    ($($atom:ident => $variant:ident),* $(,)?) => {
        impl<'a> Decoder<'a> for IsoCode639_1 {
            fn decode(term: Term<'a>) -> NifResult<Self> {
                $(
                    if atoms::$atom() == term {
                        return Ok(IsoCode639_1(lingua::IsoCode639_1::$variant));
                    }
                )*
                Err(Error::BadArg)
            }
        }

        impl Encoder for IsoCode639_1 {
            fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
                match self.0 {
                    $(
                        lingua::IsoCode639_1::$variant => atoms::$atom().encode(env),
                    )*
                }
            }
        }
    };
}

impl_iso_639_1_codec! {
    af => AF,
    sq => SQ,
    ar => AR,
    hy => HY,
    az => AZ,
    eu => EU,
    be => BE,
    bn => BN,
    nb => NB,
    bs => BS,
    bg => BG,
    ca => CA,
    zh => ZH,
    hr => HR,
    cs => CS,
    da => DA,
    nl => NL,
    en => EN,
    eo => EO,
    et => ET,
    fi => FI,
    fr => FR,
    lg => LG,
    ka => KA,
    de => DE,
    el => EL,
    gu => GU,
    he => HE,
    hi => HI,
    hu => HU,
    is => IS,
    id => ID,
    ga => GA,
    it => IT,
    ja => JA,
    kk => KK,
    ko => KO,
    la => LA,
    lv => LV,
    lt => LT,
    mk => MK,
    ms => MS,
    mi => MI,
    mr => MR,
    mn => MN,
    nn => NN,
    fa => FA,
    pl => PL,
    pt => PT,
    pa => PA,
    ro => RO,
    ru => RU,
    sr => SR,
    sn => SN,
    sk => SK,
    sl => SL,
    so => SO,
    st => ST,
    es => ES,
    sw => SW,
    sv => SV,
    tl => TL,
    ta => TA,
    te => TE,
    th => TH,
    ts => TS,
    tn => TN,
    tr => TR,
    uk => UK,
    ur => UR,
    vi => VI,
    cy => CY,
    xh => XH,
    yo => YO,
    zu => ZU,
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
