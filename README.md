# Rustentia

This is a flashcards application written in Rust with [egui](https://github.com/emilk/egui)
using [rusqlite](https://github.com/rusqlite/rusqlite) for making and managing the database.

![rustentia](https://user-images.githubusercontent.com/106421928/236637043-91a771d0-8adf-4365-abbf-597eac2cf39e.png)

## Installation


You can install from [crates.io](https://crates.io/):

```bash
cargo install rustentia
```

or by cloning this repository:

```bash
git clone https://github.com/laurnidev/rustentia
```
If you installed with cargo the executable will be in .cargo/bin.
.cargo/bin must either be in your PATH or you can run the program with

```bash
cd ~/.cargo/bin
./rustentia
```

If you cloned the repository you run it with

```bash
cd ./rustentia
cargo run
```

## About

This is a practise project I made while learning programming and Rust,
it hasn't been well tested and is missing quite a few features compared to
other such applications.

