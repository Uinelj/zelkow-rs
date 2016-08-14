extern crate zilean;
#[macro_use] extern crate nickel;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_term;
extern crate serde_json;
use slog::Logger as Logger;
use zilean::database::Database as Database;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

//Parameters of the server and the script
const VERSION : &'static str = "0.1";
const REDIS_URL : &'static str = "redis://127.0.0.1";
const TWITCH_PATH : &'static str = "../twitch/twitch.py";
const SERVER_ADDRESS : &'static str = "127.0.0.1:6767";
const GEN_LENGTH : u32 = 16u32; //riot value

//Used to format the REST response
struct Answer {
    status: u32,
    content_type: String,
    content: String
}

//TODO: Implement serialize trait.
impl Answer {
    fn serialize(&self) -> Result<String, serde_json::Error> {
        let obj = serde_json::builder::ObjectBuilder::new()
            .insert("status", self.status.clone())
            .insert("content_type", self.content_type.clone())
            .insert("content", self.content.clone())
            .unwrap();
        serde_json::to_string(&obj)
    }
}
fn main() {
    //Root logger, from which we'll instantiate twitch/rest loggers.
    let root = Logger::new_root(o!("version" => VERSION));
    root.set_drain(slog_term::async_stderr());


    root.info("Starting REST server.", b!("port" => SERVER_ADDRESS));
    let log = root.new(o!("job" => "rest_server"));

    //We spawn the server thread.
    thread::spawn({
        let log = log.clone();
        move || {

            let db = Mutex::new(Database::new(REDIS_URL.to_string()));
            log.info("Database connection established.", b!("URL" => REDIS_URL));

            let mut server = Nickel::new();

            //We react on the requests touching /gen/something
            server.get("/gen/:id", middleware! { |request, response|

                //By default, the answer status is 1 : error.
                let mut answer = Answer {
                    status: 1,
                    content_type: "err".to_string(),
                    content: "not yet initialized".to_string()};

                //Checking the validity of the id. (Exists, is a u32)
                let champion_id = match request.param("id") {
                    Some(id) => match id.parse::<u32>() {
                        Ok(id) => {
                            id
                        },
                        Err(err) => {
                            answer.status = 1;
                            answer.content_type = "err".to_string();
                            answer.content = err.to_string();
                            log.warn("Invalid id supplied", b!("err" => err.to_string()));
                            return response.send(format!("{}", answer.serialize().unwrap()))
                        }
                    },
                    None => {
                        answer.status = 1;
                        answer.content_type = "err".to_string();
                        answer.content = "No id parameter specified.".to_string();
                        return response.send(format!("{}", answer.serialize().unwrap()))
                    }
                };
                //Locking the db in order to access it.
                //Not necessary, since we just read the values.
                //I have to figure out why I've done this.
                let db_lock = match db.lock() {
                    Ok(db) => db,
                    Err(err) => {
                        answer.status = 1;
                        answer.content_type = "err".to_string();
                        answer.content = err.to_string();
                        return response.send(format!("{}", answer.serialize().unwrap()))
                    }
                };

                //Loading the requested champion_id data
                let champion = db_lock.load(champion_id);

                //Trying to generate a nickname
                match champion.gen(GEN_LENGTH) {
                    Some(nickname) => {
                        answer.status = 0;
                        answer.content_type = "nickname".to_string();
                        answer.content = nickname;
                        return response.send(format!("{}", answer.serialize().unwrap()))
                    },
                    None => {
                        answer.status = 1;
                        answer.content_type = "err".to_string();
                        answer.content = "id doesn't exist in database".to_string();
                        return response.send(format!("{}", answer.serialize().unwrap()))
                    }
                };
            });
            server.listen(SERVER_ADDRESS);
        }
    }
);

    root.info("Launching Twitch script", b!());
    let tw_log = root.new(o!("job" => "twitch"));

    //Creation of the db mutex, in order to ensure safety.
    let db = Mutex::new(Database::new(REDIS_URL.to_string()));

    loop {
        tw_log.info("Waking up", o!());
        tw_log.info("Watching for new games", o!());

        //Calling the script
        let json_output = Command::new("pythlon2.7")
            .arg(TWITCH_PATH)
            //.arg("-r euw")
            .arg("-f 60")
            .output()
            .expect("failed to execute twitch script");

        //If we don't succeed at calling it, panic with a critical log message.
        if !json_output.status.success() {
            tw_log.critical("Impossible to launch Twitch.", b!("stderr" => String::from_utf8(json_output.stderr).unwrap(), "path" => TWITCH_PATH));
            panic!("Error {} on twitch launch.", json_output.status);
        }

        //deserializing the answer
        let twitch_answer = String::from_utf8(json_output.stdout).unwrap();
        let parsed_answer = deserialize_answer(twitch_answer);

        //Watching for an eventual error status code
        if parsed_answer.get("status").unwrap() != &"0" {
            //println!("{}", parsed_answer.get("status").unwrap());
            tw_log.error("Script error", b!("stdout" => format!("{}", parsed_answer.get("content").unwrap())));
        } else {

            //Figuring out what content type was sent.
            if parsed_answer.get("content_type").unwrap() == "\"nicknames\"" {

                //If it's nicknames, deserialize the data and feed the database.
                let nicknames_data = Database::deserialize_bulk(parsed_answer.get("content").unwrap().to_owned());
                match db.lock() {
                    Ok(db) => {
                        for (champion_id, nicknames) in nicknames_data {
                                    let mut champion = db.load(champion_id);
                                    for nickname in nicknames {
                                        champion.feed(nickname);
                                    }
                                    db.store(&champion);
                                }
                    },

                    Err(err) => tw_log.error("Impossible to lock the database", b!("Error" => format!("{:?}", &err))),
                }
            } else {
                tw_log.error("content type not yet supported", b!("content_type" => format!("{}", parsed_answer.get("content_type").unwrap())));
            }
        }

        //Sleeping for the desired amount of time.
        let cooldown = parsed_answer.get("cooldown").unwrap().parse::<u64>().unwrap();
        tw_log.info("Sleeping", b!("cooldown" => cooldown));
        //println!("{:?}", parsed_answer.get("cooldown").unwrap());
        thread::sleep(Duration::from_secs(parsed_answer.get("cooldown").unwrap().parse::<u64>().unwrap()));
    }
}

//TODO: Error handling
fn deserialize_answer(answer : String) -> HashMap<String, String> {

    let answer : serde_json::Value = serde_json::from_str(&answer).unwrap();
    let obj = answer.as_object().unwrap();
    let mut ret = HashMap::new();


    for (key, value) in obj.iter() {
        ret.insert(key.to_string(), format!("{}", value));
    }
    return ret;
}
