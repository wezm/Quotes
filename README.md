Quotes
======

[![Build Status](https://api.cirrus-ci.com/github/wezm/Quotes.svg)](https://cirrus-ci.com/github/wezm/Quotes)

A small web app for quoting friends.

History
-------

When I was in university in the early 2000s I wrote a PHP app for my group of
friends to be able to log quotes from each other. These aren't so much profound
quotes but humorous ones.  The PHP app managed the user profiles and quotes in
a series of text files as a database was not available on the uni server that
hosted it.

In 2009 I imported the data into an SQLite database and built a new UI and
design for it in Ruby using [Sinatra], although this was a read-only version
of the original. That version is still present on [the ruby
branch](https://github.com/wezm/Quotes/tree/ruby)

Eleven years later I rebuilt the app again in [Rust] with [Rocket]. This time I
used the original markup almost verbatim for that genuine early 2000s feel.

Install/Run
-----------

1. Install Rust
2. `cp Rocket.toml.sample Rocket.toml`

### Run

    cargo run

### Creating the Initial DB

This is mainly for my own documentation since it imports from the old PHP
app:

    cargo run --bin import /path/to/quotes/files quotes.sqlite

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/Quotes/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/Quotes/blob/master/LICENSE-MIT))

at your option.

[Rust]: https://www.rust-lang.org/
[Rocket]: https://rocket.rs/
[Sinatra]: http://sinatrarb.com/
