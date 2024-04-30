use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, Variant};

enum Algtype {
    Struct(Fields),
    Enum(Vec<Variant>),
}

fn xn(n: usize) -> Ident {
    format_ident!("x{n}")
}

fn product_ty(path: &TokenStream, fields: &Fields) -> TokenStream {
    fields.iter().rfold(
        quote!(#path::One),
        |rest, Field { ty, .. }| quote!(#path::Product<#ty, #rest>),
    )
}
fn sum_ty<'a>(
    path: &TokenStream,
    variants: impl DoubleEndedIterator<Item = &'a Fields>,
) -> TokenStream {
    variants.map(|f| product_ty(path, f)).rfold(
        quote!(#path::Zero),
        |rest, ty| quote!(#path::Sum<#ty, #rest>),
    )
}
fn repr_ty(path: &TokenStream, data: &Algtype) -> TokenStream {
    match data {
        Algtype::Struct(fields) => sum_ty(path, std::iter::once(fields)),
        Algtype::Enum(fields) => sum_ty(path, fields.iter().map(|f| &f.fields)),
    }
}

fn product_repr(path: &TokenStream, count: usize) -> TokenStream {
    (0..count).rfold(quote!(#path::One), |rest, this| {
        let this = xn(this);
        quote!(#path::Product(#this, #rest))
    })
}
fn sum_repr(path: &TokenStream, index: usize, count: usize) -> TokenStream {
    let then = product_repr(path, count);
    (0..index).rfold(
        quote!(#path::Sum::This(#then)),
        |next, _| quote!(#path::Sum::Next(#next)),
    )
}

fn fields_val(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(f) => {
            let var = (0..).map(xn);
            let key = f.named.iter().map(|f| &f.ident);
            quote!({ #(#key: #var),* })
        }
        Fields::Unnamed(f) => {
            let var = (0..f.unnamed.len()).map(xn);
            quote!((#(#var),*))
        }
        Fields::Unit => quote!(),
    }
}
/// 返回结构和表示
fn algtype_val(path: &TokenStream, data: &Algtype) -> (Vec<TokenStream>, Vec<TokenStream>) {
    match data {
        Algtype::Struct(f) => {
            let val = fields_val(f);
            (vec![quote!(Self #val)], vec![sum_repr(path, 0, f.len())])
        }
        Algtype::Enum(v) => v
            .iter()
            .enumerate()
            .map(|(i, Variant { ident, fields, .. })| {
                let val = fields_val(fields);
                (quote!(Self::#ident #val), sum_repr(path, i, fields.len()))
            })
            .unzip(),
    }
}

fn m(path: &TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = match input.data {
        Data::Struct(data) => Algtype::Struct(data.fields),
        Data::Enum(data) => Algtype::Enum(data.variants.into_iter().collect()),
        Data::Union(_) => panic!("union is unsupported"),
    };
    let repr_ty = repr_ty(path, &data);
    let (stru, repr) = algtype_val(path, &data);

    quote! {
        impl #impl_generics #path::Generic for #name #ty_generics #where_clause {
            type Repr = #repr_ty;

            #[inline]
            fn into_repr(self) -> Self::Repr {
                match self {
                    #(#stru => #repr,)*
                    _ => unreachable!(),
                }
            }

            #[inline]
            fn from_repr(repr: Self::Repr) -> Self {
                match repr {
                    #(#repr => #stru,)*
                    _ => unreachable!(),
                }
            }

            #[inline]
            fn as_repr(&self) -> <Self::Repr as #path::Repr>::Ref<'_> {
                match self {
                    #(#stru => #repr,)*
                    _ => unreachable!(),
                }
            }

            #[inline]
            fn as_mut_repr(&mut self) -> <Self::Repr as #path::Repr>::Mut<'_> {
                match self {
                    #(#stru => #repr,)*
                    _ => unreachable!(),
                }
            }
        }
    }
    .into()
}

/// 在 struct 或 enum 上实现 `Generic`
#[proc_macro_derive(Generic)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    m(&quote!(::algtype), input)
}

/// 用于库的内部实现
#[proc_macro]
#[doc(hidden)]
pub fn impl_generic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    m(&quote!(crate), input)
}
