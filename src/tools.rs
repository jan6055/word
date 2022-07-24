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
    Ok(text)
}

pub fn wirte_vec_string(lines: Vec<(&String, &String)>, filename: &str) -> Result<(),Box<dyn Error>>{
    let mut text = String::new();
    for line in lines {
        text.push_str(line.0);
        text.push(' ');
        text.push_str(line.1);
        text.push('\n');
    }
    fs::write(filename, text)?;
    Ok(())
}