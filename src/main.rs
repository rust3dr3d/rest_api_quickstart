#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::{content};
use rocket_contrib::json::Json;

mod data;


// GET all messages
#[get("/messages")]
fn get_all_messages() -> content::Json<String>{
   content::Json(data::read_all_messages().unwrap())
}


// GET message by id
#[get("/messages/<id>")]
fn get_message_by_id(id:u8) -> content::Json<String>{
   let all_messages: Vec<data::Message> = match data::parse_all_messages(){
      Ok(contents) => contents,
      Err(_) => Vec::new()
   };

   if let Some(src_pos) = all_messages
      .iter()
      .position(|src_msg| src_msg.id == id){
         content::Json(serde_json::to_string(&all_messages[src_pos]).unwrap())
      }else{
         content::Json(String::from("{}"))
      }
}


// Post a new message
#[post("/messages", data="<message>")]
fn new_message(message: Json<data::Message>){
   let msg: data::Message = message.into_inner();
   
   let mut all_messages: Vec<data::Message> = match data::parse_all_messages(){
      Ok(contents) => contents,
      Err(_) => Vec::new()
   };

   all_messages.push(msg);

   let messages_str:String = match serde_json::to_string(&all_messages){
      Ok(contents) =>  contents,
      Err(_) => String::new()
   };

   data::save_all_to_db(messages_str).unwrap();
}


//DELETE a message by Id
#[delete("/messages/<id>")]
fn delete_message(id:u8) -> Status{

   let mut all_messages: Vec<data::Message> = match data::parse_all_messages(){
      Ok(contents) => contents,
      Err(_) => Vec::new()
   };

   if all_messages
      .iter()
      .any(|e| e.id == id){
         all_messages.retain(|msg| msg.id != id);

         let messages_str:String = match serde_json::to_string(&all_messages){
            Ok(contents) =>  contents,
            Err(_) => String::new()
      };
   
      data::save_all_to_db(messages_str).unwrap();

      Status::Ok      
   }else{
      Status::BadRequest
   }
}


// DELETE all entries from db
#[delete("/messages")]
fn delete_all_messages() -> Status{
   
   let messages_str = String::from("[]");

   data::save_all_to_db(messages_str).unwrap();

   Status::Ok
}


// PATCH/EDIT a message
#[put("/messages", data="<message>")]
fn edit_message(message:Json<data::Message>) -> Status{
   
   let msg:data::Message = message.into_inner();

   let mut all_messages: Vec<data::Message> = match data::parse_all_messages(){
      Ok(contents) => contents,
      Err(_) => Vec::new()
   };

   if let Some(src_pos) = all_messages
                     .iter()
                     .position(|src_msg| src_msg.id == msg.id){
                        all_messages[src_pos].message = msg.message;
                        all_messages[src_pos].from = msg.from;

                        let messages_str = match serde_json::to_string(&all_messages){
                           Ok(contents) =>  contents,
                           Err(_) => String::new()
                        };

                        data::save_all_to_db(messages_str).unwrap();

                        Status::Ok
                     }
   else{
      Status::BadRequest
   }

}


#[catch(404)]
fn not_found_error() -> &'static str{
   "Page you're looking for does not exist!!"
}


fn main() {
   rocket::ignite()
   .register(catchers![not_found_error])
   .mount("/", routes![
         get_all_messages,
         get_message_by_id,
         new_message,
         delete_message,
         delete_all_messages,
         edit_message
      ])
   .launch();
}
