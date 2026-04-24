#![allow(unused_imports)]

use unity2::ClassIdentity;

#[unity2::enumeration(namespace = "App", name = "Language.Langs")]
#[repr(i32)]
pub enum Langs {
    JpJapanese = 0,
    UsEnglish = 1,
    UsFrench = 2,
    UsSpanish = 3,
    EuEnglish = 4,
    EuFrench = 5,
    EuSpanish = 6,
    EuGerman = 7,
    EuItalian = 8,
    CnTraditional = 9,
    CnSimplified = 10,
    KrKorean = 11,
}

#[unity2::enumeration(namespace = "App", name = "Language.Voices")]
#[repr(i32)]
pub enum Voices {
    Japanese = 0,
    English = 1,
}

#[unity2::class(namespace = "App")]
pub struct Language {}

#[unity2::methods]
impl Language {
    #[method(name = "GetLang", args = 0)]
    pub fn get_lang() -> Langs;

    #[method(name = "SetLang", args = 1)]
    pub fn set_lang(lang: Langs);

    #[method(name = "ReflectSetting", args = 0)]
    pub fn reflect_setting();
}
