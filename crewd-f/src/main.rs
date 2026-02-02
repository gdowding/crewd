use leptos::prelude::*;
use csv::Reader;
use serde::Deserialize;

#[derive(Clone, Debug, serde::Deserialize)]
struct Event {
    date_start: String,
    date_end: String,
    series: String,
    race: String,
    sponsor: String,
    event_page: String,
}


fn main()  {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}




#[component]
fn App() -> impl IntoView {
    // changed csv format to have matching header values
    //let file_path = "/Users/gdowding/git/github/gdowding/crewd/schedule.csv";
    let data = "\
date_start,date_end,series,race,sponsor,event_page
2026-03-14,,Snowbird Race Series,Race 5,styc,https://www.styc.org/Snowbird-Series
2026-03-21,,Center Sound Series,CSS#3 - Possession Point Race,cyc seattle,https://cycseattle.theclubspot.com/regatta/M3yLIpxRaJ
2026-04-11,,Blakely Rock Benefit Race,Race 1,styc,https://www.styc.org/Blakely-Rock-Benefit-Race
2026-04-13,,Ballard Cup I,Race 1,styc,https://www.styc.org/Ballard-Cup-Series-I
2026-04-20,,Ballard Cup I,Race 2,styc,https://www.styc.org/Ballard-Cup-Series-I
2026-04-25,2026-04-26,Tri-Island,Smith Island,syc,
2026-04-27,,Ballard Cup I,Race 3,styc,https://www.styc.org/Ballard-Cup-Series-I
";

    // let mut events: Vec<csv::StringRecord> = vec![];
    let mut rdr = Reader::from_reader(data.as_bytes());

    // for record in rdr.records() {
    // 	let event = record.unwrap();
    // 	//println!(event);
    // 	events.push(event);
    // }

    //let mut rdr = Reader::from_path(file_path).unwrap();
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


    // let races :Vec<String> = events
    //     .iter()
    //     .map(|event| event.get(3).expect("reason").to_string())
    //     .collect();


    view! {
	<ul>
	{
	    races.into_iter()
		.map(|r| view! { <li>{r}</li> })
		.collect_view()
	}
	</ul>
    }

}
