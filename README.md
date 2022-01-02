# sudoku-rs

![tests](https://github.com/MitchelPaulin/sudoku-rs/actions/workflows/rust.yml/badge.svg)

Fully featured Sudoku right in your terminal, built over a weekend using [tui-rs](https://github.com/fdehau/tui-rs).

## Demo

<p align="center">
    <img src="./demo/demo.gif" height="600px">
</p>

## Building & Themes

There are two themes available, Tranquil and Dracula

| `cargo build --features tranquil` | `cargo build --features dracula` |
|:-------------------------------:|:------------------------------:|
| ![tranquil](demo/tranquil.png)  | ![tranquil](demo/dracula.png)  |

Or you can add your own in [themes.rs](./src/themes.rs)

## Puzzle Generation

Puzzle "seeds" are generated from [here](https://qqwing.com/generate.html).