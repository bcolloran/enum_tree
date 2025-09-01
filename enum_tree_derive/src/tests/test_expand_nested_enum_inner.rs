use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_inner;

#[test]
fn test_inner_simple() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_inner(RootAction, RootAction)]
        pub enum MenuFlow {
            General(General),
        }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootAction> for MenuFlow { type P = RootAction; }
        impl ::enum_tree::EnumTreeInner<RootAction> for MenuFlow {}

        impl From<MenuFlow> for RootAction {
            fn from(value: MenuFlow) -> Self { RootAction::MenuFlow(value) }
        }

        impl TryFrom<RootAction> for MenuFlow {
            type Error = ();
            fn try_from(value: RootAction) -> Result<Self, Self::Error> {
                if let RootAction::MenuFlow(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_inner(input);
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_inner_nested_child() {
    // Settings is an inner node whose parent is another inner node (MenuFlow), not the root.
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_inner(MenuFlow, RootAction)]
        pub enum Settings {
            Audio(Audio),
            Video(Video),
        }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootAction> for Settings { type P = MenuFlow; }
        impl ::enum_tree::EnumTreeInner<RootAction> for Settings {}

        impl From<Settings> for MenuFlow {
            fn from(value: Settings) -> Self { MenuFlow::Settings(value) }
        }

        impl TryFrom<MenuFlow> for Settings {
            type Error = ();
            fn try_from(value: MenuFlow) -> Result<Self, Self::Error> {
                if let MenuFlow::Settings(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_inner(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
