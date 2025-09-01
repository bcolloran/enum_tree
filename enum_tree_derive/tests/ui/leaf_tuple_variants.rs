use enum_tree_derive::EnumTree as DeriveEnumTree;

// Parent and root enums to satisfy paths in attributes
pub enum MenuFlow {
    IpSetup(IpSetup),
}
pub enum RootAction {
    MenuFlow(MenuFlow),
}

#[derive(DeriveEnumTree)]
#[enum_tree_leaf(MenuFlow, RootAction)]
pub enum IpSetup {
    UpdatePortText(i32),
    ClickStartIp,
}

fn main() {}
