use clap::{App, Arg};
use regex::Regex;
use std::fs;
use std::io::{prelude::*, BufReader, BufWriter};
// use std::iter::IntoIterator;
// use iowrap::Eof;

pub fn main() {
    // cli
    let matches = App::new("XML Format")
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

    // get the files
    let out_path = matches.value_of("output").unwrap(); // unwrap OK because required arg
    let in_path = matches.value_of("input").unwrap(); // unwrap OK because required arg
                                                      // end cli

    // bufreading
    let f = fs::File::open(in_path).expect("Unable to open file");
    let f = BufReader::new(f);
    

    // xml string to be written out
    let mut xml_out: String = String::new();

    // version lines
    let begin_version = Regex::new(r"(\Q<?\E)").unwrap();
    let end_version = Regex::new(r"(\Q?>\E)").unwrap();
    let mut version: String = String::new();
    let mut in_version = false;

    // comments
    let begin_comments = Regex::new(r"(\Q<!--\E)").unwrap();
    let end_comments = Regex::new(r"(\Q-->\E)").unwrap();
    let comment_contents = Regex::new(r"/(?<=\Q<!--\E)(.*)(?=\s*\Q-->\E)/su").unwrap();
    let mut comments = String::new();
    let mut in_comments = false;

    // perl
    let begin_cdata = Regex::new(r"\(CDATA*\[)").unwrap(); // CDATA in line
    let end_cdata = Regex::new(r"\b]]>\b").unwrap(); // End in line
    let perl_without_cdata = Regex::new(r"(\QCDATA\E)|(\Q]]>\E)").unwrap(); // only perl
    let mut perl_code: String = String::new();
    let mut in_cdata = false;

    let somfin = &fs::read("address.txt").unwrap();
    let stuff: String = String::from_utf8_lossy(somfin).parse().unwrap();

    for n_line in stuff.chars() {
        // get string from result
        let line = n_line.expect("Unable to read line");

        // the following should not be else ifs in the
        // case someone writes everything on one line?
        // Unless I write regex's that include inside of things,
        // and regex's that include what's not inside of things.
        // TODO?

        // handle comment lines
        if begin_comments.is_match(&line) || in_comments {
            in_comments = true;
            comments = [comments[..].to_string(), line[..].to_string()].concat();
            if end_comments.is_match(&line) {
                // due to todo above- might need more specific regex to capture only comments on this line
                xml_out = ["<!-- ".to_string(), comment_contents.matches(&line), "-->\n".to_string()].concat();
                in_comments = false;
            }
        }
        // handle perl lines
        else if begin_cdata.is_match(&line) || in_cdata {
            in_cdata = true;
            let perl_line: String = format!("{:?}", perl_without_cdata.find(&line).unwrap());
            perl_code = [perl_code, perl_line, line[..].to_string(), '\n'.to_string()].concat(); // make a perl string

            if end_cdata.is_match(&line) {
                // due to todo above- might need more specific regex to capture only perl on this line
                // call perl formatter with tabs included
                // todo

                in_cdata = false;
            }
            continue;
        }
        // handle version lines
        else if begin_version.is_match(&line) || in_version {
            in_version = true;
            version = [version[..].to_string(), line[..].to_string()].concat();

            if end_version.is_match(&line) {
                // due to todo above- might need more specific regex to capture only perl on this line
                xml_out = [
                    version[..].to_string(),
                    line[..].to_string(),
                    '\n'.to_string(),
                ]
                .concat();
                in_version = false;
            }
        }
        // handle xml lines
        else {
            xml_out = [xml_out, line, '\n'.to_string()].concat();
        }
    }
    for perl in perl_code.split_whitespace() {
        println!("{}", perl);
    }

    // writing
    let out_f = File::create(out_path).expect("Couldn't create file!");
    let mut out_f = BufWriter::new(out_f);
    out_f
        .write_all(&xml_out.as_bytes())
        .expect("Couldn't write contents out!");
}

// fn remove_whitespace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }
