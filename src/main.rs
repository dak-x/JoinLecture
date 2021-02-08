use join_lecture;
use structopt::*;
fn main() {
    let cli_app = Cli::from_args();
    cli_app.join();
}
/// Run with the course code to join a specific class.
#[derive(StructOpt)]
pub struct Cli {
    /// The course code for the class to join.
    #[structopt(short = "c", long = "code")]
    course_code: Option<String>,
}

impl Cli {
    fn join(&self) {
        let mut course = self.course_code.clone();
        if course.is_none() {
            course = join_lecture::TimeTable::new("timetable.json".into()).get_course();
        }
        
        if course.is_some() {
            println!("Current Course: {:?}", course);
            let url = format!(
                "https://iitjammu.ipearl.ai/extras/course-v1:ITJA+{}+2021/join_zoom",
                course.as_ref().unwrap().trim()
            );
            spawn_firefox(&url);
        } else {
            println!("No Course Currently. Focus on your projects");
        }
    }
}

fn spawn_firefox(url: &str) {
    use std::process::Command;
    let mut command = Command::new("firefox")
        .args(&["--new-tab", url])
        .spawn()
        .expect("Could Not launch firefox");
    command.wait().ok().unwrap();
}
