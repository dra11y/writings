use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, parse_macro_input};

#[derive(Debug)]
enum DescriptionPreference {
    DocComment,
    Method(String),
}

const DEFAULT_METHOD: &str = "to_string";

pub fn derive_to_enum_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let description_preference = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("schema") {
                let nested = attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .ok()?;
                for meta in nested {
                    match meta {
                        syn::Meta::NameValue(nv) if nv.path.is_ident("descriptions") => {
                            if let syn::Expr::Path(expr) = nv.value {
                                if let Some(last_seg) = expr.path.segments.last() {
                                    return match last_seg.ident.to_string().as_str() {
                                        "DocComment" => Some(DescriptionPreference::DocComment),
                                        method if !method.trim().is_empty() => {
                                            Some(DescriptionPreference::Method(
                                                method.trim().to_string(),
                                            ))
                                        }
                                        _ => Some(DescriptionPreference::Method(
                                            DEFAULT_METHOD.to_string(),
                                        )),
                                    };
                                }
                            }
                        }
                        _ => panic!("unrecognized: {:?}", meta),
                    }
                }
            }
            None
        })
        .unwrap_or(DescriptionPreference::Method(DEFAULT_METHOD.to_string()));

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("ToEnumSchema can only be derived for enums"),
    };

    let enum_description = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &nv.value
                {
                    let value = lit.value().trim().to_string();
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n");

    let enum_description = if enum_description.is_empty() {
        name.to_string()
    } else {
        enum_description
    };

    let variant_names = variants.iter().map(|v| v.ident.to_string());

    let descriptions = variants
        .iter()
        .map(|v| {
            match &description_preference {
                DescriptionPreference::DocComment => {
                    get_doc_comment(v).or_else(|| get_display_string(v, DEFAULT_METHOD))
                }
                DescriptionPreference::Method(method) => {
                    get_display_string(v, &method).or_else(|| get_doc_comment(v))
                }
            }
            .unwrap_or_else(|| v.ident.to_string())
        })
        .collect::<Vec<_>>();

    let schema = quote! {
        utoipa::openapi::schema::Object::builder()
            .schema_type(
                utoipa::openapi::schema::SchemaType::new(
                    utoipa::openapi::schema::Type::String,
                ),
            )
            .enum_values(Some([#(#variant_names),*]))
            .description(Some(#enum_description))
            .extensions(Some(
                utoipa::openapi::extensions::Extensions::builder()
                    .add(
                        "x-enum-descriptions",
                        Some([#(#descriptions),*])
                    )
                    .build()))
            .into()
    };

    let expanded = quote! {
        impl utoipa::PartialSchema for #name {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                #schema
            }
        }
        impl utoipa::ToSchema for #name {}
    };

    TokenStream::from(expanded)
}

fn get_doc_comment(variant: &syn::Variant) -> Option<String> {
    let doc_string = variant
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &nv.value
                {
                    let value = lit.value().trim().to_string();
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if doc_string.is_empty() {
        None
    } else {
        Some(doc_string)
    }
}

fn get_display_string(variant: &syn::Variant, method: &str) -> Option<String> {
    todo!()
}
