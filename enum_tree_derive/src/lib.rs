use proc_macro::TokenStream;
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
    let mut has_inner = false;
    let mut has_leaf = false;

    for attr in attrs {
        if attr.path().is_ident("enum_tree_root") {
            is_root = true;
        } else if attr.path().is_ident("enum_tree_inner") {
            has_inner = true;
        } else if attr.path().is_ident("enum_tree_leaf") {
            has_leaf = true;
        }
    }

    if is_root {
        return expand_enum_tree_root(input);
    }
    if has_inner {
        return expand_enum_tree_inner(input);
    }
    if has_leaf {
        return expand_enum_tree_leaf(input);
    }

    syn::Error::new(
        input.span(),
        "EnumTree derive requires one of #[enum_tree_root], #[enum_tree_inner(P,R)], or #[enum_tree_leaf(P,R)]",
    )
    .to_compile_error()
}

pub(crate) fn expand_enum_tree_root(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident.clone();

    quote! {
        impl ::enum_tree::EnumTree<#ident> for #ident {
            type P = ();
        }
        impl ::enum_tree::EnumTreeRoot<#ident> for #ident {}

        impl ::enum_tree::ToEnumTreeRoot<#ident> for #ident {
            fn to_root(self) -> #ident { self }
        }

        impl ::enum_tree::TryFromEnumTreeRoot<#ident> for #ident {
            fn from_root(root: #ident) -> Option<Self> { Some(root) }
        }
    }
}

pub(crate) fn expand_enum_tree_inner(input: DeriveInput) -> proc_macro2::TokenStream {
    use std::collections::HashSet;

    let ident = input.ident.clone();

    // Collect all enum_tree_inner(P,R) attributes
    let mut pairs: Vec<(Type, Type)> = Vec::new();
    for attr in input.attrs.iter() {
        if attr.path().is_ident("enum_tree_inner") {
            pairs.push(parse_two_type_args(attr));
        }
    }
    if pairs.is_empty() {
        return syn::Error::new(input.span(), "missing parent type for enum_tree_inner")
            .to_compile_error();
    }

    let variant_ident = &ident; // Variant name in parent equals enum name

    // Generate impls for each (parent, root) pair
    let mut enum_impls = Vec::new();
    let mut from_impls = Vec::new();
    let mut seen_parents: HashSet<String> = HashSet::new();

    for (p_ty, r_ty) in &pairs {
        enum_impls.push(quote! {
            impl ::enum_tree::EnumTree<#r_ty> for #ident { type P = #p_ty; }
            impl ::enum_tree::EnumTreeInner<#r_ty> for #ident {}
        });

        let p_str = quote!(#p_ty).to_string();
        if seen_parents.insert(p_str) {
            from_impls.push(quote! {
                impl From<#ident> for #p_ty {
                    fn from(value: #ident) -> Self { #p_ty::#variant_ident(value) }
                }

                impl TryFrom<#p_ty> for #ident {
                    type Error = ();
                    fn try_from(value: #p_ty) -> Result<Self, Self::Error> {
                        if let #p_ty::#variant_ident(v) = value { Ok(v) } else { Err(()) }
                    }
                }
            });
        }
    }

    quote! {
        #(#enum_impls)*
        #(#from_impls)*
    }
}

pub(crate) fn expand_enum_tree_leaf(input: DeriveInput) -> proc_macro2::TokenStream {
    use std::collections::HashSet;

    let ident = input.ident.clone();

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

    // Collect all enum_tree_leaf(P,R) attributes
    let mut pairs: Vec<(Type, Type)> = Vec::new();
    for attr in input.attrs.iter() {
        if attr.path().is_ident("enum_tree_leaf") {
            pairs.push(parse_two_type_args(attr));
        }
    }
    if pairs.is_empty() {
        return syn::Error::new(input.span(), "missing parent type for enum_tree_leaf")
            .to_compile_error();
    }

    let parent_variant_ident = &ident; // Variant name in parent equals child enum name

    let mut enum_impls = Vec::new();
    let mut from_impls = Vec::new();
    let mut seen_parents: HashSet<String> = HashSet::new();

    for (p_ty, r_ty) in &pairs {
        enum_impls.push(quote! {
            impl ::enum_tree::EnumTree<#r_ty> for #ident { type P = #p_ty; }
            impl ::enum_tree::EnumTreeLeaf<#r_ty> for #ident {}
        });

        let p_str = quote!(#p_ty).to_string();
        if seen_parents.insert(p_str) {
            from_impls.push(quote! {
                impl From<#ident> for #p_ty {
                    fn from(value: #ident) -> Self { #p_ty::#parent_variant_ident(value) }
                }

                impl TryFrom<#p_ty> for #ident {
                    type Error = ();
                    fn try_from(value: #p_ty) -> Result<Self, Self::Error> {
                        if let #p_ty::#parent_variant_ident(v) = value { Ok(v) } else { Err(()) }
                    }
                }
            });
        }
    }

    quote! {
        #(#enum_impls)*
        #(#from_impls)*
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
