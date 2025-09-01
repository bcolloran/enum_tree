use enum_tree_derive::EnumTree as DeriveEnumTree;

// Root and Parent enums
pub enum RootAction {
    MenuFlow(MenuFlow),
}
pub enum MenuFlow {
    NotSettings(Settings),
}

#[derive(DeriveEnumTree)]
#[enum_tree_inner(MenuFlow, RootAction)]
pub enum Settings {
    // pretend children
    Audio,
}

fn main() {}
