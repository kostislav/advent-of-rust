use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, GenericParam, Generics, LitStr, Meta, MetaList, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(ParseYolo, attributes(pattern, separator))]
pub fn parse_yolo_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => derive_struct(&input, data),
        Data::Enum(data) => derive_enum(&input, data),
        Data::Union(_) => panic!("Unions not supported"),
    }
}

fn derive_struct(input: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let pattern = get_pattern(&input.attrs).unwrap();
    if let Fields::Named(fields) = &struct_data.fields {
        let body = pattern_parsing_body(&pattern, fields.named.iter(), quote!(Self));
        generate_impl(&input.ident, &input.generics, body)
    } else {
        syn::Error::new(input.span(), "Only named fields are currently supported").to_compile_error().into()
    }
}

fn derive_enum(input: &DeriveInput, struct_data: &DataEnum) -> TokenStream {
    let mut body = Vec::new();
    for variant in &struct_data.variants {
        let pattern = get_pattern(&variant.attrs);
        if !body.is_empty() {
            body.push(quote!(else));
        }
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(_) => panic!("Named struct fields not supported"),
            Fields::Unnamed(unnamed) => {
                if let Some(pattern) = pattern {
                    let lambda_body = pattern_parsing_body(&pattern, unnamed.unnamed.iter(), quote!(Self::#variant_name));
                    body.push(quote!(if let Ok(x) = stream.try_parse(|stream| { #(#lambda_body)* }) { Ok(x) }));
                } else {
                    if unnamed.unnamed.len() == 1 {
                        body.push(quote!(if let Ok(x) = stream.parse_yolo() { Ok(Self::#variant_name(x)) }))
                    } else {
                        panic!("Enum variants with multiple fields require a pattern")
                    }
                }
            }
            Fields::Unit => {
                let resolved_pattern = pattern.unwrap_or_else(|| variant.ident.to_string().to_lowercase());
                body.push(quote!(if stream.try_consume(#resolved_pattern) { Ok(Self::#variant_name) }))
            }
        };
    }
    body.push(quote!(else { Err(()) }));

    generate_impl(&input.ident, &input.generics, body)
}

fn generate_impl(target_name: &Ident, generics: &Generics, body: Vec<proc_macro2::TokenStream>) -> TokenStream {
    let lifetime_params: Vec<_> = generics.params.iter()
        .filter_map(|generic| if let GenericParam::Lifetime(lifetime_generic) = generic {
            Some(lifetime_generic)
        } else {
            None
        })
        .collect();
    let (impl_lifetime, lifetime_params) = if lifetime_params.is_empty() {
        (quote!(<'_>), quote!())
    } else {
        (quote!(<#(#lifetime_params)*>), quote!(<#(#lifetime_params)*>))
    };
    let gen = quote! {
        impl #lifetime_params crate::input::ParseYolo #impl_lifetime for #target_name #lifetime_params {
            fn parse_from_stream(stream: &mut crate::input::ParseStream #lifetime_params) -> Result<Self, ()> {
                #(#body)*
            }
        }
    };
    gen.into()
}

fn pattern_parsing_body<'a, I: Iterator<Item=&'a Field>>(pattern: &str, mut fields: I, constructor: proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut body = Vec::new();
    let mut field_names = Vec::new();
    let mut named = false;
    for (i, part) in split_pattern(&pattern).into_iter().enumerate() {
        if part == "{}" {
            let field = fields.next().unwrap();
            let field_name = if let Some(name) = &field.ident {
                named = true;
                name.clone()
            } else {
                Ident::new(&format!("x{}", i), field.span())
            };
            let separator = get_attr(&field.attrs, "separator");
            let function_call = if let Some(separator) = separator {
                quote!(stream.parse_separated(#separator))
            } else {
                quote!(stream.parse_yolo())
            };
            body.push(quote!(let #field_name = #function_call?;));
            field_names.push(field_name);
        } else {
            body.push(quote!(stream.expect(#part)?;));
        }
    }
    if named {
        body.push(quote!(Ok(#constructor { #(#field_names, )* })));
    } else {
        body.push(quote!(Ok(#constructor( #(#field_names, )* ))));
    }
    body
}

fn split_pattern(pattern: &str) -> Vec<&str> {
    let mut result = Vec::new();
    for part in pattern.split_inclusive("{}") {
        if part.ends_with("{}") {
            if part.len() > 2 {
                result.push(&part[0..part.len() - 2]);
            }
            result.push("{}");
        } else {
            result.push(part);
        }
    }
    result
}

fn get_pattern(attrs: &[Attribute]) -> Option<String> {
    get_attr(attrs, "pattern")
}

fn get_attr(attrs: &[Attribute], name: &str) -> Option<String> {
    attrs.iter()
        .filter_map(|attribute|
            if let Meta::List(MetaList { path, .. }) = &attribute.meta {
                if path.is_ident(name) {
                    let pattern: LitStr = attribute.parse_args().ok()?;
                    Some(pattern.value())
                } else {
                    None
                }
            } else {
                None
            }
        )
        .next()
}