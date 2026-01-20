use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
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
    days: Vec<Day>,
    #[allow(dead_code)]
    races: Vec<Race>,
}

#[derive(Debug)]
struct Day {
    #[allow(dead_code)]
    date_str: String, // MM-DD
    #[allow(dead_code)]
    properties: HashMap<String, String>,
    #[allow(dead_code)]
    crew: Vec<String>,
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
    year: i32,
    month: String,
    start: String,
    end: String,
    series: String,
    race: String,
    crew_count: usize,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <root-directory>", args[0]);
    }
    let root = &args[1];
    let schedule = load_schedule(root)?;
    sync_schedule(&schedule, root)?;
    let schedule_reloaded = load_schedule(root)?;
    print_schedule(&schedule_reloaded);
    Ok(())
}

fn sync_schedule(schedule: &Schedule, root: &str) -> Result<()> {
    for year in &schedule.years {
        for series in &year.series {
            for race in &series.races {
                 if let Some(scheduled_raw) = race.properties.get("SCHEDULED") {
                     let (start_ts, end_ts) = if let Some((s, e)) = scheduled_raw.split_once("--") {
                         (s.trim(), e.trim())
                     } else {
                         (scheduled_raw.as_str(), scheduled_raw.as_str())
                     };

                     let start_clean = start_ts.trim_matches(|c| c == '<' || c == '>');

                     let parts: Vec<&str> = start_clean.split_whitespace().collect();
                     if let Some(date_str) = parts.get(0) {
                         if let Some((_, mm_dd)) = date_str.split_once('-') {
                             let day_dir_name = mm_dd;
                             let series_path = Path::new(root).join(year.year.to_string()).join(&series.name);
                             let day_path = series_path.join(day_dir_name);

                             if !day_path.exists() {
                                 println!("Creating missing day directory: {:?}", day_path);
                                 fs::create_dir_all(&day_path)?;

                                 let day_org_path = day_path.join("day.org");
                                 let mut f = fs::File::create(&day_org_path)?;
                                 writeln!(f, ":PROPERTIES:")?;
                                 writeln!(f, ":DATE_START: {}", start_ts)?;
                                 writeln!(f, ":DATE_END:   {}", end_ts)?;
                                 writeln!(f, ":DOCK_TIME:  <{} 09:00>", start_clean)?;
                                 writeln!(f, ":END:")?;
                                 writeln!(f, "")?;
                                 writeln!(f, "* crew")?;

                                 let race_org_path = day_path.join("race.org");
                                 let mut f = fs::File::create(&race_org_path)?;
                                 writeln!(f, "* {}", race.headline)?;
                                 writeln!(f, ":PROPERTIES:")?;
                                 writeln!(f, ":RACE_NUMBER: ")?;
                                 writeln!(f, ":START: 11:30")?;
                                 writeln!(f, ":END:")?;
                             }
                         }
                     }
                 }
            }
        }
    }
    Ok(())
}

