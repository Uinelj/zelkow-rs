//! Handles nickname parsing and generation, by champion.
//!
//! #Examples
//!
//! ## Basic generation
//!
//! ```
//! use self::zilean::champion::*;
//! let mut champion = Champion::new(10);
//!
//! champion.feed("foo".to_string());
//! champion.feed("bar".to_string());
//! champion.feed("baz".to_string());
//! champion.feed("quux".to_string());
//!
//! println!("Generated nickname : {}", champion.gen(10).unwrap());
//! ```
//!
//! ## JSON Serialization
//!
//! ```
//! use self::zilean::champion::*;
//! let mut champion = Champion::new(10);
//!
//! champion.feed("foo".to_string());
//! champion.feed("bar".to_string());
//! champion.feed("baz".to_string());
//! champion.feed("quux".to_string());
//!
//! let serialized = champion.serialize().unwrap();
//! let mut deserialized = Champion::new(10);
//! deserialized.deserialize(serialized);
//! ```
extern crate slog;
extern crate rand;
extern crate serde_json;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use self::rand::Rng;


#[derive(Debug, PartialEq, Eq)]
///Represents all the nicknames associated with a certain champion, characterized by its id.
//SHOULD IT BE A SINGLETON ?
pub struct Champion {
    id: u32,
    values: HashMap<char, HashMap<char, u64>>,
}

impl Champion {
    /// Constructs a new `Champion` by its id.
    ///
    /// # Example
    /// ```
    /// use self::zilean::champion::*;
    ///
    /// let champion = Champion::new(10);
    /// ```
    pub fn new(id : u32) -> Champion {
        Champion {
            id : id,
            values : HashMap::new(),
        }
    }

    /// Feeds a `String` onto the generator.
    ///
    /// # Arguments
    ///
    /// * `nickname` : The nickname which will feed the chain
    ///
    /// # Example
    ///
    /// ```
    /// use self::zilean::champion::*;
    ///
    ///  let mut champion = Champion::new(10);
    ///  champion.feed("foobar".to_string());
    /// ```
    pub fn feed(&mut self, nickname : String) {
        //We see if the nickname was already parsed
        if !Champion::contains(&self, nickname.clone()) {
            //We split nickname in pairs of two chars
            let splitted = Champion::split_nickname(nickname).unwrap();
            /*
                For each pair (0, 1), we see if 1 is already a key.
                If it's the case, either we increment by one the counter of this char if we already have a link between 0 and 1,
                or we simply create another entry.
            */
            for pair in &splitted {
                let mut letter_hm : HashMap<char, u64>;
                if self.values.contains_key(&pair[0]) {
                    letter_hm = self.values.get(&pair[0]).unwrap().clone();
                    let pair_hm = match letter_hm.entry(pair[1]) {
                        Vacant(entry) => entry.insert(0),
                        Occupied(entry) => entry.into_mut(),
                    };
                    *pair_hm += 1;
                } else {
                    letter_hm = HashMap::new();
                    letter_hm.insert(pair[1], 1);
                }
                self.values.insert(pair[0], letter_hm.clone());
            }
        }
    }

    //TODO: Use scan() in order to produce something nice and compact.
    /// Generates a nickname.
    ///
    /// # Arguments
    ///
    /// * `max_len` : The maximum length of the nickname.
    ///
    /// # Example
    ///
    /// ```
    /// use self::zilean::champion::*;
    ///
    /// let mut champion = Champion::new(10);
    /// champion.feed("hello".to_string());
    /// println!("{}", champion.gen(10).unwrap());
    /// ```
    ///
    /// # Panics
    ///
    /// If it is unable to generate some other character. (That's a bad thing)
    pub fn gen(&self, max_len : u32) -> Option<String> {
        if self.values.len() == 0 {
            return None;
        }
        let mut rng = rand::thread_rng();
        //First, we generate our starting letter.
        let first_key_index = rng.gen_range(0, self.values.keys().len());
        let mut next_key = self.values.keys().nth(first_key_index).unwrap().clone();
        let mut ret = "".to_string();
        //Now we will generate next letters until we reach max_len or we hit a terminating char.
        for _ in 0..max_len {
            //quickfix for \0
            if next_key != '\0' {
                ret = ret + &next_key.to_string();
            }
            // XXX : Is it dirty ?
            match next_key {
                '\0' => break,
                _ => next_key = match Champion::get_next_letter(&self.values.get(&next_key).unwrap().clone()) {
                    Some(letter) => letter,
                    None => panic!("Error during nickname generation"),
                }
            }
        }
        return Some(ret)
    }

    fn get_next_letter(current_letter : &HashMap<char, u64>) -> Option<char> {
        //We get the sum of all the occurrences of all the successors of current_letter, and we gen a random number between 0 and sum+1
        let sum = Champion::get_sum(current_letter);
        let rng = rand::thread_rng().gen_range(0, sum+1);
        let mut partial_sum = 0;
        /* As we iterate over the potential successors, we accumulate their occurences, and we stop if this accumulation
           is superior or equal to the random generated number.
        */
        for letter in current_letter {
            partial_sum = partial_sum + letter.1;
            if partial_sum >= rng {
                return Some(letter.0.clone())
            }
        }
        None
    }

    //WRN: It may not function as expected : If you have "raloud" in your db, contains("oud") will return true.
    fn contains(&self, nickname : String) -> bool {
        let splitted = Champion::split_nickname(nickname).unwrap();
        //We split the nickname and see if for each (0, 1) pair, 1 is a key of the 0 hashmap.
        for pair in &splitted {
            match self.values.get(&pair[0]) {
                None => return false,
                Some(value) => if !value.contains_key(&pair[1]){
                    return false },
            }
        }
        return true;
    }

    fn get_sum(letter : &HashMap<char, u64>) -> u64 {
        letter.iter().fold(0u64, |sum, val| sum+val.1)
    }

    /// Returns the champion's id.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    //TODO: See what happens if you have the following nickname : "\0".
    fn split_nickname(nickname : String) -> Option<Vec<Vec<char>>> {
        if !nickname.is_empty() {
            //We modify the nickname in order to include a terminating character.
            let mut null_terminated_nickname = nickname.clone();
            null_terminated_nickname.push('\0');
            let chars : Vec<char> = null_terminated_nickname.chars().collect();
            let mut ret : Vec<Vec<char>> = Vec::new();
            for pair in chars.windows(2) {
                ret.push(vec![pair[0], pair[1]])
            }
            return Some(ret);
        }
        None
    }

    // Maybe derive traits later ?
    ///Returns a JSON String representing the Champion generation data, __without the champion's id__.
    pub fn serialize(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self.values)
    }
    ///Attempts to load the data from a JSON string into the object
    /// # Arguments
    ///
    /// * `json_string` : The JSON formatted string to parse
    ///
    /// # Panics
    ///
    /// If the JSON string is invalid or malformed.
    //TODO: Change that to a Result or an option.
    pub fn deserialize(&mut self, json_string : String) {
        self.values = serde_json::from_str(&json_string).unwrap()
    }
}
