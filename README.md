# loadless :hourglass_flowing_sand:
Write less loading (progress bar) code in Rust.

![loadless_default](https://github.com/JustLeif/loadless/assets/69766831/c2080b3e-bd57-44c1-8770-b4d7dbb734f4)

### Cargo.toml
``` toml
# Package Definitions

[dependencies]
loadless = "*" # Recommended to specify version.
```

# Loadless w/ Iterators!
``` rust
// Import the loadless iterator trait extension to use the loadless api on iterators!
use loadless::LoadlessIteratorExt;

fn main() {
    // If it is an Iterator, .loadless() can be called!
    for _ in (0..10).loadless() {
        std::thread::sleep(Duration::from_millis(100));
    }
}
```

``` rust

// By default the loader goes to Stdout, but we can change it with a write target!
let mut target: Vec<u8> = Vec::new();
let mut test_vec: Vec<char> = vec!['a', 'b', 'c', 'd'];
for i in test_vec.iter_mut().loadless().write_target(&mut target) {
    *i = 'a';
}

// Now you can look at the loader String!
let _ = String::from_utf8_lossy(&target);
```

# Contributions
All contributions are welcome. This crate is in early development!
