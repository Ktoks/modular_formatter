use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
// use std::iter::IntoIterator;
// use iowrap::Eof;

fn main() {
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
    let f = File::open(in_path).expect("Unable to open file");
    let f = BufReader::new(f);

    // xml string to be written out
    let mut xml_out: String = String::new();

    // string starts with anything but a '<'
    let newspecial = Regex::new(r"/S<").unwrap();
    let get_special = Regex::new(r"(%%)").unwrap();

    // perl
    let begin_cdata = Regex::new(r"(CDATA)").unwrap(); // CDATA in line
    let end_cdata = Regex::new(r"(]])").unwrap(); // End in line
    let cdata_only = Regex::new(r"(CDATA)\s*\[").unwrap();
    let end_perl = Regex::new(r"\]\]").unwrap();
    let mut perl_code: String = String::new();
    let mut actual_perl_code: String = String::new();
    let mut in_cdata = false;

    let mut test_perl = String::new();

    for n_line in f.split(b'>') {
        // get string from result
        let line = n_line.expect("Unable to read line");

        let line = std::str::from_utf8(&line).unwrap();

        // let line = remove_whitespace(line);

        // handle perl lines
        if begin_cdata.is_match(&line) && !in_cdata {
            in_cdata = true;
        }

        if in_cdata {
            // if len of line is 0, just add a '>'
            if line.len() == 0 {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }
            // concat a perl string
            perl_code = [perl_code, line[..].to_string()].concat();

            // if not at the end of perl, and > present, just add a '>'
            if !end_cdata.is_match(&line) {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }
            perl_code = [perl_code, "\n".to_string()].concat();

            if end_cdata.is_match(&line) {
                let cdata_loc = cdata_only.find(&perl_code).unwrap(); // find the end of the CDATA string and
                let end_perl_loc = end_perl.find(&perl_code).unwrap(); // find the end of the perl string
                xml_out = [xml_out, perl_code[..cdata_loc.end()].to_string()].concat(); // to put the 'CDATA' section in the out_xml string and

                // println!("{}", perl_code);
                test_perl = [test_perl, perl_code[cdata_loc.end()..end_perl_loc.start()].to_string()].concat();

                // todo call perl formatter

                // perl_code below should be replaced with what comes out of perl formatter
                xml_out = [
                    xml_out,
                    perl_code[end_perl_loc.start()..end_perl_loc.end()].to_string(),
                    '>'.to_string(),
                ]
                .concat();
                in_cdata = false;
            }
            continue;
        }
        // handle special
        if newspecial.is_match(&line) {
            let double_percent_str = get_special.find(&line).unwrap();
            xml_out = [
                xml_out,
                line[double_percent_str.start()..double_percent_str.end()].to_string(),
                line[double_percent_str.end()..].to_string(),
            ]
            .concat();
        } else {
            xml_out = [xml_out, line.to_string(), '>'.to_string()].concat();
        }
    }

    // writing
    let out_f = File::create(out_path).expect("Couldn't create file!");
    let mut out_f = BufWriter::new(out_f);
    out_f
        .write_all(&xml_out.as_bytes())
        .expect("Couldn't write contents out!");

    // testing perl only
    let out_perl = File::create("docs/perl.pl").expect("Couldn't create perl file!");
    let mut out_perl = BufWriter::new(out_perl);
    out_perl
        .write_all(&test_perl.as_bytes())
        .expect("Couldn't write perl out!");
}

// fn remove_whitespace(s: &str) -> String {
//     let mut s = String::from(s);
//     s.retain(|c| !c.is_whitespace());
//     return s;
// }
