use anyhow::{Context, Result};
use chrono::{Local, NaiveDate, Duration, Datelike};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Schedule {
    #[allow(dead_code)]
    years: Vec<Year>,
}

#[derive(Debug)]
struct Year {
    year: i32,
    #[allow(dead_code)]
    series: Vec<Series>,
}

#[derive(Debug)]
struct Series {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    properties: HashMap<String, String>,
    #[allow(dead_code)]
    races: Vec<Race>,
}

#[derive(Debug)]
struct Race {
    #[allow(dead_code)]
    headline: String,
    #[allow(dead_code)]
    properties: HashMap<String, String>,
}

struct PrintableRace {
    date_start: String,
    date_end: String,
    first_start: String,
    dock_time: String,
    sponsor: String,
    series: String,
    race: String,
    series_event_page: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <root-directory> [--csv] [--upcoming] OR {} convert-crew <csv-file>", args[0], args[0]);
    }

    if args[1] == "convert-crew" {
        if args.len() < 3 {
            anyhow::bail!("Usage: {} convert-crew <csv-file>", args[0]);
        }
        let csv_path = &args[2];
        convert_crew(csv_path)?;
        return Ok(());
    }

    let root = args.iter().skip(1).find(|arg| !arg.starts_with("--"))
        .context("Missing root directory argument")?;

    let csv_mode = args.iter().any(|arg| arg == "--csv");
    let upcoming_mode = args.iter().any(|arg| arg == "--upcoming");

    let schedule = load_schedule(root)?;
    let rows = collect_schedule_rows(&schedule, upcoming_mode);

    if csv_mode {
        print_csv(rows);
    } else {
        print_table(rows);
    }
    Ok(())
}

fn convert_crew(path: &str) -> Result<()> {
    let file = fs::File::open(path).context("Failed to open CSV file")?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?.clone();
    let idx_name = headers.iter().position(|h| h == "Crew Member Name").unwrap_or(0);
    let idx_email = headers.iter().position(|h| h == "Email Address").unwrap_or(1);
    let idx_phone = headers.iter().position(|h| h == "Phone Number").unwrap_or(2);
    let idx_emer_name = headers.iter().position(|h| h == "Emergency Contact Name").unwrap_or(3);
    let idx_emer_phone = headers.iter().position(|h| h == "Emergency Contact Phone").unwrap_or(4);

    for result in rdr.records() {
        let record = result?;
        let name = record.get(idx_name).unwrap_or("").trim();
        let email = record.get(idx_email).unwrap_or("").trim();
        let phone = record.get(idx_phone).unwrap_or("").trim();
        let emer_name = record.get(idx_emer_name).unwrap_or("").trim();
        let emer_phone = record.get(idx_emer_phone).unwrap_or("").trim();

        println!("* {}", name);
        println!(":PROPERTIES:");
        println!(":EMAIL: {}", email);
        println!(":PHONE: {}", phone);
        println!(":EMERGENCY_CONTACT: {}", emer_name);
        println!(":EMERGENCY_PHONE: {}", emer_phone);
        println!(":END:");
        println!();
    }
    Ok(())
}

