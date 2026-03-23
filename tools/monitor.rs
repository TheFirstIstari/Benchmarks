use bench_tools::{Database, BenchmarkResult, BenchmarkStats, BenchmarkRun};
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, LegendPosition, Table, TableState};
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

const ITERATIONS: usize = 10;

fn parse_benchmark_output(output: &str, language: &str, category: &str) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    let host = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let test_name = match category {
        "matrix" => "Matrix",
        "sort" => "Sort", 
        "string" => "String",
        _ => category,
    };
    
    let mut total_ms = 0.0;
    let mut count = 0;
    
    for line in output.lines() {
        let line = line.trim();
        
        if let Some(ms) = extract_time_ms(line) {
            total_ms += ms;
            count += 1;
        }
    }
    
    if count > 0 {
        let avg_ms = total_ms / count as f64;
        results.push(BenchmarkResult {
            id: None,
            language: language.to_string(),
            category: category.to_string(),
            test_name: test_name.to_string(),
            time_ms: avg_ms,
            metric: "ms".to_string(),
            value: avg_ms,
            metadata: None,
            timestamp: Utc::now(),
            hostname: host,
        });
    }
    
    results
}

fn extract_time_ms(line: &str) -> Option<f64> {
    let patterns = [
        r"(\d+\.?\d*)\s*ms",
        r"Average\s*time:\s*(\d+\.?\d*)",
        r"(\d+\.?\d*)\s*ms\s*avg",
    ];
    
    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(caps) = re.captures(line) {
                if let Some(m) = caps.get(1) {
                    if let Ok(val) = m.as_str().parse::<f64>() {
                        return Some(val);
                    }
                }
            }
        }
    }
    None
}

fn get_unique_test_names(stats: &[BenchmarkStats]) -> Vec<String> {
    let mut names: std::collections::HashSet<String> = stats.iter().map(|s| s.test_name.clone()).collect();
    let mut names: Vec<String> = names.into_iter().collect();
    names.sort();
    names
}

fn get_flat_entries(stats: &[BenchmarkStats]) -> Vec<(String, String, f64, i32)> {
    let mut grouped: HashMap<String, Vec<&BenchmarkStats>> = HashMap::new();
    for stat in stats {
        grouped.entry(stat.test_name.clone()).or_default().push(stat);
    }
    
    let mut test_names: Vec<_> = grouped.keys().cloned().collect();
    test_names.sort();
    
    let mut flat_entries: Vec<(String, String, f64, i32)> = Vec::new();
    for test_name in &test_names {
        if let Some(test_stats) = grouped.get(test_name) {
            for stat in test_stats {
                flat_entries.push((stat.test_name.clone(), stat.language.clone(), stat.avg_ms, stat.runs));
            }
        }
    }
    flat_entries
}

fn get_test_name_at_row(stats: &[BenchmarkStats], row: usize) -> Option<String> {
    let entries = get_flat_entries(stats);
    entries.get(row).map(|(name, _, _, _)| name.clone())
}

fn get_row_count(stats: &[BenchmarkStats]) -> usize {
    get_flat_entries(stats).len()
}

#[derive(Clone)]
struct TestBatch {
    name: String,
    just_tasks: Vec<String>,
    enabled: bool,
}

enum ViewMode {
    Graph,
    Table,
    Detail,
    Stats,
}

enum View {
    Main,
    Select,
    Detail { test_name: String },
    Stats { test_name: String },
    Running { progress: f32, current: String, completed: Vec<String> },
}

struct App {
    db: Database,
    runs: Vec<BenchmarkRun>,
    stats: Vec<BenchmarkStats>,
    view: View,
    view_mode: ViewMode,
    selected_batch: usize,
    selected_row: usize,
    batches: Vec<TestBatch>,
}

