#![allow(clippy::manual_map)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

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

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Func,
    Struct,
    Enum,
    Type,
}

#[proc_macro_attribute]
pub fn ocaml_sig(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let (name, mode, n) = if let Ok(item) = syn::parse::<syn::ItemStruct>(item.clone()) {
        let name = &item.ident;
        let n_fields = match item.fields {
            syn::Fields::Named(x) => x.named.iter().count(),
            syn::Fields::Unit => 0,
            syn::Fields::Unnamed(x) => x.unnamed.iter().count(),
        };
        (name.to_string().to_lowercase(), Mode::Struct, n_fields)
    } else if let Ok(item) = syn::parse::<syn::ItemEnum>(item.clone()) {
        let name = &item.ident;
        let n = item.variants.iter().count();
        (name.to_string().to_lowercase(), Mode::Enum, n)
    } else if let Ok(item_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let name = &item_fn.sig.ident;
        let n_args = item_fn.sig.inputs.iter().count();
        (name.to_string(), Mode::Func, n_args)
    } else if let Ok(item) = syn::parse::<syn::ItemType>(item.clone()) {
        let name = &item.ident;
        (name.to_string(), Mode::Type, 0)
    } else {
        panic!("Invalid use of ocaml::sig macro: {item}")
    };

    if attribute.is_empty() && mode != Mode::Func {
        // Ok
    } else if let Ok(sig) = syn::parse::<syn::LitStr>(attribute) {
        let s = sig.value();
        match mode {
            Mode::Func => {
                let mut n_args = 0;
                let mut prev = None;
                let mut paren_level = 0;
                let iter = s.chars();
                for ch in iter {
                    if ch == '(' {
                        paren_level += 1;
                    } else if ch == ')' {
                        paren_level -= 1;
                    }

                    if ch == '>' && prev == Some('-') && paren_level == 0 {
                        n_args += 1;
                    }

                    prev = Some(ch);
                }

                if n == 0 && !s.trim().starts_with("unit") {
                    panic!("{name}: Expected a single unit argument");
                }

                if n != n_args && (n == 0 && n_args > 1) {
                    panic!(
                        "{name}: Signature and function do not have the same number of arguments (expected: {n}, got {n_args})"
                    );
                }
            }
            Mode::Enum => {
                if !s.is_empty() {
                    let mut n_variants = 1;
                    let mut bracket_level = 0;
                    let iter = s.chars();
                    for ch in iter {
                        if ch == '[' {
                            bracket_level += 1;
                        } else if ch == ']' {
                            bracket_level -= 1;
                        }

                        if ch == '|' && bracket_level == 0 {
                            n_variants += 1;
                        }
                    }
                    if s.trim().starts_with('|') {
                        n_variants -= 1;
                    }
                    if n != n_variants {
                        panic!("{name}: Signature and enum do not have the same number of variants (expected: {n}, got {n_variants})")
                    }
                }
            }
            Mode::Struct => {
                if !s.is_empty() {
                    let n_fields = s.matches(':').count();
                    if n != n_fields {
                        panic!("{name}: Signature and struct do not have the same number of fields (expected: {n}, got {n_fields})")
                    }
                }
            }
            Mode::Type => {}
        }
    } else {
        panic!("OCaml sig accepts a str literal");
    }

    item
}