fn collect_schedule_rows(schedule: &Schedule, upcoming: bool) -> Vec<PrintableRace> {
    let mut rows: Vec<PrintableRace> = Vec::new();
    let re_date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let re_day = Regex::new(r"^[A-Z][a-z]{2}$").unwrap();

    for year in &schedule.years {
        for series in &year.series {
            let series_name = series.properties.get("NAME")
                .filter(|s| !s.is_empty())
                .or_else(|| series.properties.get("FULL_NAME"))
                .map(|s| s.clone())
                .unwrap_or_else(|| series.name.clone());

            let series_first_start = series.properties.get("FIRST_START").cloned().unwrap_or_default();
            let series_sponsor = series.properties.get("SPONSOR").cloned().unwrap_or_default();
            let series_event_page = series.properties.get("EVENT_PAGE").cloned().unwrap_or_default();

            // Iterate races defined in series.org
            for race in &series.races {
                 // Determine start/end date from SCHEDULED property
                 let (date_start_only, date_end_only) = if let Some(scheduled_raw) = race.properties.get("SCHEDULED") {
                     let (start_ts, end_ts) = if let Some((s, e)) = scheduled_raw.split_once("--") {
                         (s.trim(), e.trim())
                     } else {
                         (scheduled_raw.as_str(), scheduled_raw.as_str())
                     };

                     let clean_chars: &[char] = &['<', '>', '[', ']'];
                     let start_clean = start_ts.trim_matches(clean_chars);
                     let end_clean = end_ts.trim_matches(clean_chars);

                     let d_start = start_clean.split_whitespace().next().unwrap_or("").to_string();
                     let mut d_end = end_clean.split_whitespace().next().unwrap_or("").to_string();

                     if d_start == d_end {
                         d_end = "".to_string();
                     }
                     (d_start, d_end)
                 } else {
                     ("".to_string(), "".to_string())
                 };

                 // Dock time? Check race properties, then series properties
                 let dock_time_raw = race.properties.get("DOCK_TIME").cloned()
                     .or_else(|| series.properties.get("DOCK_TIME").cloned())
                     .unwrap_or_default();

                 // Clean dock time
                 let clean_chars: &[char] = &['<', '>', '[', ']'];
                 let dock_time_clean = dock_time_raw.trim_matches(clean_chars);
                 let dt_parts: Vec<&str> = dock_time_clean.split_whitespace().collect();
                 let mut time_parts = Vec::new();
                 for part in dt_parts {
                    if re_date.is_match(part) { continue; }
                    if re_day.is_match(part) { continue; }
                    time_parts.push(part);
                 }
                 let dock_time = time_parts.join(" ");

                 // Race start time?
                 let race_start = race.properties.get("START").cloned().unwrap_or(series_first_start.clone());

                 // Event page?
                 let event_page = race.properties.get("EVENT_PAGE").cloned()
                     .or_else(|| series.properties.get("EVENT_PAGE").cloned())
                     .unwrap_or_default();

                 rows.push(PrintableRace {
                    date_start: date_start_only,
                    date_end: date_end_only,
                    first_start: race_start,
                    dock_time,
                    sponsor: series_sponsor.clone(),
                    series: series_name.clone(),
                    race: race.headline.clone(),
                    series_event_page: event_page,
                });
            }

            // Check for series-level date if no races?
            if series.races.is_empty() {
                 if let (Some(start_raw), Some(end_raw)) = (series.properties.get("DATE_START"), series.properties.get("DATE_END")) {
                     let clean_chars: &[char] = &['<', '>', '[', ']'];
                     let start_val = start_raw.trim_matches(clean_chars);
                     let end_val = end_raw.trim_matches(clean_chars);

                     let date_start_only = start_val.split_whitespace().next().unwrap_or("").to_string();
                     let mut date_end_only = end_val.split_whitespace().next().unwrap_or("").to_string();

                     if date_start_only == date_end_only {
                         date_end_only = "".to_string();
                     }

                     rows.push(PrintableRace {
                        date_start: date_start_only,
                        date_end: date_end_only,
                        first_start: series_first_start.clone(),
                        dock_time: "".to_string(),
                        sponsor: series_sponsor.clone(),
                        series: series_name.clone(),
                        race: "".to_string(),
                        series_event_page: series_event_page.clone(),
                    });
                 }
            }
        }
    }

    if upcoming {
        let now = Local::now().date_naive();
        // Calculate end of the month 3 months from now
        let mut end_year = now.year();
        let mut end_month = now.month() + 3;

        while end_month > 12 {
            end_month -= 12;
            end_year += 1;
        }

        // Find last day of end_month
        let next_month_year = if end_month == 12 { end_year + 1 } else { end_year };
        let next_month = if end_month == 12 { 1 } else { end_month + 1 };

        let first_of_next = NaiveDate::from_ymd_opt(next_month_year, next_month, 1).unwrap();
        let end_date = first_of_next - Duration::days(1);

        rows.retain(|row| {
            if let Ok(d) = NaiveDate::parse_from_str(&row.date_start, "%Y-%m-%d") {
                d >= now && d <= end_date
            } else {
                false
            }
        });
    }

    rows.sort_by(|a, b| a.date_start.cmp(&b.date_start));
    rows
}