impl App {
    fn new() -> Self {
        let db = Database::new(None).expect("Failed to open database");
        let runs = db.get_runs(50).unwrap_or_default();
        let stats = db.get_stats(None).unwrap_or_default();
        
        let batches = vec![
            TestBatch { name: "All".to_string(), just_tasks: vec!["all".to_string()], enabled: false },
            TestBatch { name: "C (clang)".to_string(), just_tasks: vec!["c-matrix".to_string(), "c-sort".to_string(), "c-string".to_string(), "c-hash".to_string(), "c-regex".to_string()], enabled: true },
            TestBatch { name: "C (gcc)".to_string(), just_tasks: vec!["gcc-matrix".to_string(), "gcc-sort".to_string(), "gcc-string".to_string(), "gcc-hash".to_string(), "gcc-regex".to_string()], enabled: true },
            TestBatch { name: "C++ (clang)".to_string(), just_tasks: vec!["cpp-matrix".to_string(), "cpp-sort".to_string(), "cpp-string".to_string(), "cpp-hash".to_string(), "cpp-regex".to_string()], enabled: true },
            TestBatch { name: "C++ (gcc)".to_string(), just_tasks: vec!["gxx-matrix".to_string(), "gxx-sort".to_string(), "gxx-string".to_string(), "gxx-hash".to_string(), "gxx-regex".to_string()], enabled: true },
            TestBatch { name: "Rust".to_string(), just_tasks: vec!["rust-matrix".to_string(), "rust-sort".to_string(), "rust-string".to_string(), "rust-hash".to_string(), "rust-regex".to_string()], enabled: true },
            TestBatch { name: "Python".to_string(), just_tasks: vec!["python-matrix".to_string(), "python-sort".to_string(), "python-string".to_string(), "python-hash".to_string(), "python-regex".to_string()], enabled: true },
            TestBatch { name: "Java".to_string(), just_tasks: vec!["java-matrix".to_string(), "java-sort".to_string(), "java-string".to_string(), "java-hash".to_string(), "java-regex".to_string()], enabled: true },
            TestBatch { name: "C#".to_string(), just_tasks: vec!["cs-matrix".to_string(), "cs-sort".to_string(), "cs-string".to_string()], enabled: false },
        ];
        
        Self {
            db,
            runs,
            stats,
            view: View::Main,
            view_mode: ViewMode::Graph,
            selected_batch: 0,
            selected_row: 0,
            batches,
        }
    }
    
    fn refresh(&mut self) {
        self.runs = self.db.get_runs(50).unwrap_or_default();
        self.stats = self.db.get_stats(None).unwrap_or_default();
    }
    
    fn run_selected(&mut self) {
        let enabled: Vec<_> = self.batches.iter().filter(|b| b.enabled).collect();
        let total_tasks = enabled.iter().map(|b| b.just_tasks.len()).sum::<usize>();
        let total = total_tasks * ITERATIONS;
        
        if total_tasks == 0 {
            return;
        }
        
        let run_name = format!("bench-{}", Utc::now().format("%Y%m%d-%H%M%S"));
        let run_id = self.db.start_run(&run_name, ITERATIONS as i32).unwrap_or(0);
        
        let mut completed = Vec::new();
        let mut failed = 0;
        let mut total_results = 0;
        
        for batch in &enabled {
            for task in &batch.just_tasks {
                let category = if task.contains("matrix") {
                    "matrix"
                } else if task.contains("sort") {
                    "sort"
                } else if task.contains("string") {
                    "string"
                } else if task.contains("hash") {
                    "hash"
                } else if task.contains("regex") {
                    "regex"
                } else {
                    "other"
                };
                
                let mut iter_failed = false;
                
                for iter in 0..ITERATIONS {
                    self.view = View::Running {
                        progress: (completed.len() as f32 + iter as f32 / ITERATIONS as f32) / (total as f32),
                        current: format!("{} ({}) [iter {}/{}]", task, batch.name, iter + 1, ITERATIONS),
                        completed: completed.clone(),
                    };
                    
                    print!("\rRunning {} iter {}/{}... ", task, iter + 1, ITERATIONS);
                    io::stdout().flush().ok();
                    
                    let output = Command::new("just").arg(task).output();
                    
                    match output {
                        Ok(out) if out.status.success() => {
                            let stdout = String::from_utf8_lossy(&out.stdout);
                            let stderr = String::from_utf8_lossy(&out.stderr);
                            let combined = format!("{}\n{}", stdout, stderr);
                            
                            let parsed_results = parse_benchmark_output(&combined, &batch.name, category);
                            for result in &parsed_results {
                                if self.db.insert_result(run_id, result).is_ok() {
                                    total_results += 1;
                                }
                            }
                            
                            println!(" OK (iter {})", iter + 1);
                        }
                        _ => {
                            iter_failed = true;
                            println!(" FAILED (iter {})", iter + 1);
                        }
                    }
                }
                
                if !iter_failed {
                    completed.push(format!("{} ({} iters)", task, ITERATIONS));
                } else {
                    failed += 1;
                }
            }
        }
        
        self.db.update_result_count(run_id).ok();
        if failed == 0 {
            self.db.complete_run(run_id, "completed").ok();
        } else {
            self.db.complete_run(run_id, "partial").ok();
        }
        
        println!("\nTotal: {} results stored", total_results);
        self.refresh();
        self.view = View::Main;
    }
    
