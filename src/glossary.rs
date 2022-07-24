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
    fn add(&mut self, word: &str, translation: &str) {
        self.table.insert(word.to_string(), translation.to_string());
    }

    fn list(&self) {
        self.table.iter().for_each(|pair| {
            println!("{}: {}", pair.0, pair.1);
        })
    }

    fn remove(&mut self, word: &str) -> Result<(), String> {
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
                    let word = it.next().unwrap();
                    let translation = it.next().unwrap();
                    glossary.add(word, translation);
                }
                glossary.filename = filename.to_string();
                glossary
            }
        }
    }


    fn list_with_sorted(&self) {
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

    pub fn run(&mut self, args: Vec<String>) -> Result<(),String>{
        if args.len() == 0 {
            return Err(String::from("no input command\n"));
        }
        let cmd = args[0].as_str();
        match cmd {
            "add" => {
                let word = args.get(1);
                if word.is_none() {
                    return Err(String::from("no input word\n"));
                }
                let word = word.unwrap();
                let translation = args.get(2);
                if translation.is_none() {
                    return Err(String::from("no input translation\n"));
                }
                let translation = translation.unwrap();
                self.add(word, translation);
            },
            "remove" => {
                let word = args.get(1);
                if word.is_none() {
                    return Err(String::from("no word input\n"));
                }
                let word = word.unwrap();
                self.remove(word)?;
            },
            "list" => {
                self.list();
            },
            other => {
                return Err(format!("not find commadn called {}\n",other));
            }
        }
        Ok(())
    }

    // fn pause(&mut self) -> Result<(), Box<dyn Error>> {

    //     Ok(())
    // }
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
