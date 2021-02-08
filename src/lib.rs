use chrono::prelude::*;
use json::JsonValue;
use std::path::PathBuf;

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
    fn in_slot(&self, slot: &str) -> bool {
        return self.slots.contains(&slot.to_string());
    }
}

// Represent exception timings for out of slot courses
#[derive(Debug, Default)]
pub struct CourseExcept {
    coursecode: String,
    day: usize,
    slot: (Time, Time),
}

impl CourseExcept {
    fn new((code, vals): (&str, &JsonValue)) -> Self {
        let (day, slot) = match vals {
            JsonValue::Array(ref arr) => {
                let slot: Vec<Time> = arr[1]
                    .to_string()
                    .split("-")
                    .map(|x| Time::from(x))
                    .collect();

                let day = match arr[0].to_string().to_lowercase().as_str() {
                    "mon" => 0,
                    "tue" => 1,
                    "wed" => 2,
                    "thu" => 3,
                    "fri" => 4,
                    _ => 6,
                };

                (day, (slot[0], slot[1]))
            }
            _ => {
                panic!("Invalid format for Exceptional Courses")
            }
        };

        CourseExcept {
            coursecode: code.to_string(),
            day,
            slot,
        }
    }

    fn in_between(&self, time: Time, day: usize) -> bool {
        self.day == day && time.in_between(self.slot)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Ord, Clone, Copy)]
pub struct Time {
    hrs: u32,
    min: u32,
}

impl Time {
    pub fn now() -> Time {
        let date_time = Local::now();
        return Time {
            hrs: date_time.time().hour(),
            min: date_time.time().minute(),
        };
    }

    fn in_between(&self, slot: (Time, Time)) -> bool {
        slot.0 <= *self && *self <= slot.1
    }
}

impl From<&str> for Time {
    fn from(s: &str) -> Self {
        let x: Vec<&str> = s.trim().splitn(2, ":").collect();
        let (hrs, min) = {
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
#[derive(Debug, Default)]
pub struct TimeTable {
    time_slots: Vec<(Time, Time)>,
    day_schedule: Vec<Vec<String>>,
    courses: Vec<Course>,
    excepts: Vec<CourseExcept>,
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

        let excepts = match &js["Exceptions"] {
            JsonValue::Object(course) => course.iter().map(|x| CourseExcept::new(x)).collect(),

            _ => panic!("Exceptions Invalid Format"),
        };

        Self {
            time_slots,
            day_schedule,
            courses,
            excepts,
        }
    }

    pub fn get_course(&self) -> Option<String> {
        let now = Time::now();
        let mut slot_id = -1;
        for (idx, &slot) in self.time_slots.iter().enumerate() {
            if now.in_between(slot) {
                slot_id = idx as isize;
                break;
            }
        }

        let day = get_day() as usize;

        if slot_id < 0 || day >= 5 {
            return None;
        }

        let slot = self.day_schedule[day][slot_id as usize].trim();

        for course in self.courses.iter() {
            if course.in_slot(slot) {
                return Some(course.coursecode.clone());
            }
        }

        for course in self.excepts.iter() {
            if course.in_between(now, day) {
                return Some(course.coursecode.clone());
            }
        }
        None
    }
}

fn get_day() -> u32 {
    Local::now().weekday() as u32
}

#[test]
fn test_day() {
    // let k = Local::now().weekday() as u32;
    let x = TimeTable::new("timetable.json".into());
    eprintln!("{:?}", x.get_course());
    assert_eq!(1, 2);
}
