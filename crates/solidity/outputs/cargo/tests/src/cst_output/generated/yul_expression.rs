// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn decimal_literal() -> Result<()> {
    run("YulExpression", "decimal_literal")
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    run("YulExpression", "decimal_trailing_ident_start")
}

#[test]
fn false_keyword() -> Result<()> {
    run("YulExpression", "false_keyword")
}

#[test]
fn function_call() -> Result<()> {
    run("YulExpression", "function_call")
}

#[test]
fn hex_literal() -> Result<()> {
    run("YulExpression", "hex_literal")
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    run("YulExpression", "hex_trailing_ident_start")
}

#[test]
fn identifier_path() -> Result<()> {
    run("YulExpression", "identifier_path")
}

#[test]
fn identifier_with_dot() -> Result<()> {
    run("YulExpression", "identifier_with_dot")
}

#[test]
fn true_keyword() -> Result<()> {
    run("YulExpression", "true_keyword")
}
