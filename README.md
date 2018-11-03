# google-somethin
A simple library that grabs Google search results... 

[![google-signin on crates.io](https://img.shields.io/crates/v/google-somethin.svg)](https://crates.io/crates/google-signin)
[![google-signin on docs.rs](https://docs.rs/google-somethin/badge.svg)](https://docs.rs/google-signin)

## Usage

Insert this in `Cargo.toml`

```toml
[dependencies]
google-somethin = "0.1"
```

And this in your main file

```rust
extern crate google_somethin;
```

### Basic usage

First import the crate and method as shown below.
```rust
extern crate google_somethin;
use google_somethin::google;
```

Now query!
```rust
let results = google("roblox", None);
// Should return results of links and titles.
println!("Roblox results! {:?}", results);
```

## Reminders
Just to verify again `ValueConstructor` to those who was wondering is only a `struct` with keys of `title` and `link`.
```rust
ValueConstructor {
    title: String,
    link: String,
} 
``` 
That's basically it!