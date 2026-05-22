# clippy-bug-of-needless-range-loop

- rustc 1.94.0 (4a4ef493e 2026-03-02)
- clippy 0.1.94 (4a4ef493e3 2026-03-02)

以下のRustソースを食わせると、 `#[warn(clippy::needless_range_loop)]` を踏む。


```rust
for j in 0..frame_count {
    for i in 0..channel_count {
        print!("{}", frames[i][j]);
    }
}
```

外側のイテレーターを内側のイテレーターよりも、内側でイテレートする必要があり、
この指摘はおかしい気がしている（確証はない）:


```
warning: the loop variable `j` is only used to index `frames`
 --> src/main.rs:6:14
  |
6 |     for j in 0..frame_count {
  |              ^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
6 -     for j in 0..frame_count {
6 +     for <item> in frames.iter().take(frame_count) {
  |

warning: the loop variable `i` is only used to index `frames`
 --> src/main.rs:7:18
  |
7 |         for i in 0..channel_count {
  |                  ^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_range_loop
help: consider using an iterator
  |
7 -         for i in 0..channel_count {
7 +         for <item> in frames.iter().take(channel_count) {
  |

warning: `clippy-bug-of-needless-range-loop` (bin "clippy-bug-of-needless-range-loop") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

そして、よく考えると、`j` の方のイテレーターは、`frames.iter().take(frame_count)` では得られない。

どうでしょうか。

