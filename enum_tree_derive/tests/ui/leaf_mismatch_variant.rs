use enum_tree_derive::EnumTree as DeriveEnumTree;

// Parent and Root enums
pub enum MenuFlow { NotGeneral(General) }
pub enum RootAction { MenuFlow(MenuFlow) }

#[derive(DeriveEnumTree)]
#[enum_tree_leaf(MenuFlow, RootAction)]
pub enum General { ClickBack }

fn main() {}
