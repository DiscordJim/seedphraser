use std::{num::ParseIntError, str::Utf8Error};

use base64::DecodeError;
use hex::FromHexError;

#[derive(Debug)]
pub enum SeedPhraserError {
    MissingArgument(String),
    FailedParsingArguments(String),
    UnrecognizableLanguage(String),
    FailedGeneratingMnemonic(String),
    FailedToReadOutputFormat(String),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    Base64Error(base64::DecodeError),
    HexDecodeError(hex::FromHexError),
    ZeroBitMnemonic,
    BitsNotDivisibleByEight,
    BadSequenceLength,
    StdinIsTerminal
}

impl From<FromHexError> for SeedPhraserError {
    fn from(value: FromHexError) -> Self {
        Self::HexDecodeError(value)
    }
}

impl From<DecodeError> for SeedPhraserError {
    fn from(value: DecodeError) -> Self {
        Self::Base64Error(value)
    }
}


impl From<Utf8Error> for SeedPhraserError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<ParseIntError> for SeedPhraserError {
    fn from(value: ParseIntError) -> Self {
        Self::FailedParsingArguments(value.to_string())
    }
}

impl From<bip39::ErrorKind> for SeedPhraserError {
    fn from(value: bip39::ErrorKind) -> Self {
        Self::FailedGeneratingMnemonic(value.to_string())
    }
}

impl From<std::io::Error> for SeedPhraserError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
