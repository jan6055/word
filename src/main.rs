use std::env;
use word::{
    glossary::{
        Glossary,
    }
}; 
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut glossary = Glossary::from("test.txt");
    match glossary.run(args) {
        Ok(_) => (),
        Err(err) => eprint!("{}",err),
    }
    // args.iter()
    //     .for_each(|arg| println!("{}",arg));
}


