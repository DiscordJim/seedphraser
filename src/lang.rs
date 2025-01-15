use std::io::{stdout, Write};

use base64::{prelude::{BASE64_STANDARD, BASE64_URL_SAFE}, Engine};
use bip39::{Language, Mnemonic};
use clap::{Arg, ArgMatches};

use crate::SeedPhraserError;

#[derive(Debug)]
pub enum OutputFormat {
    Text,
    Binary,
    Base64,
    Base64UrlSafe,
    Hex
}

impl OutputFormat {
    pub fn parse(name: &str, args: &ArgMatches) -> Result<Self, SeedPhraserError> {
        let value = args.get_one::<String>(name)
            .expect("This is always a required argument.");
        Self::try_from(&**value)
    }
    pub fn output(&self, mnemonic: Mnemonic) -> Result<(), SeedPhraserError> {
        match self {
            OutputFormat::Text => print!("{}", mnemonic.into_phrase()),
            OutputFormat::Binary => {
                stdout().write_all(mnemonic.entropy())?;
                stdout().flush()?;
            },
            OutputFormat::Base64 => print!("{}", BASE64_STANDARD.encode(mnemonic.entropy())),
            OutputFormat::Base64UrlSafe => print!("{}", BASE64_URL_SAFE.encode(mnemonic.entropy())),
            OutputFormat::Hex => print!("{}", hex::encode(mnemonic.entropy()))
        }
        Ok(())
    }
}

impl TryFrom<&str> for OutputFormat {
    type Error = SeedPhraserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "txt" => Self::Text,
            "bin" => Self::Binary,
            "b64" => Self::Base64,
            "b64url" => Self::Base64UrlSafe,
            "hex" => Self::Hex,
            _ => Err(SeedPhraserError::FailedToReadOutputFormat(format!("Unknown format \"{value}\".")))?
        })
    }
}

pub struct LanguageTool;

impl LanguageTool {
    pub fn parse(args: &ArgMatches) -> Result<Language, SeedPhraserError> {
        Ok(Self::lookup(args
            .get_one::<String>("language")
            .expect("This has a default option, should not fail here."))?)
    }
    pub fn lookup(code: &str) -> Result<Language, SeedPhraserError> {
        Ok(match &*code.to_lowercase() {
            "en" => Language::English,
            "zh-cn" => Language::ChineseSimplified,
            "zh-hant" => Language::ChineseTraditional,
            "fr" => Language::French,
            "it" => Language::Italian,
            "ja" => Language::Japanese,
            "es" => Language::Spanish,
            "ko" => Language::Korean,
            x => Err(SeedPhraserError::UnrecognizableLanguage(x.to_string()))?
        })
    }
}