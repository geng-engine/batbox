#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro2::{Span, TokenStream};
use quote::quote;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    match DeriveInput::from_derive_input(&input) {
        Ok(input) => input.derive().into(),
        Err(e) => e.write_errors().into(),
    }
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
#[darling(attributes(diff))]
struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
    #[darling(default)]
    derive: syn::punctuated::Punctuated<syn::Path, syn::Token![,]>,
}

#[derive(FromField)]
#[darling(attributes(diff))]
struct Field {
    ident: Option<syn::Ident>,
    vis: syn::Visibility,
    ty: syn::Type,
    #[darling(default)]
    mode: DiffMode,
}

#[derive(FromMeta)]
enum DiffMode {
    Diff,
    Clone,
    Eq,
    Skip,
}

impl Default for DiffMode {
    fn default() -> Self {
        Self::Diff
    }
}

impl DeriveInput {
    fn derive(&self) -> TokenStream {
        let trait_path = quote! { Diff }; // TODO batbox::Diff or smth?
        let fields = match &self.data {
            darling::ast::Data::Struct(fields) => fields,
            _ => panic!("only structs supported"),
        };
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let delta_type = syn::Ident::new(
            &format!("{}Delta", self.ident),
            proc_macro2::Span::call_site(),
        );
        let field_names = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect::<Vec<_>>();
        let field_diff_types = fields.iter().map(|field| {
            let field_ty = &field.ty;
            match field.mode {
                DiffMode::Diff => quote! {
                    <#field_ty as #trait_path>::Delta
                },
                DiffMode::Clone => quote! {
                    #field_ty
                },
                DiffMode::Eq => quote! {
                    Option<#field_ty>
                },
                DiffMode::Skip => quote! { () },
            }
        });
        let field_diffs = fields.iter().map(|field| {
            let field_name = &field.ident;
            match field.mode {
                DiffMode::Diff => quote! {
                    #trait_path::diff(&self.#field_name, &to.#field_name)
                },
                DiffMode::Clone => quote! {
                    to.#field_name.clone()
                },
                DiffMode::Eq => quote! {
                    if self.#field_name == to.#field_name {
                        None
                    } else {
                        Some(to.#field_name.clone())
                    }
                },
                DiffMode::Skip => quote! { () },
            }
        });

        let field_updates = fields.iter().map(|field| {
            let field_name = &field.ident;
            match field.mode {
                DiffMode::Diff => quote! {
                    Diff::update(&mut self.#field_name, &delta.#field_name);
                },
                DiffMode::Clone => quote! {
                    self.#field_name = delta.#field_name.clone();
                },
                DiffMode::Eq => quote! {
                    if let Some(value) = &delta.#field_name {
                        self.#field_name = value.clone();
                    }
                },
                DiffMode::Skip => quote! {},
            }
        });
        let delta_derives = self.derive.iter();
        let input_type = &self.ident;
        quote! {
            #[derive(#(#delta_derives),*)]
            pub struct #delta_type #generics {
                #(#field_names: #field_diff_types,)*
            }

            impl #impl_generics Diff for #input_type #ty_generics #where_clause {
                type Delta = #delta_type;
                fn diff(&self, to: &Self) -> Self::Delta {
                    #delta_type {
                        #(#field_names: #field_diffs,)*
                    }
                }
                fn update(&mut self, delta: &Self::Delta) {
                    #(#field_updates)*
                }
            }
        }
    }
}
