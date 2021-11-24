use proc_macro::TokenStream;
use quote::*;
use syn::PathArguments;
use syn::{parse_macro_input, parse_quote, GenericParam, ItemImpl, Type};

pub fn methods(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut parsed = parse_macro_input!(input as ItemImpl);

    let struct_name = &parsed.self_ty;

    let inner = parsed.items;

    let param_s: GenericParam = parse_quote!(S: abcf::bs3::Store);
    parsed.generics.params.push(param_s);
    let param_d: GenericParam = parse_quote!(D: abcf::digest::Digest + core::marker::Sync + core::marker::Send);
    parsed.generics.params.push(param_d);

    let mut generics_names = Vec::new();
    let mut lifetime_names = Vec::new();

    for x in &parsed.generics.params {
        if let GenericParam::Type(t) = x {
            generics_names.push(t.ident.clone());
        } else if let GenericParam::Lifetime(l) = x {
            lifetime_names.push(l.lifetime.clone());
        }
    }

    let mut pre_app: ItemImpl = parse_quote! {
        impl #struct_name {
            #(
                #inner
            )*
        }
    };

    if let Type::Path(p) = parsed.self_ty.as_mut() {
        let segments = &mut p.path.segments;
        let arguments = &mut segments.last_mut().unwrap().arguments;
        if let PathArguments::AngleBracketed(a) = arguments {
            a.args.push(parse_quote!(S));
            a.args.push(parse_quote!(D));
        } else {
            *arguments = PathArguments::AngleBracketed(parse_quote!(<S, D>));
        }
    }

    pre_app.generics = parsed.generics.clone();
    pre_app.self_ty = parsed.self_ty.clone();

    let result = quote! {
        #pre_app
    };

    TokenStream::from(result)
}
