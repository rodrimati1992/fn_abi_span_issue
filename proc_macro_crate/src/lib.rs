extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{
    visit::{self, Visit},
    Lifetime, Type, TypeBareFn, TypeReference,
};


#[proc_macro_derive(ProcMacro)]
pub fn stable_abi(input: TokenStream) -> TokenStream {
    syn::parse::<syn::DeriveInput>(input)
        .and_then(|di|{
            let mut res=Ok(());
            Visitor { err:&mut res }.visit_derive_input(&di);
            res.map(|_| quote::quote!() )
        })
        .unwrap_or_else(|e| e.to_compile_error() )
        .into()
}

pub(crate) struct Visitor<'a> {
    err:&'a mut Result<(),syn::Error>,
}

/////////////

impl<'a> Visit<'_> for Visitor<'a> {
    fn visit_type_reference(&mut self, ref_: &TypeReference) {
        *self.err=Err(syn::Error::new_spanned(ref_,format!("hello from proc macro")));
    }
    fn visit_lifetime(&mut self, lt: &Lifetime) {   
        *self.err=Err(syn::Error::new_spanned(lt,format!("hello from proc macro")));
    }
}