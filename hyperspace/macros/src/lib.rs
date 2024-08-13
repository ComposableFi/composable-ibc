// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::all)]

use proc_macro::{Span, TokenStream};
use proc_macro2::{Ident, TokenTree};
use quote::{quote, ToTokens};

use syn::{
	buffer::TokenBuffer, parse_macro_input, Attribute, Data, DeriveInput, Generics, ItemEnum, Meta,
	NestedMeta::Lit, Path, Type, TypePath,
};

#[proc_macro_attribute]
pub fn any_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(item as ItemEnum);
	input.variants = input
		.variants
		.into_iter()
		.map(|mut var| {
			var.attrs = var
				.attrs
				.into_iter()
				.map(|mut attr| {
					if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "error" {
						let mut meta = attr.parse_meta().unwrap();
						match &mut meta {
							Meta::List(ref mut list) => match &mut list.nested[0] {
								Lit(syn::Lit::Str(s)) => {
									let var_name = var.ident.to_string();
									let new_msg = format!("{var_name}{}", s.value());
									attr = syn::parse_quote!(#[error(#new_msg)]);
								},
								_ => {
									panic!("error attribute must be a string literal")
								},
							},
							_ => {
								panic!("error attribute must be a string literal")
							},
						}
						attr
					} else {
						attr
					}
				})
				.collect();
			var
		})
		.collect();
	input.to_token_stream().into()
}

#[proc_macro_derive(AnyError, attributes(error))]
pub fn derive_client_message(input: TokenStream) -> TokenStream {
	println!("input: \"{}\"", input.to_string());
	input
}
