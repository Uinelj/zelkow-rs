extern crate zilean;
extern crate serde_json;

use zilean::champion::Champion as Champion;
use std::collections::HashMap;

#[test]
fn test_get_id() {
    let champion = Champion::new(10);
    assert_eq!(10, champion.get_id());
}

#[test]
#[should_panic]
fn test_gen_empty() {
    let mut champion = Champion::new(10);
    champion.feed("".to_string());
    assert_eq!("".to_string(), champion.gen(10).unwrap());
}

#[test]
//TODO: Correct the code in order to assert "A" and "A"
fn test_gen_one() {
    let mut champion = Champion::new(10);
    champion.feed("A".to_string());
    assert_eq!("A".to_string(), champion.gen(5).unwrap());
}

#[test]
fn test_gen_limit() {
    let mut champion = Champion::new(10);
    champion.feed("Foo bar baz hello world".to_string());
    assert!(champion.gen(5).unwrap().as_bytes().len() <= 5);
}

#[test]
fn test_eq() {
    let mut champion = Champion::new(10);
    let mut champion2 = Champion::new(10);

    champion.feed("foo".to_string());
    champion.feed("bar".to_string());
    champion2.feed("foo".to_string());
    champion2.feed("bar".to_string());

    assert_eq!(champion, champion2);
}

#[should_panic]
fn test_gen_2_times() {
    let mut champion = Champion::new(10);

    champion.feed("Bonjour".to_string());
    champion.feed("Bonsoir".to_string());
    champion.feed("Bonne soirée".to_string());
    champion.feed("Merci".to_string());
    champion.feed("Au revoir".to_string());

    assert_eq!(champion.gen(5), champion.gen(5));
}
/*#[test]
fn test_serialize() {
    let mut champion = Champion::new(10);
    champion.feed("Foo".to_string());
    champion.feed("Bar".to_string());

    let mut champion2 = Champion::new(10);
    let serialized = try!(champion.serialize());
    assert_eq!("A", "A");
}*/
