# 📝 Dialogos

A super simple dialogue system for Rust.

It's nothing special, but that's the point! It's something that just works.
This library is ideal for games that are made for a game jam.
For more complex games I would recommend extending Dialogos or using something else.

## 🌽 Features

- Easy to use
- Labels
- Menus
- Variables
- Mathematical operations
- Conditional statements

## 🍄 Installation

Add Dialogos as a dependency to Cargo.toml

```toml
[dependencies]
dialogos = { git = "https://github.com/AlexandrosKap/dialogos" }
```

And then open the documentation with

```sh
cargo doc --open
```

## 🍅 Example

A Hello World example

```rust
use dialogos::*;

fn main() {
    let alex = |cont| text("Alex", cont);

    let mut d = Dialogue::new(vec![
        alex("Hello world."),
        alex("Something something."),
        alex("The end."),
    ]);

    while !d.has_end() {
        let line = d.line();
        println!("{}: {}", line.info, line.cont);
        d.next();
    }
}
```
