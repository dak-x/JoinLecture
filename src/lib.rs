use chrono::prelude::*;
use json::JsonValue;
use std::{cmp, path::PathBuf};

#[derive(Debug, Default)]
pub struct TimeTable {
    time_slots: Vec<(Time, Time)>,
    day_schedule: Vec<Vec<String>>,
    courses: Vec<Course>,
}
#[derive(Debug, Default)]
pub struct Course {
    coursecode: String,
    slots: Vec<String>,
}

impl Course {
    fn new(code: String, slots: String) -> Self {
        let coursecode = code.into();
        let slots = slots.split(",").map(|x| x.into()).collect();
        Self { coursecode, slots }
    }
}

pub struct Slot {}
#[derive(Debug, Default, PartialEq, Eq, Ord)]
pub struct Time {
    hrs: u32,
    min: u32,
}
impl From<&str> for Time {
    fn from(s: &str) -> Self {
        let x: Vec<&str> = s.trim().splitn(2, ":").collect();
        let (hrs, min) = {
            // println!("{:?}", x);

            (
                x[0].trim().parse().expect("Invalid hrs in given time "),
                x[1].trim().parse().expect("Invalid min in given time "),
            )
        };
        Self { hrs, min }
    }
}

use std::cmp::{Ord, Ordering};
impl PartialOrd for Time {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.hrs < rhs.hrs {
            Some(Ordering::Less)
        } else if self.hrs == rhs.hrs {
            Some(self.min.cmp(&rhs.min))
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl TimeTable {
    pub fn new(filename: PathBuf) -> Self {
        use std::fs::read_to_string;
        let file_contents = read_to_string(filename).expect("Cannot open timetable");
        let file_json = json::parse(file_contents.as_str())
            .expect("Could Not parse json file. Verify the format!");

        TimeTable::from_json(file_json)
    }

    fn from_json(js: JsonValue) -> Self {
        let timetable = &js["TimeTable"];

        // Get All the Time-Slots
        let time_slots: Vec<(Time, Time)> = match timetable["time_slots"] {
            JsonValue::Array(ref slots) => slots
                .iter()
                .map(|x| {
                    let s = x.to_string();
                    let x: Vec<&str> = s.splitn(2, "-").take(2).collect();
                    (x[0].trim().into(), x[1].trim().into())
                })
                .collect(),
            _ => panic!("TimeSlots format error"),
        };
        let day_schedule = match &timetable["day_slots"] {
            JsonValue::Array(slots) => slots
                .iter()
                .map(|x| {
                    let x = x.to_string();
                    x.split(",").map(|y| y.to_string()).collect::<Vec<String>>()
                })
                .collect(),
            _ => panic!("TimeTable Slots Invalid format"),
        };

        let courses = match &js["Courses"] {
            JsonValue::Object(course) => course
                .iter()
                .map(|(x, y)| Course::new(x.to_string(), y.to_string()))
                .collect(),

            _ => panic!("Courses Invalid format"),
        };

        Self {
            time_slots,
            day_schedule,
            courses,
        }
    }

    pub fn now(&self) -> Time {
        let date_time = Local::now();

        return Time {
            hrs: date_time.time().hour(),
            min: date_time.time().minute(),
        };
    }

    pub fn join(&self) {
        let course_code = self.now();
        println!("Joined Course: {:?}", course_code);
    }
}

#[test]
fn test_day() {
    let x = TimeTable::new("timetable.json".into());

    eprintln!("{:#?}", x.now());
    assert_eq!(1,2);
}
