# 🔢 Enumerare

A collection of utilities for working with enumerators.

## 🛠️ Installation

### Cargo

```sh
$ cargo add enumerare
```

## 📝 Usage

For more examples, see the [tests](./tests)

```rs
use enumerare::{Cycle, CycleError};

#[derive(Cycle, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() -> Result<(), CycleError> {
    assert_eq!(Direction::North.try_cycle_to(0)?, Direction::North);
    assert_eq!(Direction::North.try_cycle_to(1)?, Direction::East);

    assert_eq!(Direction::North.cycle_by(1), Direction::East);
    assert_eq!(Direction::West.cycle_by(1), Direction::North);
    assert_eq!(Direction::North.cycle_by(-1), Direction::West);

    assert_eq!(Direction::East.cycle_by(5), Direction::South);

    assert_eq!(Direction::North.next(), Direction::East);
    assert_eq!(Direction::North.prev(), Direction::West);

    Ok(())
}
```

## 🧩 Development

```sh
$ nix develop # If Nix
```
