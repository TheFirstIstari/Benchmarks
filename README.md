# Cross-Language Benchmark Suite

A comprehensive benchmark suite comparing performance across multiple programming languages: C, C++, Rust, Python, and Java with an interactive TUI for visualizing results.

## Features

- **Multi-language support**: C, C++, Rust, Python, Java
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
# All languages
just all

# By category
just matrix-all
just sort-all
just hash-all
just json-all

# By language
just c-matrix     # C (clang)
just gcc-matrix  # C (gcc)
just cpp-matrix  # C++ (clang)
just gxx-matrix  # C++ (g++)
just rust-matrix
just python-matrix
just python-json-multi  # Compare JSON libraries
just java-matrix
```

## Monitor TUI

Run `just monitor` to launch the interactive TUI:

| Key | Action |
|-----|--------|
| `[1]` | Graph view (line chart) |
| `[2]` | Table view (list) |
| `[3]` | Statistics view |
| `[j/k]` or arrows | Navigate rows |
| `[Enter]` | Detail view |
| `[s]` | Select benchmarks to run |
| `[r]` | Run benchmarks |
| `[R]` | Refresh data |
| `[q]` | Quit |

### Detail & Stats Views

- **Detail View**: Shows all languages compared for a specific benchmark
- **Statistics View**: Shows avg, min, max, std_dev, Q1, Q3, runs

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

- **C/C++**: `-Ofast -march=native` (maximum compiler optimizations)
- **Rust**: `opt-level = 3, lto = "fat", codegen-units = 1`
- **Java**: JIT warmup enabled before timing

## Adding New Benchmarks

1. Add source file to each language directory
2. Add just recipe to `justfile`
3. Rebuild tools if needed: `just tools-build`

## Benchmark Categories

| Category | Tests |
|----------|-------|
| **Matrix** | Multiply, Transpose, Add (2000x2000) |
| **Sort** | Qsort, Quicksort, Heapsort, Mergesort |
| **Hash** | SDBM, DJB2, FNV (1M iterations) |
| **Regex** | Find, Count, Email match |
| **JSON** | Parse, Serialize, Count fields |
| **File I/O** | Write, Read, Read lines, Random access (5MB) |
| **Math** | Trig, Exp/Log, Arithmetic |
| **Network** | Pipe throughput, Memcpy |
| **ML** | Element-wise mul, Dot product, Lerp, Sigmoid |

## License

MIT
