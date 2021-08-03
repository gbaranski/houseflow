use darling::FromVariant;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromVariant)]
#[darling(attributes(response))]
struct ServerErrorVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<syn::Type>,
    status_code: u16,
}

#[proc_macro_attribute]
pub fn server_error(_args: TokenStream, item: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs: _attrs,
        vis,
        ident,
        generics,
        data,
    } = syn::parse_macro_input!(item);

    let data = match data {
        syn::Data::Enum(data) => data,
        _ => panic!(
            "expected `enum`, got: {}",
            match data {
                syn::Data::Struct(_) => "struct",
                syn::Data::Union(_) => "union",
                _ => unreachable!(),
            }
        ),
    };

    let variants = data.variants;

    let variant_attributes = variants
        .iter()
        .map(|variant| ServerErrorVariant::from_variant(variant).unwrap());

    let status_code_match_patterns = variant_attributes
        .map(|ServerErrorVariant { ident, status_code, fields }: ServerErrorVariant| {
            let something = if fields.is_empty() {
                quote! { #ident }
            } else if fields.is_tuple() {
                quote! { #ident(_) }
            } else {
                unreachable!()
            };

            quote! {
                Self::#something => StatusCode::from_u16(#status_code).expect("invalid status code"),
            }
        })
        .collect::<Vec<_>>();

    let variants = variants
        .into_iter()
        .map(|variant| syn::Variant {
            attrs: variant
                .attrs
                .iter()
                .filter(|attr| *attr.path.get_ident().unwrap() != "response")
                .cloned()
                .collect(),
            ..variant
        })
        .collect::<Vec<_>>();

    let variants =
        <syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> as std::iter::FromIterator<
            syn::Variant,
        >>::from_iter(variants);

    let output = quote! {
        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
        #[serde(
            tag = "error",
            content = "error_description",
            rename_all = "snake_case"
        )]
        #vis enum #ident #generics {
            #[error("internal error: {0}")]
            InternalError(#[from] crate::InternalServerError),

            #[error("validation error: {0}")]
            ValidationError(#[from] crate::ValidationError),

            #variants
        }

        #[cfg(feature = "actix")]
        impl actix_web::ResponseError for #ident {
            fn status_code(&self) -> actix_web::http::StatusCode {
                use actix_web::http::StatusCode;

                match self {
                    Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    Self::ValidationError(_) => StatusCode::BAD_REQUEST,

                    #(#status_code_match_patterns)*
                }
            }

            fn error_response(&self) -> actix_web::HttpResponse {
                let json = actix_web::web::Json(self);
                actix_web::HttpResponse::build(self.status_code()).json(json)
            }
        }

    };

    output.into()
}
