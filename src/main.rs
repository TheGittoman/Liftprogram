// save things to a file
// parse lines from a file
// read things from a file
// hashmap
// dataformat
//      hissi sijainti {huoltoaika} {seuraava huolto}
// hissi1 porvoo 10.10.2023 10.4.2024
// the program should be able to read new lifts and able to tell
// when the next one should be fixed (4 months periods)
// add edit delete fix options
// ability to search lifts that have maintenance in certain time in future (1month etc.)

use std::{env, io};

// use database;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let date = chrono::NaiveDate::from_str("2023-12-12").unwrap();
    // println!("{}", date.format("%d-%y-%m"));
    database::run();
}

pub struct Config {
    option: String,
    option2: String,
}

impl Config {
    pub fn build(option: String, option2: String) -> Config {
        Config {
            option: option,
            option2: option2,
        }
    }
}

fn add() {
    //
}

/*
app.exe LIST -> lists all locations and how many lifts there are, also shows
    how many lifts have maintenance coming in months time
    - LIST + <location> prints all the names (IDs) and info about lifts in this
      location
app.exe MAINTENANCE 1m/w/d -> lists all lifts that have maintenance coming in
    specified timeframe
app.exe ADD <lift_name> <location> -> adds a new lift with current day as the
    maintenance date, and next_maintenance date 3 months in future.
app.exe DELETE <lift_name> - deletes the lift from the list
    will show the lift that will be deleted and ask for confirmation
app.exe EDIT <lift_name> ->

app.exe add <lift>
app.exe maintenance 3m
*/
