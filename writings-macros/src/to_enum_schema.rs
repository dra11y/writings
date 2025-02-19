use proc_macro::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::punctuated::Punctuated;
use syn::{Attribute, DeriveInput, Error, Ident, Meta, Variant, parse_macro_input};

pub fn derive_to_enum_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let enum_description =
        get_doc_description(&input.attrs).unwrap_or_else(|| input.ident.to_string());

    // Parse schema attributes
    let source = parse_schema_attributes(&input);

    // Process variants
    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => {
            return Error::new_spanned(name, "ToEnumSchema only works on enums")
                .to_compile_error()
                .into();
        }
    };

    let variant_names = variants.iter().map(|v| v.ident.to_string());

    let descriptions = match get_descriptions(variants, &source, name) {
        Ok(res) => res,
        Err(e) => return e.to_compile_error().into(),
    };

    // Build schema
    let mut schema = quote! {
        utoipa::openapi::schema::Object::builder()
        .schema_type(
            utoipa::openapi::schema::SchemaType::new(
                utoipa::openapi::schema::Type::String,
            ),
        )
        .enum_values(Some([#(#variant_names),*]))
        .description(Some(#enum_description))
    };
    if descriptions.len() == variants.len() {
        schema.append_all(quote! {
                .extensions(Some(
                    utoipa::openapi::extensions::Extensions::builder()
                        .add("x-enum-descriptions", Some([#(#descriptions),*]))
                        .build()
                ))
        });
    }

    let expanded = quote! {
        impl utoipa::PartialSchema for #name {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                #schema.into()
            }
        }

        impl utoipa::ToSchema for #name {}
    };

    TokenStream::from(expanded)
}

fn parse_schema_attributes(input: &DeriveInput) -> Option<DescriptionSource> {
    input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("schema"))
        .and_then(|attr| {
            attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .ok()
        })
        .and_then(|metas| {
            metas.iter().find_map(|meta| match meta {
                syn::Meta::NameValue(nv) if nv.path.is_ident("descriptions") => {
                    Some(nv.value.clone())
                }
                _ => None,
            })
        })
        .and_then(|value| match value {
            syn::Expr::Lit(lit) => match lit.lit {
                syn::Lit::Str(str) => syn::parse_str::<Ident>(&str.value()).ok(),
                _ => None,
            },
            syn::Expr::Path(path) => path.path.get_ident().cloned(),
            _ => None,
        })
        .map(|ident| {
            if ident == "DocComments" {
                DescriptionSource::DocComments
            } else {
                DescriptionSource::Method { ident }
            }
        })
}

#[derive(Debug, Default)]
enum DescriptionSource {
    Method {
        ident: Ident,
    },
    #[default]
    DocComments,
}

fn get_descriptions(
    variants: &Punctuated<Variant, syn::Token![,]>,
    source: &Option<DescriptionSource>,
    enum_name: &Ident,
) -> Result<Vec<proc_macro2::TokenStream>, Error> {
    let mut variant_exprs = Vec::new();
    let mut descriptions = Vec::new();

    for variant in variants.iter() {
        let variant_ident = variant.ident.clone();
        variant_exprs.push(variant_ident.clone().into_token_stream());

        // Verify unit variant
        if !matches!(&variant.fields, syn::Fields::Unit) {
            return Err(Error::new_spanned(
                variant,
                "Only unit variants are supported",
            ));
        }

        // Description handling
        match source {
            Some(DescriptionSource::Method { ident }) => {
                descriptions.push(quote! { #enum_name::#variant_ident.#ident() });
            }
            None | Some(DescriptionSource::DocComments) => {
                if let Some(doc) = get_doc_description(&variant.attrs) {
                    descriptions.push(doc.to_token_stream());
                } else if source.is_some() {
                    return Err(Error::new_spanned(variant, "variant has no doc comment"));
                }
            }
        };
    }

    Ok(descriptions)
}

fn get_doc_description(attrs: &[Attribute]) -> Option<String> {
    let doc_string = attrs
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
