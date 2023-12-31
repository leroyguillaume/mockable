# Contributing

## Pre-requisites

- [Rust](https://rustup.rs/)
- [pre-commit](https://pre-commit.com/)

## Setup

```bash
git clone git@github.com:leroyguillaume/mockable.git
cd mockable
pre-commit install
```

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Commit

Your commit must respect the [Angular Commit Convention](https://github.com/angular/angular/blob/68a6a07/CONTRIBUTING.md#commit).

You can check your commit message with the following command:
```bash
commitlint --from=HEAD~1
```

To install `commitlint`:
```bash
npm i -g @commitlint/{cli,config-conventional}
```