fn print_table(rows: Vec<PrintableRace>) {
    println!(
        "{:<12} {:<12} {:<12} {:<20} {:<15} {:<30} {:<30} {:<30}",
        "Date Start", "Date End", "First Start", "Dock Time", "Sponsor", "Series", "Race", "Event Page"
    );
    println!("{}", "-".repeat(170));

    for row in rows {
        println!(
            "{:<12} {:<12} {:<12} {:<20} {:<15} {:<30} {:<30} {:<30}",
            row.date_start, row.date_end, row.first_start, row.dock_time, row.sponsor, row.series, row.race, row.series_event_page
        );
    }
}

fn print_csv(rows: Vec<PrintableRace>) {
    println!("Date Start,Date End,Series,Race,Sponsor,Event Page");
    for row in rows {
        let escape = |s: &str| -> String {
            if s.contains(',') || s.contains('"') {
                format!("\"{}\"", s.replace("\"", "\"\""))
            } else {
                s.to_string()
            }
        };

        println!("{},{},{},{},{},{}",
            escape(&row.date_start),
            escape(&row.date_end),
            escape(&row.series),
            escape(&row.race),
            escape(&row.sponsor),
            escape(&row.series_event_page)
        );
    }
}

fn load_schedule(root: &str) -> Result<Schedule> {
    let mut years = Vec::new();

    for entry in fs::read_dir(root).context("Failed to read root directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(year_num) = name.parse::<i32>() {
                    let year = load_year(year_num, &path)?;
                    years.push(year);
                }
            }
        }
    }

    years.sort_by_key(|y| y.year);

    Ok(Schedule { years })
}

fn load_year(year_num: i32, path: &Path) -> Result<Year> {
    let mut series_list = Vec::new();

    for entry in fs::read_dir(path).context(format!("Failed to read year directory {:?}", path))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let series = load_series(name, &path)?;
                series_list.push(series);
            }
        }
    }

    series_list.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Year {
        year: year_num,
        series: series_list,
    })
}

fn load_series(name: &str, path: &Path) -> Result<Series> {
    let series_file = path.join("series.org");
    let mut properties = HashMap::new();
    let mut races = Vec::new();

    if series_file.exists() {
        let content = fs::read_to_string(&series_file)?;
        properties = parse_properties(&content);
        races = parse_races(&content);
    }

    Ok(Series {
        name: name.to_string(),
        properties,
        races,
    })
}

fn parse_properties(content: &str) -> HashMap<String, String> {
    let mut props = HashMap::new();
    let re = Regex::new(r"(?m)^:([A-Z0-9_]+):\s+(.*)$").unwrap();

    let mut in_properties = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == ":PROPERTIES:" {
            in_properties = true;
            continue;
        }
        if trimmed == ":END:" {
            in_properties = false;
            continue;
        }

        if line.starts_with('*') {
             break;
        }

        if in_properties {
             if let Some(caps) = re.captures(line) {
                let key = caps.get(1).unwrap().as_str().to_string();
                let value = caps.get(2).unwrap().as_str().trim().to_string();
                props.insert(key, value);
            }
        }
    }

    props
}

fn parse_races(content: &str) -> Vec<Race> {
    let mut races = Vec::new();
    let re_headline = Regex::new(r"(?m)^\* (.*)$").unwrap();

    let mut current_race_headline: Option<String> = None;
    let mut current_race_lines = Vec::new();

    for line in content.lines() {
        if let Some(caps) = re_headline.captures(line) {
            let headline = caps.get(1).unwrap().as_str().trim().to_string();
            if let Some(prev_headline) = current_race_headline {
                let race_content = current_race_lines.join("\n");
                let mut props = parse_properties(&race_content);
                if let Some(scheduled) = find_scheduled(&race_content) {
                    props.insert("SCHEDULED".to_string(), scheduled);
                }
                races.push(Race {
                    headline: prev_headline,
                    properties: props,
                });
            }
            current_race_headline = Some(headline);
            current_race_lines.clear();
        } else {
            current_race_lines.push(line);
        }
    }

    if let Some(headline) = current_race_headline {
        let race_content = current_race_lines.join("\n");
        let mut props = parse_properties(&race_content);
        if let Some(scheduled) = find_scheduled(&race_content) {
            props.insert("SCHEDULED".to_string(), scheduled);
        }
        races.push(Race {
            headline,
            properties: props,
        });
    }

    races
}

fn find_scheduled(content: &str) -> Option<String> {
    let re = Regex::new(r"SCHEDULED:\s*(.*)").unwrap();
    re.captures(content).map(|c| c.get(1).unwrap().as_str().trim().to_string())
}
