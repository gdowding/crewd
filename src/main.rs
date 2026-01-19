use anyhow::{Context, Result};
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
    days: Vec<Day>,
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <root-directory>", args[0]);
    }
    let root = &args[1];
    let schedule = load_schedule(root)?;
    println!("{:#?}", schedule);
    Ok(())
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

    // Sort years for consistent output
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
                // Assuming any directory here is a series
                let series = load_series(name, &path)?;
                series_list.push(series);
            }
        }
    }

    Ok(Year {
        year: year_num,
        series: series_list,
    })
}

fn load_series(name: &str, path: &Path) -> Result<Series> {
    let series_file = path.join("series.org");
    let mut properties = HashMap::new();
    if series_file.exists() {
        let content = fs::read_to_string(&series_file)?;
        properties = parse_properties(&content);
    }

    let mut days = Vec::new();
    for entry in fs::read_dir(path).context(format!("Failed to read series directory {:?}", path))? {
        let entry = entry?;
        let sub_path = entry.path();
        if sub_path.is_dir() {
            if let Some(day_name) = sub_path.file_name().and_then(|n| n.to_str()) {
                // Check if it looks like MM-DD
                let re = Regex::new(r"^\d{2}-\d{2}$").unwrap();
                if re.is_match(day_name) {
                    let day = load_day(day_name, &sub_path)?;
                    days.push(day);
                }
            }
        }
    }

    Ok(Series {
        name: name.to_string(),
        properties,
        days,
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
    // Regex to match :KEY: value inside a property drawer or generally
    // But typically they are inside :PROPERTIES: ... :END:
    // For simplicity, we'll scan for lines starting with :KEY:
    // A more robust parser would respect the drawer.
    let re = Regex::new(r"(?m)^:([A-Z0-9_]+):\s+(.*)$").unwrap();

    // We only want to parse top-level properties for files like series.org/day.org
    // Or properties within the first block if we aren't careful.
    // Given the simple format description, let's look for the first :PROPERTIES: block
    // or just scan the whole file if there are no headlines (like series.org seems to be mostly properties).

    // However, race.org has headlines.
    // Let's iterate over lines.

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

        // Stop parsing properties if we hit a headline (for day.org which might have * crew)
        if line.starts_with('*') {
             // If we were parsing top-level properties (implicit drawer or explicit),
             // we stop when we hit a headline.
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
            // New headline ends the crew section
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
    // Split by headlines
    let re_headline = Regex::new(r"(?m)^\* (.*)$").unwrap();

    let mut current_race_headline: Option<String> = None;
    let mut current_race_lines = Vec::new();

    for line in content.lines() {
        if let Some(caps) = re_headline.captures(line) {
            // Finish previous race
            if let Some(headline) = current_race_headline {
                let race_content = current_race_lines.join("\n");
                let props = parse_properties(&race_content); // Re-use parse_properties on the block
                races.push(Race {
                    headline,
                    properties: props,
                });
            }
            // Start new race
            current_race_headline = Some(caps.get(1).unwrap().as_str().trim().to_string());
            current_race_lines.clear();
        } else {
            current_race_lines.push(line);
        }
    }

    // Push the last one
    if let Some(headline) = current_race_headline {
        let race_content = current_race_lines.join("\n");
        let props = parse_properties(&race_content);
        races.push(Race {
            headline,
            properties: props,
        });
    }

    races
}
