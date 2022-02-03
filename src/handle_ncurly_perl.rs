// this file is only for perl without curly braces
pub fn multiple_lines(perl_code: &String, temp_perl: &mut String, tab_mult: usize) -> (String, usize){
    for prl in perl_code.split(';') {
        temp_perl.push_str(&"\t".repeat(tab_mult));
        temp_perl.push_str(&prl);
        temp_perl.push_str(&";");
    }
    // get rid of extra ';'
    temp_perl.pop();
    temp_perl.pop();
    (temp_perl.to_string(), tab_mult)
}

pub fn chunk3(xml_out: &mut String, tab_mult: usize, perl_code: &String) -> (String, usize) {
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&"<![CDATA[");
    xml_out.push_str(&perl_code);    
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&"]]>\n");
    (xml_out.to_string(), tab_mult)
}

pub fn chunk4(xml_out: &mut String, tab_mult: usize, perl_code: &String) -> (String, usize) {
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&"<![CDATA[");
    xml_out.push_str(&perl_code);    
    xml_out.push_str(&"\n");
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&"]]>\n");
    (xml_out.to_string(), tab_mult)
}

// oneliner perl
pub fn single_line(xml_out: &mut String, tab_mult: usize, perl_code: &String) -> (String, usize) {
    let mut temp_tab = "\n".to_string();
    temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
    let temp_perl = perl_code.replace("\n", &temp_tab);
    
    xml_out.push_str(&"\t".repeat(tab_mult));
    xml_out.push_str(&"<![CDATA[");
    xml_out.push_str(&temp_perl);    
    xml_out.push_str(&"]]>\n");

    (xml_out.to_string(), tab_mult)
}

pub fn inline(xml_out: &mut String, tab_mult: usize, perl_code: &String, tab_nl: regex::Regex) -> (String, usize) {
    // let mut temp_tab = "\n".to_string();
    // temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
    // perl_code = perl_code.replace("\n", &temp_tab);
    let mut temp_tab = "\n".to_string();
    temp_tab = [temp_tab, "\t".to_string().repeat(tab_mult)].concat();
    let mut temp_perl: String = perl_code.clone().to_string().replace("\n", &temp_tab);
    
    temp_perl = temp_perl.clone().replace("\t\n", "\n");
    temp_tab = ["\t".to_string().repeat(tab_mult-1), temp_tab].concat();

    if tab_nl.is_match(&temp_perl) {
        temp_perl = temp_perl.replace(&temp_tab, &"\t".to_string().repeat(tab_mult));
    }
    let mut tabular = tab_mult.clone();
    if temp_perl.as_bytes()[0] == '\n' as u8  && temp_perl.as_bytes()[temp_perl.len() - 1 - tab_mult] == '\n' as u8 {
        
        (*xml_out, tabular) = chunk3(xml_out, tab_mult, &temp_perl);
    
    } else if temp_perl.as_bytes()[0] == '\n' as u8 {

        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&"<![CDATA[");
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&temp_perl);
        xml_out.push_str(&"\n");
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&"]]>\n");

    } else if temp_perl.as_bytes()[temp_perl.len() - 1 - tab_mult] == '\n' as u8 {
        
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&"<![CDATA[");
        xml_out.push_str(&"\n");
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&temp_perl);
        xml_out.push_str(&"]]>\n");

    } else {

        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&"<![CDATA[");
        xml_out.push_str(&"\n");
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&temp_perl);
        xml_out.push_str(&"\n");
        xml_out.push_str(&"\t".repeat(tab_mult));
        xml_out.push_str(&"]]>\n");

    }
    (xml_out.to_string(), tabular)
}