fn main() {
    let events = crate::get_schedule();
    for event in events {
	println!("event: {:?}", event);
    }
    // let races :Vec<String> = events
    //     .iter()
    //     .map(|event| event.race.clone())
    //     .collect();
}
