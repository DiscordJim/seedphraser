use std::io::{stdout, Write};

use base64::{prelude::{BASE64_STANDARD, BASE64_URL_SAFE}, Engine};
use bip39::{Language, Mnemonic, MnemonicType};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{error::SeedPhraserError, lang::IoFormat};


#[derive(Clone)]
pub struct AdvancedMnemonic {
    generators: Vec<Mnemonic>,
    trim: usize,
    padding: bool
}


/// Determines if a mnemonic sequence is of a
/// valid length.
fn is_valid_length(bits: usize) -> bool {
    if [128,160,192,224,256].contains(&bits) {
        // one of the standard lengths.
        true
    } else if bits >= 256 && bits % 256 == 0 {
        // possibly valid long sequence.
        true
    } else {
        // invalid
        false
    }
}

/// Calculates how many bits we need to pad the sequence
/// to a valid size.
fn calculate_mnemonic_pad(bits: usize) -> usize {
    if bits <= 128 {
        128 - bits
    } else if bits <= 160 {
        160 - bits
    } else if bits <= 192 {
        192 - bits
    } else if bits <= 224 {
        224 - bits
    } else if bits <= 256 {
        256 - bits
    } else {
        256 - (bits - (256 * (bits / 256)))
    }
}

impl AdvancedMnemonic {
    /// Generates a new mnemonic phrase, optionally using padding to pad the output.
    pub fn generate(mut bits: usize, lang: Language, padding: bool) -> Result<Self, SeedPhraserError> {
        if bits == 0 {
            return Err(SeedPhraserError::ZeroBitMnemonic);
        }
        if bits % 8 != 0 {
            return Err(SeedPhraserError::BitsNotDivisibleByEight);
        }

        // This calculates the trim length if we are using padding.
        let trim = if padding {
            calculate_mnemonic_pad(bits)
        } else { 0 };

       
        // correct the bits amount and generate.
        bits += trim;

        if bits <= 256 {
            Ok(Self {
                generators: vec![Mnemonic::new(MnemonicType::for_key_size(bits)?, lang)],
                trim,
                padding
            })
        } else {
            let mut mnomics = vec![];
            while bits >= 256 {
                mnomics.push(Mnemonic::new(MnemonicType::for_key_size(256.min(bits))?, lang));
                bits -= 256;
            }
            Ok(Self {
                generators: mnomics,
                trim,
                padding
            })
        }
    }
    /// Reads a sequence out from the raw bytes.
    pub fn from_entropy(
        // The actual byte sequence.
        bytes: &[u8],
        // The language.
        lang: Language,
        // If we are allowed to pad this sequence.
        can_re_pad: bool
    ) -> Result<Self, SeedPhraserError> {

        // Calculate the current bits.
        let mut repad = 0;
        let current_bits = bytes.len() * 8;


        // We are working with a long sequence.
        if !is_valid_length(current_bits) && !can_re_pad {
            // Bad length and we cannot repad it.
            Err(SeedPhraserError::BadSequenceLength)?;
        } else {
            // Bad length but we can repad.
            repad = calculate_mnemonic_pad(current_bits);
        }

        // Make a new buffer to draw from.
        let mut buffer = bytes.to_vec();
        // println!("Buffer: {:?}", buffer.len());
        buffer.append(&mut vec![0u8; repad / 8]);

        
       


        let mut generators = vec![];

        // Now let us form the sequence
        if buffer.len() <= 32 {
            generators.push(Mnemonic::from_entropy(&buffer[..32.min(buffer.len())], lang)?);
        } else {
            let mut cursor = 0;
            while cursor + 32 <= buffer.len() {
                generators.push(Mnemonic::from_entropy(&buffer[cursor..cursor + 32], lang)?);
                cursor += 32;
            }
        }


        
        Ok(Self {
            generators,
            padding: repad != 0,
            trim: repad
        })
    }
    pub fn from_phrase(text: &str, lang: Language) -> Result<Self, SeedPhraserError> {
        let mut text = text.trim();
        static TRIM_EXTRACTOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<main>.*) @(?<trim>[0-9]*)").unwrap());


