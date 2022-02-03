use clap::{App, Arg};

// command line
pub fn cli() -> clap::ArgMatches<'static> {
    let matches = App::new("XML fmt")
        .args(&[
            Arg::with_name("input")
                .required(true)
                .index(1)
                .help("the input file to use"),
            Arg::with_name("output")
                .short("o")
                .required(true)
                .takes_value(true)
                .help("the output file to use"),
        ])
        .get_matches();
    matches
}