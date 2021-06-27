# 2021 Roguelike Tutorial Jam

[Reddit](https://www.reddit.com/r/roguelikedev/comments/o5x585/roguelikedev_does_the_complete_roguelike_tutorial/) - [Twitter](https://twitter.com/GridSageGames/status/1407493165100113922?s=20)

This (basic) roguelike is written in [GameLisp](https://gamelisp.rs/) on top of Rust & [bracket-lib](https://github.com/amethyst/bracket-lib).

The goal of this project is not really to make a roguelike in itself, but to learn a lisp language, embed it within a Rust binary, and make a simple and reusable API.

## Run the game

You'll need to [install Rust](https://www.rust-lang.org/tools/install).

Then, simply type `cargo run` to execute it.

## Roadmap

### Tutorial

- [x] Part 0 - Setting up
- [x] Part 1 - Drawing the ‘@’ symbol and moving it around
- [ ] Part 2 - The generic Entity, the render functions, and the map
- [ ] Part 3 - Generating a dungeon
- [ ] Part 4 - Field of view
- [ ] Part 5 - Placing enemies and kicking them (harmlessly)
- [ ] Part 6 - Doing (and taking) some damage
- [ ] Part 7 - Creating the Interface
- [ ] Part 8 - Items and Inventory
- [ ] Part 9 - Ranged Scrolls and Targeting
- [ ] Part 10 - Saving and loading
- [ ] Part 11 - Delving into the Dungeon
- [ ] Part 12 - Increasing Difficulty
- [ ] Part 13 - Gearing up

### Rust/GameLisp

- [ ] Try making a barebones ECS
- [ ] Learn how to use GameLisp features (mixins, classmacros, states)