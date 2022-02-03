use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};

fn main() {
    let matches = cli();

    // get the files
    let out_path = matches.value_of("output").unwrap(); // unwrap OK because required arg
    let in_path = matches.value_of("input").unwrap(); // unwrap OK because required arg

    // bufreading
    let f = File::open(in_path).expect("Unable to open file");
    let f = BufReader::new(f);

    // xml string to be written out
    let mut xml_out: String = String::new();

    // string starts with anything but a '<'
    let special_perc = Regex::new(r"/S<").unwrap();
    let get_special = Regex::new(r"(%%)").unwrap();

    // perl
    let begin_cdata = Regex::new(r"(CDATA)").unwrap(); // CDATA in line
    let end_cdata = Regex::new(r"(]])").unwrap(); // End in line
    let cdata_only = Regex::new(r"(CDATA)\s*\[").unwrap();
    let end_perl = Regex::new(r"\]\]").unwrap();
    let mut perl_code: String = String::new();
    let mut in_cdata = false;

    let mut test_perl = String::new();

    for n_line in f.split(b'>') {
        // get string from result
        let line = n_line.expect("Unable to read line");

        let line = std::str::from_utf8(&line).unwrap();

        // let line = remove_whitespace(line);

        // handle perl lines
        if begin_cdata.is_match(&line) || in_cdata {
            in_cdata = true;

            if line.len() == 0 {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }

            perl_code = [perl_code, line[..].to_string()].concat();

            if !end_cdata.is_match(&line) {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }

            perl_code = [perl_code, "\n".to_string()].concat();

            // find loc of <[CDATA[]]>
            let cdata_loc = cdata_only.find(&perl_code).unwrap(); 
            let end_perl_loc = end_perl.find(&perl_code).unwrap();

            // get <[CDATA[]]>
            // let c_beg = [perl_code[..cdata_loc.end()].to_string()].concat();
            // let c_end = [perl_code[end_perl_loc.start()..end_perl_loc.end()].to_string(), '>'.to_string()].concat();
            let n_perl_code = [perl_code[..end_perl_loc.end()].to_string(), '>'.to_string()].concat();

            // get <[CDATA[*]]> just the star
            perl_code = perl_code[cdata_loc.end()..end_perl_loc.start()].to_string();

            // println!("{}", perl_code);
            test_perl = [test_perl, perl_code[..].to_string()].concat();

            // send cdata end to xml_out
            xml_out = [xml_out, n_perl_code].concat();
            // println!("{}", perl_code);
            perl_code = String::new();

            in_cdata = false;
            continue;
        }

        // handle special
        if special_perc.is_match(&line) {
            let double_percent_str = get_special.find(&line).unwrap();
            xml_out = [
                xml_out,
                line[double_percent_str.start()..double_percent_str.end()].to_string(),
                line[double_percent_str.end()..].to_string(),
            ]
            .concat();
        }
        // handle xml lines
        else {
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
    out_perl.write_all(&test_perl.as_bytes()).expect("Couldn't write perl out!");
}

// fn remove_whitespace(s: &str) -> String {
//     let mut s = String::from(s);
//     s.retain(|c| !c.is_whitespace());
//     return s;
// }

fn cli() -> clap::ArgMatches<'static>  {
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
    return matches;
}