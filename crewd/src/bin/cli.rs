fn main() {
    let events = crewd::get_schedule();
    for event in events {
	println!("event: {:?}", event);
    }
    // let races :Vec<String> = events
    //     .iter()
    //     .map(|event| event.race.clone())
    //     .collect();
}
