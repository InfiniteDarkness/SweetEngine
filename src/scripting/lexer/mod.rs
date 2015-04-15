//
//	scripting/lexer/mod.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

pub mod token;

use std::string::String;
use std::collections::VecDeque;

use self::token::*;
use self::token::SwToken::*;

pub struct SwLexer
{
	queue: 	VecDeque<Option<SwToken>>,
	code:	Vec<u8>,
	buf: 	String,
	pos:	isize,
	is_num:	bool,
	is_str:	bool,
	is_sym:	bool,
}

impl SwLexer
{
	pub fn new(program: String) -> SwLexer
	{
		SwLexer
		{
			queue:	VecDeque::new(),
			code:	program.into_bytes().clone(),
			buf: 	String::new(),
			pos:	-1,
			is_num: false,
			is_str: false,
			is_sym: false,
		}
	}

	fn handle_queue(&mut self, token: Option<SwToken>) -> Option<SwToken>
	{
		match token
		{
			Some(_) => {
				if self.is_num
				{
					let b = self.buf.clone();
					self.queue.push_front(Some(SwNumber(b)));
				}
				else if self.is_str
				{
					let b = self.buf.clone();
					self.queue.push_front(Some(SwString(b)));
				}
				else if self.is_sym
				{
					let b = self.buf.clone();
					self.queue.push_front(Some(SwSymbol(b)));
				}
				self.buf.clear();
				self.is_num = false;
				self.is_str = false;
				self.is_sym = false;
				self.queue.push_front(token);
				if !self.queue.is_empty() { self.queue.pop_back().unwrap() } else { None }
			},
			None => if !self.queue.is_empty() { self.queue.pop_back().unwrap() } else { None },
		}
	}
}

impl Iterator for SwLexer
{
	type Item = SwToken;

	fn next(&mut self) -> Option<SwToken>
	{
		if !self.queue.is_empty()
		{
			return self.queue.pop_back().unwrap()
		}
		else 
		{
			self.pos += 1;
		}

		if self.pos as usize >= self.code.len()
		{
			return None;
		}

		match self.code[self.pos as usize] as char
		{
			'(' => {
				if !self.is_str
				{
					self.handle_queue(Some(SwSymbol("(".to_string())))
				}
				else 
				{
					self.buf.push('(');
					Some(SwNone)
				}
			},
			')' => {
				if !self.is_str
				{
					self.handle_queue(Some(SwSymbol(")".to_string())))
				}
				else 
				{
					self.buf.push(')');
					Some(SwNone)
				}
			},
			' ' => {
				self.handle_queue(Some(SwNone))
			},
			'"' => {
				if self.is_str
				{
					self.handle_queue(Some(SwNone))
				}
				else
				{
					self.is_str = true;
					Some(SwNone)
				}
			},
			'-' => {
				if !self.is_str
				{
					self.is_num = true;
					self.buf.push('-');
					Some(SwNone)
				}
				else
				{
					self.buf.push('-');
					Some(SwNone)
				}
			},
			'.' => {
				if self.is_num || self.is_str
				{
					self.buf.push('.');
					Some(SwNone)
				}
				else
				{
					//SwSyntaxError("symbol is not alpha-alphanum");
					None
				}
			},
			x if x.is_alphabetic() => {
				if !self.is_str
				{
					self.is_sym = true;
					self.buf.push(x);
					Some(SwNone)
				}
				else 
				{
					self.buf.push(x);
					Some(SwNone)
				}
			},
			x if x.is_numeric() => {
				if !self.is_str && !self.is_sym && !self.is_num
				{
					if self.buf.is_empty() 
					{
						self.is_num = true;
						self.buf.push(x);
						Some(SwNone)
					}
					else
					{
						//SwInternalError("unknown buffer type");
						None
					}
				}
				else if self.is_sym
				{
					if self.buf.is_empty()
					{
						//SwSyntaxError("symbol is not alpha-alphanum");
						None
					}
					else
					{
						self.buf.push(x);
						Some(SwNone)
					}
				}
				else
				{
					self.buf.push(x);
					Some(SwNone)
				}
			},
			x => {
				if !self.is_str
				{
					Some(SwNone)
				}
				else
				{
					self.buf.push(x);
					Some(SwNone)
				}
			},
		}
	}
}
