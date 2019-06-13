# doryen-fov

[![Build Status](https://travis-ci.org/jice-nospam/doryen-fov.svg)](https://travis-ci.org/jice-nospam/doryen-fov)
[![Documentation](https://docs.rs/doryen-fov/badge.svg)](https://docs.rs/doryen-fov)
[![crates.io](https://meritbadge.herokuapp.com/doryen-fov)](https://crates.io/crates/doryen-fov)

A pure rust library containing 2D field of view algorithms for roguelikes.

# compilation instructions
* install rust : https://www.rust-lang.org/learn/get-started

## native compilation
```
cargo run --example fov
```

## web assembly compilation
```
rustup target install wasm32-unknown-unknown
cargo install cargo-web
cargo web start --example fov
```

# usage
Cargo.toml :
```toml
[dependency]
doryen-fov="*"
```

main.rs :
```rust
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};

fn main() {
    let mut fov = FovRecursiveShadowCasting::new();
    let map_width = 10;
    let map_height = 10;
    let mut map = MapData::new(map_width, map_height); // build an empty map
    map.set_transparent(5, 5, false); // put some wall
    let radius = 0;
    let player_x = 5;
    let player_y = 6;
    map.clear_fov(); // compute_fov does not clear the existing fov
    fov.compute_fov(&mut map, player_x, player_y, radius, false);
    assert!(map.is_in_fov(5, 7));
}
```

# contributions

You can contribute to this library through pull requests. If you do so, please update the CHANGELOG.md and CREDITS.md files. If you provide a new feature, consider adding an example as a tutorial/showcase.
