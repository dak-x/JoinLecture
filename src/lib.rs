use std::path::{Path, PathBuf};
use std::process::*;
use chrono::prelude::*;

type CourseCode = String;
/// Struct represents a Schedule for the Event you want to join.
pub struct Schedule {
    time_slots: Vec<(String, String)>,
    day_schedule: Vec<Vec<String>>,
}

impl Schedule {
    pub fn new(filename: PathBuf) -> Schedule {
        let contents = std::fs::read_to_string(filename);
                
        Schedule {
            time_slots: Vec::new(),
            day_schedule: Vec::new(),
        }
    }

    fn now(&self) -> CourseCode {
        let time = Local::now();
        
        return "CSL380".into();
    }

    fn join(&self) {
        let course_code = self.now();
        println!("Joined Course: {}", course_code);
    }
}
