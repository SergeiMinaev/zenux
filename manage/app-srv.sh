cd $ZENUX_HOME/back
export RUSTFLAGS=-Awarnings
cargo-watch -x run --ignore saras.pid
