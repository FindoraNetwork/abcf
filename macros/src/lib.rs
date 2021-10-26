//!
//! # macro
//!

extern crate proc_macro;
use proc_macro::TokenStream;

mod application;
mod event;
mod manager;
mod methods;
mod module;
mod rpcs;
mod utils;


#[proc_macro_attribute]
pub fn manager(args: TokenStream, input: TokenStream) -> TokenStream {
    manager::manager(args, input)
}

#[proc_macro_attribute]
pub fn rpcs(_args: TokenStream, input: TokenStream) -> TokenStream {
    rpcs::rpcs(_args, input)
}

#[proc_macro_derive(Event, attributes(abcf))]
pub fn event(input: TokenStream) -> TokenStream {
    event::event(input)
}

#[proc_macro_attribute]
pub fn methods(_args: TokenStream, input: TokenStream) -> TokenStream {
    methods::methods(_args, input)
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args, input)
}

#[proc_macro_attribute]
pub fn application(_args: TokenStream, input: TokenStream) -> TokenStream {
    application::application(_args, input)
}
