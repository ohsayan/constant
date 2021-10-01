//! # Internal macros for the [`constant`](https://github.com/ohsayan/constant) crate.
//!
//! This crate contains some internal macros for the [`constant`](https://github.com/ohsayan/constant)
//! crate. Consider using the `constant` crate instead because the macros are unusable without the
//! constant crate.
//!

use ::proc_macro::TokenStream;
mod constdef;
mod utils;

#[proc_macro_derive(Constdef)]
pub fn derive_constdef(input: TokenStream) -> TokenStream {
    constdef::derive(input)
}
