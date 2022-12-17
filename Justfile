watch EXAMPLE:
    cargo watch -x 'run --example={{EXAMPLE}} --release'

run IDX:
    cargo run --example=day{{IDX}}

release IDX:
    cargo run --example=day{{IDX}} --release

profile IDX:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --example=day{{IDX}}

