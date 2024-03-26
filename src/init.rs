use std::fs::{ self, File, DirBuilder };
use std::io::{Write};
use std::path::Path;
use chrono::Utc;
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };

pub fn init() {
    let mut dir_create: bool = false;

    let current_dir = std::env::current_dir().unwrap();
    //panic!("Current dir: {current_dir:?}");
    //let dir_path = "./logs";
    let dir_path = Path::new("./logs");
    if !dir_path.exists() {
        DirBuilder::new()
            .create(dir_path)
            .expect("Unable to create logs dir")
    }


    let current_datetime = Utc::now().timestamp();
    let file_name = format!("{0}.log", current_datetime);
    let log_file_name = dir_path.join(file_name); //format!("../logs/{0}.log", current_datetime);
    let mut init_log = File::create(log_file_name).unwrap();
    let log_file_response = init_log.write(b"Initializing\n");

    match log_file_response {
        Ok(r) => {
            Log::location(TokenizeStage::Init).write();
            if dir_create {
                Log::record(
                        Some(LogType::Notify),
                        Some(TokenizeStage::Init),
                        Some(TokenizeAction::InitLogDirCreation)
                    )
                    .write()
            }
        }, 
        Err(e) => {
            panic!("Unable to create log file");
        }
    };

}
