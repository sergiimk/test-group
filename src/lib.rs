// Copyright Kamu Data, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

extern crate proc_macro2;

#[proc_macro_attribute]
pub fn group(
    args: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = proc_macro2::TokenStream::from(tokens);

    let mut groups: Vec<syn::Ident> =
        syn::parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated)
            .into_iter()
            .collect();

    use sha2::Digest;
    let digest = sha2::Sha256::digest(&tokens.to_string());
    let hash = hex::encode(digest);

    let mut body = tokens;

    while let Some(group) = groups.pop() {
        body = quote::quote! {
            mod #group {
                use super::*;
                #body
            }
        };
    }

    // Wrap into a module named after the test body hash to avoid module name
    // conflicts
    let modname = syn::Ident::new(&format!("g{}", &hash[0..4]), proc_macro2::Span::call_site());
    body = quote::quote! {
        mod #modname {
            use super::*;
            #body
        }
    };

    body.into()
}
