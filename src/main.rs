//
//	main.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

pub mod scripting;

use self::scripting::*;

fn main()
{
	execute_script("(lol -0.5)".to_string());
    println!("Hello, world!");
}
