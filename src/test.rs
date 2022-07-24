
use crate::glossary::Glossary;

const TestFile :&str = "test.txt";
#[test]
fn test_glossary_from() {
    let mut glossary = Glossary::from(TestFile);
}

#[test]
fn test_glossary_list() {
    let mut glossary = Glossary::from(TestFile);
    glossary.list();
}

#[test]
fn test_glossary_add() {
    let mut glossary = Glossary::from(TestFile);
    glossary.add("one".to_string(), "一".to_string());
    glossary.list();
}

#[test]
fn test_glossary_remove() {
    let mut glossary = Glossary::from(TestFile);
    glossary.remove("hello");
    glossary.list();
}


#[test]
fn test_glossary_list_with_sorted() {
    let mut glossary = Glossary::from(TestFile);
    glossary.list_with_sorted();
}

#[test]
fn test_glossary_drop() {
    let mut glossary = Glossary::from(TestFile);
    glossary.list();
    glossary.add(String::from("int"), String::from("整形"));

    glossary.list_with_sorted();
}