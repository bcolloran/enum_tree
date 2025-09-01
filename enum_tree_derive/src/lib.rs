use proc_macro::TokenStream;
use proc_macro2::Ident as PMIdent;
use quote::quote;
use syn::{Attribute, Data, DataEnum, DeriveInput, Fields, Type, spanned::Spanned};

#[cfg(test)]
mod tests;

/// `#[derive(EnumTree)]` only works on enums.
///
/// Needs one of the following attributes:
/// - `#[enum_tree_root]`
/// - `#[enum_tree_inner(P,R)]`, where P is the parent enum type, and R is the root enum type
/// - `#[enum_tree_leaf(P,R)]`, where P is the parent enum type, and R is the root enum type
///
/// The derive will implement the `EnumTree` trait, and depending on the attribute, one of the
/// `EnumTreeRoot`, `EnumTreeInner`, or `EnumTreeLeaf` marker traits.
///
///
/// For root nodes,  the parent type is `()`. We will NOT implement `From` or `TryFrom` for the parent type, as there is no parent. Implementation of `ToEnumTreeRoot` and `TryFromEnumTreeRoot` for root types must be handled via macro expansion to avoid conflicting with the blanket impl in the `enum_tree` crate.
///
/// For inner and leaf nodes,
/// as well as `From<P>` and `TryFrom<P>` for the parent enum type P.  Implementation of `ToEnumTreeRoot` and `TryFromEnumTreeRoot` for the all non-root is handled via a blanket impl in the `enum_tree` crate, not via macro expansion.
///
/// Inner nodes and the root node must have variants that wrap their child enum types;
/// i.e., every variant must be a single tuple variant with one slot,
/// and the type of that slot must be one of the child enum types.
/// Variant names MUST match the name of the child enum type. (Failure should result in a compile error pointing to the offending variant.)
///
/// Leaf nodes must have either unit variants, or struct variants (named fields). Tuple variants are not allowed, and should result in a compile error.
#[proc_macro_derive(EnumTree, attributes(enum_tree_root, enum_tree_inner, enum_tree_leaf))]
pub fn enum_tree_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_enum_tree(derive_input))
}

pub(crate) fn expand_enum_tree(input: DeriveInput) -> proc_macro2::TokenStream {
    let attrs = &input.attrs;
    let mut is_root = false;
    let mut is_inner = None::<(Type, Type)>;
    let mut is_leaf = None::<(Type, Type)>;

    for attr in attrs {
        if attr.path().is_ident("enum_tree_root") {
            is_root = true;
        } else if attr.path().is_ident("enum_tree_inner") {
            let (p, r) = parse_two_type_args(attr);
            is_inner = Some((p, r));
        } else if attr.path().is_ident("enum_tree_leaf") {
            let (p, r) = parse_two_type_args(attr);
            is_leaf = Some((p, r));
        }
    }

    if is_root {
        return expand_enum_tree_root(input);
    }
    if let Some((_p, _r)) = is_inner {
        return expand_enum_tree_inner(input);
    }
    if let Some((_p, _r)) = is_leaf {
        return expand_enum_tree_leaf(input);
    }

    syn::Error::new(
        input.span(),
        "EnumTree derive requires one of #[enum_tree_root], #[enum_tree_inner(P,R)], or #[enum_tree_leaf(P,R)]",
    )
    .to_compile_error()
}

pub(crate) fn expand_enum_tree_root(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;

    quote! {
        impl ::enum_tree::EnumTree for #ident {
            type P = ();
            type R = #ident;
        }
        impl ::enum_tree::EnumTreeRoot for #ident {}

        impl ::enum_tree::ToEnumTreeRoot for #ident {
            fn to_root(self) -> Self::R { self }
        }

        impl ::enum_tree::TryFromEnumTreeRoot for #ident {
            fn from_root(root: Self::R) -> Option<Self> { Some(root) }
        }
    }
}

