use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::Span;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Type};

/// Create `Type::builder(…)` and `TypeBuilder::build()` methods.
/// `Type::builder(…)` will take as params all the non-optional fields of `Type`.
///
/// `Type` is expected to derive `Builder` and have `#[builder(custom_constructor, build_fn(private, name = "try_build"), setter(into, strip_option))]`,
/// with its non-optional fields having `#[builder(private)]`
/// and optional fields having `#[builder(default)]`.
#[proc_macro_derive(MandatoryParamBuilder)]
pub fn derive_mandatory_param_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !(input.generics.lt_token.is_none()
        && input.generics.params.is_empty()
        && input.generics.gt_token.is_none()
        && input.generics.where_clause.is_none())
    {
        return TokenStream::from(quote! {
            compile_error!("Deriving MandatoryParamBuilder is not implemented for generic types");
        });
    }

    let Data::Struct(data_struct) = input.data else {
        return TokenStream::from(quote! {
            compile_error!("Deriving MandatoryParamBuilder is only implemented for structs");
        })
    };

    let Fields::Named(fields_named) = data_struct.fields else {
        return TokenStream::from(quote! {
            compile_error!("Deriving MandatoryParamBuilder is only implemented for named fields");
        })
    };

    let ident = input.ident;
    let builder_ident = format_ident!("{ident}Builder");
    let vis = input.vis;

    let mandatory_fields = fields_named
        .named
        .iter()
        .filter(is_mandatory)
        .collect::<Vec<_>>();

    // Slightly hacky, but we would probably be fine with just three of these.
    let type_param_idents = vec!["T", "U", "V", "W", "X", "Y", "Z"]
        .iter()
        .take(mandatory_fields.len())
        .map(|name| Ident::new(name, Span::call_site()))
        .collect::<Vec<_>>();

    if mandatory_fields.len() > type_param_idents.len() {
        return TokenStream::from(quote! {
            compile_error!("Deriving MandatoryParamBuilder would run out of generic param IDs");
        });
    }

    let intos = type_param_idents
        .iter()
        .zip(mandatory_fields.iter())
        .map(|(type_param_ident, field)| {
            let field_type = &field.ty;
            quote! { #type_param_ident: ::std::convert::Into<#field_type> }
        })
        .collect::<Vec<_>>();

    let params = type_param_idents
        .iter()
        .zip(mandatory_fields.iter())
        .map(|(type_param_ident, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            quote! { #field_ident: #type_param_ident }
        })
        .collect::<Vec<_>>();

    let setters = mandatory_fields
        .iter()
        .map(|field| {
            let method = field.ident.as_ref().unwrap();
            let param = field.ident.as_ref().unwrap();
            quote! {
                builder.#method(#param);
            }
        })
        .collect::<Vec<_>>();

    let builder_doc = "Create a new request builder";
    let build_doc = "Build a request";

    let expanded = quote! {
        #[automatically_derived]
        impl #ident {
            #[doc = #builder_doc]
            #vis fn builder<#(#type_param_idents),*>(#(#params),*) -> #builder_ident where #(#intos),* {
                let mut builder = #builder_ident::create_empty();
                #(#setters)*;
                builder
            }
        }

        #[automatically_derived]
        impl #builder_ident {
            #[doc = #build_doc]
            #vis fn build(&self) -> #ident {
                self.try_build()
                    .expect("One or more required fields are missing. This should not happen and probably indicates a MandatoryParamBuilder bug.")
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_mandatory(field: &&Field) -> bool {
    let Type::Path(type_path) = &field.ty else {
        return false;
    };
    let Some(last) = type_path.path.segments.iter().last() else {
        return false;
    };
    last.ident != "Option"
}
