https://www.youtube.com/watch?v=XZtlD_m59sM - axum tutorial
https://crates.io/crates/utoipa-swagger-ui - utopia-swagger-ui

Install cargo-watch: cargo install cargo-watch (https://crates.io/crates/cargo-watch)

cargo watch -q -c -w src/ -x run
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"