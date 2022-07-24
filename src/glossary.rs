use crate::tools;
use std::{
    collections::{BTreeSet, HashMap},
    process,
};
pub struct Glossary {
    table: HashMap<String, String>,
    filename: String,
}

impl Glossary {
    pub fn add(&mut self, word: String, translation: String) {
        self.table.insert(word, translation);
    }

    pub fn list(&self) {
        self.table.iter().for_each(|pair| {
            println!("{}: {}", pair.0, pair.1);
        })
    }

    pub fn remove(&mut self, word: &str) -> Result<(), String> {
        match self.table.remove(word) {
            Some(_) => Ok(()),
            None => Err(String::from("not have this word")),
        }
    }

    pub fn new() -> Glossary {
        Glossary {
            table: HashMap::new(),
            filename: String::from(""),
        }
    }
    pub fn from(filename: &str) -> Glossary {
        match tools::read_to_vec_string(filename) {
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
            Ok(lines) => {
                let mut glossary = Glossary::new();
                for line in lines {
                    let mut it = line.split(' ');
                    let word = it.next().unwrap().to_string();
                    let translation = it.next().unwrap().to_string();
                    glossary.add(word, translation);
                }
                glossary.filename = filename.to_string();
                glossary
            }
        }
    }


    pub fn list_with_sorted(&self) {
        self.table
            .iter()
            .map(|pair| pair.0)
            .collect::<BTreeSet<&String>>()
            .iter()
            .for_each(|&word| {
                println!("{},{}",
                    word,self.table.get(word).unwrap());
            });
    }

}

///
/// write word table all words to file
/// 

impl Drop for Glossary {
    fn drop(&mut self) {
        let to_write: Vec<(&String, &String)> = 
            self.table
                .iter()
                .collect();
        match tools::wirte_vec_string(to_write, &self.filename) {
            Ok(_) => (),
            Err(e) => eprint!("{}",e),
        }
    }
}
