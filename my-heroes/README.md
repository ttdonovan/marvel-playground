# My Heroes

An experimental full-stack web application where both back-end and front-end
are written in Rust. Something a little more exciting than just another todo
app.

## Prerequisites 

* [Rust](https://www.rust-lang.org/)
* [SQLite3](https://www.sqlite.org/index.html)

```
$ rustc --version
> rustc 1.37.0 (eae3437df 2019-08-13)
```

## Setup

```
$ cargo install diesel_cli
$ echo 'DATABASE_URL=./heroesdb.sqlite3' > .env
```

## First Migration

```
$ diesel migration generate heroes
$ diesel migration run
$ diesel migration redo
$ echo .dump | sqlite3 heroesdb.sqlite3
```

## Usage

```
$ cd my-heroes
$ cargo build --release
$ target/release/my-heroes-cli new --help
$ target/release/my-heroes-cli new 'Spider-Man'
$ target/release/my-heroes-cli new 'Captain America'
$ echo 'select * from heroes;' | sqlite3 heroesdb.sqlite3
> 1|Spider-Man
> 2|Captain America
$ target/release/my-heroes-cli list
```