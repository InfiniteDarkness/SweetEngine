//
//	scripting/token/token.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

pub enum SwToken
{
	SwSymbol(String),
	SwString(String),
	SwNumber(String),
	SwDouble(f64),
	SwNone
}
