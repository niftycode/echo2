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
        // TODO: Add subcommand here

        // .arg(Arg::new("no_newline")
        //     .short('n')
        //     .action(clap::ArgAction::SetFalse) // no value is allowed
        //     .help("Do not print the trailing newline character.")
        // )


        .after_help("This is the Rust version of the well known echo command.")
        .get_matches();

    // println!("{:#?}", matches);

    if let Some(text) = matches.get_one::<String>("text") {
        println!("Value for the user's input: {text}");
    }



    /*
    if let Some(matches) = matches.subcommand_matches("text") {
        if matches.get_flag("no_newline") {
            println!("Add no new line...");
        } else {
            println!("Add new line...");
        }
    }
     */

}
