/// Trait for nodes in an enum tree.
///
/// `R` is the root enum type for the tree this node belongs to and `P` is the
/// immediate parent enum type on the path to that root.  By making `R` a type
/// parameter rather than an associated type we can implement `EnumTree` for the
/// same enum multiple times with different roots, allowing a single enum to be
/// part of multiple overlapping trees.
pub trait EnumTree<R>: Sized {
    /// Immediate parent type when traversing towards the root `R`.
    type P: Sized;
}

pub use enum_tree_derive::*;

/// Marker trait for the root of an enum tree with root type `R`.
pub trait EnumTreeRoot<R>: EnumTree<R, P = ()> {}
/// Marker trait for inner nodes of an enum tree with root type `R`.
pub trait EnumTreeInner<R>: EnumTree<R> {}
/// Marker trait for leaf nodes of an enum tree with root type `R`.
pub trait EnumTreeLeaf<R>: EnumTree<R> {}

/// Convert a node into its corresponding root enum for a given tree `R`.
pub trait ToEnumTreeRoot<R>: EnumTree<R> {
    fn to_root(self) -> R;
}

impl<T, Root> ToEnumTreeRoot<Root> for T
where
    T: EnumTree<Root>,
    <T as EnumTree<Root>>::P: From<T> + EnumTree<Root> + ToEnumTreeRoot<Root>,
{
    fn to_root(self) -> Root {
        let p: <T as EnumTree<Root>>::P = self.into();
        p.to_root()
    }
}

// impl<T> ToEnumTreeRoot for T
// where
//     T: EnumTree<R = T, P = ()> + EnumTreeRoot,
// {
//     fn to_root(self) -> <T as EnumTree>::R {
//         self
//     }
// }

pub trait TryFromEnumTreeRoot<R>: EnumTree<R> {
    fn from_root(root: R) -> Option<Self>;
}

impl<T, Root> TryFromEnumTreeRoot<Root> for T
where
    T: EnumTree<Root> + TryFrom<<T as EnumTree<Root>>::P, Error = ()>,
    <T as EnumTree<Root>>::P: EnumTree<Root> + TryFromEnumTreeRoot<Root>,
{
    fn from_root(root: Root) -> Option<Self> {
        let p: <T as EnumTree<Root>>::P = <T as EnumTree<Root>>::P::from_root(root)?;
        <T as TryFrom<<T as EnumTree<Root>>::P>>::try_from(p).ok()
    }
}
