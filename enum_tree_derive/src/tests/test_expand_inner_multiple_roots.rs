use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_inner;

#[test]
fn inner_multiple_roots_different_parents() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_inner(ParentOne, RootOne)]
        #[enum_tree_inner(ParentTwo, RootTwo)]
        pub enum Child {
            Leaf(Leaf),
        }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootOne> for Child { type P = ParentOne; }
        impl ::enum_tree::EnumTreeInner<RootOne> for Child {}
        impl ::enum_tree::EnumTree<RootTwo> for Child { type P = ParentTwo; }
        impl ::enum_tree::EnumTreeInner<RootTwo> for Child {}
        impl From<Child> for ParentOne {
            fn from(value: Child) -> Self { ParentOne::Child(value) }
        }
        impl TryFrom<ParentOne> for Child {
            type Error = (); fn try_from(value: ParentOne) -> Result<Self, Self::Error> {
                if let ParentOne::Child(v) = value { Ok(v) } else { Err(()) }
            }
        }
        impl From<Child> for ParentTwo {
            fn from(value: Child) -> Self { ParentTwo::Child(value) }
        }
        impl TryFrom<ParentTwo> for Child {
            type Error = (); fn try_from(value: ParentTwo) -> Result<Self, Self::Error> {
                if let ParentTwo::Child(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_inner(input);
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn inner_multiple_roots_same_parent() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_inner(Parent, RootOne)]
        #[enum_tree_inner(Parent, RootTwo)]
        pub enum Child {
            Leaf(Leaf),
        }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootOne> for Child { type P = Parent; }
        impl ::enum_tree::EnumTreeInner<RootOne> for Child {}
        impl ::enum_tree::EnumTree<RootTwo> for Child { type P = Parent; }
        impl ::enum_tree::EnumTreeInner<RootTwo> for Child {}
        impl From<Child> for Parent {
            fn from(value: Child) -> Self { Parent::Child(value) }
        }
        impl TryFrom<Parent> for Child {
            type Error = (); fn try_from(value: Parent) -> Result<Self, Self::Error> {
                if let Parent::Child(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_inner(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
