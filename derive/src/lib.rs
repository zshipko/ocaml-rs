use proc_macro::TokenStream;
use quote::quote;

mod derive;

fn check_func(item_fn: &mut syn::ItemFn) {
    if item_fn.sig.asyncness.is_some() {
        panic!("OCaml functions cannot be async");
    }

    if item_fn.sig.variadic.is_some() {
        panic!("OCaml functions cannot be variadic");
    }

    match item_fn.vis {
        syn::Visibility::Public(_) => (),
        _ => panic!("OCaml functions must be public"),
    }

    if !item_fn.sig.generics.params.is_empty() {
        panic!("OCaml functions may not contain generics")
    }

    item_fn.sig.abi = Some(syn::Abi {
        extern_token: syn::token::Extern::default(),
        name: Some(syn::LitStr::new("C", item_fn.sig.ident.span())),
    });
}

/// `func` is used export Rust functions to OCaml, performing the necessary wrapping/unwrapping
/// automatically.
///
/// - Wraps the function body using `ocaml::body`
/// - Automatic type conversion for arguments/return value (including Result types)
/// - Defines a bytecode function automatically for functions that take more than 5 arguments. The
/// bytecode function for `my_func` would be `my_func_bytecode`
#[proc_macro_attribute]
pub fn ocaml_func(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();
    check_func(&mut item_fn);

    let name = &item_fn.sig.ident;
    let unsafety = &item_fn.sig.unsafety;
    let constness = &item_fn.sig.constness;

    let (returns, rust_return_type) = match &item_fn.sig.output {
        syn::ReturnType::Default => (false, None),
        syn::ReturnType::Type(_, t) => (true, Some(t)),
    };

    let rust_args: Vec<_> = item_fn.sig.inputs.iter().collect();

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
                quote! { mut #ident: ocaml::Value }
            }
            None => quote! { _: ocaml::Value },
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
                Some(quote! { let mut #ident = ocaml::FromValue::from_value(#ident); })
            }
            None => None,
        })
        .collect();

    if ocaml_args.is_empty() {
        ocaml_args.push(quote! { _: ocaml::Value});
    }

    let body = &item_fn.block;

    let inner = if returns {
        quote! {
            #[inline(always)]
            #constness #unsafety fn inner(#(#rust_args),*) -> #rust_return_type {
                #body
            }
        }
    } else {
        quote! {
            #[inline(always)]
            #constness #unsafety fn inner(#(#rust_args),*)  {
                #body
            }
        }
    };

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
            if s == "ocaml::bytecode_func"
                || s == "ocaml::native_func"
                || s == "ocaml_bytecode_func"
                || s == "ocaml_native_func"
                || s == "bytecode_func"
                || s == "native_func"
            {
                panic!("Cannot mix OCaml function macros");
            }
            s != "ocaml_func" && s != "ocaml::func" && s != "func"
        })
        .collect();

    let gen = quote! {
        #[no_mangle]
        #(
            #attr
        )*
        pub #constness #unsafety extern "C" fn #name(#(#ocaml_args),*) -> ocaml::Value #where_clause {
            ocaml::body!((#param_names) {
                #inner
                #(#convert_params);*
                let res = inner(#param_names);
                ocaml::ToValue::to_value(res)
            })
        }
    };

    if ocaml_args.len() > 5 {
        let bytecode = {
            let mut bc = item_fn.clone();
            bc.sig.ident = syn::Ident::new(&format!("{}_bytecode", name), name.span());
            ocaml_bytecode_func_impl(bc, Some(name))
        };

        let r = quote! {
            #gen

            #bytecode
        };
        return r.into();
    }

    gen.into()
}

/// `native_func` is used export Rust functions to OCaml, it has much lower overhead than `func`
/// and expects all arguments and return type to to be `Value`.
///
/// - Wraps the function body using `ocaml::body`
#[proc_macro_attribute]
pub fn ocaml_native_func(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();
    check_func(&mut item_fn);

    let name = &item_fn.sig.ident;
    let unsafety = &item_fn.sig.unsafety;
    let constness = &item_fn.sig.constness;

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
            if s == "ocaml::bytecode_func"
                || s == "ocaml::func"
                || s == "ocaml_bytecode_func"
                || s == "ocaml_func"
                || s == "bytecode_func"
                || s == "func"
            {
                panic!("Cannot mix OCaml function macros");
            }
            s != "ocaml_native_func" && s != "ocaml::native_func" && s != "native_func"
        })
        .collect();

    let rust_args = &item_fn.sig.inputs;

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
            Some(ident) => quote! { mut #ident: ocaml::Value },
            None => quote! { _: ocaml::Value },
        })
        .collect();

    let param_names: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => Some(ident.ident.clone()),
            None => None,
        })
        .collect();

    if ocaml_args.is_empty() {
        ocaml_args.push(quote! { _: ocaml::Value});
    }

    let body = &item_fn.block;

    let (_, rust_return_type) = match &item_fn.sig.output {
        syn::ReturnType::Default => (false, None),
        syn::ReturnType::Type(_, t) => (true, Some(t)),
    };

    let gen = quote! {
        #[no_mangle]
        #(
            #attr
        )*
        pub #constness #unsafety extern "C" fn #name (#rust_args) -> #rust_return_type #where_clause {
            ocaml::body!((#param_names) {
                #body
            })
        }
    };
    gen.into()
}

