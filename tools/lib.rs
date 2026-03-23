use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub id: Option<i64>,
    pub language: String,
    pub category: String,
    pub test_name: String,
    pub time_ms: f64,
    pub metric: String,
    pub value: f64,
    pub metadata: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRun {
    pub id: Option<i64>,
    pub name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub total_results: i32,
}

#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    pub language: String,
    pub test_name: String,
    pub category: String,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub std_dev_ms: f64,
    pub q1_ms: f64,
    pub q3_ms: f64,
    pub runs: i32,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: Option<PathBuf>) -> SqliteResult<Self> {
        let db_path = path.unwrap_or_else(|| {
            let exe_path = std::env::current_exe()
                .unwrap_or_else(|_| PathBuf::from("."));
            let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
            exe_dir.join("benchmarks.db")
        });
        
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let conn = Connection::open(&db_path)?;
        let db = Self { conn: Mutex::new(conn) };
        db.init()?;
        Ok(db)
    }
    
    fn init(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS benchmark_runs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                status TEXT NOT NULL DEFAULT 'running',
                total_results INTEGER DEFAULT 0,
                iterations INTEGER DEFAULT 10
            );
            
            CREATE TABLE IF NOT EXISTS benchmark_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id INTEGER NOT NULL,
                language TEXT NOT NULL,
                category TEXT NOT NULL,
                test_name TEXT NOT NULL,
                time_ms REAL NOT NULL,
                metric TEXT NOT NULL,
                value REAL NOT NULL,
                metadata TEXT,
                timestamp TEXT NOT NULL,
                hostname TEXT NOT NULL,
                FOREIGN KEY (run_id) REFERENCES benchmark_runs(id)
            );
            
            CREATE INDEX IF NOT EXISTS idx_results_run ON benchmark_results(run_id);
            CREATE INDEX IF NOT EXISTS idx_results_lang ON benchmark_results(language);
            CREATE INDEX IF NOT EXISTS idx_results_category ON benchmark_results(category);
            CREATE INDEX IF NOT EXISTS idx_results_test ON benchmark_results(test_name);
            "
        )?;
        Ok(())
    }
    
    pub fn start_run(&self, name: &str, iterations: i32) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO benchmark_runs (name, started_at, status, iterations) VALUES (?1, ?2, 'running', ?3)",
            (name, &now, iterations),
        )?;
        Ok(conn.last_insert_rowid())
    }
    
    pub fn complete_run(&self, run_id: i64, status: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE benchmark_runs SET completed_at = ?1, status = ?2 WHERE id = ?3",
            (&now, status, run_id),
        )?;
        Ok(())
    }
    
    pub fn update_result_count(&self, run_id: i64) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE benchmark_runs SET total_results = (
                SELECT COUNT(*) FROM benchmark_results WHERE run_id = ?1
            ) WHERE id = ?1",
            [run_id],
        )?;
        Ok(())
    }
    
    pub fn insert_result(&self, run_id: i64, result: &BenchmarkResult) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO benchmark_results 
             (run_id, language, category, test_name, time_ms, metric, value, metadata, timestamp, hostname)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                run_id,
                &result.language,
                &result.category,
                &result.test_name,
                result.time_ms,
                &result.metric,
                result.value,
                &result.metadata,
                result.timestamp.to_rfc3339(),
                &result.hostname,
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }
    
    pub fn get_runs(&self, limit: i32) -> SqliteResult<Vec<BenchmarkRun>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, started_at, completed_at, status, total_results 
             FROM benchmark_runs ORDER BY started_at DESC LIMIT ?1"
        )?;
        
        let runs = stmt.query_map([limit], |row| {
            Ok(BenchmarkRun {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                completed_at: row.get::<_, Option<String>>(3)?
                    .map(|s| chrono::DateTime::parse_from_rfc3339(&s)
                        .unwrap_or_default()
                        .with_timezone(&Utc)),
                status: row.get(4)?,
                total_results: row.get(5)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(runs)
    }
    
    pub fn get_categories(&self) -> SqliteResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT category FROM benchmark_results ORDER BY category"
        )?;
        
        let categories = stmt.query_map([], |row| row.get(0))?
            .collect::<SqliteResult<Vec<String>>>()?;
        
        Ok(categories)
    }
    
    pub fn get_stats(&self, category: Option<&str>) -> SqliteResult<Vec<BenchmarkStats>> {
        let conn = self.conn.lock().unwrap();
        
        let query = if let Some(cat) = category {
            format!(
                "SELECT language, test_name, category,
                        AVG(time_ms) as avg_ms,
                        MIN(time_ms) as min_ms,
                        MAX(time_ms) as max_ms,
                        COUNT(*) as runs
                 FROM benchmark_results
                 WHERE category = '{}'
                 GROUP BY language, test_name
                 ORDER BY test_name, avg_ms",
                cat.replace("'", "''")
            )
        } else {
            "SELECT language, test_name, category,
                    AVG(time_ms) as avg_ms,
                    MIN(time_ms) as min_ms,
                    MAX(time_ms) as max_ms,
                    COUNT(*) as runs
             FROM benchmark_results
             GROUP BY language, test_name
             ORDER BY category, test_name, avg_ms".to_string()
        };
        
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            Ok(BenchmarkStats {
                language: row.get(0)?,
                test_name: row.get(1)?,
                category: row.get(2)?,
                avg_ms: row.get(3)?,
                min_ms: row.get(4)?,
                max_ms: row.get(5)?,
                std_dev_ms: 0.0,
                q1_ms: 0.0,
                q3_ms: 0.0,
                runs: row.get(6)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        drop(stmt);
        
        let mut results: Vec<BenchmarkStats> = Vec::new();
        
        for mut stat in rows {
            let values_query = format!(
                "SELECT time_ms FROM benchmark_results 
                 WHERE language = '{}' AND test_name = '{}'
                 ORDER BY time_ms",
                stat.language.replace("'", "''"),
                stat.test_name.replace("'", "''")
            );
            
            let mut q_stmt = conn.prepare(&values_query)?;
            let values: Vec<f64> = q_stmt.query_map([], |row| row.get(0))?
                .collect::<SqliteResult<Vec<f64>>>()?;
            
            if values.len() >= 2 {
                let mean = stat.avg_ms;
                let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
                stat.std_dev_ms = variance.sqrt();
            }
            
            if values.len() >= 4 {
                let q1_idx = values.len() / 4;
                let q3_idx = (values.len() * 3) / 4;
                stat.q1_ms = values[q1_idx];
                stat.q3_ms = values[q3_idx];
            } else {
                stat.q1_ms = stat.min_ms;
                stat.q3_ms = stat.max_ms;
            }
            
            results.push(stat);
        }
        
        Ok(results)
    }
    
    pub fn get_latest_results(&self, limit: i32) -> SqliteResult<Vec<BenchmarkResult>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT r.id, r.language, r.category, r.test_name, r.time_ms, r.metric, r.value, 
                    r.metadata, r.timestamp, r.hostname
             FROM benchmark_results r
             WHERE r.run_id = (
                 SELECT id FROM benchmark_runs 
                 WHERE status IN ('completed', 'partial') 
                 ORDER BY started_at DESC LIMIT 1
             )
             ORDER BY r.category, r.language, r.test_name
             LIMIT ?1"
        )?;
        
        let results = stmt.query_map([limit], |row| {
            Ok(BenchmarkResult {
                id: Some(row.get(0)?),
                language: row.get(1)?,
                category: row.get(2)?,
                test_name: row.get(3)?,
                time_ms: row.get(4)?,
                metric: row.get(5)?,
                value: row.get(6)?,
                metadata: row.get(7)?,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                hostname: row.get(9)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(results)
    }
    
    pub fn get_benchmark_results(&self, test_name: &str, language: &str) -> SqliteResult<Vec<BenchmarkResult>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT r.id, r.language, r.category, r.test_name, r.time_ms, r.metric, r.value, 
                    r.metadata, r.timestamp, r.hostname, r.run_id
             FROM benchmark_results r
             WHERE r.test_name = ?1 AND r.language = ?2
             ORDER BY r.timestamp DESC
             LIMIT 100"
        )?;
        
        let results = stmt.query_map([test_name, language], |row| {
            Ok(BenchmarkResult {
                id: Some(row.get(0)?),
                language: row.get(1)?,
                category: row.get(2)?,
                test_name: row.get(3)?,
                time_ms: row.get(4)?,
                metric: row.get(5)?,
                value: row.get(6)?,
                metadata: row.get(7)?,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                hostname: row.get(9)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        Ok(results)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new(None).expect("Failed to create database")
    }
}
