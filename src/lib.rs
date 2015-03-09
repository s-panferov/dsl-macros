#![feature(plugin)]
#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;

use rustc::plugin;
use syntax::parse::token;

use syntax::{codemap};
use syntax::ext::base;
use syntax::parse::parser;

#[macro_use] mod dsl;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(token::intern("dsl"),
        syntax::ext::base::NormalTT(Box::new(dsl::dsl), None, false));
}

pub trait Parser<Cfg> {
    fn parse(&mut parser::Parser, Cfg) -> Self;
}

pub trait Generator<Cfg> {
    fn generate<'a>(self, codemap::Span, &mut base::ExtCtxt, Cfg) -> Box<base::MacResult + 'a>;
}
