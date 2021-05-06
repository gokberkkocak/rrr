# RRR (result-reader-rs) [![Build/Test](https://github.com/gokberkkocak/rrr/actions/workflows/ci.yml/badge.svg)](https://github.com/gokberkkocak/rrr/actions/workflows/ci.yml)

Experiment database maintenance tool.

## Build

```rust
cargo build --release
```

## Usage

```
rrr 0.5.5
Result Reader Rust

USAGE:
    rrr [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    local     Use RRR in local JSON file(s) mode
    remote    Use RRR in remote MySQL DB mode
```

### Local Usage
```
rrr-local
Use RRR in local JSON file(s) mode

USAGE:
    rrr local [FLAGS] --input <input> [SUBCOMMAND]

FLAGS:
    -d, --decompress    Set if JSON file is compressed with zstd
    -f, --folder        Set if you want to give folder dump rather than single JSON file
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -i, --input <input>    Sets the json file to use

SUBCOMMANDS:
    best-time      Brings the best time of an instance
    convert        Converts json to the plotter suited version.
    csv-dump       Converts json as csv for R.
    folder-dump    Converts json into multiple jsons in a folder.
    help           Prints this message or the help of the given subcommand(s)
    sol            Brings the number of solution of an instance
    time           Brings the exact min time of an instance
    write          Writes to json, merges the side input into main and deletes sides.
```

### Remote Usage
```
rrr-remote
Use RRR in remote MySQL DB mode

USAGE:
    rrr remote --db-config <DB_CONFIG> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --db-config <DB_CONFIG>    DB conf file

SUBCOMMANDS:
    best-time     Brings the best sr time of an instance
    commit        Commits the new entry to db
    help          Prints this message or the help of the given subcommand(s)
    init          Init/clear the table and optionally populate from json
    nb-success    Checks the db to find how many distinct seed successful runs on db.
    sol           Brings the number of solutions of an instance
    time          Finds the exact min time of an instance from the db
```
