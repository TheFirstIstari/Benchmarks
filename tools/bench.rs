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

fn parse_time_line(line: &str) -> Option<(String, f64)> {
    // Match "Name: 123.45 ms" or "Name: 123.45 ms (...)"
    let re = regex::Regex::new(
        r"^([a-zA-Z0-9_ ]+?):\s*([\d.]+)\s*ms"
    ).ok()?;

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

struct App {
    db: Database,
    stats: Vec<bench_tools::BenchmarkStats>,
    tests: Vec<String>,
    langs: Vec<String>,
    /// test_name → (lang → avg_ms)
    grid: HashMap<String, HashMap<String, f64>>,
    selected: usize,
    show_bars: bool,
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
            selected: 0,
            show_bars: false,
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        self.stats = self.db.get_stats(None).unwrap_or_default();

        // Build grid
        self.grid.clear();
        let mut tests_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut langs_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

        for s in &self.stats {
            tests_set.insert(s.test_name.clone());
            langs_set.insert(s.language.clone());
            self.grid
                .entry(s.test_name.clone())
                .or_default()
                .insert(s.language.clone(), s.avg_ms);
        }

        self.tests = tests_set.into_iter().collect();
        self.langs = langs_set.into_iter().collect();
        if self.selected >= self.tests.len() {
            self.selected = self.tests.len().saturating_sub(1);
        }
    }

    fn fastest(&self, test: &str) -> Option<f64> {
        self.grid.get(test)?.values().fold(None, |acc, &v| {
            match acc {
                None => Some(v),
                Some(min) if v < min => Some(v),
                other => other,
            }
        })
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

    let mut rows = vec![header];

    for (i, test) in app.tests.iter().enumerate() {
        let fastest = app.fastest(test).unwrap_or(f64::MAX);
        let is_sel = i == app.selected;

        let mut cells = vec![Span::raw(test.clone())];
        for l in &app.langs {
            if let Some(&ms) = app.grid.get(test).and_then(|m| m.get(l)) {
                let is_fastest = (ms - fastest).abs() < fastest * 1e-9;
                if is_fastest {
                    cells.push(Span::styled(
                        format!(" {:>9}★", fmt_time(ms)),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    let ratio = ms / fastest;
                    let color = if ratio > 10.0 {
                        Color::DarkGray
                    } else if ratio > 3.0 {
                        Color::Yellow
                    } else {
                        Color::White
                    };
                    cells.push(Span::styled(
                        format!(" {:>8}x", ratio),
                        Style::default().fg(color),
                    ));
                }
            } else {
                cells.push(Span::styled(" {:>10}", Style::default().fg(Color::DarkGray)));
            }
        }

        let style = if is_sel {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        rows.push(Row::new(vec![Line::from(cells)]).style(style));
    }

    // Column widths
    let mut widths = vec![Constraint::Length(20)];
    for _ in &app.langs {
        widths.push(Constraint::Length(12));
    }

    let mut state = TableState::default();
    state.select(Some(app.selected));

    let table = Table::new(rows, widths)
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

    let title = format!(" {} — relative to fastest ({}x scale) ", test, fmt_time(fastest));

    let bar_data: Vec<(&str, u64)> = app.langs.iter()
        .filter_map(|l| {
            let ms = app.grid.get(test)?.get(l)?;
            let ratio = ms / fastest;
            let bar_val = (100.0 / ratio) as u64;
            Some((l.as_str(), bar_val))
        })
        .collect();

    if bar_data.is_empty() {
        frame.render_widget(
            Paragraph::new("No data for this benchmark"),
            area,
        );
        return;
    }

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .data(&bar_data)
        .bar_width(10)
        .bar_gap(2)
        .max(100)
        .label_style(Style::default().fg(Color::White))
        .value_style(Style::default().fg(Color::Black).add_modifier(Modifier::BOLD));

    frame.render_widget(chart, area);
}

fn render_footer(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let line = Line::from(vec![
        Span::styled("[j/k]", Style::default().fg(Color::Cyan)),
        Span::raw(" navigate  "),
        Span::styled("[Tab]", Style::default().fg(Color::Cyan)),
        Span::raw(" table/bars  "),
        Span::styled("[r]", Style::default().fg(Color::Cyan)),
        Span::raw(" run  "),
        Span::styled("[R]", Style::default().fg(Color::Cyan)),
        Span::raw(" refresh  "),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::raw(" quit  "),
        Span::raw(format!("│ {} benchmarks, {} languages", app.tests.len(), app.langs.len())),
    ]);
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
            if app.show_bars {
                render_bars(f, chunks[1], &app);
            } else {
                render_table(f, chunks[1], &app);
            }
            render_footer(f, chunks[2], &app);
        }).ok();

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
                KeyCode::Char('R') => {
                    app.refresh();
                }
                KeyCode::Char('r') => {
                    // Drop to shell, run benchmarks, come back
                    execute!(io::stdout(), LeaveAlternateScreen).ok();
                    crossterm::terminal::disable_raw_mode().ok();
                    run_benchmarks(None, 3);
                    println!("\nPress Enter to return...");
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf).ok();
                    crossterm::terminal::enable_raw_mode().ok();
                    execute!(io::stdout(), EnterAlternateScreen).ok();
                    app.refresh();
                }
                _ => {}
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
