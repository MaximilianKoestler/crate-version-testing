An example for https://users.rust-lang.org/t/crate-interoperability-and-3rd-party-types-in-interfaces/53431/2

The `my-bin-crate` does not compile:
```text
   Compiling my-bin-crate v0.1.0 (crate-version-testing/my-bin-crate)
error[E0308]: mismatched types
 --> src/main.rs:3:21
  |
3 |     draw::set_color(color);
  |                     ^^^^^ expected struct `draw::RGBA`, found struct `skycolor::RGBA`
  |
  = note: expected struct `draw::RGBA<u8>`
             found struct `skycolor::RGBA<u8>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `my-bin-crate`.
```
