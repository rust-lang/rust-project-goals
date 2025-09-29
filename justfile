mermaid_assets:
    mdbook-mermaid install .

serve: mermaid_assets
    mdbook serve --port 3001

build: mermaid_assets
    mdbook build

check:
    cargo rpg check
