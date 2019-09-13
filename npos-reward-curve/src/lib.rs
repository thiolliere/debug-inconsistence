extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn build(input: TokenStream) -> TokenStream {
	input
}
