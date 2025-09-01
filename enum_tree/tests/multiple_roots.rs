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

#[derive(EnumTree, Clone, Debug, PartialEq)]
#[enum_tree_inner(RootOne, RootOne)]
#[enum_tree_inner(RootTwo, RootTwo)]
pub enum Parent {
    Leaf(Leaf),
}

#[derive(EnumTree, Clone, Debug, PartialEq)]
#[enum_tree_leaf(Parent, RootOne)]
#[enum_tree_leaf(Parent, RootTwo)]
pub enum Leaf {
    A,
}

#[test]
fn test_to_root_multiple() {
    let leaf = Leaf::A;
    let r1: RootOne = leaf.clone().to_root();
    let r2: RootTwo = leaf.clone().to_root();
    assert!(matches!(r1, RootOne::Parent(Parent::Leaf(Leaf::A))));
    assert!(matches!(r2, RootTwo::Parent(Parent::Leaf(Leaf::A))));
}

#[test]
fn test_from_root_multiple() {
    let leaf = Leaf::A;
    let r1: RootOne = leaf.clone().to_root();
    let r2: RootTwo = leaf.clone().to_root();
    let l1 = <Leaf as TryFromEnumTreeRoot<RootOne>>::from_root(r1).unwrap();
    let l2 = <Leaf as TryFromEnumTreeRoot<RootTwo>>::from_root(r2).unwrap();
    assert_eq!(l1, leaf);
    assert_eq!(l2, leaf);
}