    fn toggle_batch(&mut self) {
        self.batches[self.selected_batch].enabled = !self.batches[self.selected_batch].enabled;
    }
}

fn get_lang_color(lang: &str) -> Color {
    let lang_lower = lang.to_lowercase();
    if lang_lower.contains("c (clang") || lang_lower == "c" {
        Color::Cyan
    } else if lang_lower.contains("c (gcc") || lang_lower.contains("c++ (gcc") || lang_lower.contains("g++") {
        Color::Green
    } else if lang_lower.contains("c++") {
        Color::Yellow
    } else if lang_lower.contains("rust") {
        Color::Red
    } else if lang_lower.contains("java") {
        Color::Magenta
    } else if lang_lower.contains("python") {
        Color::Blue
    } else if lang_lower.contains("c#") {
        Color::Rgb(200, 100, 255)
    } else {
        Color::White
    }
}

fn render_line_graph(frame: &mut ratatui::Frame, area: Rect, stats: &[BenchmarkStats]) {
    if stats.is_empty() {
        let text = "No results yet. Run benchmarks with [r]";
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Results "));
        frame.render_widget(paragraph, area);
        return;
    }
    
    let mut grouped: HashMap<String, Vec<&BenchmarkStats>> = HashMap::new();
    for stat in stats {
        grouped.entry(stat.test_name.clone()).or_default().push(stat);
    }
    
    let mut test_names: Vec<_> = grouped.keys().cloned().collect();
    test_names.sort();
    let num_tests = test_names.len();
    
    if num_tests == 0 {
        return;
    }
    
    let mut lang_data: HashMap<String, Vec<(f64, f64, f64)>> = HashMap::new();
    
    for test_name in &test_names {
        if let Some(test_stats) = grouped.get(test_name) {
            for stat in test_stats {
                lang_data
                    .entry(stat.language.clone())
                    .or_default()
                    .push((stat.avg_ms, stat.min_ms, stat.max_ms));
            }
        }
    }
    
    let mut max_val = 0.0f64;
    
    let mut all_lang_points: Vec<(String, Vec<(f64, f64)>)> = Vec::new();
    
    for (lang, values) in &lang_data {
        let data_points: Vec<(f64, f64)> = values
            .iter()
            .enumerate()
            .map(|(i, (avg, _, _))| {
                max_val = max_val.max(*avg);
                (i as f64, *avg)
            })
            .collect();
        
        all_lang_points.push((lang.clone(), data_points));
    }
    
    let datasets: Vec<Dataset> = all_lang_points
        .iter()
        .map(|(lang, data_points)| {
            let color = get_lang_color(lang);
            Dataset::default()
                .name(lang.clone())
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(color))
                .data(data_points.as_slice())
        })
        .collect();
    
    let x_labels: Vec<Span> = test_names
        .iter()
        .map(|s| {
            if s.len() > 8 {
                Span::raw(format!("{}..", &s[..6]))
            } else {
                Span::raw(s.clone())
            }
        })
        .collect();
    
    let x_axis = Axis::default()
        .style(Style::default().fg(Color::DarkGray))
        .bounds([0.0, (num_tests - 1).max(1) as f64])
        .labels(x_labels);
    
    let y_max = if max_val > 0.0 { max_val * 1.1 } else { 100.0 };
    let y_labels: Vec<Span> = if y_max > 1000.0 {
        vec![
            Span::raw("0"),
            Span::raw(format!("{:.0}k", y_max / 2.0 / 1000.0)),
            Span::raw(format!("{:.0}k", y_max / 1000.0))
        ]
    } else if y_max > 100.0 {
        vec![
            Span::raw("0"),
            Span::raw(format!("{:.0}", y_max / 2.0)),
            Span::raw(format!("{:.0}", y_max))
        ]
    } else {
        vec![
            Span::raw("0"),
            Span::raw(format!("{:.0}", y_max / 2.0)),
            Span::raw(format!("{:.0}", y_max))
        ]
    };
    
    let y_axis = Axis::default()
        .style(Style::new().fg(Color::DarkGray))
        .title(Span::styled("ms", Color::Yellow))
        .bounds([0.0, y_max])
        .labels(y_labels);
    
    let chart = Chart::new(datasets)
        .block(Block::default().borders(Borders::ALL).title(" Benchmark Results "))
        .x_axis(x_axis)
        .y_axis(y_axis)
        .legend_position(Some(LegendPosition::TopRight))
        .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)));
    
    frame.render_widget(chart, area);
}

