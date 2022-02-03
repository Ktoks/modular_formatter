use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};

mod handle_ncurly_perl;
mod my_cli;
mod xml_fmt;

fn main() {
    ////////////////////////////////// Declare variables //////////////////////////////////
    // get command line arguments
    let matches = my_cli::cli();

    // get the files
    let out_path = matches.value_of("output").unwrap(); // unwrap OK because required arg
    let in_path = matches.value_of("input").unwrap(); // unwrap OK because required arg

    // bufreading
    let f = File::open(in_path).expect("Unable to open file");
    let f = BufReader::new(f);

    // xml string to be written out
    let mut xml_out: String = String::new();

    // string starts with anything but a '<'    ([^<\s]+\s*)+<   (\S+\s*)+<
    let special_perc = Regex::new(r#"(\S+\s*)+<"#).unwrap();
    // let special_perc = Regex::new(r"(\s*[^<\s]+\s*)+<").unwrap();
    // let get_special = Regex::new(r"(%%)").unwrap();

    // perl
    let begin_cdata = Regex::new(r#"<!\[CDATA\["#).unwrap(); // CDATA in line
    let one_line_cdata = Regex::new(r#"<!\[CDATA\[[^\n]+\]\]"#).unwrap(); // CDATA in line   
    let end_cdata = Regex::new(r#"]]"#).unwrap(); // End in line
    let semicolon = Regex::new(r#";"#).unwrap();
    let curly = Regex::new(r#"[{}]"#).unwrap();
    let curly_semi_alligator_newl_comm_dolla = Regex::new(r#"[<>{};\s#$]"#).unwrap();
    let cdata_comment = Regex::new(r#"\s+#"#).unwrap();
    let just_cdata_comment_loc = Regex::new(r#"#[^\n]+\n"#).unwrap();
    let tab_nl = Regex::new(r#"\t\n"#).unwrap();

    let mut perl_code: String = String::new();
    let mut in_cdata = false;

    // looking for whitespace in body of text
    let rm_ws = Regex::new(r"\s+").unwrap();

    // comments
    let xml_comment_body = Regex::new(r#"<!--[\s\S]+[^-][^-]"#).unwrap();
    let end_xml_comment_body = Regex::new(r#"<!--[\s\S]+[^-][^-]$"#).unwrap();

    // handle ? xml line
    let xml_file_version = Regex::new(r#"<\?[^?]*\?"#).unwrap();

    // head xml lines
    let head_string = Regex::new(r#"<(\w+)"#).unwrap(); // <powerstream

    // handle body lines
    let bod_strings = Regex::new(r#"(\s*[[:word:]]+="[^"]*")+"#).unwrap(); //  win_run_path="%%runpath2" unix_run_path="%%runpath" name="powerstream"
    let in_declaration = Regex::new(r#"^\s*(.+)=$"#).unwrap(); // memo=
    let in_quotes = Regex::new(r#"[^=]$"#).unwrap(); // "string to be saved"
    let end_slash = Regex::new(r#"\s*/$"#).unwrap(); // "/>"

    // handle foot lines
    let foot_slash = Regex::new(r#"</[[:word:]]+"#).unwrap(); // </powerstream

    // count tabs
    let mut tab_mult: usize = 0;

    ////////////////////////////////// End Declare variables //////////////////////////////////

    // loop over the file
    for n_line in f.split(b'>') {
        // line cleanup
        let line = n_line.expect("Unable to read line");
        let line = std::str::from_utf8(&line).unwrap();
        
        let mut perl_comments: Vec<String> = Vec::new();
        let mut line = line.replace("\n\n", "\n");
        line = line.replace("\t", "");
        line = line.replace("     ", " ");
        line = line.replace("    ", " ");
        line = line.replace("   ", " ");
        line = line.replace("  ", " ");
        
        // handle perl comment issue before line breaks
        if cdata_comment.is_match(&line) {
            for comment in just_cdata_comment_loc.find_iter(&line) {
                perl_comments.push(line[comment.start()..comment.end()].to_string());
            }
        }

        // handle perl lines
        if begin_cdata.is_match(&line) || in_cdata {in_cdata = true;

            if line.is_empty() {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }
        
            perl_code = [perl_code, line[..].to_string()].concat();
        
            // if '>' in perl code, add one and continue
            if !end_cdata.is_match(&line) {
                perl_code = [perl_code, '>'.to_string()].concat();
                continue;
            }
        
            // find loc of <[CDATA[]]>
            let cdata_loc = begin_cdata.find(&perl_code).unwrap();
            let end_perl_loc = end_cdata.find(&perl_code).unwrap();
        
            // get <[CDATA[*]]> just the star
            perl_code = perl_code[cdata_loc.end()..end_perl_loc.start()].to_string();
        
            let mut temp_perl = String::new();
        
            // perl_code = perl_code.replace("\n  ", "\n");
            perl_code = perl_code.replace("\n ", "\n");
        
            // check for ;
            if semicolon.is_match(&perl_code) {
                let mut last_curly_semi: usize = 0;
        
                tab_mult += 1;
                let mut temp_tab = "\n".to_string();
                temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
                perl_code = perl_code.replace("\n", &temp_tab);
                
                perl_code = perl_code.replace("\t\n", "\n");
                temp_tab = ["\t".to_string().repeat(tab_mult-1), temp_tab].concat();
                
                // check if \t\n exists
                if tab_nl.is_match(&perl_code) {
                    perl_code = perl_code.replace(&temp_tab, &"\t".to_string().repeat(tab_mult));
                }
        
                // perl_code = perl_code.replace("\t\n", "");
        
                // check for {}
                if curly.is_match(&perl_code) {
                    let mut in_alligator = false;
                    let mut in_comments = false;
                    let mut in_dolla = false;
                    for current_ender in curly_semi_alligator_newl_comm_dolla.find_iter(&perl_code) {
                        let this_one = &perl_code[current_ender.start()..current_ender.start() + 1];
                        // temp_perl = [temp_perl, '\t'.to_string().repeat(tab_mult)].concat();
                        if this_one == " " || this_one == "\n" || this_one == "\t" || this_one == ";" {
                            in_dolla = false;
                        } if this_one == "\n" {
                            in_comments = false;
                        } else if this_one == "<" {
                            in_alligator = true;
                        } else if this_one == ">" {
                            in_alligator = false;
                        } else if this_one == "#" {
                            in_comments = true;
                        } else if this_one == "$" {
                            in_dolla = true;
                        } else if !(in_comments || in_alligator || in_dolla) {
                            if this_one == "{" {
                                // if a \n before
                                if &perl_code[current_ender.start() - tab_mult - 1..current_ender.start() - tab_mult] == "\n" {
                                    temp_perl = [
                                    temp_perl,
                                    perl_code[last_curly_semi..current_ender.end() - 1].to_string(),
                                    "{".to_string(),
                                ]
                                .concat();
                                } else 
                                {
                                    temp_perl = [
                                    temp_perl,
                                    perl_code[last_curly_semi..current_ender.end() - 1].to_string(),
                                    "\n".to_string(),
                                    "\t".to_string().repeat(tab_mult),
                                    "{".to_string(),
                                ]
                                .concat();
                                }
                    
                                
                                last_curly_semi = current_ender.end();
                                tab_mult += 1;
                            }
                            // if it's a semi-colon inside curlys
                            else if this_one == ";" {
                                temp_perl = [
                                    temp_perl,
                                    perl_code[last_curly_semi ..current_ender.end()].to_string(),
                                ]
                                .concat();
                                last_curly_semi = current_ender.end();
                            }
                            // if it's a close curly
                            else if this_one == "}" {
                                temp_perl = [
                                    temp_perl,
                                    "\n".to_string(),
                                    perl_code[last_curly_semi + 1..current_ender.end()].to_string(),
                                ]
                                .concat();
                                tab_mult -= 1;
                                last_curly_semi = current_ender.end();
                            }
                        }
                    }
                } else {
                    // deal with mutltiple lines of perl
                    (temp_perl, tab_mult) = handle_ncurly_perl::multiple_lines(&mut perl_code, &mut temp_perl, tab_mult);
                }
                // temp_perl = [temp_perl, "\t".to_string().repeat(tab_mult)].concat();
                perl_code = temp_perl;
                tab_mult -= 1;
        
                if perl_code.as_bytes()[perl_code.len() - 1 - tab_mult] == '\n' as u8 {
                    (xml_out, tab_mult) = handle_ncurly_perl::chunk3(&mut xml_out, tab_mult, &perl_code);
                } else {
                    (xml_out, tab_mult) = handle_ncurly_perl::chunk4(&mut xml_out, tab_mult, &perl_code);
                }
        
        
            } else if one_line_cdata.is_match(&line) {
                handle_ncurly_perl::single_line(&mut xml_out, tab_mult, &perl_code);
            } else  {
                handle_ncurly_perl::inline(&mut xml_out, tab_mult, &perl_code, tab_nl.clone());
            }
            perl_code = String::new();
            in_cdata = false;
            continue;
        }


        // handle xml comments
        if end_xml_comment_body.is_match(&line) {
            (line, tab_mult, xml_out) = xml_fmt::comments(line, xml_comment_body.clone(), tab_mult, &mut xml_out);
            continue;
        } else if xml_comment_body.is_match(&line) {
            (line, tab_mult, xml_out) = xml_fmt::body(line, xml_comment_body.clone(), tab_mult, &mut xml_out);
            continue;
        }
        line = line.replace("\n", " ");

        // handle outside of elements
        if special_perc.is_match(&line) {
            let spec = special_perc.find(&line).unwrap();
            xml_out = [
                xml_out,
                // "\n".to_string(),
                "\t".to_string().repeat(tab_mult),
                line[spec.start()..spec.end() - 1].to_string(),
                "\n".to_string(),
            ]
            .concat();
            // continue;
        }

        // handle the ? header
        if xml_file_version.is_match(&line) {
            xml_out = [xml_out, line[..].to_string(), ">\n".to_string()].concat();
            continue;
        }

        line = line.replace(" =", "=");
        line = line.replace("= ", "=");
        // handle normal xml
        let mut xml_peices = String::new();

        // handle header
        if head_string.is_match(&line) {
            // get the header
            let head = head_string.find(&line).unwrap();
            xml_peices = [
                xml_peices,
                "\t".to_string().repeat(tab_mult),
                line[head.start()..head.end()].to_string(),
                "\n".to_string(),
            ]
            .concat();
            tab_mult += 1;
        }

        // handle body of xml
        if bod_strings.is_match(&line) {
            let bod = bod_strings.find(&line).unwrap();

            let peices = line[bod.start()..bod.end()].to_string();
            let mut to_concat = String::new();
            let mut dec = false;

            // for loop to stack each body peice
            for peice in peices.split('"') {
                if in_declaration.is_match(peice) {
                    dec = true;
                    let temp = peice.replace(" ", "");
                    to_concat = [
                        to_concat,
                        "\t".to_string().repeat(tab_mult),
                        temp.to_string(),
                    ]
                    .concat();
                } else if in_quotes.is_match(peice) {
                    let temp = rm_ws.replace_all(peice, " ");
                    let temp = temp.to_string();

                    to_concat = [
                        to_concat,
                        '"'.to_string(),
                        temp.to_string(),
                        '"'.to_string(),
                        '\n'.to_string(),
                    ]
                    .concat();
                    dec = false;
                } else if peice.is_empty() && dec {
                    to_concat = [to_concat, "\"\"".to_string(), '\n'.to_string()].concat();
                } else {
                    to_concat.pop();
                }
            }
            xml_peices = [xml_peices, to_concat.to_string(), "\n".to_string()].concat();
        }

        // handle "/>" (self closing / no children)
        if end_slash.is_match(&line) {
            if tab_mult > 0 {
                tab_mult -= 1;
            } else {
                println!("tab mult issue: {}", &line);
            }
            xml_peices = [
                xml_peices,
                "\t".to_string().repeat(tab_mult),
                "/>\n".to_string(),
            ]
            .concat();
        }
        // handle </powerstream (footer / end of children and self)
        else if foot_slash.is_match(&line) {
            let foot = foot_slash.find(&line).unwrap(); // have to find, this could be on the same line as other <>'s
            xml_peices = [
                xml_peices,
                "\t".to_string().repeat(tab_mult),
                line[foot.start()..foot.end()].to_string(),
                ">\n".to_string(),
            ]
            .concat();
            if tab_mult > 0 {
                tab_mult -= 1;
            } else {
                println!("tab mult issue");
            }
        } else {
            if !xml_peices.is_empty() {
                xml_peices = [
                    xml_peices,
                    "\t".to_string().repeat(tab_mult),
                    ">\n".to_string(),
                ]
                .concat();
            }
        }

        xml_out = [xml_out, xml_peices.to_string()].concat();
    }

    // writing
    let out_f = File::create(out_path).expect("Couldn't create file!");
    let mut out_f = BufWriter::new(out_f);
    out_f
        .write_all(xml_out.as_bytes())
        .expect("Couldn't write contents out!");
}
