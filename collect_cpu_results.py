#!/usr/bin/env python3
"""Collect CPU benchmark results and save as JSON."""
import subprocess, json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))

BENCHMARKS = {
    "C":       ["./benchmarks/c_cpu"],
    "C++":     ["./benchmarks/cpp_cpu"],
    "Rust":    ["Rust/target/release/cpu"],
    "Java":    ["mise", "exec", "java", "--", "java", "-cp", "benchmarks/java", "CpuBench"],
    "Python":  ["python3", "Python/cpu.py"],
}

results = {}
for lang, cmd in BENCHMARKS.items():
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=HERE)
    output = r.stdout + r.stderr
    results[lang] = {"raw": output}
    parsed = {}
    for line in output.splitlines():
        m = re.match(r'^cpu_([a-z_]+):\s*([\d.]+)\s*ms', line)
        if m:
            name, ms = m.groups()
            parsed[name] = float(ms)
    results[lang]["parsed"] = parsed
    print(f"{lang}: {len(parsed)} tests parsed")

with open(os.path.join(HERE, "cpu_results.json"), "w") as f:
    json.dump(results, f, indent=2)

print()

tests = ["fibonacci", "prime_sieve", "popcount", "branch_sorted", "branch_unsorted",
         "memory_latency", "simd_neon", "simd_numpy", "simd_auto_vec",
         "atomic_add", "nonatomic_add"]

header = "{:<30}".format("Test")
for lang in BENCHMARKS:
    header += "{:>12}".format(lang)
print(header)
print("-" * (30 + 12 * len(BENCHMARKS)))
for test in tests:
    row = test.replace("_", " ").title()
    vals = "{:<30}".format(row)
    for lang in BENCHMARKS:
        p = results[lang]["parsed"]
        found = False
        for k, v in p.items():
            if test in k:
                vals += "{:>10.2f}ms".format(v)
                found = True
                break
        if not found:
            vals += "{:>12}".format("N/A")
    print(vals)
