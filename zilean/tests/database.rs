extern crate zilean;

use zilean::database::Database as Database;
use zilean::champion::Champion as Champion;

const REDIS_URL : &'static str = "redis://redis";
#[test]
fn test_new() {
    let db = Database::new(REDIS_URL.to_string());
}

#[test]
#[should_panic]
fn test_new_invalid_url() {
    let db = Database::new("radis://265.4.2.4".to_string());
}

#[test]
fn test_load_store() {
    let db = Database::new(REDIS_URL.to_string());
    let mut champion = Champion::new(10);
    champion.feed("foo".to_string());
    champion.feed("bar".to_string());
    db.store(&champion);

    let champion2 = db.load(10);

    assert_eq!(champion, champion2);
}

#[test]
fn test_update() {
    let db = Database::new(REDIS_URL.to_string());
    let mut champion = Champion::new(10);
    champion.feed("foo".to_string());
    champion.feed("bar".to_string());
    champion.feed("baz".to_string());

    // let mut champion2 = Champion::new(10);
    // champion2.feed("foo".to_string());
    // champion2.feed("bar".to_string());
    //
    // db.store(champion2);
    // champion2 = db.load(10);
    // champion2.feed("baz".to_string());
    // db.store(champion2);
    //
    // assert_eq!(champion, db.load(10));
}
