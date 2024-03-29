#[allow(non_snake_case)]
#[allow(unused_parens)]
use std::fs::{ File, DirBuilder };
use std::io::Write;
use std::path::Path;

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
    
   let file_list = std::fs::read_dir(Path::new("./..")
                                     .join("target")
                                     .join("logs"));
   let mut log_number = 1;
   match file_list {
        Ok(flcon) => {
            let file_count = flcon.count();
            log_number = file_count + 1;
        }, 
        Err(_) => {
            panic!("Error with log numbering");
        }
   };


    let file_name = format!("tok_{0}.log", log_number);
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
