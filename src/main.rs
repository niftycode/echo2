use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echo2")
        .author("Bodo Schönfeld, bodo.schoenfeld@niftycode.de")
        .version("0.1.0")
        .about("Write arguments to the standard output")
        .arg(Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)

        )
        .arg(Arg::new("no_newline")
            .short('n')
            .action(clap::ArgAction::SetFalse) // no value is allowed
            .help("Do not print the trailing newline character.")
        )
        .after_help("This is the Rust version of the well known echo command.")
        .get_matches();

    println!("{:#?}", matches);

    // TODO: Create program output
}
