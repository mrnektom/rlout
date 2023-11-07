use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Fields, FieldsNamed, ItemStruct, parse_quote, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn element(_: TokenStream, s: TokenStream) -> TokenStream {
    let s = parse_macro_input!(s as ItemStruct);

    let ItemStruct {
        fields,
        vis,
        ident,
        ..
    } = s;

    let fields = match fields {
        Fields::Named(FieldsNamed { named: fields, .. }) => fields,
        Fields::Unnamed(_) => todo!(),
        Fields::Unit => Punctuated::default(),
    };
    
    let expanded = quote!(
        #vis struct #ident { 
            #fields
            children: Vec<rlout_runtime::ElementRef>,
            parent: Option<rlout_runtime::ElementRef>,
         }

        impl rlout_runtime::Element for #ident {
            fn children(&self) -> Vec<rlout_runtime::ElementRef> {
                &self.children
            }

            fn push_child(&mut self, element: rlout_runtime::ElementRef) {
                rlout_runtime::push_child(&mut self.children, element);
            }
            fn remove_child(&mut self, element: rlout_runtime::ElementRef) {
                rlout_runtime::remove_child(&mut self.children, element);
            }

            fn remove(&mut self) {

            }

            fn parent(&self) -> Option<rlout_runtime::ElementRef> {
                self.parent.clone()
            }
        }
    );
    println!("{expanded}");

    TokenStream::from(expanded)
}
