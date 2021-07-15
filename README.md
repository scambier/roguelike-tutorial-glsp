# 2021 Roguelike Tutorial Jam

[Reddit](https://www.reddit.com/r/roguelikedev/comments/o5x585/roguelikedev_does_the_complete_roguelike_tutorial/) - [Twitter](https://twitter.com/GridSageGames/status/1407493165100113922?s=20)

This (basic) roguelike is written in [GameLisp](https://gamelisp.rs/) on top of Rust & [bracket-lib](https://github.com/amethyst/bracket-lib). It roughly follows the [equivalent Rust tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html).

The goal of this project is not really to make a roguelike in itself, but to learn a lisp language, embed it within a Rust binary, and make a simple and reusable API.

## Run the game

You'll need to [install Rust](https://www.rust-lang.org/tools/install).

Then, simply type `cargo run` to execute it.

## Roadmap

### Tutorial

- [x] Part 0 - Setting up
- [x] [Part 1](https://github.com/scambier/roguelike-tutorial-glsp/tree/2c2947a1557b69e87e5a94225f5c4964c90af878) - Drawing the ‘@’ symbol and moving it around
- [x] Part 2 - The generic Entity, the render functions, and the map
- [x] [Part 3](https://github.com/scambier/roguelike-tutorial-glsp/tree/c43611f3893810bf3e816cb2faa1ab2f1a5b21f6) - [Demo](https://scambier.xyz/roguelike/week2/) - Generating a dungeon
- [x] Part 4 - Field of view
- [x] [Part 5](https://github.com/scambier/roguelike-tutorial-glsp/tree/adef553af912c3dbd033b7cd351cb0c88b1f3c53) - [Demo](https://scambier.xyz/roguelike/week3/) - Placing enemies and kicking them (harmlessly)
- [ ] Part 6 - Doing (and taking) some damage
- [ ] Part 7 - Creating the Interface
- [ ] Part 8 - Items and Inventory
- [ ] Part 9 - Ranged Scrolls and Targeting
- [ ] Part 10 - Saving and loading
- [ ] Part 11 - Delving into the Dungeon
- [ ] Part 12 - Increasing Difficulty
- [ ] Part 13 - Gearing up

### Rust/GameLisp

- [x] Make a barebones ECS
	- [ ] Optimize
	- [ ] Implement cache
- [x] Compile a self-contained executable (actually has issues with glsp files)
- [x] Web build
- [ ] Live reload of glsp code

## Builds

## Standalone binary

```sh
$ cargo build --release --features "compiler"
```

### Web

```sh
$ cargo build --release --target wasm32-unknown-unknown --features "compiler"
$ wasm-bindgen target\wasm32-unknown-unknown\release\roguelike-glsp.wasm --out-dir wasm --no-modules --no-typescript
```
