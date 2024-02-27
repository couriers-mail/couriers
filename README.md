<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./docs/logo.png" height="120" width="120" />
  </div>
  <h1 align="center">Couriers</h1>
  <h4 align="center">
    📨 SMTP Server with Web UI for Email solutions development
  </h4>
</div>

## Development

Couriers is a project fully written in Rust from CLI, SMTP/REST Server to Client
and Web UI Logic.

### Workspace

The software stack is the following:

- `cli`: Uses Clap as CLI Framework
- `client`: Uses Reqwest as WASM and Native HTTP Client
- `server`: Uses Axum as HTTP Server
- `web`: Uses Leptos as Web UI Framework

### Use of the Rust `Nightly` Channel

This project uses Rust's Nightly Channel as de-facto Rust Toolchain, this is
because [Leptos takes advantage of the nightly channel][1] to provide a better
development experience.

## License

Licensed under the MIT License

[1]: https://book.leptos.dev/getting_started/index.html#hello-world-getting-set-up-for-leptos-csr-development
