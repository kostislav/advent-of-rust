use proc_macro::TokenStream;

use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, GenericParam, LitStr, Meta, MetaList, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(ParseYolo, attributes(pattern))]
pub fn parse_yolo_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => derive_struct(&input, data),
        Data::Enum(_) => todo!(),
        Data::Union(_) => panic!("Unions not supported"),
    }
}

fn derive_struct(input: &DeriveInput, struct_data: &DataStruct) -> TokenStream {
    let pattern = get_pattern(&input.attrs);
    let struct_name = &input.ident;
    if let Fields::Named(fields) = &struct_data.fields {
        let mut field_iter = fields.named.iter();
        let mut body = Vec::new();
        let mut field_names = Vec::new();
        for part in split_pattern(&pattern) {
            if part == "{}" {
                let field_name = field_iter.next().unwrap().ident.as_ref().unwrap();
                body.push(quote!(let #field_name = stream.parse_yolo();)); // TODO array
                field_names.push(field_name);
            } else {
                body.push(quote!(stream.expect(#part);));
            }
        }
        let lifetime_params: Vec<_> = input.generics.params.iter()
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
            impl #lifetime_params crate::input::ParseYolo #impl_lifetime for #struct_name #lifetime_params {
                fn parse_from_stream(stream: &mut crate::input::ParseStream #lifetime_params) -> Self {
                    #(#body)*
                    Self { #(#field_names, )* }
                }
            }
        };
        gen.into()
    } else {
        syn::Error::new(input.span(), "Only named fields are currently supported").to_compile_error().into()
    }
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

fn get_pattern(attrs: &[Attribute]) -> String {
    attrs.iter()
        .filter_map(|attribute|
            if let Meta::List(MetaList { path, .. }) = &attribute.meta {
                if path.is_ident("pattern") {
                    let pattern: LitStr = attribute.parse_args().ok()?;
                    Some(pattern.value())
                } else {
                    None
                }
            } else {
                None
            }
        )
        .next().unwrap()
}
