use bench_tools::{Database, BenchmarkResult};
use chrono::Utc;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "store")]
#[command(about = "Benchmark result storage CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long, default_value = "20")]
        limit: i32,
    },
    Show {
        #[arg(short, long)]
        run_id: Option<i64>,
    },
    Add {
        #[arg(long)]
        language: String,
        #[arg(long)]
        category: String,
        #[arg(long)]
        test_name: String,
        #[arg(long)]
        value: f64,
        #[arg(long, default_value = "time_ms")]
        metric: String,
    },
    Run {
        #[arg(short, long)]
        name: String,
        command: Vec<String>,
    },
    Clear {
        #[arg(short, long)]
        confirm: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let db = Database::new(None).expect("Failed to open database");
    
    match cli.command {
        Commands::List { limit } => {
            let runs = db.get_runs(limit).expect("Failed to get runs");
            println!("\n{:<6} {:<30} {:<20} {:<12} {}\n", 
                     "ID", "Name", "Started", "Status", "Results");
            println!("{}", "-".repeat(80));
            for run in runs {
                println!(
                    "{:<6} {:<30} {:<20} {:<12} {}",
                    run.id.unwrap_or(0),
                    run.name,
                    run.started_at.format("%Y-%m-%d %H:%M:%S"),
                    run.status,
                    run.total_results
                );
            }
            println!();
        }
        
        Commands::Show { run_id } => {
            let id = run_id.or_else(|| {
                db.get_runs(1).ok()?.first()?.id
            }).expect("No run ID provided and no runs found");
            
            let results = db.get_latest_results(10000).expect("Failed to get results");
            
            if results.is_empty() {
                println!("No results for run {}", id);
                return;
            }
            
            println!("\nResults for run {}:\n", id);
            println!("{:<10} {:<10} {:<20} {:<15} {:>12} {:>10}\n",
                     "Language", "Category", "Test", "Metric", "Value", "Time (ms)");
            println!("{}", "-".repeat(90));
            
            for r in results {
                println!(
                    "{:<10} {:<10} {:<20} {:<15} {:>12.2} {:>10.2}",
                    r.language,
                    r.category,
                    r.test_name,
                    r.metric,
                    r.value,
                    r.time_ms
                );
            }
            println!();
        }
        
        Commands::Add { language, category, test_name, value, metric } => {
            let hostname = hostname::get()
                .map(|h| h.to_string_lossy().into_owned())
                .unwrap_or_else(|_| "unknown".to_string());
            
            let result = BenchmarkResult {
                id: None,
                language,
                category,
                test_name,
                time_ms: 0.0,
                metric,
                value,
                metadata: None,
                timestamp: Utc::now(),
                hostname,
            };
            
            db.insert_result(0, &result).expect("Failed to insert result");
            println!("Result added successfully");
        }
        
        Commands::Run { name, command } => {
            let run_id = db.start_run(&name, 10).expect("Failed to start run");
            println!("Started run {}: {}", run_id, name);
            
            let output = std::process::Command::new(&command[0])
                .args(&command[1..])
                .output();
            
            match output {
                Ok(out) => {
                    if out.status.success() {
                        db.complete_run(run_id, "completed").unwrap();
                        println!("Run completed successfully");
                    } else {
                        db.complete_run(run_id, "failed").unwrap();
                        eprintln!("Run failed with exit code: {:?}", out.status.code());
                    }
                }
                Err(e) => {
                    db.complete_run(run_id, "error").unwrap();
                    eprintln!("Failed to execute: {}", e);
                }
            }
        }
        
        Commands::Clear { confirm } => {
            if confirm {
                unimplemented!("Clear functionality requires manual SQL");
            } else {
                println!("Use --confirm to clear all data");
            }
        }
    }
}
