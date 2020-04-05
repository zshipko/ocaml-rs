use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn ocaml_func(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();

    let name = &item_fn.sig.ident;

    match item_fn.vis {
        syn::Visibility::Public(_) => (),
        _ => panic!("OCaml functions must be public"),
    }

    item_fn.sig.abi = Some(syn::Abi {
        extern_token: syn::token::Extern::default(),
        name: Some(syn::LitStr::new("C", item_fn.sig.ident.span())),
    });

    let (returns, rust_return_type) = match &item_fn.sig.output {
        syn::ReturnType::Default => (false, None),
        syn::ReturnType::Type(_, t) => (true, Some(t)),
    };

    let rust_args = &item_fn.sig.inputs;

    if rust_args.len() > 5 {
        panic!("OCaml functions must have 5 or fewer arguments");
    }

    let mut ocaml_args: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("OCaml functions cannot take a self argument"),
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => quote! { mut #ident: ::ocaml::Value },
                _ => quote! { _: ::ocaml::Value },
            },
        })
        .collect();

    let param_inner_values: Vec<_> = rust_args
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => {
                    let ident = ident.ident.clone();
                    quote! { #ident.0 }
                }
                _ => panic!("OCaml function parameters must be named"),
            },
            _ => panic!("OCaml function parameters must be named"),
        })
        .collect();

    let param_names: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> = rust_args
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => panic!("OCaml function parameters must be named"),
            },
            _ => panic!("OCaml function parameters must be named"),
        })
        .collect();

    let convert_params: Vec<_> = rust_args
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => {
                    let ident = ident.ident.clone();
                    quote! { let #ident = ::ocaml::FromValue::from_value(#ident);  }
                }
                _ => panic!("OCaml function parameters must be named"),
            },
            _ => panic!("OCaml function parameters must be named"),
        })
        .collect();

    if ocaml_args.len() == 0 {
        ocaml_args.push(quote! { _: ::ocaml::Value});
    }

    let body = &item_fn.block;

    let inner = if returns {
        quote! {
            #[inline(always)]
            fn inner(#rust_args) -> #rust_return_type {
                #body
            }
        }
    } else {
        quote! {
            #[inline(always)]
            fn inner(#rust_args)  {
                #body
            }
        }
    };

    let gen = quote! {
        #[no_mangle]
        pub extern "C" fn #name(#(#ocaml_args),*) -> ::ocaml::Value {
            ::ocaml::sys::caml_body!((#(#param_inner_values),*) {
                #inner
                #(#convert_params);*
                let res = inner(#param_names);
                ::ocaml::ToValue::to_value(&res)
            })
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