        // Check if there is a trim on the sequence.
        let trim = match TRIM_EXTRACTOR.captures(text) {
            Some(num) => {
                text = num.name("main").unwrap().as_str();
                str::parse::<usize>(num.name("trim").unwrap().as_str())?
            },
            None => 0
        };

        


        // Calculate how many words are contained within the phrase.
        let word_count = text.split(' ').count();

        // Make a list of all the mnemonics
        let mut generators = vec![];
        if word_count > 24 {
            // This means we are working with a long mnemonic.
            let word_list = text.split(' ').collect::<Vec<&str>>();
            let mut cursor = 0;
            while cursor < word_count {
                let frase = word_list[cursor..cursor + 24].join(" ");
                generators.push(Mnemonic::from_phrase(&frase, lang)?);
                cursor += 24;
            }
        } else {
            // Can fit in a single phrase.
            generators.push(Mnemonic::from_phrase(text, lang)?);
        }

        Ok(Self {
            generators,
            trim: trim * 8,
            padding: trim != 0
        })
    }
    /// Will output the advanced mnemonic using the
    /// formatting specified.
    pub fn output(self, format: IoFormat) -> Result<(), SeedPhraserError> {
        if format.is_text() {
            print!("{}", self.to_string())
        } else {
            match format {
                IoFormat::Text => unreachable!(),
                IoFormat::Base64 => print!("{}", BASE64_STANDARD.encode(self.to_vec())),
                IoFormat::Base64UrlSafe => print!("{}", BASE64_URL_SAFE.encode(self.to_vec())),
                IoFormat::Hex => print!("{}", hex::encode(self.to_vec())),
                IoFormat::Binary => {
                    stdout().write_all(&self.to_vec())?;
                    stdout().flush()?;
                } 
            }
        }
        Ok(())
    }
    /// Prints the mnemonic out to a string.
    /// 
    /// If it is a long mnenomic that has padding,
    /// this will include the padding amount.
    pub fn to_string(self) -> String {
        let total = self.generators.len();
        let mut buffer = String::new();
        for (index, m) in self.generators.into_iter().enumerate() {
            buffer.push_str(&m.into_phrase());
            if index != total - 1 {
                buffer.push(' ');
            }
        }
        if self.padding && self.trim != 0 {
            buffer.push_str(&format!(" @{}", self.trim / 8));
        }
        buffer
    }
    pub fn to_vec(self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for m in &self.generators {
            buffer.extend_from_slice(m.entropy());
        }
        if self.padding {
            // Trim the mnemonic to the padding length.
            println!("{} {}", self.trim, self.padding);
            buffer.drain(buffer.len() - (self.trim / 8)..);
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use bip39::Language;


    use super::AdvancedMnemonic;


    #[test]
    pub fn normal_size_sequence_without_padding() {
        for bits in [128, 160, 192, 224, 256, 512, 1024, 2048] {
            let original = AdvancedMnemonic::generate(bits, Language::English, false).unwrap();
            let o_bytes = original.clone().to_vec();
            let o_string = original.clone().to_string();
    
            let from_phrase = AdvancedMnemonic::from_phrase(&o_string, Language::English).unwrap();
            assert_eq!(o_bytes, from_phrase.to_vec());

            let from_entropy = AdvancedMnemonic::from_entropy(&o_bytes, Language::English, false).unwrap();
            assert_eq!(o_bytes, from_entropy.to_vec());
        }
    }

    #[test]
    pub fn normal_size_sequence_with_padding() {
        for bits in 1..128 {
            let original = AdvancedMnemonic::generate(bits * 8, Language::English, true).unwrap();
            let o_bytes = original.clone().to_vec();
            let o_string = original.clone().to_string();

            let from_phrase = AdvancedMnemonic::from_phrase(&o_string, Language::English).unwrap();
            assert_eq!(o_bytes, from_phrase.to_vec());

      
            let from_entropy = AdvancedMnemonic::from_entropy(&o_bytes, Language::English, true).unwrap();
            assert_eq!(o_bytes, from_entropy.to_vec());
        }
    }
}