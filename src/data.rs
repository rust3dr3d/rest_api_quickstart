use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use rocket::response::NamedFile;
use std::io::prelude::*;
use std::str;

pub const _DB_PATH:&'static str = "messages_db.txt";

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub id: u8,
    pub message:String,
    pub from:String
}

// Return all messages as Result<String>
pub fn read_all_messages() -> std::io::Result<String>{
    let messages_db = NamedFile::open(_DB_PATH);
    let mut buffer:String = String::new();

    match messages_db{
        Ok(_) => {
            buffer = parse_all_messages_str()?;
        },
        Err(_) => {
            let _f = File::create(_DB_PATH)?;
        }
    };

    Ok(buffer)
}

// Parse and return as Messages
pub fn parse_all_messages() ->std::io::Result<Vec<Message>>{
    let mut contents_u8:Vec<u8> = Vec::new();
    
    let mut messages_db = match File::open(_DB_PATH){
        Ok(db) => db,
        Err(_) => File::create(_DB_PATH)?
    };

    messages_db.read_to_end(&mut contents_u8)?;
    let msgs_str = str::from_utf8(&contents_u8[..]).unwrap();
    
    let msgs:Vec<Message> = serde_json::from_str(msgs_str)?;

    Ok(msgs)
}


// Parse and return as String
pub fn parse_all_messages_str() ->std::io::Result<String>{
    let mut contents_u8:Vec<u8> = Vec::new();

    let mut messages_db = NamedFile::open(_DB_PATH)?;
    
    messages_db.read_to_end(&mut contents_u8)?;
    let msgs_str = str::from_utf8(&contents_u8[..]).unwrap();
    let _msgs:Vec<Message> = serde_json::from_str(msgs_str)?;

    Ok(String::from(msgs_str))
}


// Truncate existing contents and writes all the messages to the db
pub fn save_all_to_db(messages: String) -> std::io::Result<()>{
    let mut messages_db = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(_DB_PATH)?;

    messages_db.write_all(&messages.as_bytes())?;
    Ok(())
}