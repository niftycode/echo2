use clap::{Arg, Command};

fn main() {
    let _matches = Command::new("echo2")
        .author("Bodo Schönfeld, bodo.schoenfeld@niftycode.de")
        .version("0.1.0")
        .about("Display the text entered")
        .arg(Arg::new("in_file"))
        .after_help("This is the Rust version of the well known echo command.")
        .get_matches();
}

