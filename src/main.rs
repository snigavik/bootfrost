mod misc;
mod term;
mod answer;
mod context;
mod question;
mod solver;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub pcf); // synthesized by LALRPOP

fn main() {
    let prepcf = crate::pcf::SymbolParser::new().parse("Helloworld").unwrap();
    println!("{}", prepcf);
}
