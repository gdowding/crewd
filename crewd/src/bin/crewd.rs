use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Debug, serde::Deserialize)]
struct Event {
    date_start: String,
    date_end: String,
    series: String,
    race: String,
    sponsor: String,
    event_page: String,
}

fn main() {
    let file_path = Path::new("../schedule.csv");
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut events: Vec<Event> = Vec::new();
    for result in rdr.deserialize() {
	let event: Event = result.unwrap();
	println!("event: {:?}", event);
	events.push(event);
    }
    let races :Vec<String> = events
        .iter()
        .map(|event| event.race.clone())
        .collect();
}
