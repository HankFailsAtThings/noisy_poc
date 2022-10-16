use chrono::{Datelike, Timelike, Utc};
use std::fs;

pub fn build_datetime_folder(argument : String) -> String {
    let now = Utc::now();
    let s = format!(
        "Year{}Month{}Day{}Hour{}Min{}Sec{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        );
    let dirstr = format!("{}{}",argument,s);

//    fs::create_dir_all(&dirstr).expect("Directory unable to be created");
    dirstr
}
