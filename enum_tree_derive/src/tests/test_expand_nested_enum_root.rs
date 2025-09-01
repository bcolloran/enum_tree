use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_root;

#[test]
fn test_root_simple() {
    // Root enum with one child branch `MenuFlow` and another `AudioActions`.
    // The derive should mark it as EnumTreeRoot with Parent=(), Root=Self
    // and implement ToEnumTreeRoot and TryFromEnumTreeRoot trivial conversions.
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_root]
        pub enum RootAction {
            MenuFlow(MenuFlow),
            AudioActions(AudioActions),
        }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree for RootAction {
            type P = ();
            type R = RootAction;
        }
        impl ::enum_tree::EnumTreeRoot for RootAction {}

        impl ::enum_tree::ToEnumTreeRoot for RootAction {
            fn to_root(self) -> Self::R { self }
        }

        impl ::enum_tree::TryFromEnumTreeRoot for RootAction {
            fn from_root(root: Self::R) -> Option<Self> { Some(root) }
        }
    };

    let actual = expand_enum_tree_root(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
