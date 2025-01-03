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
|  6 | 10:58:37 | 46183 |   4.2ms |     >24h | 51335 |   4.6 s |
|  7 |     >24h | 62712 |   3.4ms |     >24h | 59633 | 394.7ms |
|  8 |     >24h | 52718 |  37.7µs |     >24h | 50478 | 124.8µs |
|  9 |     >24h | 48793 |   1.2ms |     >24h | 38667 | 142.9ms |
| 10 | 06:59:50 | 23795 | 302.3µs | 07:11:03 | 22921 | 250.3µs |
| 11 | 02:41:41 | 16319 | 321.5µs | 03:23:07 | 11219 |  11.5ms |
| 12 | 13:05:32 | 29477 |   8.1ms | 15:06:06 | 20011 |  11.3ms |
| 13 | 03:48:25 | 13296 | 843.5µs | 03:52:58 |  9043 | 617.7µs |
| 14 | 03:57:00 | 12130 |  59.2µs | 04:33:22 |  9732 | 260.5ms |
| 15 | 13:01:39 | 24361 | 433.5µs |     >24h | 23428 |   2.6ms |
| 16 | 14:20:14 | 18761 |   2.7ms | 15:44:03 | 14556 |   7.8ms |
| 17 | 04:09:36 | 10705 |   4.0µs | 06:50:24 |  5516 |  18.6ms |
| 18 | 04:07:11 | 10852 | 678.6µs | 07:48:43 | 14194 | 659.5ms |
| 19 | 15:00:27 | 21055 |  18.3ms | 15:05:20 | 18308 | 412.1ms |
| 20 | 04:51:55 |  9403 |  74.6ms | 04:56:19 |  5983 |  98.6ms |
| 21 |     >24h | 16863 | 333.9µs |     >24h | 13074 |   3.6ms |
| 22 |     >24h | 21695 |   8.7ms |     >24h | 18477 | 863.3ms |
| 23 |     >24h | 19744 | 665.9ms |     >24h | 17409 | 128.0ms |
| 24 | 10:41:25 | 15258 | 207.6µs | 11:57:22 |  6403 |  93.6µs |
| 25 | 08:55:12 | 13176 | 435.6µs | 08:56:13 |  8247 |       - |
<!--|  1 | 00:13:19 |  5740 |  19.5µs | 00:21:33 |  5187 |  20.7µs | -->

## 🎄 Have a wonderful Holiday Season everyone! 🎄

![koch flakes](koch_snowflakes.gif)


[^1]: `cargo run -p day** --release`, does not include the reading of the input file but includes parsing.
[^2]: Some values are missing since I solved that days without using the usual device I use
