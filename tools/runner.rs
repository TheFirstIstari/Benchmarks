use std::process::Command;
use std::time::Instant;
use clap::Parser;
use bench_tools::{Database, BenchmarkResult};
use chrono::Utc;

#[derive(Parser)]
#[command(name = "runner")]
#[command(about = "Benchmark runner with result storage", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
    
    #[arg(short, long)]
    quick: bool,
    
    #[arg(short, long)]
    categories: Option<Vec<String>>,
}

struct BenchmarkTask {
    name: &'static str,
    language: &'static str,
    category: &'static str,
    just_task: &'static str,
}

fn get_tasks() -> Vec<BenchmarkTask> {
    vec![
        BenchmarkTask { name: "c-matrix", language: "C", category: "matrix", just_task: "c-matrix" },
        BenchmarkTask { name: "c-allocator", language: "C", category: "allocator", just_task: "c-allocator" },
        BenchmarkTask { name: "c-string", language: "C", category: "string", just_task: "c-string" },
        BenchmarkTask { name: "c-sort", language: "C", category: "sort", just_task: "c-sort" },
        BenchmarkTask { name: "c-concurrency", language: "C", category: "concurrency", just_task: "c-concurrency" },
        BenchmarkTask { name: "cpp-matrix", language: "C++", category: "matrix", just_task: "cpp-matrix" },
        BenchmarkTask { name: "cpp-allocator", language: "C++", category: "allocator", just_task: "cpp-allocator" },
        BenchmarkTask { name: "cpp-string", language: "C++", category: "string", just_task: "cpp-string" },
        BenchmarkTask { name: "cpp-sort", language: "C++", category: "sort", just_task: "cpp-sort" },
        BenchmarkTask { name: "cpp-concurrency", language: "C++", category: "concurrency", just_task: "cpp-concurrency" },
        BenchmarkTask { name: "python-matrix", language: "Python", category: "matrix", just_task: "python-matrix" },
        BenchmarkTask { name: "python-string", language: "Python", category: "string", just_task: "python-string" },
        BenchmarkTask { name: "python-sort", language: "Python", category: "sort", just_task: "python-sort" },
        BenchmarkTask { name: "python-async", language: "Python", category: "async", just_task: "python-async" },
        BenchmarkTask { name: "java-matrix", language: "Java", category: "matrix", just_task: "java-matrix" },
        BenchmarkTask { name: "java-string", language: "Java", category: "string", just_task: "java-string" },
        BenchmarkTask { name: "java-sort", language: "Java", category: "sort", just_task: "java-sort" },
        BenchmarkTask { name: "java-concurrency", language: "Java", category: "concurrency", just_task: "java-concurrency" },
        BenchmarkTask { name: "rust-matrix", language: "Rust", category: "matrix", just_task: "rust-matrix" },
        BenchmarkTask { name: "rust-allocator", language: "Rust", category: "allocator", just_task: "rust-allocator" },
        BenchmarkTask { name: "rust-string", language: "Rust", category: "string", just_task: "rust-string" },
        BenchmarkTask { name: "rust-sort", language: "Rust", category: "sort", just_task: "rust-sort" },
        BenchmarkTask { name: "rust-concurrency", language: "Rust", category: "concurrency", just_task: "rust-concurrency" },
        BenchmarkTask { name: "c-hash", language: "C", category: "hash", just_task: "c-hash" },
        BenchmarkTask { name: "cpp-hash", language: "C++", category: "hash", just_task: "cpp-hash" },
        BenchmarkTask { name: "python-hash", language: "Python", category: "hash", just_task: "python-hash" },
        BenchmarkTask { name: "java-hash", language: "Java", category: "hash", just_task: "java-hash" },
        BenchmarkTask { name: "rust-hash", language: "Rust", category: "hash", just_task: "rust-hash" },
        BenchmarkTask { name: "c-regex", language: "C", category: "regex", just_task: "c-regex" },
        BenchmarkTask { name: "cpp-regex", language: "C++", category: "regex", just_task: "cpp-regex" },
        BenchmarkTask { name: "python-regex", language: "Python", category: "regex", just_task: "python-regex" },
        BenchmarkTask { name: "java-regex", language: "Java", category: "regex", just_task: "java-regex" },
        BenchmarkTask { name: "rust-regex", language: "Rust", category: "regex", just_task: "rust-regex" },
        BenchmarkTask { name: "c-json", language: "C", category: "json", just_task: "c-json" },
        BenchmarkTask { name: "cpp-json", language: "C++", category: "json", just_task: "cpp-json" },
        BenchmarkTask { name: "python-json", language: "Python", category: "json", just_task: "python-json" },
        BenchmarkTask { name: "java-json", language: "Java", category: "json", just_task: "java-json" },
        BenchmarkTask { name: "rust-json", language: "Rust", category: "json", just_task: "rust-json" },
        BenchmarkTask { name: "c-fileio", language: "C", category: "fileio", just_task: "c-fileio" },
        BenchmarkTask { name: "cpp-fileio", language: "C++", category: "fileio", just_task: "cpp-fileio" },
        BenchmarkTask { name: "python-fileio", language: "Python", category: "fileio", just_task: "python-fileio" },
        BenchmarkTask { name: "java-fileio", language: "Java", category: "fileio", just_task: "java-fileio" },
        BenchmarkTask { name: "rust-fileio", language: "Rust", category: "fileio", just_task: "rust-fileio" },
        BenchmarkTask { name: "c-math", language: "C", category: "math", just_task: "c-math" },
        BenchmarkTask { name: "cpp-math", language: "C++", category: "math", just_task: "cpp-math" },
        BenchmarkTask { name: "python-math", language: "Python", category: "math", just_task: "python-math" },
        BenchmarkTask { name: "java-math", language: "Java", category: "math", just_task: "java-math" },
        BenchmarkTask { name: "rust-math", language: "Rust", category: "math", just_task: "rust-math" },

    ]
}

