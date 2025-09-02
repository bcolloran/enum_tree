use enum_tree::{EnumTree, ToEnumTreeRoot, TryFromEnumTreeRoot};

#[derive(EnumTree, Clone, Debug, PartialEq)]
#[enum_tree_root]
pub enum RootOne {
    Parent(Parent),
}

#[derive(EnumTree, Clone, Debug, PartialEq)]
#[enum_tree_root]
pub enum RootTwo {
    Parent(Parent),
}

mod nested {
    use super::*;

    #[derive(EnumTree, Clone, Debug, PartialEq)]
    #[enum_tree_inner(super::Parent, RootOne)]
    #[enum_tree_inner(crate::Parent, RootTwo)]
    pub enum Child {
        Leaf(Leaf),
    }

    #[derive(EnumTree, Clone, Debug, PartialEq)]
    #[enum_tree_leaf(Child, RootOne)]
    #[enum_tree_leaf(crate::nested::Child, RootTwo)]
    pub enum Leaf {
        A,
    }
}

#[derive(EnumTree, Clone, Debug, PartialEq)]
#[enum_tree_inner(RootOne, RootOne)]
#[enum_tree_inner(RootTwo, RootTwo)]
pub enum Parent {
    Child(nested::Child),
}

#[test]
fn test_to_root_alias_parent() {
    let leaf = nested::Leaf::A;
    let r1: RootOne = leaf.clone().to_root();
    let r2: RootTwo = leaf.clone().to_root();
    assert!(matches!(r1, RootOne::Parent(Parent::Child(nested::Child::Leaf(nested::Leaf::A)))));
    assert!(matches!(r2, RootTwo::Parent(Parent::Child(nested::Child::Leaf(nested::Leaf::A)))));
}

#[test]
fn test_from_root_alias_parent() {
    let leaf = nested::Leaf::A;
    let r1: RootOne = leaf.clone().to_root();
    let r2: RootTwo = leaf.clone().to_root();
    let l1 = <nested::Leaf as TryFromEnumTreeRoot<RootOne>>::from_root(r1).unwrap();
    let l2 = <nested::Leaf as TryFromEnumTreeRoot<RootTwo>>::from_root(r2).unwrap();
    assert_eq!(l1, leaf);
    assert_eq!(l2, leaf);
}
