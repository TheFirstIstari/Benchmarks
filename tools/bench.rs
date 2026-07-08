// bench — standalone benchmark comparison tool
// Shows relative performance across languages for each benchmark.
// Usage: bench          (TUI)
//        bench run      (run all benchmarks, store to DB)
//        bench run -c sort  (run one category)
//        bench show     (print results to stdout)

use bench_tools::{Database, BenchmarkResult};
use chrono::Utc;
use clap::{Parser, Subcommand};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{BarChart, Block, Borders, Paragraph, Row, Table, TableState};
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "bench", about = "Benchmark comparison tool")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run benchmarks and store results
    Run {
        #[arg(short, long)]
        categories: Option<Vec<String>>,
        #[arg(short, long, default_value = "10")]
        iterations: i32,
    },
    /// Print results to stdout
    Show,
}

// ── Task definitions ──────────────────────────────────────────────────

struct Task {
    mise: &'static str,
    lang: &'static str,
    cat: &'static str,
}

fn tasks() -> Vec<Task> {
    vec![
        Task { mise: "c-matrix",       lang: "C",      cat: "matrix" },
        Task { mise: "cpp-matrix",     lang: "C++",    cat: "matrix" },
        Task { mise: "rust-matrix",    lang: "Rust",   cat: "matrix" },
        Task { mise: "python-matrix",  lang: "Python", cat: "matrix" },
        Task { mise: "java-matrix",    lang: "Java",   cat: "matrix" },
        Task { mise: "c-sort",         lang: "C",      cat: "sort" },
        Task { mise: "cpp-sort",       lang: "C++",    cat: "sort" },
        Task { mise: "rust-sort",      lang: "Rust",   cat: "sort" },
        Task { mise: "python-sort",    lang: "Python", cat: "sort" },
        Task { mise: "java-sort",      lang: "Java",   cat: "sort" },
        Task { mise: "c-string",       lang: "C",      cat: "string" },
        Task { mise: "cpp-string",     lang: "C++",    cat: "string" },
        Task { mise: "rust-string",    lang: "Rust",   cat: "string" },
        Task { mise: "python-string",  lang: "Python", cat: "string" },
        Task { mise: "java-string",    lang: "Java",   cat: "string" },
        Task { mise: "c-hash",         lang: "C",      cat: "hash" },
        Task { mise: "cpp-hash",       lang: "C++",    cat: "hash" },
        Task { mise: "rust-hash",      lang: "Rust",   cat: "hash" },
        Task { mise: "python-hash",    lang: "Python", cat: "hash" },
        Task { mise: "java-hash",      lang: "Java",   cat: "hash" },
        Task { mise: "c-regex",        lang: "C",      cat: "regex" },
        Task { mise: "cpp-regex",      lang: "C++",    cat: "regex" },
        Task { mise: "rust-regex",     lang: "Rust",   cat: "regex" },
        Task { mise: "python-regex",   lang: "Python", cat: "regex" },
        Task { mise: "java-regex",     lang: "Java",   cat: "regex" },
        Task { mise: "c-json",         lang: "C",      cat: "json" },
        Task { mise: "cpp-json",       lang: "C++",    cat: "json" },
        Task { mise: "rust-json",      lang: "Rust",   cat: "json" },
        Task { mise: "python-json",    lang: "Python", cat: "json" },
        Task { mise: "java-json",      lang: "Java",   cat: "json" },
        Task { mise: "c-fileio",       lang: "C",      cat: "fileio" },
        Task { mise: "cpp-fileio",     lang: "C++",    cat: "fileio" },
        Task { mise: "rust-fileio",    lang: "Rust",   cat: "fileio" },
        Task { mise: "python-fileio",  lang: "Python", cat: "fileio" },
        Task { mise: "java-fileio",    lang: "Java",   cat: "fileio" },
        Task { mise: "c-math",         lang: "C",      cat: "math" },
        Task { mise: "cpp-math",       lang: "C++",    cat: "math" },
        Task { mise: "rust-math",      lang: "Rust",   cat: "math" },
        Task { mise: "python-math",    lang: "Python", cat: "math" },
        Task { mise: "java-math",      lang: "Java",   cat: "math" },
        Task { mise: "c-crypto",       lang: "C",      cat: "crypto" },
        Task { mise: "cpp-crypto",     lang: "C++",    cat: "crypto" },
        Task { mise: "rust-crypto",    lang: "Rust",   cat: "crypto" },
        Task { mise: "python-crypto",  lang: "Python", cat: "crypto" },
        Task { mise: "java-crypto",    lang: "Java",   cat: "crypto" },
        Task { mise: "c-cpu",          lang: "C",      cat: "cpu" },
        Task { mise: "cpp-cpu",        lang: "C++",    cat: "cpu" },
        Task { mise: "rust-cpu",       lang: "Rust",   cat: "cpu" },
        Task { mise: "python-cpu",     lang: "Python", cat: "cpu" },
        Task { mise: "java-cpu",       lang: "Java",   cat: "cpu" },
        Task { mise: "c-network",      lang: "C",      cat: "network" },
        Task { mise: "cpp-network",    lang: "C++",    cat: "network" },
        Task { mise: "rust-network",   lang: "Rust",   cat: "network" },
        Task { mise: "python-network", lang: "Python", cat: "network" },
        Task { mise: "c-ml",           lang: "C",      cat: "ml" },
        Task { mise: "rust-ml",        lang: "Rust",   cat: "ml" },
        Task { mise: "python-ml",      lang: "Python", cat: "ml" },
        Task { mise: "c-concurrency",  lang: "C",      cat: "concurrency" },
        Task { mise: "cpp-concurrency",lang: "C++",    cat: "concurrency" },
        Task { mise: "rust-concurrency",lang: "Rust",  cat: "concurrency" },
        Task { mise: "java-concurrency",lang: "Java",  cat: "concurrency" },
    ]
}