fn parse_results(output: &str, language: &str, category: &str) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    let host = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('=') {
            continue;
        }
        
        // Try various patterns
        let patterns = [
            // "Name: value unit" (with optional scientific notation, handles spaces in name)
            (r"^([a-zA-Z0-9_ ]+):\s*([\d.e+-]+)\s*(ms|ns|ops/sec|elements/sec|GFLOPS|MOPS)", 1, 2, 3),
            // "Name value unit"
            (r"^([a-zA-Z0-9_]+)\s+([\d.e+-]+)\s+(ms|ns|ops/sec|elements/sec|GFLOPS|MOPS)", 1, 2, 3),
        ];
        
        for (pattern, name_idx, val_idx, unit_idx) in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(caps) = re.captures(line) {
                    let test_name = caps.get(name_idx).map(|m| m.as_str()).unwrap_or("");
                    let value_str = caps.get(val_idx).map(|m| m.as_str()).unwrap_or("0");
                    let unit = caps.get(unit_idx).map(|m| m.as_str()).unwrap_or("value");
                    
                    if let Ok(value) = value_str.parse::<f64>() {
                        let time_ms = match unit {
                            "ms" => value,
                            "ns" => value / 1e6,
                            _ => value,
                        };
                        
                        let test_name_title = test_name
                            .replace('_', " ")
                            .split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(first) => first.to_uppercase().chain(chars).collect(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        
                        results.push(BenchmarkResult {
                            id: None,
                            language: language.to_string(),
                            category: category.to_string(),
                            test_name: test_name_title,
                            time_ms,
                            metric: unit.to_string(),
                            value,
                            metadata: None,
                            timestamp: Utc::now(),
                            hostname: host.clone(),
                        });
                        break;
                    }
                }
            }
        }
    }
    
    results
}

fn run_just_task(task: &str) -> (bool, String, f64) {
    let start = Instant::now();
    let output = Command::new("just")
        .arg(task)
        .output();
    
    let elapsed = start.elapsed().as_secs_f64();
    
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let combined = format!("{}\n{}", stdout, stderr);
            (out.status.success(), combined, elapsed)
        }
        Err(e) => (false, format!("Failed to run just: {}", e), elapsed),
    }
}

fn main() {
    let cli = Cli::parse();
    let db = Database::new(None).expect("Failed to open database");
    let _hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let run_name = cli.name.unwrap_or_else(|| {
        format!("run-{}", Utc::now().format("%Y%m%d-%H%M%S"))
    });
    
    let run_id = db.start_run(&run_name, 10).expect("Failed to start run");
    
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("  BENCHMARK SUITE: {}", run_name);
    println!("═══════════════════════════════════════════════════════════════════════\n");
    
    let tasks = get_tasks();
    let mut total_results = 0;
    let mut failed = Vec::new();
    
    // Filter by category if specified
    let filtered_tasks: Vec<_> = if let Some(ref cats) = cli.categories {
        tasks.iter().filter(|t| cats.contains(&t.category.to_string())).collect()
    } else {
        tasks.iter().collect()
    };
    
    for task in filtered_tasks {
        print!("[{}/{}] {}... ", task.language, task.category, task.name);
        std::io::Write::flush(&mut std::io::stdout()).ok();
        
        let (success, output, elapsed) = run_just_task(task.just_task);
        
        if success {
            let results = parse_results(&output, task.language, task.category);
            for result in &results {
                if db.insert_result(run_id, result).is_ok() {
                    total_results += 1;
                }
            }
            println!("OK ({:.1}s) - {} results", elapsed, results.len());
        } else {
            println!("FAILED ({:.1}s)", elapsed);
            failed.push((task.name, task.language, task.category));
        }
    }
    
    db.update_result_count(run_id).ok();
    
    if failed.is_empty() {
        db.complete_run(run_id, "completed").ok();
    } else {
        db.complete_run(run_id, "partial").ok();
    }
    
    println!("\n═══════════════════════════════════════════════════════════════════════");
    println!("  COMPLETED: {} results stored", total_results);
    if !failed.is_empty() {
        println!("  FAILED: {} benchmarks", failed.len());
        for (name, lang, cat) in &failed {
            println!("    - {}/{}: {}", lang, cat, name);
        }
    }
    println!("═══════════════════════════════════════════════════════════════════════\n");
    
    println!("View results with: just monitor");
}
