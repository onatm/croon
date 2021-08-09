# croon

croon is a CLI written in `rust` and provides provides parsing and formatting of standard cron format with five time fields plus a command:

```text
* * * * * command
- - - - -
| | | | |
| | | | +----- day of week    (0 - 6)  (Sunday=0)
| | | +------- month          (1 - 12)
| | +--------- day of month   (1 - 31)
| +----------- hour           (0 - 23)
+------------- minute         (0 - 59)
```

## Usage

### Windows, Linux and macOS

#### Prerequisites

- [Install rustup](https://rustup.rs/)

##### Running

```sh
cargo run "*/10 0 1,15 * 1-5 /usr/bin/find"
```

#### Testing

```sh
cargo test
```

## Design

croon consists of two main parts:

- CLI entry point
- Library to parse cron expressions

It has two external dependencies for development convenience: `nom`, `linked_hash_set` and `lazy_static`.

### How it works

croon expects only a single input argument and it parses the argument as in the format of `minute hour day_of_month month day_of_week command`. Everything except **command** MUST be a valid cron expression otherwise parsing will fail. **command** is a "free-text" area. Users can write valid/invalid commands and it is not the responsibility of croon to check whether the command could run.

`Parser` type in `parser.rs` is just a thin wrapper around `cron_table` parser combinator. The type is not really necessary to have but it provides a logical distinction between a parser and cron table.

`CronTab` in `cron_table.rs` is instantiated by `Parser` when the parsing is successful. `from_cron_expression_list` function is a constructor-like function that converts all parsed `CronExpression`s to lists of `u32` values.

When the input argument is parsed successfully, CLI produces an output according the requirement of a table with the field taking the first 14 columns and the times as a space-separated list.

```text
minute        0 10 20 30 40 50
hour          0
day of month  1 15
month         1 2 3 4 5 6 7 8 9 10 11 12
day of week   1 2 3 4 5
command       "/usr/bin/find"
```

If there is any failure during the parsing, croon will write to `stderr` and exits with code 1.

### CLI

`main.rs` is the entry point of cron. It is just a thin wrapper around the library and does a few basic checks for error handling and formats crontab output as requested.

### Library

Library is where the main functionality of croon is implemented. It uses `nom` to combine multiple parsers and parses cron expressions according to the standard cron format. There are only tests in the parser which could be considered as functional tests.

`Error` type is created to enable `FromStr` trait on `CronTab` type. It does not do a good job about giving details of the error ocurred during parsing or validation.

The purpose of using `linked_hash_set` dependency is solely to have `HashSet` that preserves the order of insertion.

### Choosing Rust

As a software engineer, I didn't have any chance to use parsers or parser combinators during my career. However, since I started playing with rust in my free time, I implemented parsers 80% of the projects that I finished. I find rust really comfortable to build parsers because of the ergonomics it provides with enums (union type functionality but in the form of algebraic data type), powerful pattern matching and absence of `null`.
