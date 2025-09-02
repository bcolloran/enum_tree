use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_leaf;

#[test]

fn test_simple() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(MenuFlow, RootAction)]
            pub enum General {
              ClickBack,
              }

    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootAction> for General { type P = MenuFlow; }
        impl ::enum_tree::EnumTreeLeaf<RootAction> for General {}

        impl From<General> for MenuFlow {
            fn from(value: General) -> Self {
                MenuFlow::General(value)
            }
        }

        impl TryFrom<MenuFlow> for General {
            type Error = ();
            fn try_from(value: MenuFlow) -> Result<Self, Self::Error> {
                if let MenuFlow::General(v) = value {
                    Ok(v)
                } else {
                    Err(())
                }
            }
        }
    };

    let actual = expand_enum_tree_leaf(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
