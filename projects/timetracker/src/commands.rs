use clap::{Parser, Subcommand};
use std::collections::HashMap;
use chrono::Local;
use crate::storage::{self, load_sessions, save_session, WorkSession};
use crate::time_utils::{self, calculate_duration, format_duration}; // Import from time_utils

#[derive(Parser)]
#[command(name = "TimeTracker")]
#[command(about = "A simple CLI for tracking project time", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub fn generate_report() {
    let sessions = match load_sessions() {
        Ok(sessions) => sessions,
        Err(_) => {
            eprintln!("Error loading sessions");
            return;
        }
    };

    let mut project_durations: HashMap<String, i64> = HashMap::new();

    for session in sessions {
        if let Some(end_time) = session.end {
            if let Some(duration) = calculate_duration(&session.start, &end_time) {
                *project_durations.entry(session.project).or_insert(0) += duration;
            }
        }
    }

    println!("Project Time Report:");
    for (project, duration) in project_durations {
        println!("{}: {}", project, format_duration(duration));
    }
}

#[derive(Subcommand)]
enum Commands {
    Start { project: String },
    Stop,
    Report,
}

pub fn run() {
    let cli = Cli::parse();
    handle_command(cli);
}

pub fn handle_command(cli: Cli) {
    match &cli.command {
        // Start Command
        Commands::Start { project } => {
            let start_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let session = WorkSession {
                project: project.to_string(),
                start: start_time,
                end: None,
            };
            if save_session(&session).is_ok() {
                println!("Starting work on project: {}", session.project);
            } else {
                eprintln!("Failed to start tracking project.");
            }
        }

        // Stop Command
        Commands::Stop => {
            let mut sessions = match load_sessions() {
                Ok(sessions) => sessions,
                Err(_) => {
                    eprintln!("Error loading sessions");
                    return;
                }
            };

            // Find the last ongoing session (the one with None as the end time)
            if let Some(session) = sessions.iter_mut().rev().find(|s| s.end.is_none()) {
                session.end = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

                // Save the updated session
                if storage::save_session(&session).is_err() {
                    eprintln!("Failed to save the stopped session.");
                } else if let Some(end_time) = &session.end {
                    println!(
                        "Stopped tracking project: {} (from {} to {})",
                        session.project, session.start, end_time
                    );
                } else {
                    eprintln!("Error: Stopped session has no end time recorded.");
                }
            } else {
                eprintln!("No ongoing session found.");
            }
        }

        // Report Command
        Commands::Report => {
            generate_report();  
        }
    }
}
