use crate::utils;
use crate::utils::NamedFieldInfo;
use crate::utils::UnnamedFieldInfo;
mod process_types;
use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, spanned::Spanned, DeriveInput, Type};
use utils::FieldInfo;

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let fields = match utils::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    match fields {
        FieldInfo::Named(fields) => generate_named(fields, &ast),
        FieldInfo::Unnamed(fields) => generate_unnamed(fields, &ast),
    }
}

fn generate_unnamed(fields: Vec<UnnamedFieldInfo<'_>>, ast: &DeriveInput) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = ast.generics.split_for_impl();
    let struct_name = &ast.ident;
    let mut self_args = quote! {};
    for (ty, _attrs) in fields {
        let ret: Option<::quote::__private::TokenStream> = match ty {
            Type::Path(_) => Some(quote! { ::constant::Constdef::DEFAULT }),
            Type::Array(arr) => self::process_types::process_array(arr).map(|tokens| {
                quote! {
                    [#tokens]
                }
            }),
            Type::Tuple(tpl) => self::process_types::recursive_process_tuple(tpl).map(|tokens| {
                quote! {
                    (#tokens)
                }
            }),
            _ => {
                return syn::Error::new(ty.span(), "Unsupported type for `Constdef`")
                    .into_compile_error()
                    .into()
            }
        };
        if let Some(r) = ret {
            self_args = quote! {
                #self_args
                #r,
            }
        } else {
            return syn::Error::new(ty.span(), "Unsupported type for `Constdef`")
                .into_compile_error()
                .into();
        }
    }
    let tokens = quote! {
        impl #impl_gen #struct_name #ty_gen #where_clause {
            pub const fn default() -> Self {
                Self(#self_args)
            }
        }
        impl #impl_gen ::core::default::Default for #struct_name #ty_gen #where_clause {
            fn default() -> Self {
                Self::default()
            }
        }
        impl #impl_gen ::constant::Constdef for #struct_name #ty_gen #where_clause {
            const DEFAULT: Self = Self::default();
        }
    };
    tokens.into()
}

fn generate_named(fields: Vec<NamedFieldInfo<'_>>, ast: &DeriveInput) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = ast.generics.split_for_impl();
    let struct_name = &ast.ident;
    let mut self_args = quote! {};
    for (field, ty, _attrs) in fields {
        let ret: Option<::quote::__private::TokenStream> = match ty {
            Type::Path(_) => Some(quote! {
                ::constant::Constdef::DEFAULT
            }),
            Type::Array(arr) => self::process_types::process_array(arr).map(|tokens| {
                quote! {
                    [#tokens]
                }
            }),
            Type::Tuple(tpl) => self::process_types::recursive_process_tuple(tpl).map(|tokens| {
                quote! {
                    (#tokens)
                }
            }),
            _ => {
                return syn::Error::new(field.span(), "Unsupported type for `Constdef`")
                    .into_compile_error()
                    .into()
            }
        };
        if let Some(r) = ret {
            self_args = quote! {
                #self_args
                #field: #r,
            };
        } else {
            return syn::Error::new(ty.span(), "Unsupported type for `Constdef`")
                .into_compile_error()
                .into();
        }
    }
    let tokens = quote! {
        impl #impl_gen #struct_name #ty_gen #where_clause {
            pub const fn default() -> Self {
                Self {
                    #self_args
                }
            }
        }
        impl #impl_gen ::core::default::Default for #struct_name #ty_gen #where_clause {
            fn default() -> Self {
                Self::default()
            }
        }
        impl #impl_gen ::constant::Constdef for #struct_name #ty_gen #where_clause {
            const DEFAULT: Self = Self::default();
        }
    };
    tokens.into()
}
