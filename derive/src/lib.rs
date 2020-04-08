use proc_macro::TokenStream;
use quote::quote;

mod derive;

#[proc_macro_attribute]
pub fn ocaml_bare_func(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();

    let name = &item_fn.sig.ident;
    let lt = &item_fn.sig.generics.lt_token;
    let generic_params = &item_fn.sig.generics.params;
    let gt = &item_fn.sig.generics.gt_token;
    let where_clause = &item_fn.sig.generics.where_clause;
    let attr: Vec<_> = item_fn
        .attrs
        .iter()
        .filter(|x| {
            let seg: Vec<_> = x
                .path
                .segments
                .iter()
                .map(|x| format!("{}", x.ident))
                .collect();
            let s = seg.join("::");
            s != "ocaml_func" && s != "ocaml::func" && s != "func"
        })
        .collect();

    match item_fn.vis {
        syn::Visibility::Public(_) => (),
        _ => panic!("OCaml functions must be public"),
    }

    item_fn.sig.abi = Some(syn::Abi {
        extern_token: syn::token::Extern::default(),
        name: Some(syn::LitStr::new("C", item_fn.sig.ident.span())),
    });

    let rust_args = &item_fn.sig.inputs;

    if rust_args.len() > 5 {
        panic!("OCaml functions must have 5 or fewer arguments");
    }

    let args: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("OCaml functions cannot take a self argument"),
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => Some(ident),
                _ => None,
            },
        })
        .collect();

    let mut ocaml_args: Vec<_> = args
        .iter()
        .map(|t| match t {
            Some(ident) => quote! { mut #ident: ::ocaml::Value },
            None => quote! { _: ::ocaml::Value },
        })
        .collect();

    let param_inner_values: Vec<_> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => {
                let ident = ident.ident.clone();
                Some(quote! {#ident})
            }
            None => None,
        })
        .collect();

    if ocaml_args.len() == 0 {
        ocaml_args.push(quote! { _: ::ocaml::Value});
    }

    let body = &item_fn.block;

    let gen = quote! {
        #[no_mangle]
        #(
            #attr
        )*
        pub extern "C" fn #name #lt #generic_params #gt (#(#ocaml_args),*) -> ::ocaml::Value #where_clause {
            ::ocaml::body!((#(#param_inner_values),*) {
                #body
            })
        }
    };
    gen.into()
}

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

    let rust_args: Vec<_> = item_fn.sig.inputs.iter().collect();

    if rust_args.len() > 5 {
        panic!("OCaml functions must have 5 or fewer arguments");
    }

    let args: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("OCaml functions cannot take a self argument"),
            syn::FnArg::Typed(t) => match t.pat.as_ref() {
                syn::Pat::Ident(ident) => Some(ident),
                _ => None,
            },
        })
        .collect();

    let mut ocaml_args: Vec<_> = args
        .iter()
        .map(|t| match t {
            Some(ident) => {
                let ident = &ident.ident;
                quote! { mut #ident: ::ocaml::Value }
            }
            None => quote! { _: ::ocaml::Value },
        })
        .collect();

    let param_inner_values: Vec<_> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => {
                let ident = &ident.ident;
                Some(quote! {#ident})
            }
            None => None,
        })
        .collect();

    let param_names: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => Some(ident.ident.clone()),
            None => None,
        })
        .collect();

    let convert_params: Vec<_> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => {
                let ident = ident.ident.clone();
                Some(quote! { let mut #ident = ::ocaml::FromValue::from_value(#ident); })
            }
            None => None,
        })
        .collect();

    if ocaml_args.len() == 0 {
        ocaml_args.push(quote! { _: ::ocaml::Value});
    }

    let body = &item_fn.block;

    let inner = if returns {
        quote! {
            #[inline(always)]
            fn inner(#(#rust_args),*) -> #rust_return_type {
                #body
            }
        }
    } else {
        quote! {
            #[inline(always)]
            fn inner(#(#rust_args),*)  {
                #body
            }
        }
    };

    let gen = quote! {
        #[no_mangle]
        pub extern "C" fn #name(#(#ocaml_args),*) -> ::ocaml::Value {
            ::ocaml::body!((#(#param_inner_values),*) {
                #inner
                #(#convert_params);*
                let res = inner(#param_names);
                ::ocaml::ToValue::to_value(&res)
            })
        }
    };
    gen.into()
}

synstructure::decl_derive!([ToValue, attributes(ocaml)] => derive::tovalue_derive);
synstructure::decl_derive!([FromValue, attributes(ocaml)] => derive::fromvalue_derive);
