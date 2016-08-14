//! Handles a Redis database and enables storing and loading champions from it.
//!
//! # Examples
//!
//! # Storing
//!
//! ```rust,ignore
//! use self::zilean::database::*;
//! use self::zilean::champion::*;
//! let db = Database::new("redis://127.0.0.1/".to_string());
//! let champion = Champion::new(10);
//!
//! //do things with champion
//!
//! db.store(&champion);
//! ```
//!
//! # Loading
//!
//! ```rust,ignore
//! use self::zilean::database::*;
//! use self::zilean::champion::*;
//! let db = Database::new("redis://127.0.0.1/".to_string());
//!
//! let champion = db.load(10);
//! ```
extern crate redis;
extern crate serde_json;
use ::champion::Champion as Champion;
use std::collections::HashMap;
use self::redis::Commands;

///Represents a connection to the Redis database.
pub struct Database {
    url : String,
    con : redis::Connection,
}
impl Database {
    /// Creates a new Database object, connected to a redis instance.
    ///
    /// # Arguments
    ///
    /// * `champion_id` : the id of the champion.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use self::zilean::database::*;
    ///
    /// let db = Database::new("redis://127.0.0.1".to_string());
    /// ```
    ///
    /// # Panics
    ///
    /// If we aren't able to connect to the redis instance. (That's bad)
    pub fn new(url : String) -> Database {
        let con;
        match Database::connect(&url) {
            Ok(client) => con = client,
            Err(_) => panic!("Error trying to connect to database using URL {:?}", url),
        }
        Database {
            url : url,
            con : con,
        }
    }

    fn connect(url : &String) -> redis::RedisResult<redis::Connection> {
        let redis_url;
        match redis::parse_redis_url(url) {
            Ok(url) => redis_url=url,
            Err(url) => panic!("Malformed Database URL : {:?}", url),
        }
        let client = try!(redis::Client::open(redis_url));
        let con = try!(client.get_connection());
        Ok(con)
    }

    /// Loads a Champion from the database.
    ///
    /// If the said champion could not be found, then no data is loaded and an empty Champion object is returned.
    ///
    /// # Arguments
    ///
    /// `champion_id` : the id of the champion.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use self::zilean::database::*;
    ///
    /// let db = Database::new("redis://127.0.0.1".to_string());
    /// let champion = db.load(10);
    /// ```
    //maybe return Some
    pub fn load(&self, champion_id : u32) -> Champion {
        let mut champion = Champion::new(champion_id);
        match self.con.get(champion_id) {
            Ok(val) => {
                champion.deserialize(val);
                return champion;
            },
            Err(_) => return champion,
        }
    }

    /// Attempts to store a champion into the redis database.
    ///
    /// # Arguments
    ///
    /// * `champion` : A reference to the champion to be stored.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use self::zilean::champion::*;
    /// use self::zilean::database::*;
    ///
    /// let db = Database::new("redis://127.0.0.1".to_string());
    /// let mut champion = Champion::new(10);
    /// champion.feed("Hello".to_string());
    /// db.store(&champion);
    /// ```
    ///
    /// # Panics
    ///
    /// If it's unable to serialize the champion into JSON data.
    pub fn store(&self, champion : &Champion) -> redis::RedisResult<()> {
        match champion.serialize() {
            Ok(val) => self.con.set(champion.get_id(), val),
            Err(val) => panic!(val),
        }
    }

    ///Returns the redis instance URL.
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    //breaks <S>OLID ?
    //TODO: Err handling
    /// Deserializes data sent by Twitch, the python script that sends game data.
    ///
    /// # Arguments
    ///
    /// * `json_string` : The string sent by Twitch.
    ///
    /// #Example
    ///
    /// ```rust,ignore
    /// use self::zilean::database::*;
    ///
    /// let twitch_data : String;
    /// twitch_data = r#"{"14" : ["foo", "bar", "quux"], "81" : ["hello", "world"]}"#.to_string();
    /// // Assuming we put the String in this var.
    /// let deserialized_data = Database::deserialize_bulk(twitch_data);
    /// ```
    ///
    /// # Panics
    ///
    /// If the data aren't deserializable.
    pub fn deserialize_bulk(json_string : String) -> HashMap<u32, Vec<String>> {
        let json_hm : HashMap<u32, Vec<String>> = serde_json::from_str(&json_string).unwrap();
        return json_hm
    }
}
