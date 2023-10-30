# BrainFuck compiler using Rust proc macro
More precisely, the BrainFuck-to-Rust transpiler using Rust proc macro

```rust
fn main() {
    bflib::brain_fuck!(
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
        >---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    );
}
```