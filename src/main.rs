use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use serde_json::Value;

// Ensure serde_json is available
extern crate serde_json;

fn remove_quotes(path: &str) -> String {
    path.trim_matches('"').to_string()
}

fn create_progress_bar(percent: f64, width: usize) -> String {
    let filled = (percent / 100.0 * width as f64).round() as usize;
    let empty = width - filled;
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}

fn main() {
    println!("================================================================================");
    println!("o7 CMDR! Enter the full path for the journal file you'd like to look at:");
    println!("--------------------------------------------------------------------------------");
    println!("Windows: C:\\Users\\YourName\\Saved Games\\Frontier Developments\\Elite Dangerous\\Journal.Date.log");
    println!("Linux: ~/.local/share/Frontier Developments/Elite Dangerous/Journal.Date.log");
    println!("Proton: /Path-to-SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/Journal.Date.log");
    println!("================================================================================");

    let mut journal_path = String::new();
    print!("Path = ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut journal_path).expect("Failed to read input");
    journal_path = remove_quotes(journal_path.trim());

    println!("\nProcessing journal file: {}\n", journal_path);

    if let Err(e) = process_journal(&journal_path) {
        eprintln!("ERROR: {}", e);
    }
}

fn process_journal(path: &str) -> io::Result<()> {
    let file = File::open(Path::new(path))?;
    let reader = BufReader::new(file);

    let commander_marker = "Commander";
    let rank_marker = "Rank";
    let progress_marker = "Progress";

    let mut cmdr_name = String::from("Unknown");
    let mut ranks = vec![None; 8];
    let mut progress_values = vec![None; 8];

    let rank_names = vec!["Combat", "Trade", "Exploration", "Soldier", "Exobiology", "Empire", "Federation", "CQC"];
    let log_rank_names = vec!["Combat", "Trade", "Explore", "Soldier", "Exobiologist", "Empire", "Federation", "CQC"];
    let max_ranks = vec![13, 13, 13, 13, 13, 14, 14, 13];

    let rank_titles: Vec<Vec<&str>> = vec![
        vec!["Harmless", "Mostly Harmless", "Novice", "Competent", "Expert", "Master", "Dangerous", "Deadly", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"],
        vec!["Penniless", "Mostly Penniless", "Peddler", "Dealer", "Merchant", "Broker", "Entrepreneur", "Tycoon", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"],
        vec!["Aimless", "Mostly Aimless", "Scout", "Surveyor", "Trailblazer", "Pathfinder", "Ranger", "Pioneer", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"],
        vec!["Defenceless", "Mostly Defenceless", "Rookie", "Soldier", "Gunslinger", "Warrior", "Gladiator", "Deadeye", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"],
        vec!["Directionless", "Mostly Directionless", "Compiler", "Collector", "Cataloguer", "Taxonomist", "Ecologist", "Geneticist", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"],
        vec!["None", "Outsider", "Serf", "Master", "Squire", "Knight", "Lord", "Baron", "Viscount", "Count", "Earl", "Marquis", "Duke", "Prince", "King"],
        vec!["None", "Recruit", "Cadet", "Midshipman", "Petty Officer", "Chief Petty Officer", "Warrant Officer", "Ensign", "Lieutenant", "Lieutenant Commander", "Post Commander", "Post Captain", "Rear Admiral", "Vice Admiral", "Admiral"],
        vec!["Helpless", "Mostly Helpless", "Amateur", "Semi Professional", "Professional", "Champion", "Hero", "Legend", "Elite", "Elite I", "Elite II", "Elite III", "Elite IV", "Elite V"]
    ];

    println!("================================================================================");
    println!("Reading Journal File...");
    println!("================================================================================");

    for line in reader.lines() {
        let line = line?;
        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            if json["event"] == commander_marker {
                if let Some(name) = json["Name"].as_str() {
                    cmdr_name = name.to_string();
                }
            } else if json["event"] == rank_marker {
                for (i, career) in log_rank_names.iter().enumerate() {
                    if let Some(value) = json.get(*career) {
                        ranks[i] = value.as_u64();
                    }
                }
            } else if json["event"] == progress_marker {
                for (i, career) in log_rank_names.iter().enumerate() {
                    if let Some(value) = json.get(*career) {
                        progress_values[i] = value.as_u64();
                    }
                }
            }
        }
    }

    println!("================================================================================");
    println!("Greetings, CMDR {}!", cmdr_name);
    println!("================================================================================");

    println!("Rank Progress:");
    println!("--------------------------------------------------------------------------------");
    for (i, career) in rank_names.iter().enumerate() {
        if let Some(rank_num) = ranks[i] {
            if rank_num as usize == max_ranks[i] {
                let bar = create_progress_bar(100.0, 20);
                println!("{: <15}: {} 100% [{}] MAX!", career, bar, rank_titles[i][rank_num as usize]);
            } else {
                let next_rank = (rank_num as usize + 1).min(max_ranks[i]);
                let progress = progress_values[i].unwrap_or(0) as u64;
                let bar = create_progress_bar(progress as f64, 20);
                println!("{: <15}: {} {}% [{} → {}]", career, bar, progress, rank_titles[i][rank_num as usize], rank_titles[i][next_rank]);
            }
        }
    }
    println!("================================================================================");
    println!("Processing Complete. o7, CMDR {}!", cmdr_name);
    println!("================================================================================");

    Ok(())
}
