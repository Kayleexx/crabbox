use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkSession {
    pub project: String,
    pub start: String,
    pub end: Option<String>,
}

const FILE_PATH: &str = "sessions.json";

pub fn save_session(session: &WorkSession) -> io::Result<()> {
    let mut sessions = load_sessions()?;
    sessions.push(session.clone());
    let file = OpenOptions::new().write(true).create(true).open(FILE_PATH)?;
    serde_json::to_writer_pretty(file, &sessions)?;
    Ok(())
    
}

pub fn load_sessions() -> io::Result<Vec<WorkSession>> {
    match File::open(FILE_PATH) {
        Ok(mut file) => {
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            let sessions: Vec<WorkSession> = serde_json::from_str(&data).unwrap_or_default();
            Ok(sessions)
        }
        Err(_) => Ok(Vec::new()),
    }
}