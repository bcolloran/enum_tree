use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

use crate::expand_enum_tree_leaf;

#[test]
fn leaf_multiple_roots_different_parents() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(ParentOne, RootOne)]
        #[enum_tree_leaf(ParentTwo, RootTwo)]
        pub enum Leaf { Action }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootOne> for Leaf { type P = ParentOne; }
        impl ::enum_tree::EnumTreeLeaf<RootOne> for Leaf {}
        impl ::enum_tree::EnumTree<RootTwo> for Leaf { type P = ParentTwo; }
        impl ::enum_tree::EnumTreeLeaf<RootTwo> for Leaf {}
        impl From<Leaf> for ParentOne {
            fn from(value: Leaf) -> Self { ParentOne::Leaf(value) }
        }
        impl TryFrom<ParentOne> for Leaf {
            type Error = (); fn try_from(value: ParentOne) -> Result<Self, Self::Error> {
                if let ParentOne::Leaf(v) = value { Ok(v) } else { Err(()) }
            }
        }
        impl From<Leaf> for ParentTwo {
            fn from(value: Leaf) -> Self { ParentTwo::Leaf(value) }
        }
        impl TryFrom<ParentTwo> for Leaf {
            type Error = (); fn try_from(value: ParentTwo) -> Result<Self, Self::Error> {
                if let ParentTwo::Leaf(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_leaf(input);
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn leaf_multiple_roots_same_parent() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(Parent, RootOne)]
        #[enum_tree_leaf(Parent, RootTwo)]
        pub enum Leaf { Action }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootOne> for Leaf { type P = Parent; }
        impl ::enum_tree::EnumTreeLeaf<RootOne> for Leaf {}
        impl ::enum_tree::EnumTree<RootTwo> for Leaf { type P = Parent; }
        impl ::enum_tree::EnumTreeLeaf<RootTwo> for Leaf {}
        impl From<Leaf> for Parent {
            fn from(value: Leaf) -> Self { Parent::Leaf(value) }
        }
        impl TryFrom<Parent> for Leaf {
            type Error = (); fn try_from(value: Parent) -> Result<Self, Self::Error> {
                if let Parent::Leaf(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_leaf(input);
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn leaf_multiple_roots_same_parent_alias() {
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(super::Parent, RootOne)]
        #[enum_tree_leaf(crate::mods::Parent, RootTwo)]
        pub enum Leaf { Action }
    };

    let expected = quote! {
        impl ::enum_tree::EnumTree<RootOne> for Leaf { type P = super::Parent; }
        impl ::enum_tree::EnumTreeLeaf<RootOne> for Leaf {}
        impl ::enum_tree::EnumTree<RootTwo> for Leaf { type P = crate::mods::Parent; }
        impl ::enum_tree::EnumTreeLeaf<RootTwo> for Leaf {}
        impl From<Leaf> for super::Parent {
            fn from(value: Leaf) -> Self { super::Parent::Leaf(value) }
        }
        impl TryFrom<super::Parent> for Leaf {
            type Error = (); fn try_from(value: super::Parent) -> Result<Self, Self::Error> {
                if let super::Parent::Leaf(v) = value { Ok(v) } else { Err(()) }
            }
        }
    };

    let actual = expand_enum_tree_leaf(input);
    assert_eq!(actual.to_string(), expected.to_string());
}
