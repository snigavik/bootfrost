mod misc;
mod term;
mod answer;
mod context;
mod question;
pub mod solver;
mod plain;
mod parser;
mod ifunctions;
mod strategies;
mod base;
mod environment;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub tqfline); // synthesized by LALRPOP