fn render_detail(frame: &mut ratatui::Frame, area: Rect, app: &App, test_name: &str) {
    let header_area = Rect::new(area.x, area.y, area.width, 3);
    let main_area = Rect::new(area.x, area.y + 3, area.width, area.height.saturating_sub(6));
    let footer_area = Rect::new(area.x, area.y + area.height.saturating_sub(3), area.width, 3);
    
    let title = ratatui::widgets::Paragraph::new(format!("Detail View: {}", test_name))
        .style(Style::new().bg(Color::Blue).fg(Color::White))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(title, header_area);
    
    let filtered: Vec<_> = app.stats.iter().filter(|s| s.test_name == test_name).collect();
    
    if filtered.is_empty() {
        let text = "No results for this benchmark";
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(Block::new().borders(Borders::ALL));
        frame.render_widget(paragraph, main_area);
    } else {
        let mut rows: Vec<ratatui::widgets::Row> = vec![
            ratatui::widgets::Row::new(vec![
                Span::styled("Language", Style::new().bold()),
                Span::styled("Avg (ms)", Style::new().bold()),
                Span::styled("Min", Style::new().bold()),
                Span::styled("Max", Style::new().bold()),
                Span::styled("Std Dev", Style::new().bold()),
                Span::styled("Runs", Style::new().bold()),
            ]).style(Style::new().bg(Color::Blue).fg(Color::White))
        ];
        
        for stat in &filtered {
            let color = get_lang_color(&stat.language);
            rows.push(ratatui::widgets::Row::new(vec![
                Span::styled(stat.language.clone(), Style::new().fg(color)),
                Span::raw(format!("{:.2}", stat.avg_ms)),
                Span::raw(format!("{:.2}", stat.min_ms)),
                Span::raw(format!("{:.2}", stat.max_ms)),
                Span::raw(format!("{:.2}", stat.std_dev_ms)),
                Span::raw(stat.runs.to_string()),
            ]));
        }
        
        let table = ratatui::widgets::Table::new(rows, [
            Constraint::Length(14),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
        ])
        .block(Block::new().borders(Borders::ALL))
        .column_spacing(1);
        
        frame.render_widget(table, main_area);
    }
    
    let footer = ratatui::widgets::Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[enter]", Style::new().bg(Color::DarkGray)),
            Span::raw(" stats  "),
            Span::styled("[esc]", Style::new().bg(Color::DarkGray)),
            Span::raw(" back  "),
            Span::raw("│ Enter for statistical detail view"),
        ]),
    ])
    .style(Style::new().bg(Color::Black))
    .block(Block::new().borders(Borders::TOP));
    frame.render_widget(footer, footer_area);
}

