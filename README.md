Cargo.toml
``` toml
alternate_screen_wrapper = { version = "<current-version>", features = ["crossterm"] } # add "crossterm-bracketed-paste" to enable it
```

main.rs
``` rust
fn main() {
    let alternate_screen = alternate_screen_wrapper::AlternateScreen::enter();
    // ..do some work
    let printable = // do other work
    //
    drop(alternate_screen);
    println!("{printable}"); // it works
}
```

You can don't call drop manually.
It will be dropped automatically when you exit scope or panic.
