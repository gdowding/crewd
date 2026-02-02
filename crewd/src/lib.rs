pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}


// data input
use csv::Reader;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Event {
    date_start: String,
    date_end: String,
    series: String,
    race: String,
    sponsor: String,
    event_page: String,
}


pub fn get_schedule() -> Vec<Event> {
    let file_path = Path::new("../schedule.csv");
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut events: Vec<Event> = Vec::new();
    for result in rdr.deserialize() {
	let event: Event = result.unwrap();
	events.push(event);
    }
    events
}
