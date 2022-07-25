use std::{
    fs, 
    error::Error,
};

pub fn read_to_vec_string(filename: &str) -> Result<Vec<String>,Box<dyn Error>> {
    let text = fs::read_to_string(filename)?;

    let text: Vec<String> = text
        .lines()
        .map(|s| s.to_string())
        .collect();
    // dbg!(&text); Ok
    Ok(text)
}

pub fn wirte_vec_string(lines: Vec<(&String, &String)>, filename: &str) -> Result<(),Box<dyn Error>>{
    let mut text = String::new();
    for line in lines {
        text.push_str(&format!("{:<10}",line.0));
        text.push_str(&format!("{:<}",line.1));
        text.push('\n');
    }
    fs::write(filename, text)?;
    Ok(())
}

#[inline]
pub fn cut_word(line: &mut String) {
    let idx = line.find(" ").unwrap();
    line.replace_range(idx..line.len(), "");
}

#[inline]
pub fn print_vec(liens: &Vec<String>) {
    liens.iter()
         .for_each(|line| println!("{:<10}",line));
}
