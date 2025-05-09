# zero2prod

<a href="https://github.com/JohnnyGOX17/zero2prod/actions">
  <img src="https://img.shields.io/github/actions/workflow/status/JohnnyGOX17/zero2prod/general.yml?branch=master&label=CI%20Tests&logo=github&style=flat-square" height="20" alt="GitHub Workflow Status">
</a>

Repo for working with [Zero To Production Rust book](https://www.zero2prod.com/index.html?country_code=US)

> currently on page: 81

## Install

Run `$ ./install.sh`

## Dev

* Can use `cargo-watch` to (`cargo install cargo-watch`) to automatically run in the background and run a series of commands when project files change, for instance running `$ cargo watch -x check -x test -x run` will run `cargo check`, then if succeeds `cargo test`, then if that succeeds `cargo run`, all when files change.

## CI

Run `$ ./run_checks.sh`, as well as GitHub actions.

## Documentation

```sh
$ cargo doc --open
```
