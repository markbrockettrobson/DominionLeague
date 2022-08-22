set -e

echo "runing clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "runing coverage"
cargo llvm-cov --html && cargo llvm-cov --no-run --ignore-filename-regex main

echo "runing mutants"
cargo mutants -- --all-targets --all-features
