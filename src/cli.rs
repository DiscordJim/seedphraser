use clap::{Arg, Command};



pub fn create() -> Command {
    Command::new("seedphraser")
        .version(env!("CARGO_PKG_VERSION"))
        .about("For secret management")
        .subcommand_required(true)
        .subcommand(Command::new("generate")
            .about("Generates a new seed phrase from entropy.")
            .arg(Arg::new("language")
                    .short('l')
                    .help("The ISO 639-1 language code of the seed phrase that should be generated.")
                    .default_value("en")
                    .required(false)
            )
            .arg(Arg::new("bits")
                .short('b')
                .help("The amount of bits the seed phrase should encode.")
                .required(true)
            )
            .arg(Arg::new("nopad")
                .help("Prevents the tool from performing padding.")
                .default_missing_value("false")
            )
            .arg(Arg::new("output")
                .short('o')
                .help("The output format. [txt, bin, b64, b64url, hex]")
                .default_value("txt")
            )
        )
        .subcommand(Command::new("decode")
            .about("Decode a seedphrase")
            .arg(Arg::new("language")
                .short('l')
                .help("The ISO 639-1 language code of the language.")
                .default_value("en")
            )
            .arg(Arg::new("input")
                .short('i')
                .help("The input format.")
                .default_value("txt")
            )
            .arg(Arg::new("output")
                .short('o')
                .help("The output format.")
                .default_value("bin")
            )
            .arg(Arg::new("nopad")
                .help("Prevents the tool from performing padding.")
            )
            .arg(Arg::new("phrase")
                .short('p')
                .help("Decode the phrase in the command line instead of reading from stdin")
                .required(false)
            )
        )
}