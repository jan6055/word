use std::{env,process::exit};
use word::{
    glossary::{
        Glossary,
    }
}; 
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let default_filename = env::var("WORD_DEFAULT_FILE");
    let has_input_file = args.get(0)
                                    .unwrap_or(&String::from("###"))
                                    .as_str() == "-f";
    if !has_input_file && default_filename.is_err() {
        eprint!("Error: not found path WORD_DEFAULT_FILE_PATH and not have input file\n");
        exit(1);
    }
    let mut glossary;
    if has_input_file {
        args.remove(0);
        if args.len() == 0 {
            eprint!("no input file");
        }
        let filename = args[0].clone();
        args.remove(0);
        glossary = Glossary::from(&filename);
    } else {
        glossary = Glossary::from(default_filename.unwrap().as_str())
    }
    // let mut glossary = Glossary::from("/home/jan/code/rust/word/test.txt");
    match glossary.run(args) {
        Ok(_) => (),
        Err(err) => eprint!("{}",err),
    }
    // args.iter()
    //     .for_each(|arg| println!("{}",arg));
}


