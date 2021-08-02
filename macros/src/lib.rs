use darling::FromVariant;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromVariant)]
#[darling(attributes(response))]
struct ServerErrorVariant {
    ident: syn::Ident,
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
        .map(|ServerErrorVariant { ident, status_code }: ServerErrorVariant| {
            quote! {
                Self::#ident => StatusCode::from_u16(#status_code).expect("invalid status code"),
            }
        })
        .collect::<Vec<_>>();

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
        }

    };

    output.into()
}
