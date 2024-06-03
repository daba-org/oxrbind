extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn xr_instance(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let name: syn::Ident = input.ident.clone();

    let gen = quote! {
        use std::sync::OnceLock;

        static oxr_generated_instance: OnceLock<#name> = OnceLock::new();

        #[no_mangle]
        pub extern "C" fn xrCreateInstance(
            createInfo: *const XrInstanceCreateInfo,
            instance: *mut XrInstance,
        ) -> XrResult {
            unsafe {
                let new_instance = #name{
                    create
                }
                instance = &#name{}
            }

            XrResult_XR_SUCCESS
        }

        #input
    };

    TokenStream::from(gen)
}
