# Rusty Dev CLI Utility

This project is a simple command-line interface (CLI) tool implemented in Rust.

## Installation from GitHub

### Install

Use the `cargo install` command with the `--git` option to specify a Git URL.

```shell
cargo install --git https://github.com/codemountains/rusty-dev-cli-utility
```

### Usage

```
% rusty-dev-cli-utility --greeting Hi Alice
```

### Uninstall

```shell
cargo uninstall rusty-dev-cli-utility
```

## Build for Windows

Navigate to that directory in our terminal. In the terminal, run the following command:

```shell
docker build . -t rust_cross_compile/windows
```

Once you’ve created the image, then you can run the container by executing the following command:

```shell
docker run --rm -v $(pwd):/app rust_cross_compile/windows
```

## LICENSE

This project is licensed under the [MIT license](LICENSE).