/// `func` is used export Rust functions to OCaml, performing the necessary wrapping/unwrapping
/// automatically.
///
/// - Wraps the function body using `ocaml::body`
/// - Automatic type conversion for arguments/return value (including Result types)
/// - Defines a bytecode function automatically for functions that take more than 5 arguments. The
/// bytecode function for `my_func` would be `my_func_bytecode`
/// - Allows for an optional ident argument specifying the name of the `gc` handle parameter
#[proc_macro_attribute]
pub fn ocaml_func(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();
    check_func(&mut item_fn);

    let name = &item_fn.sig.ident;
    let unsafety = &item_fn.sig.unsafety;
    let constness = &item_fn.sig.constness;
    let mut gc_name = syn::Ident::new("gc", name.span());
    let mut use_gc = quote!({let _ = &#gc_name;});

    if let Ok(ident) = syn::parse::<syn::Ident>(attribute) {
        gc_name = ident;
        use_gc = quote!();
    }

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
                quote! { #ident: ocaml::Raw }
            }
            None => quote! { _: ocaml::Raw },
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
                Some(quote! { let #ident = ocaml::FromValue::from_value(unsafe { ocaml::Value::new(#ident) }); })
            }
            None => None,
        })
        .collect();

    if ocaml_args.is_empty() {
        ocaml_args.push(quote! { _: ocaml::Raw});
    }

    let body = &item_fn.block;

    let inner = if returns {
        quote! {
            #[inline(always)]
            #constness #unsafety fn inner(#gc_name: &mut ocaml::Runtime, #(#rust_args),*) -> #rust_return_type {
                #use_gc
                #body
            }
        }
    } else {
        quote! {
            #[inline(always)]
            #constness #unsafety fn inner(#gc_name: &mut ocaml::Runtime, #(#rust_args),*)  {
                #use_gc
                #body
            }
        }
    };

    let where_clause = &item_fn.sig.generics.where_clause;
    let attr: Vec<_> = item_fn.attrs.iter().collect();

    let gen = quote! {
        #[no_mangle]
        #(
            #attr
        )*
        pub #constness #unsafety extern "C" fn #name(#(#ocaml_args),*) -> ocaml::Raw #where_clause {
            #inner

            ocaml::body!(#gc_name: {
                #(#convert_params);*
                let res = inner(#gc_name, #param_names);
                #[allow(unused_unsafe)]
                let mut gc_ = unsafe { ocaml::Runtime::recover_handle() };
                unsafe { ocaml::ToValue::to_value(&res, &gc_).raw() }
            })
        }
    };

    if ocaml_args.len() > 5 {
        let bytecode = {
            let mut bc = item_fn.clone();
            bc.attrs.retain(|x| {
                let s = x
                    .path
                    .segments
                    .iter()
                    .map(|x| x.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                s != "ocaml::sig" && s != "sig"
            });
            bc.sig.ident = syn::Ident::new(&format!("{}_bytecode", name), name.span());
            ocaml_bytecode_func_impl(bc, gc_name, use_gc, Some(name))
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
/// - Allows for an optional ident argument specifying the name of the `gc` handle parameter
#[proc_macro_attribute]
pub fn ocaml_native_func(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = syn::parse(item).unwrap();
    check_func(&mut item_fn);

    let name = &item_fn.sig.ident;
    let unsafety = &item_fn.sig.unsafety;
    let constness = &item_fn.sig.constness;

    let mut gc_name = syn::Ident::new("gc", name.span());
    let mut use_gc = quote!({let _ = &#gc_name;});
    if let Ok(ident) = syn::parse::<syn::Ident>(attribute) {
        gc_name = ident;
        use_gc = quote!();
    }

    let where_clause = &item_fn.sig.generics.where_clause;
    let attr: Vec<_> = item_fn.attrs.iter().collect();

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
            Some(ident) => quote! { #ident: ocaml::Raw },
            None => quote! { _: ocaml::Raw },
        })
        .collect();

    if ocaml_args.is_empty() {
        ocaml_args.push(quote! { _: ocaml::Raw});
    }

    let body = &item_fn.block;

    let (_, rust_return_type) = match &item_fn.sig.output {
        syn::ReturnType::Default => (false, None),
        syn::ReturnType::Type(_, _t) => (true, Some(quote! {ocaml::Raw})),
    };

    let gen = quote! {
        #[no_mangle]
        #(
            #attr
        )*
        pub #constness #unsafety extern "C" fn #name (#rust_args) -> #rust_return_type #where_clause {
            let r = ocaml::body!(#gc_name: {
                #use_gc
                #body
            });
            r.raw()
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
/// - Allows for an optional ident argument specifying the name of the `gc` handle parameter
#[proc_macro_attribute]
pub fn ocaml_bytecode_func(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: syn::ItemFn = syn::parse(item).unwrap();
    let mut gc_name = syn::Ident::new("gc", item_fn.sig.ident.span());
    let mut use_gc = quote!({let _ = &#gc_name;});
    if let Ok(ident) = syn::parse::<syn::Ident>(attribute) {
        gc_name = ident;
        use_gc = quote!();
    }
    ocaml_bytecode_func_impl(item_fn, gc_name, use_gc, None).into()
}

fn ocaml_bytecode_func_impl(
    mut item_fn: syn::ItemFn,
    gc_name: syn::Ident,
    use_gc: impl quote::ToTokens,
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
        .clone()
        .into_iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("OCaml functions cannot take a self argument"),
            syn::FnArg::Typed(mut t) => match t.pat.as_mut() {
                syn::Pat::Ident(ident) => {
                    ident.mutability = None;
                    Some(ident.clone())
                }
                _ => None,
            },
        })
        .collect();

    let mut ocaml_args: Vec<_> = args
        .iter()
        .map(|t| match t {
            Some(ident) => {
                quote! { #ident: ocaml::Raw }
            }
            None => quote! { _: ocaml::Raw },
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
        ocaml_args.push(quote! { _unit: ocaml::Raw});
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
                        #[allow(unused_variables)]
                        let #gc_name = unsafe { ocaml::Runtime::recover_handle() };
                        #use_gc
                        #body
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #constness #unsafety fn inner(#(#rust_args),*)  {
                        #[allow(unused_variables)]
                        let #gc_name = unsafe { ocaml::Runtime::recover_handle() };
                        #use_gc
                        #body
                    }
                }
            }
        }
    };

    let where_clause = &item_fn.sig.generics.where_clause;
    let attr: Vec<_> = item_fn.attrs.iter().collect();

    let len = ocaml_args.len();

    if len > 5 {
        let convert_params: Vec<_> = args
            .iter()
            .filter_map(|arg| match arg {
                Some(ident) => Some(quote! {
                    #[allow(clippy::not_unsafe_ptr_arg_deref)]
                    let #ident = ocaml::FromValue::from_value(unsafe {
                        core::ptr::read(__ocaml_argv.add(__ocaml_arg_index as usize))
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
            pub #constness unsafe extern "C" fn #name(__ocaml_argv: *mut ocaml::Value, __ocaml_argc: i32) -> ocaml::Raw #where_clause {
                assert!(#len <= __ocaml_argc as usize, "len: {}, argc: {}", #len, __ocaml_argc);

                let #gc_name = unsafe { ocaml::Runtime::recover_handle() };

                #inner

                let mut __ocaml_arg_index = 0;
                #(#convert_params);*
                let res = inner(#param_names);
                ocaml::ToValue::to_value(&res, &#gc_name).raw()
            }
        }
    } else {
        let convert_params: Vec<_> = args
            .iter()
            .filter_map(|arg| match arg {
                Some(ident) => {
                    let ident = ident.ident.clone();
                    Some(quote! { let #ident = ocaml::FromValue::from_value(unsafe { ocaml::Value::new(#ident) }); })
                }
                None => None,
            })
            .collect();
        quote! {
            #[no_mangle]
            #(
                #attr
            )*
            pub #constness #unsafety extern "C" fn #name(#(#ocaml_args),*) -> ocaml::Raw #where_clause {
                #[allow(unused_variables)]
                let #gc_name = unsafe { ocaml::Runtime::recover_handle() };

                #inner

                #(#convert_params);*
                let res = inner(#param_names);
                ocaml::ToValue::to_value(&res, &#gc_name).raw()
            }
        }
    }
}

// Derive macros for ToValue/FromValue

fn is_double_array_struct(fields: &syn::Fields) -> bool {
    fields.iter().all(|field| match &field.ty {
        syn::Type::Path(p) => {
            let s = p.path.segments.iter().map(|x| x.ident.to_string()).fold(
                String::new(),
                |mut acc, x| {
                    if !acc.is_empty() {
                        acc += "::";
                        acc += &x;
                        acc
                    } else {
                        x
                    }
                },
            );
            s == "ocaml::Float" || s == "Float" || s == "f64" || s == "f32"
        }
        _ => false,
    })
}

#[derive(Default)]
struct Attrs {
    float_array: bool,
    unboxed: bool,
}

// Get struct-level attributes
fn attrs(attrs: &[syn::Attribute]) -> Attrs {
    let mut acc = Attrs::default();
    attrs.iter().for_each(|attr| {
        if let syn::Meta::Path(p) = attr.parse_meta().unwrap() {
            if let Some(ident) = p.get_ident() {
                if ident == "float_array" {
                    if acc.unboxed {
                        panic!("cannot use float_array and unboxed");
                    }
                    acc.float_array = true;
                } else if ident == "unboxed" {
                    if acc.float_array {
                        panic!("cannot use float_array and unboxed");
                    }
                    acc.unboxed = true;
                }
            }
        }
    });
    acc
}

/// Derive `ocaml::FromValue`
#[proc_macro_derive(FromValue, attributes(float_array, unboxed))]
pub fn derive_from_value(item: TokenStream) -> TokenStream {
    if let Ok(item_struct) = syn::parse::<syn::ItemStruct>(item.clone()) {
        let attrs = attrs(&item_struct.attrs);
        let g = item_struct.generics;
        let name = item_struct.ident;

        // Tuple structs have unnamed fields
        let tuple_struct = item_struct.fields.is_empty()
            || item_struct.fields.iter().take(1).all(|x| x.ident.is_none());

        // This is true when all struct fields are `float`s
        let is_double_array_struct =
            attrs.float_array || is_double_array_struct(&item_struct.fields);

        if attrs.unboxed && item_struct.fields.len() > 1 {
            panic!("cannot unbox structs with more than 1 field")
        }

        let fields =
            item_struct
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| match &field.ident {
                    Some(name) => {
                        // Named fields
                        if is_double_array_struct {
                            let ty = &field.ty;
                            quote!(#name: value.double_field(#index) as #ty)
                        } else if attrs.unboxed {
                            quote!(#name: ocaml::FromValue::from_value(value))
                        } else {
                            quote!(#name: ocaml::FromValue::from_value(value.field(#index)))
                        }
                    }
                    None => {
                        // Unnamed fields, tuple struct
                        if is_double_array_struct {
                            let ty = &field.ty;
                            quote!(value.double_field(#index) as #ty)
                        } else if attrs.unboxed {
                            quote!(ocaml::FromValue::from_value(value))
                        } else {
                            quote!(ocaml::FromValue::from_value(value.field(#index)))
                        }
                    }
                });

        let inner = if tuple_struct {
            quote!(Self(#(#fields),*))
        } else {
            quote!(Self{#(#fields),*})
        };

        let lt = g.lifetimes();
        let tp = g.type_params();
        let wh = &g.where_clause;

        // Generate FromValue for structs
        quote! {
            unsafe impl #g ocaml::FromValue for #name<#(#lt),* #(#tp),*> #wh {
                fn from_value(value: ocaml::Value) -> Self {
                    unsafe {
                        #inner
                    }
                }
            }
        }
        .into()
    } else if let Ok(item_enum) = syn::parse::<syn::ItemEnum>(item) {
        let g = item_enum.generics;
        let name = item_enum.ident;
        let attrs = attrs(&item_enum.attrs);
        let mut unit_tag = 0u8;
        let mut non_unit_tag = 0u8;
        if attrs.unboxed && item_enum.variants.len() > 1 {
            panic!("cannot unbox enums with more than 1 variant")
        }
        let variants =
            item_enum.variants.iter().map(|variant| {
                let arity = variant.fields.len();
                let is_block = arity != 0;
                let tag_ref = if arity > 0 {
                    &mut non_unit_tag
                } else {
                    &mut unit_tag
                };

                // Get current tag index
                let tag = *tag_ref;

                // Increment the tag for next time
                *tag_ref += 1;

                let v_name = &variant.ident;
                let n_fields = variant.fields.len();

                // Tuple enums have unnamed fields
                let tuple_enum = variant.fields.is_empty()
                    || variant.fields.iter().take(1).all(|x| x.ident.is_none());

                // Handle enums with no fields first
                if n_fields == 0 {
                    quote! {
                        (#is_block, #tag) => {
                            #name::#v_name
                        }
                    }
                } else {
                    let fields = variant.fields.iter().enumerate().map(
                        |(index, field)| match &field.ident {
                            Some(name) => {
                                // Struct enum variant
                                if attrs.unboxed {
                                    quote!(#name: ocaml::FromValue::from_value(value))
                                } else {
                                    quote!(#name: ocaml::FromValue::from_value(value.field(#index)))
                                }
                            }
                            None => {
                                // Tuple enum variant
                                if attrs.unboxed {
                                    quote!(#name: ocaml::FromValue::from_value(value))
                                } else {
                                    quote!(ocaml::FromValue::from_value(value.field(#index)))
                                }
                            }
                        },
                    );
                    let inner = if tuple_enum {
                        quote!(#name::#v_name(#(#fields),*))
                    } else {
                        quote!(#name::#v_name{#(#fields),*})
                    };

                    // Generate match case
                    quote! {
                        (#is_block, #tag) => {
                            #inner
                        }
                    }
                }
            });

        let lt = g.lifetimes();
        let tp = g.type_params();
        let wh = &g.where_clause;

        // Generate FromValue for enums
        quote! {
            unsafe impl #g ocaml::FromValue for #name<#(#lt),* #(#tp),*> #wh {
                fn from_value(value: ocaml::Value) -> Self {
                    unsafe {
                        let is_block = value.is_block();
                        let tag = if !is_block { value.int_val() as u8 } else { value.tag().0 as u8 };
                        match (is_block, tag) {
                            #(#variants),*,
                            _ => panic!("invalid variant, tag: {}", tag)
                        }
                    }
                }
           }
        }
        .into()
    } else {
        panic!("invalid type for FromValue");
    }
}

/// Derive `ocaml::ToValue`
#[proc_macro_derive(ToValue, attributes(float_array, unboxed))]
pub fn derive_to_value(item: TokenStream) -> TokenStream {
    if let Ok(item_struct) = syn::parse::<syn::ItemStruct>(item.clone()) {
        let attrs = attrs(&item_struct.attrs);
        let g = item_struct.generics;
        let name = item_struct.ident;

        // Double array structs occur when all fields are `float`s
        let is_double_array_struct =
            attrs.float_array || is_double_array_struct(&item_struct.fields);
        if attrs.unboxed && item_struct.fields.len() > 1 {
            panic!("cannot unbox structs with more than 1 field")
        }
        let fields: Vec<_> = item_struct
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| match &field.ident {
                Some(name) => {
                    // Named fields
                    if is_double_array_struct {
                        quote!(value.store_double_field(#index, self.#name as f64))
                    } else if attrs.unboxed {
                        quote!(value = self.#name.to_value(rt))
                    } else {
                        quote!(value.store_field(rt, #index, &self.#name))
                    }
                }
                None => {
                    // Tuple struct
                    if is_double_array_struct {
                        quote!(value.store_double_field(#index, self.#index as f64))
                    } else if attrs.unboxed {
                        quote!(value = self.#index.to_value(rt))
                    } else {
                        quote!(value.store_field(rt, #index, &self.#index))
                    }
                }
            })
            .collect();

        let tag = if is_double_array_struct {
            quote!(ocaml::Tag::DOUBLE_ARRAY)
        } else {
            quote!(0.into())
        };
        let n = fields.len();

        let lt = g.lifetimes();
        let tp = g.type_params();
        let wh = &g.where_clause;

        let value_decl = if attrs.unboxed {
            // Only allocate a singlue value for unboxed structs
            quote!(
                let mut value = ocaml::Value::unit();
            )
        } else {
            quote!(
                let mut value = ocaml::Value::alloc(#n, #tag);
            )
        };

        // Generate ToValue for structs
        quote! {
            unsafe impl #g ocaml::ToValue for #name<#(#lt),* #(#tp),*> #wh {
                fn to_value(&self, rt: &ocaml::Runtime) -> ocaml::Value {
                    unsafe {
                        #value_decl
                        #(#fields);*;
                        value
                    }
                }
            }
        }
        .into()
    } else if let Ok(item_enum) = syn::parse::<syn::ItemEnum>(item) {
        let g = item_enum.generics;
        let name = item_enum.ident;
        let attrs = attrs(&item_enum.attrs);
        let mut unit_tag = 0u8;
        let mut non_unit_tag = 0u8;

        if attrs.unboxed && item_enum.variants.len() != 1 {
            panic!("cannot unbox enums with more than 1 variant")
        }

        let variants = item_enum.variants.iter().map(|variant| {
            let arity = variant.fields.len();
            let tag_ref = if arity > 0 {
                &mut non_unit_tag
            } else {
                &mut unit_tag
            };

            // Get current tag and increment for next iteration
            let tag = *tag_ref;
            *tag_ref += 1;

            let v_name = &variant.ident;

            let n_fields = variant.fields.len();

            if n_fields == 0 {
                // A variant with no fields is represented by an int value
                quote! {
                    #name::#v_name => {
                        ocaml::Value::int(#tag as ocaml::Int)
                    }
                }
            } else {
                // Generate conversion for the fields of each variant
                let fields: Vec<_> = variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| match &field.ident {
                        Some(name) => {
                            // Struct-like variant
                            if attrs.unboxed {
                                quote!(value = #name.to_value(rt);)
                            } else {
                                quote!(value.store_field(rt, #index, #name))
                            }
                        }
                        None => {
                            // Tuple-like variant
                            let x = format!("x{index}");
                            let x = syn::Ident::new(&x, proc_macro2::Span::call_site());
                            if attrs.unboxed {
                                quote!(value = #x.to_value(rt);)
                            } else {
                                quote!(value.store_field(rt, #index, #x))
                            }
                        }
                    })
                    .collect();

                let n = variant.fields.len();
                let tuple_enum = variant.fields.is_empty()
                    || variant.fields.iter().take(1).all(|x| x.ident.is_none());

                // Generate fields
                let mut v = quote!();
                for (index, field) in variant.fields.iter().enumerate() {
                    let xindex = format!("x{index}");
                    let i = syn::Ident::new(&xindex, proc_macro2::Span::call_site());
                    let f_name = field.ident.as_ref().unwrap_or(&i);
                    if index == 0 {
                        v = quote!(#f_name)
                    } else {
                        v = quote!(#v, #f_name);
                    }
                }

                let match_fields = if tuple_enum {
                    quote!(#name::#v_name(#v))
                } else {
                    quote!(#name::#v_name{#v})
                };

                let value_decl = if attrs.unboxed {
                    quote!(let mut value = ocaml::Value::unit())
                } else {
                    quote!(
                        let mut value = ocaml::Value::alloc(#n, #tag.into());
                    )
                };
                quote!(#match_fields => {
                    #value_decl
                    #(#fields);*;
                    value
                })
            }
        });

        let lt = g.lifetimes();
        let tp = g.type_params();
        let wh = &g.where_clause;

        // Generate ToValue implementation for enums
        quote! {
            unsafe impl #g ocaml::ToValue for #name<#(#lt),* #(#tp),*> #wh {
                fn to_value(&self, rt: &ocaml::Runtime) -> ocaml::Value {
                    unsafe {
                        match self {
                            #(#variants),*,
                        }
                    }
                }
           }
        }
        .into()
    } else {
        panic!("invalid type for ToValue");
    }
}
