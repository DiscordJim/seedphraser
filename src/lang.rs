use std::io::{stdout, Write};

use base64::{prelude::{BASE64_STANDARD, BASE64_URL_SAFE}, Engine};
use bip39::{Language, Mnemonic};
use clap::ArgMatches;

use crate::error::SeedPhraserError;



#[derive(Debug)]
pub enum IoFormat {
    Text,
    Binary,
    Base64,
    Base64UrlSafe,
    Hex
}

impl IoFormat {
    /// If the formatting to be used is just
    /// the seed phrase or not.
    pub fn is_text(&self) -> bool {
        match self {
            IoFormat::Text => true,
            _ => false
        }
    }
    /// Parses an [IoFormat] from text.
    pub fn parse(name: &str, args: &ArgMatches) -> Result<Self, SeedPhraserError> {
        let value = args.get_one::<String>(name)
            .expect("This is always a required argument.");
        Self::try_from(&**value)
    }
    /// Outputs a [Mnemonic] using the currently selected format.
    pub fn output(&self, mnemonic: Mnemonic) -> Result<(), SeedPhraserError> {
        match self {
            IoFormat::Text => print!("{}", mnemonic.into_phrase()),
            IoFormat::Binary => {
                stdout().write_all(mnemonic.entropy())?;
                stdout().flush()?;
            },
            IoFormat::Base64 => print!("{}", BASE64_STANDARD.encode(mnemonic.entropy())),
            IoFormat::Base64UrlSafe => print!("{}", BASE64_URL_SAFE.encode(mnemonic.entropy())),
            IoFormat::Hex => print!("{}", hex::encode(mnemonic.entropy()))
        }
        Ok(())
    }
}

impl TryFrom<&str> for IoFormat {
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

/// Contains helper methods that allow us to easily deal with [Language].
pub struct LanguageTool;

impl LanguageTool {
    /// Parses out the [Language] from the arguments.
    pub fn parse(args: &ArgMatches) -> Result<Language, SeedPhraserError> {
        Ok(Self::lookup(args
            .get_one::<String>("language")
            .expect("This has a default option, should not fail here."))?)
    }
    /// Looks up the code from a string and returns a [Language].
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