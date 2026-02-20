use chrono::NaiveDate;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Event {
    date_start: NaiveDate,
    date_end: Option<NaiveDate>,
    series: String,
    race: String,
    sponsor: String,
    event_page: Option<String>,
}

pub fn read_schedule(filename: String) -> Result<Vec<Event>> {
    println!("filename in read_schedule: {filename}");
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut schedule = Vec::new();
    for result in rdr.deserialize() {
        let record: Event = result?;
        schedule.push(record);
    }
    Ok(schedule)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_schedule() -> Result<(), Box<dyn std::error::Error>> {
	let _sched = read_schedule()?;
	Ok(())
    }
}
