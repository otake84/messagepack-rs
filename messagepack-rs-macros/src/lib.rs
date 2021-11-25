#![forbid(unsafe_code)]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MessagePackFrom)]
pub fn message_pack_from_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_message_pack_from_macro(&ast)
}

fn impl_message_pack_from_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<T: Into<#name>> From<Option<T>> for #name {
            fn from(value: Option<T>) -> Self {
                value.map_or(Self::Nil, Into::into)
            }
        }

        impl From<bool> for #name {
            fn from(value: bool) -> Self {
                Self::Bool(value)
            }
        }

        impl From<f32> for #name {
            fn from(value: f32) -> Self {
                Self::Float32(value)
            }
        }

        impl From<f64> for #name {
            fn from(value: f64) -> Self {
                Self::Float64(value)
            }
        }

        impl From<u8> for #name {
            fn from(value: u8) -> Self {
                Self::UInt8(value)
            }
        }

        impl From<u16> for #name {
            fn from(value: u16) -> Self {
                Self::UInt16(value)
            }
        }

        impl From<u32> for #name {
            fn from(value: u32) -> Self {
                Self::UInt32(value)
            }
        }

        impl From<u64> for #name {
            fn from(value: u64) -> Self {
                Self::UInt64(value)
            }
        }

        impl From<i8> for #name {
            fn from(value: i8) -> Self {
                Self::Int8(value)
            }
        }

        impl From<i16> for #name {
            fn from(value: i16) -> Self {
                Self::Int16(value)
            }
        }

        impl From<i32> for #name {
            fn from(value: i32) -> Self {
                Self::Int32(value)
            }
        }

        impl From<i64> for #name {
            fn from(value: i64) -> Self {
                Self::Int64(value)
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                Self::String(value)
            }
        }

        impl From<Binary> for #name {
            fn from(value: Binary) -> Self {
                Self::Binary(value)
            }
        }

        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                Self::String(String::from(value))
            }
        }

        impl From<Vec<Self>> for #name {
            fn from(value: Vec<Self>) -> Self {
                Self::Array(value)
            }
        }

        impl From<&[Self]> for #name {
            fn from(value: &[Self]) -> Self {
                Self::Array(Vec::from(value))
            }
        }

        impl From<BTreeMap<String, Self>> for #name {
            fn from(value: BTreeMap<String, Self>) -> Self {
                Self::Map(value)
            }
        }

        impl From<Extension> for #name {
            fn from(value: Extension) -> Self {
                Self::Extension(value)
            }
        }

        impl From<DateTime<Utc>> for #name {
            fn from(value: DateTime<Utc>) -> Self {
                Self::Timestamp(value)
            }
        }
    };
    gen.into()
}
