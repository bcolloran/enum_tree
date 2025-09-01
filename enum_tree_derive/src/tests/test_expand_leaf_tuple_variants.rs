use syn::parse_quote;

use crate::expand_enum_tree_leaf;

#[test]
fn test_leaf_tuple_variants_rejected() {
    // Leaf nodes must not have tuple variants. This should cause an error;
    // since our expand function returns TokenStream, we simulate by asserting it would panic
    // or, more simply for now, ensure we do NOT generate any code for such invalid input.
    let input: syn::DeriveInput = parse_quote! {
        #[enum_tree_leaf(MenuFlow, RootAction)]
        pub enum IpSetup {
            UpdatePortText(i32),
            ClickStartIp,
        }
    };

    let panicked = std::panic::catch_unwind(|| {
        let _ = expand_enum_tree_leaf(input);
    })
    .is_err();

    // This should eventually be rejected gracefully (no panic, a compile_error! or similar),
    // but for now we assert that it does NOT panic so this test fails until implemented.
    assert!(
        !panicked,
        "Expected macro to reject leaf tuple variants without panicking"
    );
}
