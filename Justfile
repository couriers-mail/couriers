default:
  just --list

# Runs Web UI for Development
web-dev:
  cd ./crates/web && trunk serve --config ./Trunk.toml

# Builds Web UI for Production
web-build:
  cd ./crates/web && trunk build --release --config ./Trunk.toml