fn render_stats_view(frame: &mut ratatui::Frame, area: Rect, app: &App, test_name: &str) {
    let header_area = Rect::new(area.x, area.y, area.width, 3);
    let main_area = Rect::new(area.x, area.y + 3, area.width, area.height.saturating_sub(6));
    let footer_area = Rect::new(area.x, area.y + area.height.saturating_sub(3), area.width, 3);
    
    let title = ratatui::widgets::Paragraph::new(format!("Statistics: {}", test_name))
        .style(Style::new().bg(Color::Blue).fg(Color::White))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(title, header_area);
    
    let filtered: Vec<_> = app.stats.iter().filter(|s| s.test_name == test_name).collect();
    
    if filtered.is_empty() {
        let text = "No results for this benchmark";
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(Block::new().borders(Borders::ALL));
        frame.render_widget(paragraph, main_area);
    } else {
        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(" Language   │  Avg  │  Min  │  Max  │ StdDev  │  Q1   │  Q3   │ Runs ", Style::new().bold())),
            Line::from(Span::raw("─────────────┼────────┼────────┼────────┼─────────┼────────┼────────┼──────")),
        ];
        
        for stat in &filtered {
            let color = get_lang_color(&stat.language);
            let lang = format!("{:12}", stat.language);
            lines.push(Line::from(vec![
                Span::styled(lang, Style::new().fg(color)),
                Span::raw(format!("│{:7.1} ", stat.avg_ms)),
                Span::raw(format!("│{:7.1} ", stat.min_ms)),
                Span::raw(format!("│{:7.1} ", stat.max_ms)),
                Span::raw(format!("│{:6.2} ", stat.std_dev_ms)),
                Span::raw(format!("│{:7.1} ", stat.q1_ms)),
                Span::raw(format!("│{:7.1} ", stat.q3_ms)),
                Span::raw(format!("│{:5} ", stat.runs)),
            ]));
        }
        
        let para = ratatui::widgets::Paragraph::new(lines)
            .style(Style::new().fg(Color::White))
            .block(Block::new().borders(Borders::ALL));
        frame.render_widget(para, main_area);
    }
    
    let footer = ratatui::widgets::Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[esc]", Style::new().bg(Color::DarkGray)),
            Span::raw(" back  "),
            Span::raw("│ StdDev=Standard Deviation, Q1/Q3=25th/75th percentiles"),
        ]),
    ])
    .style(Style::new().bg(Color::Black))
    .block(Block::new().borders(Borders::TOP));
    frame.render_widget(footer, footer_area);
}

fn render_table_view(frame: &mut ratatui::Frame, area: Rect, stats: &[BenchmarkStats], selected_row: usize) {
    if stats.is_empty() {
        let text = "No results yet. Run benchmarks with [r]";
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Results "));
        frame.render_widget(paragraph, area);
        return;
    }
    
    let mut grouped: HashMap<String, Vec<&BenchmarkStats>> = HashMap::new();
    for stat in stats {
        grouped.entry(stat.test_name.clone()).or_default().push(stat);
    }
    
    let mut test_names: Vec<_> = grouped.keys().cloned().collect();
    test_names.sort();
    
    let mut all_languages: std::collections::HashSet<String> = std::collections::HashSet::new();
    for stat in stats {
        all_languages.insert(stat.language.clone());
    }
    let mut languages: Vec<_> = all_languages.into_iter().collect();
    languages.sort();
    
    let col_widths = vec![
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];
    
    let flat_entries = get_flat_entries(stats);
    let total_rows = flat_entries.len();
    
    if total_rows == 0 {
        let text = "No results yet";
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(Block::new().borders(Borders::ALL));
        frame.render_widget(paragraph, area);
        return;
    }
    
    let header = ratatui::widgets::Row::new(vec![
        Span::styled("Test", Style::new().bold()),
        Span::styled("Language", Style::new().bold()),
        Span::styled("Avg (ms)", Style::new().bold()),
        Span::styled("Runs", Style::new().bold()),
    ]).style(Style::new().bg(Color::Blue).fg(Color::White));
    
    let mut rows: Vec<ratatui::widgets::Row> = vec![header];
    
    for (i, (test_name, lang, avg, runs)) in flat_entries.iter().enumerate() {
        let color = get_lang_color(lang);
        let is_selected = i == selected_row;
        let style = if is_selected {
            Style::new().bg(Color::Blue).fg(Color::White)
        } else {
            Style::new().fg(Color::White)
        };
        let row = ratatui::widgets::Row::new(vec![
            Span::raw(test_name.clone()),
            Span::styled(lang.clone(), Style::new().fg(color)),
            Span::raw(format!("{:.1}", avg)),
            Span::raw(runs.to_string()),
        ]).style(style);
        rows.push(row);
    }
    
    let mut table_state = TableState::new();
    table_state.select(Some(selected_row));
    
    let table = Table::new(rows, col_widths)
        .block(Block::default().borders(Borders::ALL).title(" Benchmark Results "))
        .style(Style::new().fg(Color::White))
        .column_spacing(1)
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("► ");
    
    frame.render_widget(table, area);
}

