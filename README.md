# Create GPUI App

Create a new [GPUI](https://www.gpui.rs/) app in a single command.

GPUI is a fast, productive UI framework for Rust from the creators of [Zed](https://zed.dev/).

## Quick Start

```sh
cargo install create-gpui-app
create-gpui-app --name my-app
cd my-app
```

## Creating an App

**You'll need to have Rust and Cargo installed on your machine**. You can install Rust through [rustup](https://rustup.rs/).

To create a new app, run:

```sh
create-gpui-app --name my-app
cd my-app
```

By default this will output:

```
my-app
├── src
│   ├── main.rs
├── Cargo.toml
├── README.md
```

To set up your application as a workspace, run:

```sh
create-gpui-app --workspace --name my-app
cd my-app
```

This will output a directory structure like this:

```
my-app
├── Cargo.toml
├── crates
│   └── my-app
│       ├── Cargo.toml
│       └── src
│           └── main.rs
└── README.md
```

`create-gpui-app` with no arguments will create a new app called `gpui-app`.

### Running the App

- During development: `cargo run`
- For production/performance testing: `cargo build --release`

### Troubleshooting
See the [zed development troubleshooting guide](https://github.com/zed-industries/zed/blob/main/docs/src/development/macos.md#troubleshooting) for assistance with common errors.

## Contributing

Your contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

`create-gpui-app` is open source software [licensed as MIT](LICENSE).
