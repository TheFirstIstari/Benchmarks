# Cross-Language Benchmark Suite

A comprehensive benchmark suite comparing performance across multiple programming languages: C, C++, Rust, Python, and Java with an interactive TUI for visualizing results.

## Features

- **Multi-language support**: C (clang), C++ (clang), Rust, Python, Java
- **12 benchmark categories**: Matrix, Sort, String, Hash, Regex, JSON, File I/O, Math, Network, Crypto, ML, Concurrency
- **Interactive TUI**: Graph and table views with statistical analysis
- **SQLite storage**: Persistent benchmark result history
- **Row selection**: Navigate and drill down into specific benchmarks
- **Statistical view**: Standard deviation, quartiles, min/max

## Quick Start

```bash
# Install dependencies
mise install

# Compile all benchmarks (do this once before running)
mise compile

# Run all benchmarks
mise all

# View results in TUI
mise monitor
```

## Requirements

- **mise** - Language version manager and task runner (`brew install mise`)
- **C/C++** - GCC or Clang with optimizations
- **Python** - 3.x with numpy
- **Rust** - Latest stable
- **Java** - OpenJDK 21+

## Benchmark Commands

```bash
# Compile all (before running)
mise compile

# Run all benchmarks (compile + execute)
mise all

# Run only (requires compiled binaries)
mise run-all-benchmarks

# By language
mise c-run          # Compile + run all C benchmarks
mise cpp-run        # Compile + run all C++ benchmarks
mise rust-run       # Compile + run all Rust benchmarks
mise python-all     # Run all Python benchmarks
mise java-run       # Compile + run all Java benchmarks

# Compile by language
mise c-compile
mise cpp-compile
mise rust-compile
mise java-compile

# Individual benchmarks (requires compilation first)
mise c-matrix
mise rust-sort
mise python-hash
```

### By Category

```bash
mise matrix-all      # Run all matrix benchmarks
mise sort-all        # Run all sort benchmarks
mise hash-all        # Run all hash benchmarks
mise regex-all       # Run all regex benchmarks
mise json-all        # Run all json benchmarks
mise fileio-all      # Run all file I/O benchmarks
mise math-all        # Run all math benchmarks
mise network-all     # Run all network benchmarks
mise crypto-all      # Run all crypto benchmarks
mise ml-all          # Run all ML benchmarks
```

### Tools

```bash
mise monitor         # Launch interactive TUI
mise store           # List stored benchmark runs
mise runner          # Run benchmarks with automatic result storage
```

## Monitor TUI

Run `mise monitor` to launch the interactive TUI:

| Key | Action |
|-----|--------|
| `[1]` | Graph view (line chart) |
| `[2]` | Table view (list) |
| `[3]` | Statistics view |
| `[j/k]` or arrows | Navigate rows in table |
| `[Enter]` | Detail view for selected benchmark |
| `[s]` | Select benchmarks to run |
| `[r]` | Run selected benchmarks |
| `[R]` | Refresh data from database |
| `[q]` | Quit |

### Detail & Stats Views

- **Detail View**: Shows all languages compared for a specific benchmark with min/max
- **Statistics View**: Shows avg, min, max, std_dev (standard deviation), Q1 (25th percentile), Q3 (75th percentile), runs

## Project Structure

```
Benchmarks/
├── C/              # C benchmarks (.c files)
├── C++/            # C++ benchmarks (.cpp files)
├── Python/         # Python benchmarks (.py files)
├── Java/           # Java benchmarks (.java files)
├── Rust/           # Rust benchmarks (.rs files) + Cargo.toml
├── tools/          # Monitor & database tools
│   ├── monitor.rs  # TUI application
│   ├── runner.rs   # Benchmark runner
│   ├── lib.rs      # Database & types
│   └── Cargo.toml
├── mise.toml       # Tool versions & task definitions
└── README.md
```

## Optimization Flags

- **C/C++**: `-O3 -ffast-math -march=native` (maximum compiler optimizations)
- **Rust**: `opt-level = 3, lto = "fat", codegen-units = 1`
- **Java**: JIT warmup enabled before timing

## Benchmark Categories

| Category | Tests |
|----------|-------|
| **Matrix** | Multiply, Transpose, Add (2000x2000 float matrices) |
| **Sort** | Qsort, Quicksort, Heapsort, Mergesort (5M integers) |
| **String** | Concatenation, Split, Replace, Search |
| **Hash** | SDBM, DJB2, FNV (10K iterations) |
| **Regex** | Find, Count, Email pattern match |
| **JSON** | Parse, Serialize, Count fields (1MB payload) |
| **File I/O** | Write, Read, Read lines, Random access (5MB files) |
| **Math** | Trig (sin/cos), Exp/Log, Arithmetic |
| **Network** | Pipe throughput, Memcpy 8K |
| **Crypto** | XOR, Integer ops |
| **ML** | Element-wise mul, Dot product, Lerp, Sigmoid |
| **Concurrency** | Thread pool, Atomic counter |

## Adding New Benchmarks

1. Add source file to each language directory
2. Add tasks to `mise.toml` with `*-compile` and `*` (run) variants
3. Rebuild tools if needed: `mise tools-build`

## Sharing Results

Results are stored in SQLite. To share results between machines:

```bash
# Export results
sqlite3 ~/.local/share/benchmarks/benchmarks.db .dump > results.sql

# Import on another machine
sqlite3 ~/.local/share/benchmarks/benchmarks.db < results.sql
```

Or copy the database file:
```
~/.local/share/benchmarks/benchmarks.db
```

## License

MIT
