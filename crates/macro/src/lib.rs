use proc_macro::TokenStream;
use quote::quote;

// ======================
// === Dec19x19 macro ===
// ======================

#[allow(non_snake_case)]
#[proc_macro]
pub fn Dec19x19(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let repr = fixed_num_helper::parse_dec19x19_internal(&input_str).expect("Parsing failed");
    let output = quote! {
        fixed_num::Dec19x19::from_repr(#repr)
    };
    output.into()
}
