extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use core::mem::size_of_val;


#[proc_macro_derive(TotalSize)]
pub fn derive_total_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let gen = quote! {
        impl TotalSize for #name {
            fn total_size(&self) -> usize {
                100
            }
        }
    };

    proc_macro::TokenStream::from(gen)
}

