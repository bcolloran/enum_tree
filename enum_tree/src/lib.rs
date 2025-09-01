/// Trait for nodes in an enum tree
/// - `P`: The immediate parent enum type
/// - `R`: The root enum type of the entire tree
pub trait EnumTree: Sized {
    type P: Sized;
    type R: Sized;
}

pub use enum_tree_derive::*;

/// Marker trait for the root of an enum tree
pub trait EnumTreeRoot: EnumTree<P = ()> {}
/// Marker trait for inner nodes of an enum tree
pub trait EnumTreeInner: EnumTree {}
/// Marker trait for leaf nodes of an enum tree
pub trait EnumTreeLeaf: EnumTree {}

pub trait ToEnumTreeRoot: EnumTree {
    fn to_root(self) -> Self::R;
}

impl<T, Root> ToEnumTreeRoot for T
where
    T: EnumTree<R = Root>,
    <T as EnumTree>::P: From<T> + EnumTree<R = Root> + ToEnumTreeRoot,
{
    fn to_root(self) -> <T as EnumTree>::R {
        let p: <T as EnumTree>::P = self.into();
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

pub trait TryFromEnumTreeRoot: EnumTree {
    fn from_root(root: Self::R) -> Option<Self>;
}

impl<T, Root> TryFromEnumTreeRoot for T
where
    T: EnumTree<R = Root> + TryFrom<<T as EnumTree>::P, Error = ()>,
    <T as EnumTree>::P: EnumTree<R = Root> + TryFromEnumTreeRoot,
{
    fn from_root(root: Self::R) -> Option<Self> {
        let p: <T as EnumTree>::P = <T as EnumTree>::P::from_root(root)?;
        <T as TryFrom<<T as EnumTree>::P>>::try_from(p).ok()
    }
}