fn render_header(frame: &mut ratatui::Frame, area: Rect, runs: &[BenchmarkRun]) {
    let completed = runs.iter().filter(|r| r.status == "completed").count();
    let text = vec![Line::from(vec![
        Span::styled(" BENCHMARK SUITE ", Style::new().bg(Color::Blue).fg(Color::White)),
        Span::raw("  │  "),
        Span::styled("runs:", Style::new().fg(Color::DarkGray)),
        Span::raw(format!(" {}  ", completed)),
        Span::styled("iters:", Style::new().fg(Color::DarkGray)),
        Span::raw(format!(" {}  ", ITERATIONS)),
    ])];
    
    let paragraph = ratatui::widgets::Paragraph::new(text)
        .style(Style::new().bg(Color::Black))
        .block(Block::new().borders(Borders::BOTTOM));
    frame.render_widget(paragraph, area);
}

fn render_results(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    let tabs_height = 1;
    let footer_height = 3;
    let main_height = area.height.saturating_sub(tabs_height + footer_height);
    
    let tabs_area = Rect::new(area.x, area.y, area.width, tabs_height);
    let main_area = Rect::new(area.x, area.y + tabs_height, area.width, main_height);
    let footer_area = Rect::new(area.x, area.y + tabs_height + main_height, area.width, footer_height);
    
    render_tabs(frame, tabs_area, &app.view_mode);
    
    match app.view_mode {
        ViewMode::Graph => render_line_graph(frame, main_area, &app.stats),
        ViewMode::Table => render_table_view(frame, main_area, &app.stats, app.selected_row),
        ViewMode::Detail => {}
        ViewMode::Stats => {}
    }
    
    let legend = ratatui::widgets::Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[1]", Style::new().bg(Color::DarkGray)),
            Span::raw(" graph  "),
            Span::styled("[2]", Style::new().bg(Color::DarkGray)),
            Span::raw(" table  "),
            Span::styled("[3]", Style::new().bg(Color::DarkGray)),
            Span::raw(" stats  "),
            Span::styled("[j/k]", Style::new().bg(Color::DarkGray)),
            Span::raw(" move  "),
            Span::styled("[enter]", Style::new().bg(Color::DarkGray)),
            Span::raw(" detail  "),
            Span::styled("[s]", Style::new().bg(Color::DarkGray)),
            Span::raw(" select  "),
            Span::styled("[r]", Style::new().bg(Color::DarkGray)),
            Span::raw(" run  "),
            Span::styled("[R]", Style::new().bg(Color::DarkGray)),
            Span::raw(" refresh  "),
            Span::styled("[q]", Style::new().bg(Color::DarkGray)),
            Span::raw(" quit"),
        ]),
    ])
    .style(Style::new().bg(Color::Black))
    .block(Block::new().borders(Borders::TOP));
    frame.render_widget(legend, footer_area);
}

fn render_tabs(frame: &mut ratatui::Frame, area: Rect, current: &ViewMode) {
    let tabs_line = Line::from(vec![
        match current {
            ViewMode::Graph => Span::styled(" [1] GRAPH ", Style::new().bg(Color::Cyan).fg(Color::Black)),
            ViewMode::Table => Span::styled(" [1] GRAPH ", Style::new().fg(Color::DarkGray)),
            ViewMode::Detail => Span::styled(" [1] GRAPH ", Style::new().fg(Color::DarkGray)),
            ViewMode::Stats => Span::styled(" [1] GRAPH ", Style::new().fg(Color::DarkGray)),
        },
        Span::raw(" │ "),
        match current {
            ViewMode::Table => Span::styled(" [2] TABLE ", Style::new().bg(Color::Cyan).fg(Color::Black)),
            ViewMode::Graph => Span::styled(" [2] TABLE ", Style::new().fg(Color::DarkGray)),
            ViewMode::Detail => Span::styled(" [2] TABLE ", Style::new().fg(Color::DarkGray)),
            ViewMode::Stats => Span::styled(" [2] TABLE ", Style::new().fg(Color::DarkGray)),
        },
        Span::raw(" │ "),
        match current {
            ViewMode::Stats => Span::styled(" [3] STATS ", Style::new().bg(Color::Cyan).fg(Color::Black)),
            _ => Span::styled(" [3] STATS ", Style::new().fg(Color::DarkGray)),
        },
    ]);
    
    let paragraph = ratatui::widgets::Paragraph::new(vec![tabs_line])
        .style(Style::new().bg(Color::Rgb(20, 20, 30)))
        .block(Block::new().borders(Borders::BOTTOM));
    frame.render_widget(paragraph, area);
}

