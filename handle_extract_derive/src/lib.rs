extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HandleVisit)]
pub fn handle_visit(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data) => struct_impls(name, data),
        syn::Data::Enum(data) => enum_impls(name, data),
        _ => unimplemented!(),
    }
}

fn struct_impls(name: &syn::Ident, data: &syn::DataStruct) -> TokenStream {
    let accessors: Vec<syn::Ident> = match &data.fields {
        // prost should only generate structs with named fields, so we only consider this case.
        syn::Fields::Named(fields) => fields.named.iter().flat_map(|f| f.ident.clone()).collect(),
        _ => unimplemented!(),
    };

    let gen = quote! {
        impl ::handle_extract::HandleVisit for #name {
            fn visit<F: FnMut(&mut ::handle_extract::Handle)>(&mut self, visitor: F) -> F {
                let mut _v = visitor;
                #(
                    _v  = self.#accessors.visit(_v);
                )*
                _v
            }
        }
    };
    gen.into()
}

fn enum_impls(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variants: Vec<syn::Ident> = data
        .variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                // Prost should only generate a single unnamed field, check to make sure.
                syn::Fields::Unnamed(fields) => assert_eq!(fields.unnamed.len(), 1),
                _ => unimplemented!(),
            }
            variant.ident.clone()
        })
        .collect();

    let gen = quote! {
        impl ::handle_extract::HandleVisit for #name {
            fn visit<F: FnMut(&mut ::handle_extract::Handle)>(&mut self, visitor: F) -> F {
                match self {
                    #(
                        #name::#variants(v) => v.visit(visitor),
                    )*
                }
            }
        }
    };
    gen.into()
}
