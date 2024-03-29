#[allow(unused_parens)]
#[allow(non_snake_case)]
use std::fmt;
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
// What do I want for the logging? 
//
// - want to know location
#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Critical,
    Important,
    Notify,
    Routine,
    None,
}

impl fmt::Display for LogType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {  
        match self {
            LogType::Critical   => write!(formatter, "Critial"), 
            LogType::Important  => write!(formatter, "Important"),
            LogType::Notify     => write!(formatter,"Notify"),
            LogType::Routine    => write!(formatter,"Routine"),
            LogType::None       => write!(formatter, "None"),
            _ => write!(formatter,"Unknown Log Type")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenizeStage {
    Init,
    Start,
    Space,
    Punct,
    PunctStringSngl,
    PunctStringDbl,
    Char,
    Num,
    End,
    None,
}

impl fmt::Display for TokenizeStage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenizeStage::Init     => write!(formatter,"Init"),
            TokenizeStage::Start    => write!(formatter,"Start"),
            TokenizeStage::Space    => write!(formatter,"Space"),
            TokenizeStage::Punct    => write!(formatter,"Punct"),
            TokenizeStage::Char     => write!(formatter,"Char"),
            TokenizeStage::Num      => write!(formatter,"Num"),
            TokenizeStage::End      => write!(formatter,"None"),
            TokenizeStage::None     => write!(formatter, "None"),
            _ => write!(formatter,"Unknown TokenizeStage")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenizeAction {
    Init,
    InitLogDirCreation,
    InitArgsSupplied,

    MainPush,

    CurrentTokenCreate,
    CurrentTokenConcat,
    CurrentTokenSet,

    PunctCheck,
    PunctTranslate,
    PunctUnknown,
    PunctUnexpected,

    CharUnexpected,

    NumUnexpected,

    None

 
}


impl fmt::Display for TokenizeAction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenizeAction::Init                => write!(formatter,"Init"),
            TokenizeAction::MainPush            => write!(formatter,"MainPush"),
            TokenizeAction::CurrentTokenCreate  => write!(formatter,"New Token Creation"),
            TokenizeAction::CurrentTokenConcat  => write!(formatter,"Current Token concation"),
            TokenizeAction::CurrentTokenSet     => write!(formatter,"Successful Current Token concation"),
            TokenizeAction::PunctCheck          => write!(formatter,"Character Check:: Punctuation"),
            TokenizeAction::PunctTranslate      => write!(formatter,"Punctuation Translation"),
            TokenizeAction::PunctUnknown        => write!(formatter,"Unknown Punctuation encounter"),
            TokenizeAction::PunctUnexpected     => write!(formatter,"Unexpected Punctuation encounter"),
            TokenizeAction::CharUnexpected      => write!(formatter,"Unexpected Character encounter"),
            TokenizeAction::NumUnexpected       => write!(formatter,"Unexpected num encounter"),
            TokenizeAction::InitLogDirCreation  => write!(formatter, "Log Directory creation"),
            TokenizeAction::InitArgsSupplied    => write!(formatter, "Arguments supply"),
            TokenizeAction::None                => write!(formatter, "None"),
            _ => write!(formatter,"Unknown TokenizeAction")

        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Log {
    pub ltype: Option<LogType>,
    pub stage: Option<TokenizeStage>,
    pub event: Option<TokenizeAction>
}

impl Log {
    pub fn record(log_type: Option<LogType>, stage: Option<TokenizeStage>, event: Option<TokenizeAction>) -> Self {
        let log_location = Log {
            ltype: log_type,
            stage: stage,
            event: event,
        };
        
        return log_location
    }

    pub fn location(location: TokenizeStage) -> Self {
       let log_location = Log {
           ltype: Some(LogType::Routine),
           stage: Some(location),
           event: None,
       };
       println!("Logging: {:?}", log_location.clone());
       return  log_location
    }

    pub fn write(&self) {
//        let file_list = std::fs::read_dir(Path::new("./..")
//                                          .join("target")
//                                          .join("logs"));
//
//        match file_list {
//            Ok(flcon) => {
//                let file_count = flcon.count();
//                let log_number = file_count;
//                let filename = format!("tok_{0}.log", &log_number);
//                let filename_log = Path::new("./..")
//                    .join("target")
//                    .join("logs")
//                    .join(filename);
        let filename_log = get_log_name();
        let log_type = self.ltype;
        let log_stage = self.stage;
        let log_event = self.event;
    
        let date_time = chrono::Utc::now().format("%Y/%m/%d %H:%M:%S"); 
        let log_text = format!(
            "{0} :: Type: {1} :: Stage: {2} :: Event: {3} ||\n",
            date_time,
            log_type.unwrap_or(LogType::None),
            log_stage.unwrap_or(TokenizeStage::None),
            log_event.unwrap_or(TokenizeAction::None));

        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename_log)
            .expect("Unable to append log file");

        let _ = log_file.write(log_text.as_bytes()); 
            

//  }, 
//    Err(_) => { 
//        println!("Error!")
//    }
    }
   // }
}

pub fn get_log_name() -> OsString {
    let file_path = Path::new("./..")
        .join("target")
        .join("logs");

    let file_list = std::fs::read_dir(file_path);

    match file_list {
        Ok(flcon) => {
            let file_count = flcon.count();
            let log_number = file_count;
            let filename = format!("tok_{0}.log", log_number);
            let filename_log = Path::new("./..")
                .join("target")
                .join("logs")
                .join(filename);
            return filename_log.into_os_string()
        }, 
        Err(_) => {
            panic!("Unable to locate log file name");
        }
    }
}
