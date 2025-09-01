use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_leaf;

#[test]
fn leaf_directly_under_root() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(RootAction, RootAction)]
        pub enum AudioActions { ToggleMuteAll }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree for AudioActions {
            type P = RootAction;
            type R = RootAction;
        }
        impl ::enum_tree::EnumTreeLeaf for AudioActions {}

        impl From<AudioActions> for RootAction {
            fn from(value: AudioActions) -> Self { RootAction::AudioActions(value) }
        }

        impl TryFrom<RootAction> for AudioActions {
            type Error = ();
            fn try_from(value: RootAction) -> Result<Self, Self::Error> {
                if let RootAction::AudioActions(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_leaf(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
