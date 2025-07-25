api:
    cargo rpg json 2024h2 --json-path src/api/2024h2.json

mermaid_assets:
    mdbook-mermaid install .

serve: api mermaid_assets
    mdbook serve --port 3001

build: api mermaid_assets
    mdbook build

check:
    cargo rpg check
