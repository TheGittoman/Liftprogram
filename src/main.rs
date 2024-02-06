// save things to a file
// parse lines from a file
// read things from a file
// hashmap
// dataformat
//      hissi sijainti {huoltoaika} {seuraava huolto}
// hissi1 porvoo 10.10.2023 10.4.2024
// the program should be able to read new lifts and able to tell
// when the next one should be fixed (4 months periods)

use chrono::{Date, DateTime, NaiveDate};
use database;
use std::{env, fs, panic::Location, str::FromStr};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let date = chrono::NaiveDate::from_str("2023-12-12").unwrap();
    // println!("{}", date.format("%d-%y-%m"));
    database::run();
}
