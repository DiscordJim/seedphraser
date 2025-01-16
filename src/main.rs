use std::io::{stdin, IsTerminal, Read, Stdin};

use base64::{prelude::{BASE64_STANDARD, BASE64_URL_SAFE}, Engine};
use clap::ArgMatches;
use cli::create;
use error::SeedPhraserError;
use gen::AdvancedMnemonic;
use lang::{LanguageTool, IoFormat};

pub mod cli;
pub mod lang;
pub mod gen;
pub mod error;


/// Generates a new sequence.
pub fn generate(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let bits = sub_matches
        .get_one::<String>("bits")
        .map(|f| str::parse::<usize>(f))
        .expect("Required argument.")?;
    let output = IoFormat::parse("output", sub_matches)?;
    let should_pad = !sub_matches.get_one::<String>("nopad").is_some();

    // Generate a new mnemonic.
    AdvancedMnemonic::generate(bits, language, should_pad)?.output(output)?;
    Ok(())
}

/// Reads all the available stdin, terminating the stream
/// if none is available.
fn read_all_stdin() -> Result<Vec<u8>, SeedPhraserError> {
    if Stdin::is_terminal(&stdin()) {
        Err(SeedPhraserError::StdinIsTerminal)?;
    }

    // Read until we are out of bytes.
    let mut total = Vec::new();
    let mut buf = [0u8; 256];
    loop {
        let pos = stdin().read(&mut buf)?;
        if pos == 0 {
            break
        }
        total.extend_from_slice(&buf[..pos]);
    }
    Ok(total)
}

/// This handles the case where the phrase is passed in directly instead
/// of being read from STDIN.
pub fn decode_sequence_direct(
    phrase: &String,
    sub_matches: &ArgMatches
) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let output = IoFormat::parse("output", sub_matches)?;

    // Parse a mnemonic phrase.
    AdvancedMnemonic::from_phrase(phrase.trim(), language)?.output(output)?;
    Ok(())
}

/// Decode a sequence.
pub fn decode_sequence(sub_matches: &ArgMatches) -> Result<(), SeedPhraserError> {
    let language = LanguageTool::parse(sub_matches)?;
    let input = IoFormat::parse("input", sub_matches)?;
    let output = IoFormat::parse("output", sub_matches)?;
    let should_pad = !sub_matches.get_one::<String>("nopad").is_some();



    // Read the stdin.
    let buf = read_all_stdin()?;
    let buffer = &buf[..buf.len() - 2];

    // Read the input.
    let entropy = match input {
        IoFormat::Text => {
            AdvancedMnemonic::from_phrase(std::str::from_utf8(&buffer)?, language)?.output(output)?;
            return Ok(())
        },
        IoFormat::Base64 => BASE64_STANDARD.decode(buffer)?,
        IoFormat::Base64UrlSafe => BASE64_URL_SAFE.decode(buffer)?,
        IoFormat::Binary => buffer.to_vec(),
        IoFormat::Hex => hex::decode(buffer)?
    };

    // if we are reading in byte type data.
    AdvancedMnemonic::from_entropy(&entropy, language, should_pad)?.output(output)?;
    Ok(())
}

pub fn main() -> Result<(), SeedPhraserError> {

    let matches = create().get_matches();
    match matches.subcommand() {
        Some(("generate", sub_matches)) => generate(sub_matches)?,
        Some(("decode", sub_matches)) => match sub_matches.get_one::<String>("phrase") {
            Some(phrase) => decode_sequence_direct(phrase, sub_matches)?,
            None => decode_sequence(sub_matches)?
        },
        None => (),
        _ => (),
    }
    Ok(())
}
