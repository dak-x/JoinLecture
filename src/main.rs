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
        if self.course_code.is_some() {
            let url = format!(
                "https://iitjammu.ipearl.ai/extras/course-v1:ITJA+{}+2021/join_zoom",
                self.course_code.as_ref().unwrap().trim()
            );
            spawn_firefox(&url);
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