fn render_select(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(area);
    
    let header = ratatui::widgets::Paragraph::new("Select Test Batches to Run (10 iterations each)")
        .style(Style::new().bg(Color::Blue).fg(Color::White))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);
    
    let rows: Vec<ratatui::widgets::Row> = app.batches.iter().enumerate().map(|(i, batch)| {
        let marker = if batch.enabled { "[x]" } else { "[ ]" };
        let style = if i == app.selected_batch {
            Style::new().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::new().fg(Color::White)
        };
        let marker_style = if batch.enabled {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::DarkGray)
        };
        ratatui::widgets::Row::new(vec![
            Span::raw(format!("{}{}", Span::styled(marker, marker_style), batch.name)),
            Span::raw(format!(" ({} tasks)", batch.just_tasks.len())),
        ]).style(style)
    }).collect();
    
    let table = ratatui::widgets::Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
        .block(Block::new().borders(Borders::ALL))
        .style(Style::new().fg(Color::White));
    frame.render_widget(table, chunks[1]);
    
    let enabled_count = app.batches.iter().filter(|b| b.enabled).count();
    let enabled_tasks = app.batches.iter().filter(|b| b.enabled)
        .map(|b| b.just_tasks.len()).sum::<usize>();
    
    let footer = ratatui::widgets::Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[space]", Style::new().bg(Color::DarkGray)),
            Span::raw(" toggle  "),
            Span::styled("[j/k]", Style::new().bg(Color::DarkGray)),
            Span::raw(" move  "),
            Span::styled("[a]", Style::new().bg(Color::DarkGray)),
            Span::raw(" all  "),
            Span::styled("[n]", Style::new().bg(Color::DarkGray)),
            Span::raw(" none  "),
        ]),
        Line::from(vec![
            Span::styled(format!(" {} batches, {} tasks  ", enabled_count, enabled_tasks), Style::new().fg(Color::Green)),
            Span::styled("[r]", Style::new().bg(Color::Green)),
            Span::raw(" run  "),
            Span::styled("[esc]", Style::new().bg(Color::DarkGray)),
            Span::raw(" back"),
        ]),
    ])
    .style(Style::new().bg(Color::Black))
    .block(Block::new().borders(Borders::TOP));
    frame.render_widget(footer, chunks[2]);
}

fn render_running(frame: &mut ratatui::Frame, area: Rect, progress: &f32, current: &str, completed: &[String]) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);
    
    let title = ratatui::widgets::Paragraph::new("Running Benchmarks...")
        .style(Style::new().bg(Color::Green).fg(Color::Black))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);
    
    let current_text = ratatui::widgets::Paragraph::new(format!("Current: {}", current))
        .style(Style::new().fg(Color::Yellow))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(current_text, chunks[1]);
    
    let completed_text: Vec<Line> = completed.iter().map(|s| {
        Line::from(vec![Span::styled("  OK  ", Style::new().fg(Color::Green)), Span::raw(s.clone())])
    }).collect();
    let completed_para = ratatui::widgets::Paragraph::new(completed_text)
        .style(Style::new().fg(Color::Green))
        .block(Block::new().title(" Completed ").borders(Borders::ALL));
    frame.render_widget(completed_para, chunks[2]);
    
    let bar_len = (*progress * (area.width - 4) as f32) as usize;
    let bar = "█".repeat(bar_len);
    let progress_text = ratatui::widgets::Paragraph::new(vec![Line::from(vec![
        Span::raw(format!("[{}{}] {:.0}%", bar, " ".repeat((area.width as usize - 4).saturating_sub(bar_len)), progress * 100.0)),
    ])])
    .style(Style::new().fg(Color::Cyan))
    .block(Block::new().borders(Borders::ALL));
    frame.render_widget(progress_text, chunks[3]);
}