pub(crate) fn expand_enum_tree_inner(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;

    // Find enum_tree_inner(P,R)
    let mut parent_ty: Option<Type> = None;
    let mut root_ty: Option<Type> = None;
    for attr in input.attrs.iter() {
        if attr.path().is_ident("enum_tree_inner") {
            let (p, r) = parse_two_type_args(attr);
            parent_ty = Some(p);
            root_ty = Some(r);
            break;
        }
    }
    let p_ty = parent_ty.expect("missing parent type for enum_tree_inner");
    let r_ty = root_ty.expect("missing root type for enum_tree_inner");

    // Variant path name in parent/root assumed to match enum ident
    let variant_ident = &ident;
    let parent_ident = type_ident_from_type(&p_ty);
    let root_ident = type_ident_from_type(&r_ty);
    let parent_is_root =
        parent_ident.is_some() && root_ident.is_some() && parent_ident == root_ident;

    if parent_is_root {
        quote! {
            impl ::enum_tree::EnumTree for #ident {
                type P = #p_ty;
                type R = #r_ty;
            }
            impl ::enum_tree::EnumTreeInner for #ident {}

            impl From<#ident> for #p_ty {
                fn from(value: #ident) -> Self { #p_ty::#variant_ident(value) }
            }

            impl TryFrom<#p_ty> for #ident {
                type Error = ();
                fn try_from(value: #p_ty) -> Result<Self, Self::Error> {
                    if let #p_ty::#variant_ident(v) = value { Ok(v) } else { Err(()) }
                }
            }


        }
    } else {
        quote! {
            impl ::enum_tree::EnumTree for #ident {
                type P = #p_ty;
                type R = #r_ty;
            }
            impl ::enum_tree::EnumTreeInner for #ident {}

            impl From<#ident> for #p_ty {
                fn from(value: #ident) -> Self { #p_ty::#variant_ident(value) }
            }

            impl TryFrom<#p_ty> for #ident {
                type Error = ();
                fn try_from(value: #p_ty) -> Result<Self, Self::Error> {
                    if let #p_ty::#variant_ident(v) = value { Ok(v) } else { Err(()) }
                }
            }


        }
    }
}

pub(crate) fn expand_enum_tree_leaf(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;

    // Validate leaf enum variants: only unit or struct (named fields). No tuple variants allowed.
    if let Data::Enum(DataEnum { variants, .. }) = &input.data {
        for v in variants {
            match &v.fields {
                Fields::Unnamed(_) => {
                    let msg = format!(
                        "EnumTree leaf '{}' cannot have tuple variants (found tuple variant '{}')",
                        ident, v.ident
                    );
                    return syn::Error::new(v.span(), msg).to_compile_error();
                }
                Fields::Named(_) | Fields::Unit => {}
            }
        }
    }

    // Find enum_tree_leaf(P,R)
    let mut parent_ty: Option<Type> = None;
    let mut root_ty: Option<Type> = None;
    for attr in input.attrs.iter() {
        if attr.path().is_ident("enum_tree_leaf") {
            let (p, r) = parse_two_type_args(attr);
            parent_ty = Some(p);
            root_ty = Some(r);
            break;
        }
    }
    let p_ty = parent_ty.expect("missing parent type for enum_tree_leaf");
    let r_ty = root_ty.expect("missing root type for enum_tree_leaf");

    let parent_variant_ident = &ident; // Variant name in parent equals child enum name

    quote! {
        impl ::enum_tree::EnumTree for #ident {
            type P = #p_ty;
            type R = #r_ty;
        }
        impl ::enum_tree::EnumTreeLeaf for #ident {}

        impl From<#ident> for #p_ty {
            fn from(value: #ident) -> Self { #p_ty::#parent_variant_ident(value) }
        }

        impl TryFrom<#p_ty> for #ident {
            type Error = ();
            fn try_from(value: #p_ty) -> Result<Self, Self::Error> {
                if let #p_ty::#parent_variant_ident(v) = value { Ok(v) } else { Err(()) }
            }
        }

    // #to_root_impl
    // #try_from_root_impl
    }
}

fn parse_two_type_args(attr: &Attribute) -> (Type, Type) {
    // Expect attribute like #[enum_tree_inner(P, R)] or #[enum_tree_leaf(P, R)]
    use syn::{Token, punctuated::Punctuated};
    let args: Punctuated<Type, Token![,]> = attr
        .parse_args_with(Punctuated::<Type, Token![,]>::parse_terminated)
        .expect("failed to parse attribute type arguments");
    let mut args_iter = args.into_iter();
    let p = args_iter
        .next()
        .expect("expected parent type as first attribute argument");
    let r = args_iter
        .next()
        .expect("expected root type as second attribute argument");
    (p, r)
}

fn type_ident_from_type(ty: &Type) -> Option<PMIdent> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            return Some(seg.ident.clone());
        }
    }
    None
}
