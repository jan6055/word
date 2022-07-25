use crate::tools;
use std::{
    collections::{BTreeSet, HashMap},
    process::{
        self,
        // Command,
    },
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
            println!("{:<10} {:<10}", pair.0, pair.1);
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
                eprintln!("Error: {} called {}", err, filename);
                process::exit(1);
            }
            Ok(lines) => {
                let mut glossary = Glossary::new();
                for line in lines {
                    let mut it = line.split(' ');
                    let word = it.next().unwrap();
                    let translation = it.next().unwrap();
                    glossary.add(word.trim(), translation.trim());
                    // glossary.add(word, translation);
                }
                glossary.filename = filename.to_string();
                glossary
            }
        }
    }

    #[allow(unused)]
    fn list_with_sorted(&self) {
        self.table
            .iter()
            .map(|pair| pair.0)
            .collect::<BTreeSet<&String>>()
            .iter()
            .for_each(|&word| {
                println!("{:<10} {:<10}",
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
            "remove" | "rm" => {
                let word = args.get(1);
                if word.is_none() {
                    return Err(String::from("no input word\n"));
                }
                let word = word.unwrap();
                self.remove(word)?;
            },
            "list" | "l" => {
                let other = args.get(1);
                if other.is_none() {
                    self.list();
                } else {
                    self.pause_with_list(other.unwrap())?;
                }

            },
            other => {
                return Err(format!("not find commadn called {}\n",other));
            }
        }
        Ok(())
    }

    #[allow(unused)]
    fn set_default_file(filename: &str) -> Result<(),String> {
        Ok(())
    }


    fn pause_with_list(&self, arg: &str) -> Result<(),String>{
        if arg.len() == 1 { //only -
            return Err(String::from("not have specific parameters\n"));
        }
        let mut it = arg.chars();
        if it.next().unwrap() != '-' {
            return Err(format!("parameters illegal this -> {}",arg));
        }
        let mut result: Vec<String> = self.table
            .iter()
            .map(|pair| { format!("{:<10} {:<10}",pair.0, pair.1) })
            .collect();
        for ch in it {
            match ch {
                's' => {
                    result.sort();
                },
                'n' => {
                    for line in result.iter_mut() {
                        tools::cut_word(line);
                    }
                }
                other => {
                    return Err(format!("not have commad called -{}\n",other));
                }
            }
        }
        tools::print_vec(&result);
        Ok(())
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