fn print_schedule(schedule: &Schedule) {
    let mut rows: Vec<PrintableRace> = Vec::new();

    for year in &schedule.years {
        for series in &year.series {
            let series_name = series.properties.get("NAME")
                .map(|s| s.clone())
                .unwrap_or_else(|| series.name.clone());

            let mut has_days = false;

            for day in &series.days {
                has_days = true;
                let parts: Vec<&str> = day.date_str.split('-').collect();
                let month = parts.get(0).unwrap_or(&"??").to_string();

                let start_raw = day.properties.get("DOCK_TIME").map(|s| s.as_str()).unwrap_or("?");
                let end_raw = day.properties.get("DATE_END").map(|s| s.as_str()).unwrap_or("?");

                let clean_chars: &[char] = &['<', '>', '[', ']'];
                let start = start_raw.trim_matches(clean_chars).to_string();
                let end = end_raw.trim_matches(clean_chars).to_string();

                let crew_count = day.crew.len();

                if day.races.is_empty() {
                    rows.push(PrintableRace {
                        year: year.year,
                        month,
                        start,
                        end,
                        series: series_name.clone(),
                        race: "-".to_string(),
                        crew_count,
                    });
                } else {
                    for race in &day.races {
                        rows.push(PrintableRace {
                            year: year.year,
                            month: month.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            series: series_name.clone(),
                            race: race.headline.clone(),
                            crew_count,
                        });
                    }
                }
            }

            if !has_days {
                 if let (Some(start_raw), Some(end_raw)) = (series.properties.get("DATE_START"), series.properties.get("DATE_END")) {
                     let clean_chars: &[char] = &['<', '>', '[', ']'];
                     let start_val = start_raw.trim_matches(clean_chars);
                     let end_val = end_raw.trim_matches(clean_chars);

                     let parts: Vec<&str> = start_val.split_whitespace().collect();
                     let date_part = parts.get(0).unwrap_or(&"??");
                     let date_subparts: Vec<&str> = date_part.split('-').collect();
                     let month = date_subparts.get(1).unwrap_or(&"??").to_string();

                     rows.push(PrintableRace {
                        year: year.year,
                        month,
                        start: start_val.to_string(),
                        end: end_val.to_string(),
                        series: series_name.clone(),
                        race: "".to_string(),
                        crew_count: 0,
                    });
                 }
            }
        }
    }

    rows.sort_by(|a, b| a.start.cmp(&b.start));

    println!(
        "{:<6} {:<6} {:<25} {:<20} {:<30} {:<30} {:<10}",
        "Year", "Month", "Day Start", "Day End", "Series", "Race", "Crew"
    );
    println!("{}", "-".repeat(130));

    for row in rows {
        println!(
            "{:<6} {:<6} {:<25} {:<20} {:<30} {:<30} {:<10}",
            row.year, row.month, row.start, row.end, row.series, row.race, row.crew_count
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

    let mut days = Vec::new();
    for entry in fs::read_dir(path).context(format!("Failed to read series directory {:?}", path))? {
        let entry = entry?;
        let sub_path = entry.path();
        if sub_path.is_dir() {
            if let Some(day_name) = sub_path.file_name().and_then(|n| n.to_str()) {
                let re = Regex::new(r"^\d{2}-\d{2}$").unwrap();
                if re.is_match(day_name) {
                    let day = load_day(day_name, &sub_path)?;
                    days.push(day);
                }
            }
        }
    }

    days.sort_by(|a, b| a.date_str.cmp(&b.date_str));

    Ok(Series {
        name: name.to_string(),
        properties,
        days,
        races,
    })
}

fn load_day(date_str: &str, path: &Path) -> Result<Day> {
    let day_file = path.join("day.org");
    let mut properties = HashMap::new();
    let mut crew = Vec::new();

    if day_file.exists() {
        let content = fs::read_to_string(&day_file)?;
        properties = parse_properties(&content);
        crew = parse_crew(&content);
    }

    let race_file = path.join("race.org");
    let mut races = Vec::new();
    if race_file.exists() {
        let content = fs::read_to_string(&race_file)?;
        races = parse_races(&content);
    }

    Ok(Day {
        date_str: date_str.to_string(),
        properties,
        crew,
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

fn parse_crew(content: &str) -> Vec<String> {
    let mut crew = Vec::new();
    let mut in_crew_section = false;

    for line in content.lines() {
        if line.to_lowercase().starts_with("* crew") {
            in_crew_section = true;
            continue;
        }
        if line.starts_with('*') && in_crew_section {
            break;
        }

        if in_crew_section {
            let trimmed = line.trim();
            if trimmed.starts_with('-') {
                 let member = trimmed.trim_start_matches('-').trim().to_string();
                 if !member.is_empty() {
                     crew.push(member);
                 }
            }
        }
    }
    crew
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
