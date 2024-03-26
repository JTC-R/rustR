use std::fs::{ File };
use std::io::{Write};
use chrono::Utc;
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };
pub fn init() {
    let current_datetime = Utc::now().timestamp();    
    let log_file_name = format!("./../logs/{0}.log", current_datetime);
    let mut init_log = File::create(log_file_name.clone()).unwrap();
    let log_file_response = init_log.write(b"Initializing");
    println!("Current log: {:?}", log_file_name.clone());

    match log_file_response {
        Ok(r) => {
            println!("Log file created");
            println!("In init");
        Log::location(TokenizeStage::Init).write();
        }, 
        Err(e) => {
            panic!("Unable to create log file");
        }
    };

}
