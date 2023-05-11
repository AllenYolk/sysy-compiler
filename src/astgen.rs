pub mod ast;
use ast::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(sysy);

pub fn parse_sysy(input: &str) -> Result<CompUnit, ()> {
    sysy::CompUnitParser::new().parse(input).map_err(|_| ())
}