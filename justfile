set dotenv-load := true
set fallback := true

@_default:
    just --list

export ROOT := `pwd`

install:
    mise install

all: c-all cpp-all python-all java-all rust-all gcc-all gxx-all

matrix-all: c-matrix gcc-matrix cpp-matrix gxx-matrix python-matrix java-matrix rust-matrix
sort-all: c-sort gcc-sort cpp-sort gxx-sort python-sort java-sort rust-sort
string-all: c-string gcc-string cpp-string gxx-string python-string java-string rust-string
hash-all: c-hash cpp-hash python-hash java-hash rust-hash
regex-all: c-regex cpp-regex python-regex java-regex rust-regex
json-all: c-json cpp-json python-json java-json rust-json
fileio-all: c-fileio cpp-fileio python-fileio java-fileio rust-fileio
math-all: c-math cpp-math python-math java-math rust-math
network-all: c-network cpp-network python-network
crypto-all: c-crypto cpp-crypto python-crypto java-crypto rust-crypto
ml-all: c-ml python-ml rust-ml

# ============================================================================
# C Benchmarks
# ============================================================================
c-all: c-matrix c-allocator c-string c-sort c-concurrency c-hash c-regex c-network c-crypto c-ml

c-matrix:
    cc "{{ROOT}}/C/matrix.c" -O3 -ffast-math -march=native -o benchmarks/c_matrix -lm
    ./benchmarks/c_matrix

c-allocator:
    cc "{{ROOT}}/C/allocator.c" -O3 -ffast-math -march=native -o benchmarks/c_allocator -lm
    ./benchmarks/c_allocator

c-string:
    cc "{{ROOT}}/C/string.c" -O3 -ffast-math -march=native -o benchmarks/c_string
    ./benchmarks/c_string

c-sort:
    cc "{{ROOT}}/C/sort.c" -O3 -ffast-math -march=native -o benchmarks/c_sort -lm
    ./benchmarks/c_sort

c-concurrency:
    cc "{{ROOT}}/C/concurrency.c" -O3 -ffast-math -march=native -o benchmarks/c_concurrency -lpthread
    ./benchmarks/c_concurrency

c-hash:
    cc "{{ROOT}}/C/hash.c" -O3 -march=native -ffast-math -o benchmarks/c_hash
    ./benchmarks/c_hash

c-regex:
    cc "{{ROOT}}/C/regex.c" -O3 -ffast-math -march=native -o benchmarks/c_regex
    ./benchmarks/c_regex

c-json:
    cc "{{ROOT}}/C/json.c" -O3 -ffast-math -march=native -o benchmarks/c_json
    ./benchmarks/c_json

c-fileio:
    cc "{{ROOT}}/C/fileio.c" -O3 -ffast-math -march=native -o benchmarks/c_fileio
    ./benchmarks/c_fileio

c-math:
    cc "{{ROOT}}/C/math.c" -O3 -ffast-math -march=native -o benchmarks/c_math -lm
    ./benchmarks/c_math

c-network:
    cc "{{ROOT}}/C/network.c" -O3 -ffast-math -march=native -o benchmarks/c_network
    ./benchmarks/c_network

c-crypto:
    cc "{{ROOT}}/C/crypto.c" -O3 -ffast-math -march=native -o benchmarks/c_crypto
    ./benchmarks/c_crypto

c-ml:
    cc "{{ROOT}}/C/ml.c" -O3 -ffast-math -march=native -o benchmarks/c_ml -lm
    ./benchmarks/c_ml

# ============================================================================
# GCC C Benchmarks (homebrew gcc-15)
# ============================================================================
gcc-all: gcc-matrix gcc-allocator gcc-string gcc-sort gcc-concurrency gcc-hash gcc-regex

gcc-matrix:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/matrix.c" -O3 -ffast-math -march=native -o benchmarks/gcc_matrix -lm
    ./benchmarks/gcc_matrix

gcc-allocator:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/allocator.c" -O3 -ffast-math -march=native -o benchmarks/gcc_allocator -lm
    ./benchmarks/gcc_allocator

gcc-string:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/string.c" -O3 -ffast-math -march=native -o benchmarks/gcc_string
    ./benchmarks/gcc_string

gcc-sort:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/sort.c" -O3 -ffast-math -march=native -o benchmarks/gcc_sort -lm
    ./benchmarks/gcc_sort

gcc-concurrency:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/concurrency.c" -O3 -ffast-math -march=native -o benchmarks/gcc_concurrency -lpthread
    ./benchmarks/gcc_concurrency

gcc-hash:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/hash.c" -O3 -ffast-math -march=native -o benchmarks/gcc_hash
    ./benchmarks/gcc_hash

gcc-regex:
    /opt/homebrew/bin/gcc-15 "{{ROOT}}/C/regex.c" -O3 -ffast-math -march=native -o benchmarks/gcc_regex
    ./benchmarks/gcc_regex

