//
//	scripting/parser/mod.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

pub mod AST;

use super::lexer::token::*;
use super::lexer::token::SwToken::*;

use self::AST::*;

pub fn parse(tokens: Vec<SwToken>) -> SwAST
{
	let AST = SwAST::new();

	AST
}
