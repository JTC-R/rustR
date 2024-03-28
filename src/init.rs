#[allow(non_snake_case)]
#[allow(unusedparens)]
use std::fs::{ self, File, DirBuilder };
use std::io::{Write};
use std::path::Path;
use chrono::Utc;
use crate::log::{ Log, LogType, TokenizeStage, TokenizeAction };

pub fn init() {
    let mut dir_create: bool = false;

    let dir_path = Path::new("./..").join("target").join("logs");
    if !dir_path.exists() {
        DirBuilder::new()
            .create(&dir_path)
            .expect("Unable to create logs dir");
        dir_create = true;
    }
    let current_datetime = Utc::now().timestamp();
    let file_name = format!("{0}.log", current_datetime);
    let log_file_name = dir_path.join(file_name);
    let mut init_log = File::create(log_file_name).unwrap();
    let log_file_response = init_log.write(b"Initializing\n");

    match log_file_response {
        Ok(_r) => {
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
        Err(_e) => {
            panic!("Unable to create log file");
        }
    };

}