# ============================================================================
# C++ Benchmarks (Clang)
# ============================================================================
cpp-all: cpp-matrix cpp-allocator cpp-string cpp-sort cpp-concurrency cpp-hash cpp-regex cpp-network cpp-crypto

cpp-matrix:
    clang++ "{{ROOT}}/C++/matrix.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_matrix -lm
    ./benchmarks/cpp_matrix

cpp-allocator:
    clang++ "{{ROOT}}/C++/allocator.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_allocator -lm
    ./benchmarks/cpp_allocator

cpp-string:
    clang++ "{{ROOT}}/C++/string.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_string
    ./benchmarks/cpp_string

cpp-sort:
    clang++ "{{ROOT}}/C++/sort.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_sort -lm
    ./benchmarks/cpp_sort

cpp-concurrency:
    clang++ "{{ROOT}}/C++/concurrency.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_concurrency -lpthread
    ./benchmarks/cpp_concurrency

cpp-hash:
    clang++ "{{ROOT}}/C++/hash.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_hash
    ./benchmarks/cpp_hash

cpp-regex:
    clang++ "{{ROOT}}/C++/regex.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_regex
    ./benchmarks/cpp_regex

cpp-json:
    clang++ "{{ROOT}}/C++/json.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_json
    ./benchmarks/cpp_json

cpp-fileio:
    clang++ "{{ROOT}}/C++/fileio.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_fileio
    ./benchmarks/cpp_fileio

cpp-math:
    clang++ "{{ROOT}}/C++/math.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_math -lm
    ./benchmarks/cpp_math

cpp-network:
    clang++ "{{ROOT}}/C++/network.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_network
    ./benchmarks/cpp_network

cpp-crypto:
    clang++ "{{ROOT}}/C++/crypto.cpp" -O3 -ffast-math -march=native -o benchmarks/cpp_crypto
    ./benchmarks/cpp_crypto

# ============================================================================
# Python Benchmarks
# ============================================================================
python-all: python-matrix python-string python-sort python-async python-hash python-regex python-json python-json-multi python-fileio python-math python-network python-crypto python-ml

python-matrix:
    python "{{ROOT}}/Python/matrix.py"

python-string:
    python "{{ROOT}}/Python/string_ops.py"

python-sort:
    python "{{ROOT}}/Python/sort.py"

python-async:
    python "{{ROOT}}/Python/async_bench.py"

python-hash:
    python "{{ROOT}}/Python/hash.py"

python-regex:
    python "{{ROOT}}/Python/regex.py"

python-json:
    python "{{ROOT}}/Python/json_bench.py"

python-fileio:
    python "{{ROOT}}/Python/fileio.py"

python-math:
    python "{{ROOT}}/Python/math_bench.py"

python-network:
    python "{{ROOT}}/Python/network.py"

python-crypto:
    python "{{ROOT}}/Python/crypto.py"

python-ml:
    python "{{ROOT}}/Python/ml.py"

python-json-multi:
    python "{{ROOT}}/Python/json_multi.py"

# ============================================================================
# Java Benchmarks
# ============================================================================
JAVA_HOME := "/opt/homebrew/opt/openjdk"
JAVA := JAVA_HOME + "/bin/java"
JAVAC := JAVA_HOME + "/bin/javac"

java-all: java-matrix java-string java-sort java-concurrency java-hash java-regex java-json java-fileio java-math java-crypto

_java_compile:
    mkdir -p benchmarks/java
    "{{JAVAC}}" -d benchmarks/java \
        Java/MatrixBench.java \
        Java/StringBench.java \
        Java/SortBench.java \
        Java/ConcurrencyBench.java \
        Java/HashBench.java \
        Java/RegexBench.java \
        Java/JsonBench.java \
        Java/FileIOBench.java \
        Java/MathBench.java

java-matrix: _java_compile
    "{{JAVA}}" -cp benchmarks/java MatrixBench

java-string: _java_compile
    "{{JAVA}}" -cp benchmarks/java StringBench

java-sort: _java_compile
    "{{JAVA}}" -cp benchmarks/java SortBench

java-concurrency: _java_compile
    "{{JAVA}}" -cp benchmarks/java ConcurrencyBench

java-hash: _java_compile
    "{{JAVA}}" -cp benchmarks/java HashBench

java-regex: _java_compile
    "{{JAVA}}" -cp benchmarks/java RegexBench

java-json: _java_compile
    "{{JAVA}}" -cp benchmarks/java JsonBench

java-fileio: _java_compile
    "{{JAVA}}" -cp benchmarks/java FileIOBench

java-math: _java_compile
    "{{JAVA}}" -cp benchmarks/java MathBench

java-crypto: _java_compile
    "{{JAVA}}" -cp benchmarks/java CryptoBench

# ============================================================================
# Rust Benchmarks
# ============================================================================
RUST_DIR := ROOT + "/Rust"
RUST_TARGET := ROOT + "/Rust/target/release"

