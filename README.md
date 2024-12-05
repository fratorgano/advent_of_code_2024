# 🎄 Advent of Code 2024 🎄
Happy holidays everyone! 

I'm back again with the Advent of Code challenge, but I'm officially done being a student and I started working. I can't guarantee I'll find the time to do them all the day they come out, but I'll try my best! 

My language of choice is Rust, mainly because I truly enjoy programming with it, and it's fun working on a “lower level” language. I might switch to Python if doing it in Rust would take too much time, but I don't plan to.

🏆 Goal: Get all 50 stars before the 31st of December! 🏆

## ⛄ How to run solutions ⛄
`cargo run -p day**` - Runs a specific day

`cargo run -p day** --release` - Runs a specific day with compiler optimizations

`cargo test -p day**` - Tests a specific day

`cargo test` - Tests all

## ❄️ How to use the "template" ❄️
1. Clone this repository
1. Install [cargo-generate](https://github.com/cargo-generate/cargo-generate): `cargo install cargo-generate`
1. Create file $CARGO_HOME/cargo-generate.toml with the following content: 
```toml
[favorites.aoc]
description = "Advent of Code 2024 Template"
vcs = "None"
path = "../template"
```
4. Go back to the repository, in the crates folder and run `cargo generate aoc --name day**`


## 🥛 Results 🍪
| Day | Part 1 Time | Part 1 Rank | Part 1 Runtime[^1][^2] | Part 2 Time | Part 2 Rank | Part 2 Runtime[^1][^2] |
|:-:|-:|-:|-:|-:|-:|-:|
|  1 | 04:21:36 | 24096 |  87.3µs | 04:37:10 | 23290 |  96.8µs |
|  2 | 03:58:04 | 34218 | 176.7µs | 04:11:46 | 23378 | 251.5µs |
|  3 | 01:26:49 | 18450 | 469.3µs | 11:41:37 | 60997 | 454.1µs |
|  4 | 03:52:31 | 25968 | 993.2µs | 04:12:09 | 22459 | 153.0µs |
|  5 | 03:01:55 | 21006 | 280.3µs | 03:23:32 | 16865 |   1.2ms |
<!--|  1 | 00:13:19 |  5740 |  19.5µs | 00:21:33 |  5187 |  20.7µs | -->

## 🎄 Have a wonderful Holiday Season everyone! 🎄

![koch flakes](https://raw.githubusercontent.com/fratorgano/advent_of_code_2020/main/snow.gif)


[^1]: `cargo run -p day** --release`, does not include the reading of the input file but includes parsing.
[^2]: Some values are missing since I solved that days without using the usual device I use
