use crate::common::get_field_names;
use crate::common::{get_connect_all, get_has_changed, get_update_all, TS};
use quote::quote;
use syn::Result;

pub(crate) fn get_impl_for_logic_interface(input: &syn::DeriveInput) -> Result<TS> {
    let fields = get_field_names(input)?;
    let link = get_link(fields.clone())?;
    let link_hdl = get_link_hdl(fields.clone())?;
    let update_all = get_update_all(fields.clone())?;
    let has_changed = get_has_changed(fields.clone())?;
    let connect_all = get_connect_all(fields.clone())?;
    let accept = get_accept(fields.clone())?;
    let name = &input.ident;
    let (impl_generics, ty_generics, _where_clause) = &input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics rust_hdl_core::logic::Logic for #name #ty_generics {
            fn update(&mut self) {}
            fn connect(&mut self) {}
        }

        impl #impl_generics rust_hdl_core::block::Block for #name #ty_generics {
            #connect_all
            #update_all
            #has_changed
            #accept
        }

        impl #impl_generics rust_hdl_core::logic::LogicLink for #name #ty_generics {
            #link
            #link_hdl
        }

    })
}

fn get_link(fields: Vec<TS>) -> Result<TS> {
    Ok(quote! {
        fn link(&mut self, other: &mut Self) {
            #(self.#fields.link(&mut other.#fields);)*
        }
    })
}

fn get_link_hdl(fields: Vec<TS>) -> Result<TS> {
    let fields_as_strings = fields.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    Ok(quote! {
        fn link_hdl(&self, my_name: &str, this: &str, that: &str) -> Vec<rust_hdl_core::ast::VerilogLink> {
            let mut ret = vec![];
            #(ret.push(self.#fields.link_hdl(#fields_as_strings, this, that));)*
            ret
        }
    })
}

fn get_accept(fields: Vec<TS>) -> Result<TS> {
    let fields_as_strings = fields.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    Ok(quote! {
        fn accept(&self, name: &str, probe: &mut dyn rust_hdl_core::probe::Probe) {
            probe.visit_start_namespace(name, self);
            #(self.#fields.accept(#fields_as_strings, probe);)*
            probe.visit_end_namespace(name, self);
        }
    })
}