/// `bytecode_func` is used export Rust functions to OCaml, performing the necessary wrapping/unwrapping
/// automatically.
///
/// Since this is automatically applied to `func` functions, this is primarily be used when working with
/// unboxed functions, or `native_func`s directly. `ocaml::body` is not applied since this is
/// typically used to call the native function, which is wrapped with `ocaml::body` or performs the
/// equivalent work to register values with the garbage collector
///
/// - Automatic type conversion for arguments/return value
#[proc_macro_attribute]
pub fn ocaml_bytecode_func(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: syn::ItemFn = syn::parse(item).unwrap();
    ocaml_bytecode_func_impl(item_fn, None).into()
}

fn ocaml_bytecode_func_impl(
    mut item_fn: syn::ItemFn,
    original: Option<&proc_macro2::Ident>,
) -> proc_macro2::TokenStream {
    check_func(&mut item_fn);

    let name = &item_fn.sig.ident;
    let unsafety = &item_fn.sig.unsafety;
    let constness = &item_fn.sig.constness;

    let (returns, rust_return_type) = match &item_fn.sig.output {
        syn::ReturnType::Default => (false, None),
        syn::ReturnType::Type(_, t) => (true, Some(t)),
    };

    let rust_args: Vec<_> = item_fn.sig.inputs.iter().collect();

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
            Some(ident) => quote! { #ident: ocaml::Value },
            None => quote! { _: ocaml::Value },
        })
        .collect();

    let mut param_names: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> = args
        .iter()
        .filter_map(|arg| match arg {
            Some(ident) => Some(ident.ident.clone()),
            None => None,
        })
        .collect();

    if ocaml_args.is_empty() {
        ocaml_args.push(quote! { _unit: ocaml::Value});
        param_names.push(syn::Ident::new("__ocaml_unit", name.span()));
    }

    let body = &item_fn.block;

    let inner = match original {
        Some(o) => {
            quote! {
                #[allow(unused)]
                let __ocaml_unit = ocaml::Value::unit();
                let inner = #o;
            }
        }
        None => {
            if returns {
                quote! {
                    #[inline(always)]
                    #constness #unsafety fn inner(#(#rust_args),*) -> #rust_return_type {
                        #body
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #constness #unsafety fn inner(#(#rust_args),*)  {
                        #body
                    }
                }
            }
        }
    };

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
            if s == "ocaml::func"
                || s == "ocaml::native_func"
                || s == "ocaml_func"
                || s == "ocaml_native_func"
                || s == "func"
                || s == "native_func"
            {
                panic!("Cannot mix OCaml function macros");
            }
            s != "ocaml_bytecode_func"
                && s != "ocaml::bytecode_func"
                && s != "bytecode_func"
                && s != "ocaml_func"
                && s != "ocaml::func"
                && s != "func"
        })
        .collect();

    let len = rust_args.len();

    if len > 5 {
        let convert_params: Vec<_> = args
            .iter()
            .filter_map(|arg| match arg {
                Some(ident) => Some(quote! {
                    #[allow(clippy::not_unsafe_ptr_arg_deref)]
                    let mut #ident = ocaml::FromValue::from_value(unsafe {
                        std::ptr::read(__ocaml_argv.add(__ocaml_arg_index as usize))
                    });
                    __ocaml_arg_index += 1 ;
                }),
                None => None,
            })
            .collect();
        quote! {
            #[no_mangle]
            #(
                #attr
            )*
            pub #constness #unsafety extern "C" fn #name(__ocaml_argv: *mut ocaml::Value, __ocaml_argc: i32) -> ocaml::Value #where_clause {
                assert!(#len == __ocaml_argc as usize);

                #inner

                let mut __ocaml_arg_index = 0;
                #(#convert_params);*
                let res = inner(#param_names);
                ocaml::ToValue::to_value(res)
            }
        }
    } else {
        let convert_params: Vec<_> = args
            .iter()
            .filter_map(|arg| match arg {
                Some(ident) => {
                    let ident = ident.ident.clone();
                    Some(quote! { let mut #ident = ocaml::FromValue::from_value(#ident); })
                }
                None => None,
            })
            .collect();
        quote! {
            #[no_mangle]
            #(
                #attr
            )*
            pub #constness #unsafety extern "C" fn #name(#(#ocaml_args),*) -> ocaml::Value #where_clause {
                #inner

                #(#convert_params);*
                let res = inner(#param_names);
                ocaml::ToValue::to_value(res)
            }
        }
    }
}

synstructure::decl_derive!([ToValue, attributes(ocaml)] => derive::tovalue_derive);
synstructure::decl_derive!([FromValue, attributes(ocaml)] => derive::fromvalue_derive);
