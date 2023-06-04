use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, Type};

#[proc_macro_derive(ViewWrapper)]
pub fn view_wrapper_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_view_wrapper(&ast)
}

/// Implement ViewWrapper trait from cursive library.
///
/// Take field type for associated type and field name.
fn impl_view_wrapper(ast: &syn::DeriveInput) -> TokenStream {
    let mut associated_types = Vec::new();
    let mut field_names = Vec::new();

    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    field_names.push(ident);

                    let stype = match &field.ty {
                        Type::Path(v) => v,
                        _ => unimplemented!(),
                    };
                    associated_types.push(quote! { type V = #stype; });
                }
            }
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    let associated_type = associated_types.first().unwrap();
    let field_name = field_names.first().unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl ViewWrapper for #name {
            #associated_type

            fn with_view<F, R>(&self, f: F) -> Option<R>
            where
                F: FnOnce(&Self::V) -> R,
            {
                Some(f(&self.#field_name))
            }

            fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
            where
                F: FnOnce(&mut Self::V) -> R,
            {
                Some(f(&mut self.#field_name))
            }

            fn wrap_call_on_any<'a>(
                &mut self,
                selector: &cursive::view::Selector<'_>,
                callback: cursive::event::AnyCb<'a>,
            ) {
                self.with_view_mut(|v| v.call_on_any(selector, callback));
            }
        }
    };
    gen.into()
}
