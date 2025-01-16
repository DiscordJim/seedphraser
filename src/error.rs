use std::num::ParseIntError;
use thiserror::Error;

/// The error type for this command line utility.
#[derive(Error, Debug)]
pub enum SeedPhraserError {

    #[error("Failed to parse an integer in the sequence.")]
    FailedToParseInteger(#[from] ParseIntError),

    #[error("Missing an argument that should have been passed to the command line tool.")]
    MissingArgument(String),

    #[error("Failed parsing a string.")]
    FailedParsingArguments(String),

    #[error("Could not recognize the language code passed into the program.")]
    UnrecognizableLanguage(String),

    #[error("Failed to generate the mnemonic phrase.")]
    FailedGeneratingMnemonic(#[from] bip39::ErrorKind),

    #[error("Failed to read a format for output/input.")]
    FailedToReadOutputFormat(String),

    #[error("IOError")]
    IOError(#[from] std::io::Error),

    #[error("UTF8Error")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Failed decoding Base64")]
    Base64Error(#[from] base64::DecodeError),


    #[error("Failed decoding hexadecimal.")]
    HexDecodeError(#[from] hex::FromHexError),

    #[error("Zero bits is not an acceptable secret size.")]
    ZeroBitMnemonic,

    #[error("The bits specified are not divisible by eight.")]
    BitsNotDivisibleByEight,

    #[error("The sequence length is innapropriate. This usually happens when padding is turned off.")]
    BadSequenceLength,

    #[error("No standard input was detected.")]
    StdinIsTerminal,

    #[error("No argument was specified when there should have been one specified.")]
    NoArgumentSpecified
}

