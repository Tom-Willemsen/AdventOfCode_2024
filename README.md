### Advent of code

Build:
```
cargo build --release
```

Unit tests (needs personal inputs):
```
cargo test
```

Run individual day:
```
./target/release/2024_01 --input inputs/real/2024_01
```

Run all days with hyperfine benchmarks (needs personal inputs):
```
./run_all_2024.sh
```

Run rust benchmarks (needs rust nightly & personal inputs):
```
cargo +nightly bench --features bench
```
