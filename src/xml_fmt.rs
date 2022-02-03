
pub fn comments(line: String, xml_comment_body: regex::Regex, tab_mult: usize, xml_out: &mut String) -> (String, usize, String) {
    let mut temp_tab = "\n".to_string();
    temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
    let line = line.replace("\n", &temp_tab);
    let c_bod = xml_comment_body.find(&line).unwrap();
    // println!("{}", &line);
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&line[c_bod.start()..c_bod.end()]);
    xml_out.push_str(&"> -->");
    (line, tab_mult, xml_out.to_string())
}

pub fn body(line: String, xml_comment_body: regex::Regex, tab_mult: usize, xml_out: &mut String) -> (String, usize, String) {
    let mut temp_tab = "\n".to_string();
    temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
    let line = line.replace("\n", &temp_tab);
    let c_bod = xml_comment_body.find(&line).unwrap();
    // println!("{}", &line);

    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&line[c_bod.start()..c_bod.end()]);
    xml_out.push_str(&"-->");

    (line, tab_mult, xml_out.to_string())
}