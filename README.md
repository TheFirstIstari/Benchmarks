# Cross-Language Benchmark Suite

A comprehensive benchmark suite comparing performance across multiple programming languages: C, C++, Rust, Python, and Java with an interactive TUI for visualizing results.

## Features

- **Multi-language support**: C (clang/gcc), C++ (clang/g++), Rust, Python, Java
- **9 benchmark categories**: Matrix, Sort, Hash, Regex, JSON, File I/O, Math, Network, ML
- **Interactive TUI**: Graph and table views with statistical analysis
- **SQLite storage**: Persistent benchmark result history
- **Row selection**: Navigate and drill down into specific benchmarks
- **Statistical view**: Standard deviation, quartiles, min/max

## Quick Start

```bash
# Install dependencies
just install

# Run all benchmarks
just all

# View results in TUI
just monitor
```

## Requirements

- **just** - Task runner (`brew install just`)
- **mise** - Language version manager (`brew install mise`)
- **C/C++** - GCC or Clang with optimizations
- **Python** - 3.x with numpy
- **Rust** - Latest stable
- **Java** - OpenJDK 21+

## Benchmark Commands

```bash
# All benchmarks
just all

# All languages for a category
just matrix-all
just sort-all
just hash-all
just json-all
just network-all

# By language (clang)
just c-matrix
just c-sort
just c-hash
just cpp-matrix

# By language (gcc)
just gcc-matrix
just gxx-matrix

# By language
just rust-matrix
just python-matrix
just python-json-multi  # Compare stdlib json vs ujson vs orjson
just java-matrix
```

## Monitor TUI

Run `just monitor` to launch the interactive TUI:

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
├── justfile        # Task definitions
├── mise.toml       # Tool versions
└── README.md
```

## Optimization Flags

- **C/C++**: `-Ofast -march=native` (maximum compiler optimizations including `-ffast-math`)
- **Rust**: `opt-level = 3, lto = "fat", codegen-units = 1`
- **Java**: JIT warmup enabled before timing

## Benchmark Categories

| Category | Tests |
|----------|-------|
| **Matrix** | Multiply, Transpose, Add (2000x2000 float matrices) |
| **Sort** | Qsort, Quicksort, Heapsort, Mergesort (1M integers) |
| **Hash** | SDBM, DJB2, FNV (1M iterations) |
| **Regex** | Find, Count, Email pattern match |
| **JSON** | Parse, Serialize, Count fields (1MB payload) |
| **File I/O** | Write, Read, Read lines, Random access (5MB files) |
| **Math** | Trig (sin/cos), Exp/Log, Arithmetic (x*x, x*x*x) |
| **Network** | Pipe throughput, Memcpy 8K |
| **ML** | Element-wise mul, Dot product, Lerp, Sigmoid |

## Adding New Benchmarks

1. Add source file to each language directory
2. Add just recipe to `justfile`
3. Rebuild tools if needed: `just tools-build`

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