fn render_view(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    match &mut app.view {
        View::Main => render_results(frame, area, app),
        View::Select => render_select(frame, area, app),
        View::Running { progress, current, completed } => render_running(frame, area, progress, current, completed),
        View::Detail { test_name } => {
            let test_name = test_name.clone();
            render_detail(frame, area, app, &test_name);
        }
        View::Stats { test_name } => {
            let test_name = test_name.clone();
            render_stats_view(frame, area, app, &test_name);
        }
    }
}

fn main() {
    terminal::enable_raw_mode().ok();
    execute!(io::stdout(), EnterAlternateScreen).ok();
    
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    loop {
        terminal.draw(|f| {
            let size = f.size();
            render_header(f, Rect::new(0, 0, size.width, 3), &app.runs);
            render_view(f, Rect::new(0, 3, size.width, size.height - 3), &mut app);
        }).ok();
        
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            
            match &mut app.view {
                View::Main => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('r') => {
                            execute!(io::stdout(), LeaveAlternateScreen).ok();
                            terminal::disable_raw_mode().ok();
                            app.run_selected();
                            terminal::enable_raw_mode().ok();
                            execute!(io::stdout(), EnterAlternateScreen).ok();
                        }
                        KeyCode::Char('s') => app.view = View::Select,
                        KeyCode::Char('R') => app.refresh(),
                        KeyCode::Char('1') => app.view_mode = ViewMode::Graph,
                        KeyCode::Char('2') => app.view_mode = ViewMode::Table,
                        KeyCode::Char('3') => {
                            let row_count = get_row_count(&app.stats);
                            if row_count > 0 {
                                if let Some(name) = get_test_name_at_row(&app.stats, app.selected_row) {
                                    app.view = View::Stats { test_name: name };
                                }
                            }
                        }
                        KeyCode::Tab | KeyCode::BackTab => {
                            app.view_mode = match app.view_mode {
                                ViewMode::Graph => ViewMode::Table,
                                ViewMode::Table => ViewMode::Stats,
                                ViewMode::Stats => ViewMode::Graph,
                                ViewMode::Detail => ViewMode::Graph,
                            };
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let row_count = get_row_count(&app.stats);
                            if row_count > 0 {
                                app.selected_row = (app.selected_row + 1) % row_count;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            let row_count = get_row_count(&app.stats);
                            if row_count > 0 {
                                if app.selected_row == 0 {
                                    app.selected_row = row_count - 1;
                                } else {
                                    app.selected_row -= 1;
                                }
                            }
                        }
                        KeyCode::Enter => {
                            let row_count = get_row_count(&app.stats);
                            if row_count > 0 {
                                if let Some(name) = get_test_name_at_row(&app.stats, app.selected_row) {
                                    app.view = View::Detail { test_name: name };
                                }
                            }
                        }
                        _ => {}
                    }
                }
                View::Detail { test_name } => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.view = View::Main,
                        KeyCode::Enter => app.view = View::Stats { test_name: test_name.clone() },
                        _ => {}
                    }
                }
                View::Stats { test_name } => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.view = View::Main,
                        _ => {}
                    }
                }
                View::Select => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.view = View::Main,
                        KeyCode::Char(' ') => app.toggle_batch(),
                        KeyCode::Char('r') => {
                            execute!(io::stdout(), LeaveAlternateScreen).ok();
                            terminal::disable_raw_mode().ok();
                            app.run_selected();
                            terminal::enable_raw_mode().ok();
                            execute!(io::stdout(), EnterAlternateScreen).ok();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.selected_batch = (app.selected_batch + 1) % app.batches.len();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.selected_batch == 0 {
                                app.selected_batch = app.batches.len() - 1;
                            } else {
                                app.selected_batch -= 1;
                            }
                        }
                        KeyCode::Char('a') => {
                            for batch in &mut app.batches {
                                batch.enabled = true;
                            }
                        }
                        KeyCode::Char('n') => {
                            for batch in &mut app.batches {
                                batch.enabled = false;
                            }
                        }
                        _ => {}
                    }
                }
                View::Running { .. } => {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }
    }
    
    execute!(io::stdout(), LeaveAlternateScreen).ok();
    terminal::disable_raw_mode().ok();
}
