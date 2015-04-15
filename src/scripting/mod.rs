//
//	scripting/mod.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

//	Modules
pub mod lexer;
pub mod parser;

use self::lexer::*;
use self::lexer::token::*;
use self::lexer::token::SwToken::*;

use self::parser::*;

pub fn execute_script(source: String)
{
	let lexer = SwLexer::new(source);

	let mut tokens = vec![];

	for tok in lexer
	{
		match tok
		{
			SwSymbol(_) => tokens.push(tok),
			SwString(_) => tokens.push(tok),
			SwNumber(x) => tokens.push(SwDouble(x.parse::<f64>().unwrap())),
			SwDouble(_) => {},
			SwNone => {},
		}
	}

	let AST = parse(tokens);

	//AST.execute();
}
