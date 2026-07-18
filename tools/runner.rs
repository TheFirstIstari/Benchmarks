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
    mise_task: &'static str,
}

fn get_tasks() -> Vec<BenchmarkTask> {
    vec![
        BenchmarkTask { name: "c-matrix", language: "C", category: "matrix", mise_task: "c-matrix" },
        BenchmarkTask { name: "cpp-matrix", language: "C++", category: "matrix", mise_task: "cpp-matrix" },
        BenchmarkTask { name: "rust-matrix", language: "Rust", category: "matrix", mise_task: "rust-matrix" },
        BenchmarkTask { name: "python-matrix", language: "Python", category: "matrix", mise_task: "python-matrix" },
        BenchmarkTask { name: "java-matrix", language: "Java", category: "matrix", mise_task: "java-matrix" },
        BenchmarkTask { name: "go-matrix", language: "Go", category: "matrix", mise_task: "go-matrix" },
        BenchmarkTask { name: "ruby-matrix", language: "Ruby", category: "matrix", mise_task: "ruby-matrix" },
        BenchmarkTask { name: "node-matrix", language: "Node", category: "matrix", mise_task: "node-matrix" },
        BenchmarkTask { name: "php-matrix", language: "PHP", category: "matrix", mise_task: "php-matrix" },
        BenchmarkTask { name: "zig-matrix", language: "Zig", category: "matrix", mise_task: "zig-matrix" },
        BenchmarkTask { name: "swift-matrix", language: "Swift", category: "matrix", mise_task: "swift-matrix" },
        BenchmarkTask { name: "kotlin-matrix", language: "Kotlin", category: "matrix", mise_task: "kotlin-matrix" },
        BenchmarkTask { name: "cs-matrix", language: "C#", category: "matrix", mise_task: "cs-matrix" },
        BenchmarkTask { name: "c-sort", language: "C", category: "sort", mise_task: "c-sort" },
        BenchmarkTask { name: "cpp-sort", language: "C++", category: "sort", mise_task: "cpp-sort" },
        BenchmarkTask { name: "rust-sort", language: "Rust", category: "sort", mise_task: "rust-sort" },
        BenchmarkTask { name: "python-sort", language: "Python", category: "sort", mise_task: "python-sort" },
        BenchmarkTask { name: "java-sort", language: "Java", category: "sort", mise_task: "java-sort" },
        BenchmarkTask { name: "go-sort", language: "Go", category: "sort", mise_task: "go-sort" },
        BenchmarkTask { name: "ruby-sort", language: "Ruby", category: "sort", mise_task: "ruby-sort" },
        BenchmarkTask { name: "node-sort", language: "Node", category: "sort", mise_task: "node-sort" },
        BenchmarkTask { name: "php-sort", language: "PHP", category: "sort", mise_task: "php-sort" },
        BenchmarkTask { name: "zig-sort", language: "Zig", category: "sort", mise_task: "zig-sort" },
        BenchmarkTask { name: "swift-sort", language: "Swift", category: "sort", mise_task: "swift-sort" },
        BenchmarkTask { name: "kotlin-sort", language: "Kotlin", category: "sort", mise_task: "kotlin-sort" },
        BenchmarkTask { name: "cs-sort", language: "C#", category: "sort", mise_task: "cs-sort" },
        BenchmarkTask { name: "c-string", language: "C", category: "string", mise_task: "c-string" },
        BenchmarkTask { name: "cpp-string", language: "C++", category: "string", mise_task: "cpp-string" },
        BenchmarkTask { name: "rust-string", language: "Rust", category: "string", mise_task: "rust-string" },
        BenchmarkTask { name: "python-string", language: "Python", category: "string", mise_task: "python-string" },
        BenchmarkTask { name: "java-string", language: "Java", category: "string", mise_task: "java-string" },
        BenchmarkTask { name: "go-string", language: "Go", category: "string", mise_task: "go-string" },
        BenchmarkTask { name: "ruby-string", language: "Ruby", category: "string", mise_task: "ruby-string" },
        BenchmarkTask { name: "node-string", language: "Node", category: "string", mise_task: "node-string" },
        BenchmarkTask { name: "php-string", language: "PHP", category: "string", mise_task: "php-string" },
        BenchmarkTask { name: "zig-string", language: "Zig", category: "string", mise_task: "zig-string" },
        BenchmarkTask { name: "swift-string", language: "Swift", category: "string", mise_task: "swift-string" },
        BenchmarkTask { name: "kotlin-string", language: "Kotlin", category: "string", mise_task: "kotlin-string" },
        BenchmarkTask { name: "cs-string", language: "C#", category: "string", mise_task: "cs-string" },
        BenchmarkTask { name: "c-hash", language: "C", category: "hash", mise_task: "c-hash" },
        BenchmarkTask { name: "cpp-hash", language: "C++", category: "hash", mise_task: "cpp-hash" },
        BenchmarkTask { name: "rust-hash", language: "Rust", category: "hash", mise_task: "rust-hash" },
        BenchmarkTask { name: "python-hash", language: "Python", category: "hash", mise_task: "python-hash" },
        BenchmarkTask { name: "java-hash", language: "Java", category: "hash", mise_task: "java-hash" },
        BenchmarkTask { name: "go-hash", language: "Go", category: "hash", mise_task: "go-hash" },
        BenchmarkTask { name: "ruby-hash", language: "Ruby", category: "hash", mise_task: "ruby-hash" },
        BenchmarkTask { name: "node-hash", language: "Node", category: "hash", mise_task: "node-hash" },
        BenchmarkTask { name: "php-hash", language: "PHP", category: "hash", mise_task: "php-hash" },
        BenchmarkTask { name: "zig-hash", language: "Zig", category: "hash", mise_task: "zig-hash" },
        BenchmarkTask { name: "swift-hash", language: "Swift", category: "hash", mise_task: "swift-hash" },
        BenchmarkTask { name: "kotlin-hash", language: "Kotlin", category: "hash", mise_task: "kotlin-hash" },
        BenchmarkTask { name: "cs-hash", language: "C#", category: "hash", mise_task: "cs-hash" },
        BenchmarkTask { name: "c-regex", language: "C", category: "regex", mise_task: "c-regex" },
        BenchmarkTask { name: "cpp-regex", language: "C++", category: "regex", mise_task: "cpp-regex" },
        BenchmarkTask { name: "rust-regex", language: "Rust", category: "regex", mise_task: "rust-regex" },
        BenchmarkTask { name: "python-regex", language: "Python", category: "regex", mise_task: "python-regex" },
        BenchmarkTask { name: "java-regex", language: "Java", category: "regex", mise_task: "java-regex" },
        BenchmarkTask { name: "go-regex", language: "Go", category: "regex", mise_task: "go-regex" },
        BenchmarkTask { name: "ruby-regex", language: "Ruby", category: "regex", mise_task: "ruby-regex" },
        BenchmarkTask { name: "cs-regex", language: "C#", category: "regex", mise_task: "cs-regex" },
        BenchmarkTask { name: "c-json", language: "C", category: "json", mise_task: "c-json" },
        BenchmarkTask { name: "cpp-json", language: "C++", category: "json", mise_task: "cpp-json" },
        BenchmarkTask { name: "rust-json", language: "Rust", category: "json", mise_task: "rust-json" },
        BenchmarkTask { name: "python-json", language: "Python", category: "json", mise_task: "python-json" },
        BenchmarkTask { name: "java-json", language: "Java", category: "json", mise_task: "java-json" },
        BenchmarkTask { name: "go-json", language: "Go", category: "json", mise_task: "go-json" },
        BenchmarkTask { name: "ruby-json", language: "Ruby", category: "json", mise_task: "ruby-json" },
        BenchmarkTask { name: "cs-json", language: "C#", category: "json", mise_task: "cs-json" },
        BenchmarkTask { name: "c-fileio", language: "C", category: "fileio", mise_task: "c-fileio" },
        BenchmarkTask { name: "cpp-fileio", language: "C++", category: "fileio", mise_task: "cpp-fileio" },
        BenchmarkTask { name: "rust-fileio", language: "Rust", category: "fileio", mise_task: "rust-fileio" },
        BenchmarkTask { name: "python-fileio", language: "Python", category: "fileio", mise_task: "python-fileio" },
        BenchmarkTask { name: "java-fileio", language: "Java", category: "fileio", mise_task: "java-fileio" },
        BenchmarkTask { name: "go-fileio", language: "Go", category: "fileio", mise_task: "go-fileio" },
        BenchmarkTask { name: "ruby-fileio", language: "Ruby", category: "fileio", mise_task: "ruby-fileio" },
        BenchmarkTask { name: "cs-fileio", language: "C#", category: "fileio", mise_task: "cs-fileio" },
        BenchmarkTask { name: "c-math", language: "C", category: "math", mise_task: "c-math" },
        BenchmarkTask { name: "cpp-math", language: "C++", category: "math", mise_task: "cpp-math" },
        BenchmarkTask { name: "rust-math", language: "Rust", category: "math", mise_task: "rust-math" },
        BenchmarkTask { name: "python-math", language: "Python", category: "math", mise_task: "python-math" },
        BenchmarkTask { name: "java-math", language: "Java", category: "math", mise_task: "java-math" },
        BenchmarkTask { name: "go-math", language: "Go", category: "math", mise_task: "go-math" },
        BenchmarkTask { name: "ruby-math", language: "Ruby", category: "math", mise_task: "ruby-math" },
        BenchmarkTask { name: "cs-math", language: "C#", category: "math", mise_task: "cs-math" },
        BenchmarkTask { name: "c-network", language: "C", category: "network", mise_task: "c-network" },
        BenchmarkTask { name: "cpp-network", language: "C++", category: "network", mise_task: "cpp-network" },
        BenchmarkTask { name: "rust-network", language: "Rust", category: "network", mise_task: "rust-network" },
        BenchmarkTask { name: "python-network", language: "Python", category: "network", mise_task: "python-network" },
        BenchmarkTask { name: "java-network", language: "Java", category: "network", mise_task: "java-network" },
        BenchmarkTask { name: "go-network", language: "Go", category: "network", mise_task: "go-network" },
        BenchmarkTask { name: "ruby-network", language: "Ruby", category: "network", mise_task: "ruby-network" },
        BenchmarkTask { name: "cs-network", language: "C#", category: "network", mise_task: "cs-network" },
        BenchmarkTask { name: "c-crypto", language: "C", category: "crypto", mise_task: "c-crypto" },
        BenchmarkTask { name: "cpp-crypto", language: "C++", category: "crypto", mise_task: "cpp-crypto" },
        BenchmarkTask { name: "rust-crypto", language: "Rust", category: "crypto", mise_task: "rust-crypto" },
        BenchmarkTask { name: "python-crypto", language: "Python", category: "crypto", mise_task: "python-crypto" },
        BenchmarkTask { name: "java-crypto", language: "Java", category: "crypto", mise_task: "java-crypto" },
        BenchmarkTask { name: "go-crypto", language: "Go", category: "crypto", mise_task: "go-crypto" },
        BenchmarkTask { name: "ruby-crypto", language: "Ruby", category: "crypto", mise_task: "ruby-crypto" },
        BenchmarkTask { name: "cs-crypto", language: "C#", category: "crypto", mise_task: "cs-crypto" },
        BenchmarkTask { name: "c-ml", language: "C", category: "ml", mise_task: "c-ml" },
        BenchmarkTask { name: "cpp-ml", language: "C++", category: "ml", mise_task: "cpp-ml" },
        BenchmarkTask { name: "rust-ml", language: "Rust", category: "ml", mise_task: "rust-ml" },
        BenchmarkTask { name: "python-ml", language: "Python", category: "ml", mise_task: "python-ml" },
        BenchmarkTask { name: "java-ml", language: "Java", category: "ml", mise_task: "java-ml" },
        BenchmarkTask { name: "go-ml", language: "Go", category: "ml", mise_task: "go-ml" },
        BenchmarkTask { name: "ruby-ml", language: "Ruby", category: "ml", mise_task: "ruby-ml" },
        BenchmarkTask { name: "cs-ml", language: "C#", category: "ml", mise_task: "cs-ml" },
        BenchmarkTask { name: "c-concurrency", language: "C", category: "concurrency", mise_task: "c-concurrency" },
        BenchmarkTask { name: "cpp-concurrency", language: "C++", category: "concurrency", mise_task: "cpp-concurrency" },
        BenchmarkTask { name: "rust-concurrency", language: "Rust", category: "concurrency", mise_task: "rust-concurrency" },
        BenchmarkTask { name: "python-async", language: "Python", category: "concurrency", mise_task: "python-async" },
        BenchmarkTask { name: "java-concurrency", language: "Java", category: "concurrency", mise_task: "java-concurrency" },
        BenchmarkTask { name: "go-concurrency", language: "Go", category: "concurrency", mise_task: "go-concurrency" },
        BenchmarkTask { name: "ruby-concurrency", language: "Ruby", category: "concurrency", mise_task: "ruby-concurrency" },
        BenchmarkTask { name: "cs-concurrency", language: "C#", category: "concurrency", mise_task: "cs-concurrency" },
        BenchmarkTask { name: "c-cpu", language: "C", category: "cpu", mise_task: "c-cpu" },
        BenchmarkTask { name: "cpp-cpu", language: "C++", category: "cpu", mise_task: "cpp-cpu" },
        BenchmarkTask { name: "rust-cpu", language: "Rust", category: "cpu", mise_task: "rust-cpu" },
        BenchmarkTask { name: "python-cpu", language: "Python", category: "cpu", mise_task: "python-cpu" },
        BenchmarkTask { name: "java-cpu", language: "Java", category: "cpu", mise_task: "java-cpu" },
        BenchmarkTask { name: "go-cpu", language: "Go", category: "cpu", mise_task: "go-cpu" },
        BenchmarkTask { name: "ruby-cpu", language: "Ruby", category: "cpu", mise_task: "ruby-cpu" },
        BenchmarkTask { name: "cs-cpu", language: "C#", category: "cpu", mise_task: "cs-cpu" },
        BenchmarkTask { name: "c-allocator", language: "C", category: "allocator", mise_task: "c-allocator" },
        BenchmarkTask { name: "cpp-allocator", language: "C++", category: "allocator", mise_task: "cpp-allocator" },
        BenchmarkTask { name: "rust-allocator", language: "Rust", category: "allocator", mise_task: "rust-allocator" },
        BenchmarkTask { name: "python-allocator", language: "Python", category: "allocator", mise_task: "python-allocator" },
        BenchmarkTask { name: "java-allocator", language: "Java", category: "allocator", mise_task: "java-allocator" },
        BenchmarkTask { name: "go-allocator", language: "Go", category: "allocator", mise_task: "go-allocator" },
        BenchmarkTask { name: "ruby-allocator", language: "Ruby", category: "allocator", mise_task: "ruby-allocator" },
        BenchmarkTask { name: "cs-allocator", language: "C#", category: "allocator", mise_task: "cs-allocator" },
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
            (r"^([a-zA-Z0-9_\- \()]+):\s*([\d.eE+-]+)\s*(ms|ns|ops/sec|elements/sec|GFLOPS|MOPS|GB/s)", 1, 2, 3),
            // "Name value unit"
            (r"^([a-zA-Z0-9_\- \()]+)\s+([\d.eE+-]+)\s+(ms|ns|ops/sec|elements/sec|GFLOPS|MOPS|GB/s)", 1, 2, 3),
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

fn run_mise_task(task: &str) -> (bool, String, f64) {
    let start = Instant::now();
    let output = Command::new("mise")
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
        Err(e) => (false, format!("Failed to run mise: {}", e), elapsed),
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
        
        let (success, output, elapsed) = run_mise_task(task.mise_task);
        
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
    
    println!("View results with: mise monitor");
}