rust-build:
    mise exec -- cargo build --release --manifest-path "{{RUST_DIR}}/Cargo.toml"

rust-all: rust-matrix rust-allocator rust-string rust-sort rust-concurrency rust-hash rust-regex rust-json rust-fileio rust-math rust-network rust-crypto rust-ml

rust-matrix: rust-build
    "{{RUST_TARGET}}/matrix"

rust-allocator: rust-build
    "{{RUST_TARGET}}/allocator"

rust-string: rust-build
    "{{RUST_TARGET}}/string"

rust-sort: rust-build
    "{{RUST_TARGET}}/sort"

rust-concurrency: rust-build
    "{{RUST_TARGET}}/concurrency"

rust-hash: rust-build
    "{{RUST_TARGET}}/hash"

rust-regex: rust-build
    "{{RUST_TARGET}}/regex"

rust-json: rust-build
    "{{RUST_TARGET}}/json"

rust-fileio: rust-build
    "{{RUST_TARGET}}/fileio"

rust-math: rust-build
    "{{RUST_TARGET}}/math"

rust-network: rust-build
    "{{RUST_TARGET}}/network"

rust-crypto: rust-build
    "{{RUST_TARGET}}/crypto"

rust-ml: rust-build
    "{{RUST_TARGET}}/ml"

# ============================================================================
# GCC C++ Benchmarks (homebrew g++-15)
# ============================================================================
gxx-all: gxx-matrix gxx-allocator gxx-string gxx-sort gxx-concurrency gxx-hash gxx-regex

gxx-matrix:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/matrix.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_matrix -lm
    ./benchmarks/gxx_matrix

gxx-allocator:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/allocator.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_allocator -lm
    ./benchmarks/gxx_allocator

gxx-string:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/string.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_string
    ./benchmarks/gxx_string

gxx-sort:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/sort.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_sort -lm
    ./benchmarks/gxx_sort

gxx-concurrency:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/concurrency.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_concurrency -lpthread
    ./benchmarks/gxx_concurrency

gxx-hash:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/hash.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_hash
    ./benchmarks/gxx_hash

gxx-regex:
    /opt/homebrew/bin/g++-15 "{{ROOT}}/C++/regex.cpp" -O3 -ffast-math -march=native -o benchmarks/gxx_regex
    ./benchmarks/gxx_regex

# ============================================================================
# Carbon Benchmarks (experimental)
# ============================================================================
carbon-all: carbon-matrix carbon-string carbon-sort

carbon-matrix:
    @echo "Carbon benchmarks TBD - needs carbon compiler"
    # Carbon language: https://github.com/carbon-language/carbon-lang

carbon-string:
    @echo "Carbon benchmarks TBD"

carbon-sort:
    @echo "Carbon benchmarks TBD"

# ============================================================================
# C# Benchmarks
# ============================================================================
CS_DIR := ROOT + "/CS"

cs-build:
    dotnet build "{{CS_DIR}}/benchmarks.csproj" -c Release -o benchmarks/cs

cs-all: cs-string cs-sort cs-matrix

cs-string: cs-build
    dotnet run --project "{{CS_DIR}}/benchmarks.csproj" --configuration Release -- string

cs-sort: cs-build
    dotnet run --project "{{CS_DIR}}/benchmarks.csproj" --configuration Release -- sort

cs-matrix: cs-build
    dotnet run --project "{{CS_DIR}}/benchmarks.csproj" --configuration Release -- matrix

# ============================================================================
# Unified Runner (Rust-based)
# ============================================================================
run-all: tools-build rust-build
    "{{TOOLS_TARGET}}/runner"

run-all-matrix: tools-build rust-build
    "{{TOOLS_TARGET}}/runner" --categories matrix

run-all-sort: tools-build rust-build
    "{{TOOLS_TARGET}}/runner" --categories sort

run-all-string: tools-build rust-build
    "{{TOOLS_TARGET}}/runner" --categories string

# ============================================================================
# Tools (Monitor, Store, Runner)
# ============================================================================
TOOLS_DIR := ROOT + "/tools"
TOOLS_TARGET := ROOT + "/tools/target/release"

tools-build:
    mise exec -- cargo build --release --manifest-path "{{TOOLS_DIR}}/Cargo.toml"

monitor: tools-build
    "{{TOOLS_TARGET}}/monitor"

store:
    "{{TOOLS_TARGET}}/store" list

store-list:
    "{{TOOLS_TARGET}}/store" list

store-show:
    "{{TOOLS_TARGET}}/store" show

# ============================================================================
# Utility
# ============================================================================
clean:
    rm -rf benchmarks/* C/*~ C++/*~ Python/*~
    rm -rf benchmarks/java
    cargo clean --manifest-path "{{RUST_DIR}}/Cargo.toml"
    dotnet clean "{{CS_DIR}}/benchmarks.csproj"

clean-all: clean
    rm -rf ~/.local/share/benchmarks

benchmarks:
    mkdir -p benchmarks

default := "all"
