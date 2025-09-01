# enum_tree

`enum_tree` provides traits and derive macros for modeling a tree of enums.
Each enum in the tree knows its parent and the root so values can be
converted to or recovered from the root at runtime.

The repository contains two crates:

- **enum_tree** – defines the [`EnumTree`] trait along with marker traits
  [`EnumTreeRoot`], [`EnumTreeInner`], [`EnumTreeLeaf`] and conversion traits
  [`ToEnumTreeRoot`] and [`TryFromEnumTreeRoot`].
- **enum_tree_derive** – implements `#[derive(EnumTree)]` and supporting
  attributes that generate the boilerplate implementations.

## Deriving a tree

An enum tree always has a single root. The `#[enum_tree_root]` attribute marks
that root enum. Inner and leaf enums specify their parent and root types using
`#[enum_tree_inner(P, R)]` and `#[enum_tree_leaf(P, R)]` respectively. Variant
names of parents must match the child enum names.

```rust
use enum_tree::EnumTree;

#[derive(EnumTree)]
#[enum_tree_root]
pub enum AppAction {
    Menu(Menu),
    Network(Network),
}

#[derive(EnumTree)]
#[enum_tree_inner(AppAction, AppAction)]
pub enum Menu {
    Settings(Settings),
    Quit(Quit),
}

#[derive(EnumTree)]
#[enum_tree_leaf(Menu, AppAction)]
pub enum Settings {
    ToggleSound,
}
```

## Working with the tree

Any node can be converted to the root via [`ToEnumTreeRoot::to_root`].
The reverse conversion is available with
[`TryFromEnumTreeRoot::from_root`].

```rust
use enum_tree::{EnumTree, ToEnumTreeRoot, TryFromEnumTreeRoot};

let root = SettingsAction::ToggleSound.to_root();
assert!(matches!(root, AppAction::Menu(MenuAction::Settings(SettingsAction::ToggleSound))));

let back = SettingsAction::from_root(root).unwrap();
assert_eq!(back, SettingsAction::ToggleSound);
```

The derive macros also implement `From` and `TryFrom` between parents and
children, so manual conversions are straightforward.

## License

This project is licensed under the terms of the [MIT License](LICENSE).

[`EnumTree`]: enum_tree/src/lib.rs
[`EnumTreeRoot`]: enum_tree/src/lib.rs
[`EnumTreeInner`]: enum_tree/src/lib.rs
[`EnumTreeLeaf`]: enum_tree/src/lib.rs
[`ToEnumTreeRoot`]: enum_tree/src/lib.rs
[`TryFromEnumTreeRoot`]: enum_tree/src/lib.rs