// ── Output parsing ────────────────────────────────────────────────────

fn parse_output(output: &str, lang: &str, cat: &str) -> Vec<BenchmarkResult> {
    let host = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".into());
    let now = Utc::now();
    let mut out = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('=') {
            continue;
        }
        // "Name: value ms" or "Name: value ms (extra)"
        if let Some((name, ms)) = parse_time_line(line) {
            out.push(BenchmarkResult {
                id: None,
                language: lang.into(),
                category: cat.into(),
                test_name: name,
                time_ms: ms,
                metric: "ms".into(),
                value: ms,
                metadata: None,
                timestamp: now,
                hostname: host.clone(),
            });
        }
    }
    out
}

use std::sync::OnceLock;

fn parse_time_line(line: &str) -> Option<(String, f64)> {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"^([a-zA-Z0-9_ ]+?):\s*([\d.]+)\s*ms").unwrap()
    });

    let caps = re.captures(line)?;
    let name = caps.get(1)?.as_str().trim().to_string();
    let ms: f64 = caps.get(2)?.as_str().parse().ok()?;
    if ms <= 0.0 || !ms.is_finite() {
        return None;
    }
    Some((name, ms))
}

// ── Run subcommand ────────────────────────────────────────────────────

fn run_benchmarks(categories: Option<Vec<String>>, iterations: i32) {
    let db = Database::new(None).expect("DB");
    let name = format!("run-{}", Utc::now().format("%Y%m%d-%H%M%S"));
    let run_id = db.start_run(&name, iterations).expect("start run");

    println!("Running benchmarks ({} iterations each)...", iterations);
    println!("{}", "─".repeat(60));

    let all_tasks = tasks();
    let filtered: Vec<_> = match &categories {
        Some(cats) => all_tasks.iter().filter(|t| cats.contains(&t.cat.to_string())).collect(),
        None => all_tasks.iter().collect(),
    };

    let mut ok = 0u32;
    let mut fail = 0u32;
    let mut results_stored = 0u32;

    for t in &filtered {
        print!("  {:<20} {:<8} ... ", t.mise, t.lang);
        io::stdout().flush().ok();

        let mut cmd_results = 0u32;
        for _ in 0..iterations {
            let output = Command::new("mise").arg(t.mise).output();
            match output {
                Ok(out) if out.status.success() => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let combined = format!("{}\n{}", stdout, stderr);
                    let parsed = parse_output(&combined, t.lang, t.cat);
                    for r in &parsed {
                        if db.insert_result(run_id, r).is_ok() {
                            results_stored += 1;
                            cmd_results += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        if cmd_results > 0 {
            ok += 1;
            println!("OK ({} results)", cmd_results);
        } else {
            fail += 1;
            println!("FAIL");
        }
    }

    db.update_result_count(run_id).ok();
    let status = if fail == 0 { "completed" } else { "partial" };
    db.complete_run(run_id, status).ok();

    println!("{}", "─".repeat(60));
    println!("Done: {} ok, {} failed, {} results stored", ok, fail, results_stored);
    println!("View with: bench");
}

// ── Show subcommand ───────────────────────────────────────────────────

fn show_results() {
    let db = Database::new(None).expect("DB");
    let stats = db.get_stats(None).unwrap_or_default();

    // Group by test_name
    let mut grouped: HashMap<String, Vec<&bench_tools::BenchmarkStats>> = HashMap::new();
    for s in &stats {
        grouped.entry(s.test_name.clone()).or_default().push(s);
    }

    let mut tests: Vec<_> = grouped.keys().cloned().collect();
    tests.sort();

    // Collect all languages
    let mut langs: Vec<String> = stats.iter().map(|s| s.language.clone()).collect();
    langs.sort();
    langs.dedup();

    // Print table
    print!("{:<20}", "Benchmark");
    for l in &langs {
        print!(" {:>10}", l);
    }
    println!();
    println!("{}", "─".repeat(20 + langs.len() * 11));

    for test in &tests {
        let row = grouped.get(test).unwrap();
        let mut fastest = f64::MAX;
        for s in row {
            if s.avg_ms < fastest {
                fastest = s.avg_ms;
            }
        }
        print!("{:<20}", test);
        for l in &langs {
            if let Some(s) = row.iter().find(|s| &s.language == l) {
                let is_fastest = (s.avg_ms - fastest).abs() < fastest * 1e-9;
                let ratio = s.avg_ms / fastest;
                if is_fastest {
                    print!(" {:>9.1}★", s.avg_ms);
                } else {
                    print!(" {:>8.1}x", ratio);
                }
            } else {
                print!(" {:>10}", "—");
            }
        }
        println!();
    }
}

// ── TUI ───────────────────────────────────────────────────────────────

fn lang_color(lang: &str) -> Color {
    match lang {
        "C" => Color::Cyan,
        "C++" => Color::Yellow,
        "Rust" => Color::Red,
        "Java" => Color::Green,
        "Python" => Color::Blue,
        _ => Color::Gray,
    }
}

fn fmt_time(ms: f64) -> String {
    if ms >= 1000.0 {
        format!("{:.1}s", ms / 1000.0)
    } else if ms >= 1.0 {
        format!("{:.1}ms", ms)
    } else {
        format!("{:.0}μs", ms * 1000.0)
    }
}

#[derive(Clone, Copy)]
struct Stat {
    avg_ms: f64,
    min_ms: f64,
    max_ms: f64,
    std_dev_ms: f64,
    runs: i32,
}

struct App {
    db: Database,
    stats: Vec<bench_tools::BenchmarkStats>,
    tests: Vec<String>,
    langs: Vec<String>,
    /// test_name → (lang → Stat)
    grid: HashMap<String, HashMap<String, Stat>>,
    /// category → Vec<test_name>
    cat_tests: HashMap<String, Vec<String>>,
    /// test_name → category
    test_cat: HashMap<String, String>,
    categories: Vec<String>,
    selected: usize,
    show_bars: bool,
    /// None = overview, Some(cat) = sub-benchmark detail view
    detail_cat: Option<String>,
    detail_selected: usize,
    // Background run progress
    run_done: Arc<AtomicUsize>,
    run_total: Arc<AtomicUsize>,
    run_active: bool,
    wipe_confirm: bool,
}

impl App {
    fn new() -> Self {
        let db = Database::new(None).expect("DB");
        let mut app = Self {
            db,
            stats: Vec::new(),
            tests: Vec::new(),
            langs: Vec::new(),
            grid: HashMap::new(),
            cat_tests: HashMap::new(),
            test_cat: HashMap::new(),
            categories: Vec::new(),
            selected: 0,
            show_bars: false,
            detail_cat: None,
            detail_selected: 0,
            run_done: Arc::new(AtomicUsize::new(0)),
            run_total: Arc::new(AtomicUsize::new(0)),
            run_active: false,
            wipe_confirm: false,
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        self.stats = self.db.get_stats(None).unwrap_or_default();

        // Build grid + category mappings
        self.grid.clear();
        self.cat_tests.clear();
        self.test_cat.clear();
        let mut tests_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut langs_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut cats_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

        for s in &self.stats {
            tests_set.insert(s.test_name.clone());
            langs_set.insert(s.language.clone());
            cats_set.insert(s.category.clone());
            self.test_cat.insert(s.test_name.clone(), s.category.clone());
            self.cat_tests.entry(s.category.clone()).or_default().push(s.test_name.clone());
            self.grid
                .entry(s.test_name.clone())
                .or_default()
                .insert(s.language.clone(), Stat {
                    avg_ms: s.avg_ms,
                    min_ms: s.min_ms,
                    max_ms: s.max_ms,
                    std_dev_ms: s.std_dev_ms,
                    runs: s.runs,
                });
        }

        self.tests = tests_set.into_iter().collect();
        self.langs = langs_set.into_iter().collect();
        self.categories = cats_set.into_iter().collect();
        if self.selected >= self.tests.len() {
            self.selected = self.tests.len().saturating_sub(1);
        }
    }

    fn fastest(&self, test: &str) -> Option<f64> {
        self.grid.get(test)?.values().fold(None, |acc, v| {
            match acc {
                None => Some(v.avg_ms),
                Some(min) if v.avg_ms < min => Some(v.avg_ms),
                other => other,
            }
        })
    }

    /// Get sub-tests for a category
    fn sub_tests(&self, cat: &str) -> Vec<String> {
        self.cat_tests.get(cat).cloned().unwrap_or_default()
    }

    /// Spawn background benchmark run for given tasks
    fn start_bg_run(&mut self, run_tasks: Vec<Task>) {
        if self.run_active {
            return;
        }
        let total = run_tasks.len();
        self.run_done.store(0, Ordering::Relaxed);
        self.run_total.store(total, Ordering::Relaxed);
        self.run_active = true;

        let done = self.run_done.clone();
        std::thread::spawn(move || {
            let db = Database::new(None).expect("DB");
            let name = format!("bg-{}", Utc::now().format("%H%M%S"));
            let run_id = db.start_run(&name, 1).expect("start run");
            for t in &run_tasks {
                let output = Command::new("mise").arg(t.mise).output();
                if let Ok(out) = output {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let combined = format!("{}\n{}", stdout, stderr);
                    for r in parse_output(&combined, t.lang, t.cat) {
                        db.insert_result(run_id, &r).ok();
                    }
                }
                done.fetch_add(1, Ordering::Relaxed);
            }
            db.update_result_count(run_id).ok();
            db.complete_run(run_id, "completed").ok();
        });
    }
}

fn render_table(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    if app.tests.is_empty() {
        let p = Paragraph::new("No data. Run 'bench run' first.")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(p, area);
        return;
    }

    // Header row: Benchmark | C | C++ | Rust | Java | Python
    let mut header_cells = vec![Span::styled("Benchmark", Style::default().add_modifier(Modifier::BOLD))];
    for l in &app.langs {
        header_cells.push(Span::styled(
            format!(" {:>10}", l),
            Style::default().fg(lang_color(l)).add_modifier(Modifier::BOLD),
        ));
    }
    let header = Row::new(vec![Line::from(header_cells)]);

    let mut rows: Vec<Row> = Vec::with_capacity(app.tests.len());
    for (_i, test) in app.tests.iter().enumerate() {
        let fastest = app.fastest(test).unwrap_or(f64::MAX);

        // Show category as prefix for context
        let cat = app.test_cat.get(test).map(|s| s.as_str()).unwrap_or("?");
        let mut cells = vec![Span::raw(format!("{:<8} {}", cat, test))];
        for l in &app.langs {
            if let Some(st) = app.grid.get(test).and_then(|m| m.get(l)) {
                let is_fastest = (st.avg_ms - fastest).abs() < fastest * 1e-9;
                let spread = st.max_ms - st.min_ms;
                let err_str = if st.runs > 1 && spread > 0.0 {
                    format!("±{}", fmt_time(spread / 2.0))
                } else {
                    String::new()
                };
                if is_fastest {
                    cells.push(Span::styled(
                        format!(" {:>6}★{}", fmt_time(st.avg_ms), if err_str.is_empty() { String::new() } else { format!(" {}", err_str) }),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    let ratio = st.avg_ms / fastest;
                    let color = if ratio > 10.0 {
                        Color::DarkGray
                    } else if ratio > 3.0 {
                        Color::Yellow
                    } else {
                        Color::White
                    };
                    cells.push(Span::styled(
                        format!(" {:>6}x {}", ratio, err_str),
                        Style::default().fg(color),
                    ));
                }
            } else {
                cells.push(Span::styled(" {:>10}", Style::default().fg(Color::DarkGray)));
            }
        }

        rows.push(Row::new(vec![Line::from(cells)]));
    }

    // Column widths
    let mut widths = vec![Constraint::Length(20)];
    for _ in &app.langs {
        widths.push(Constraint::Length(12));
    }

    let mut state = TableState::default();
    state.select(Some(app.selected));

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Benchmark Comparison ★ = fastest, Nx = N times slower "))
        .highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_stateful_widget(table, area, &mut state);
}

fn render_bars(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    if app.tests.is_empty() {
        return;
    }

    let test = &app.tests[app.selected];
    let fastest = app.fastest(test).unwrap_or(1.0);

    // Build bar data with error info
    struct BarEntry { lang: String, val: u64, lo: u64, hi: u64, runs: i32 }
    let bars: Vec<BarEntry> = app.langs.iter()
        .filter_map(|l| {
            let st = app.grid.get(test)?.get(l)?;
            let ratio = st.avg_ms / fastest;
            let val = (100.0 / ratio) as u64;
            // error bars as ratio of fastest
            let lo_ratio = st.min_ms / fastest;
            let hi_ratio = st.max_ms / fastest;
            let lo = (100.0 / hi_ratio) as u64; // slower = smaller bar
            let hi = (100.0 / lo_ratio) as u64; // faster = bigger bar
            Some(BarEntry { lang: l.clone(), val, lo, hi, runs: st.runs })
        })
        .collect();

    if bars.is_empty() {
        frame.render_widget(Paragraph::new("No data for this benchmark"), area);
        return;
    }

    // Title with error info
    let n_runs = bars.first().map(|b| b.runs).unwrap_or(0);
    let title = if n_runs > 1 {
        format!(" {} — fastest: {} ({} runs, [min..max] shown) ", test, fmt_time(fastest), n_runs)
    } else {
        format!(" {} — fastest: {} ", test, fmt_time(fastest))
    };

    // Use simple bar chart with value labels showing avg ± spread
    let bar_data: Vec<(&str, u64)> = bars.iter()
        .map(|b| (b.lang.as_str(), b.val))
        .collect();

    // Build extra info as text below the chart
    let info_lines: Vec<Line> = bars.iter().map(|b| {
        let st = app.grid.get(test).and_then(|m| m.get(&b.lang)).unwrap();
        let spread = st.max_ms - st.min_ms;
        Line::from(vec![
            Span::styled(format!("{:<8}", b.lang), Style::default().fg(lang_color(&b.lang))),
            Span::raw(format!("  avg: {}", fmt_time(st.avg_ms))),
            if spread > 0.0 {
                Span::raw(format!("  [{}..{}]", fmt_time(st.min_ms), fmt_time(st.max_ms)))
            } else {
                Span::raw("")
            },
            Span::raw(format!("  σ={}", fmt_time(st.std_dev_ms))),
        ])
    }).collect();

    // Split area: top 60% for bars, bottom 40% for error stats
    let sub = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .data(&bar_data)
        .bar_width(10)
        .bar_gap(2)
        .max(100)
        .label_style(Style::default().fg(Color::White))
        .value_style(Style::default().fg(Color::Black).add_modifier(Modifier::BOLD));

    frame.render_widget(chart, sub[0]);

    // Error stats panel
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title(" Error bars (min..max, σ std dev) ");
    let stats_para = Paragraph::new(info_lines)
        .block(stats_block)
        .style(Style::default().fg(Color::White));
    frame.render_widget(stats_para, sub[1]);
}

fn render_detail(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let cat = app.detail_cat.as_ref().unwrap();
    let sub_tests = app.sub_tests(cat);

    if sub_tests.is_empty() {
        frame.render_widget(Paragraph::new("No sub-tests for this category"), area);
        return;
    }

    // Find fastest per sub-test across languages
    let mut rows: Vec<Row> = Vec::new();
    let mut header_cells = vec![Span::styled("Sub-test", Style::default().add_modifier(Modifier::BOLD))];
    for l in &app.langs {
        header_cells.push(Span::styled(
            format!(" {:>10}", l),
            Style::default().fg(lang_color(l)).add_modifier(Modifier::BOLD),
        ));
    }
    let header = Row::new(vec![Line::from(header_cells)]);

    for test in &sub_tests {
        let fastest = app.fastest(test).unwrap_or(f64::MAX);
        let mut cells = vec![Span::raw(test.clone())];
        for l in &app.langs {
            if let Some(st) = app.grid.get(test).and_then(|m| m.get(l)) {
                let is_fastest = (st.avg_ms - fastest).abs() < fastest * 1e-9;
                if is_fastest {
                    cells.push(Span::styled(
                        format!(" {:>9}★", fmt_time(st.avg_ms)),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    let ratio = st.avg_ms / fastest;
                    let color = if ratio > 10.0 { Color::DarkGray }
                        else if ratio > 3.0 { Color::Yellow }
                        else { Color::White };
                    cells.push(Span::styled(
                        format!(" {:>8}x", ratio),
                        Style::default().fg(color),
                    ));
                }
            } else {
                cells.push(Span::styled(" {:>10}", Style::default().fg(Color::DarkGray)));
            }
        }
        rows.push(Row::new(vec![Line::from(cells)]));
    }

    let mut widths = vec![Constraint::Length(20)];
    for _ in &app.langs {
        widths.push(Constraint::Length(12));
    }

    let mut state = TableState::default();
    state.select(Some(app.detail_selected));

    let title = format!(" {} — sub-benchmarks (Enter=back) ", cat);
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_stateful_widget(table, area, &mut state);
}

fn render_footer(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut spans = vec![
        Span::styled("[j/k]", Style::default().fg(Color::Cyan)),
        Span::raw(" navigate  "),
        Span::styled("[Tab]", Style::default().fg(Color::Cyan)),
        Span::raw(" table/bars  "),
        Span::styled("[r]", Style::default().fg(Color::Cyan)),
        Span::raw(" run all  "),
        Span::styled("[s]", Style::default().fg(Color::Cyan)),
        Span::raw(" run cat  "),
        Span::styled("[d]", Style::default().fg(Color::Cyan)),
        Span::raw(" wipe  "),
        Span::styled("[R]", Style::default().fg(Color::Cyan)),
        Span::raw(" refresh  "),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::raw(" quit  "),
    ];

    if app.wipe_confirm {
        spans.push(Span::styled("│ Press d again to WIPE!", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
    } else if app.run_active {
        let done = app.run_done.load(Ordering::Relaxed);
        let total = app.run_total.load(Ordering::Relaxed);
        spans.push(Span::styled(format!("│ Running {}/{}...", done, total), Style::default().fg(Color::Yellow)));
    } else {
        spans.push(Span::raw(format!("│ {} benchmarks, {} languages", app.tests.len(), app.langs.len())));
    }

    let line = Line::from(spans);
    let p = Paragraph::new(vec![line])
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(p, area);
}

fn render_header(frame: &mut ratatui::Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled(" BENCH ", Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::raw("  language performance comparison  "),
        Span::styled(format!("{} ", chrono::Utc::now().format("%Y-%m-%d")), Style::default().fg(Color::DarkGray)),
    ]);
    let p = Paragraph::new(vec![line])
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(p, area);
}

fn run_tui() {
    crossterm::terminal::enable_raw_mode().ok();
    execute!(io::stdout(), EnterAlternateScreen).ok();

    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).expect("terminal");

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(size);

            render_header(f, chunks[0]);
            if app.detail_cat.is_some() {
                render_detail(f, chunks[1], &app);
            } else if app.show_bars {
                render_bars(f, chunks[1], &app);
            } else {
                render_table(f, chunks[1], &app);
            }
            render_footer(f, chunks[2], &app);
        }).ok();

        // Check if background run finished
        if app.run_active && app.run_done.load(Ordering::Relaxed) >= app.run_total.load(Ordering::Relaxed) {
            app.run_active = false;
            app.refresh();
        }

        // Non-blocking poll: redraw every 200ms for live progress
        if !event::poll(std::time::Duration::from_millis(200)).unwrap_or(false) {
            continue;
        }

        if let Ok(Event::Key(key)) = event::read() {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.tests.is_empty() {
                        app.selected = (app.selected + 1) % app.tests.len();
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !app.tests.is_empty() {
                        if app.selected == 0 {
                            app.selected = app.tests.len() - 1;
                        } else {
                            app.selected -= 1;
                        }
                    }
                }
                KeyCode::Tab => {
                    app.show_bars = !app.show_bars;
                }
                KeyCode::Enter => {
                    if app.detail_cat.is_some() {
                        app.detail_cat = None;
                    } else if let Some(test) = app.tests.get(app.selected) {
                        if let Some(cat) = app.test_cat.get(test) {
                            let sub = app.sub_tests(cat);
                            if sub.len() > 1 {
                                app.detail_cat = Some(cat.clone());
                                app.detail_selected = 0;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    app.detail_cat = None;
                }
                KeyCode::Char('R') => {
                    app.refresh();
                }
                KeyCode::Char('r') => {
                    // Background run all benchmarks
                    app.start_bg_run(tasks());
                }
                KeyCode::Char('s') => {
                    // Background run selected category only
                    if let Some(test) = app.tests.get(app.selected) {
                        if let Some(cat) = app.test_cat.get(test) {
                            let cat_tasks: Vec<Task> = tasks().into_iter()
                                .filter(|t| t.cat == cat.as_str())
                                .collect();
                            app.start_bg_run(cat_tasks);
                        }
                    }
                }
                KeyCode::Char('d') => {
                    // Wipe DB — double press to confirm
                    if app.wipe_confirm {
                        app.db.wipe().ok();
                        app.wipe_confirm = false;
                        app.refresh();
                    } else {
                        app.wipe_confirm = true;
                    }
                }
                _ => {}
            }
            // Reset wipe confirmation after any other key
            if key.code != KeyCode::Char('d') {
                app.wipe_confirm = false;
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen).ok();
    crossterm::terminal::disable_raw_mode().ok();
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Cmd::Run { categories, iterations }) => {
            run_benchmarks(categories, iterations);
        }
        Some(Cmd::Show) => {
            show_results();
        }
        None => {
            run_tui();
        }
    }
}
