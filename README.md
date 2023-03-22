# Skybox Merger
[![fmt/clippy/test](https://github.com/d3-dmitriy-onypko/skybox_composer/actions/workflows/ci.yml/badge.svg)](https://github.com/d3-dmitriy-onypko/skybox_composer/actions/workflows/ci.yml)

Simple console app to merge skybox tiles into a single skybox image.
Supports PNG files only for now.

## Usage
1. Start the app from directory with files following the pattern
```
*_back.png
*_down.png
*_front.png
*_left.png
*_right.png
*_up.png
```

Check the [samples directory](sample) for acceptable input.

2. You can use `-d` flag to delete input files

## Buiding
1. Install [Rust]

1. Build

```shell
cargo buid --release
```


[Rust]:<https://www.rust-lang.org/tools/install>

## Compiled binaries
Check Releases
