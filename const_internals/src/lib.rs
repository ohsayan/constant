use ::proc_macro::TokenStream;
mod constdef;
mod utils;

#[proc_macro_derive(Constdef)]
pub fn derive_constdef(input: TokenStream) -> TokenStream {
    constdef::derive(input)
}
