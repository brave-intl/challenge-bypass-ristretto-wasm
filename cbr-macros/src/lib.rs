#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Base64)]
pub fn base64(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let parent_name = Ident::new(&format!("_{}", name), Span::call_site());

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl From<#parent_name> for #name {
            fn from(t: #parent_name) -> Self {
                #name(t)
            }
        }

        #[wasm_bindgen]
        impl #name {
            /// Encode to a base64 string
            pub fn encode_base64(&self) -> ::std::string::String {
                self.0.encode_base64()
            }
            /// Decode from a base64 string
            pub fn decode_base64(s: &str) -> Result<#name, JsValue> {
                #parent_name::decode_base64(s).map(|t| t.into()).map_err(|e| convert_error(e))
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
