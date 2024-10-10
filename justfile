api:
    cargo run -- json 2024h2 --json-path src/api/2024h2.json
    
serve: api
    mdbook serve

build: api
    mdbook build

check:
    cargo build
    cargo run -- check
    