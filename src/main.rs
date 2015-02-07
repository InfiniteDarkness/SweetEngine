//
//	main.rs
//	SweetEngine
//
//	Copyright (c) 2015 Infinite
//	Licensed under the terms of the Apache License, v2
//

//----------------------------------------------------------------

//	Feature gates
#![feature(std_misc)]
#![feature(plugin)]
#![feature(core)]
#![feature(io)]

//	Imports
#[plugin] extern crate glium_macros;
#[macro_use] extern crate glium;
extern crate glutin;

//	Global namespace pollution ;P
use std::old_io::timer;
use std::time::Duration;
use glium::{DisplayBuild, Surface};

//----------------------------------------------------------------

//	Our vertex struct
#[vertex_format]
#[derive(Copy)]
struct Vertex
{
	position: [f32; 2],
	color: [f32; 3],
}

//----------------------------------------------------------------

fn main()
{
	let display = glutin::WindowBuilder::new()
		.with_vsync()
		.with_gl_version((4, 1))
		.with_dimensions(960, 540)
		.with_title(format!("SweetWindow"))
		.build_glium().unwrap();

	let vertex_buffer = {
		glium::VertexBuffer::new(&display, 
			vec!
			[
				Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
				Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
				Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
			]
		)
	};

	let index_buffer = glium::IndexBuffer::new(&display,
		glium::index_buffer::TrianglesList(vec![0u16, 1, 2]));

	let program = glium::Program::from_source(&display,
		"#version 410
		 uniform mat4 matrix;

		 in vec2 position;
		 in vec3 color;

		 out vec3 vColor;

		 void main()
		 {
		 	gl_Position = vec4(position, 0.0, 1.0) * matrix;
		 	vColor = color;
		 }",

		"#version 410
		 in vec3 vColor;

		 out vec4 fragColor;

		 void main()
		 {
		 	fragColor = vec4(vColor, 1.0);
		 }", None).unwrap();

	'main: loop
	{
		let uniforms = uniform!
		{
			matrix:
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0f32]
			]
		};

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);
		target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
			&std::default::Default::default());
		target.finish();

		timer::sleep(Duration::milliseconds(10));

		for event in display.poll_events()
		{
			match event
			{
				glutin::Event::KeyboardInput(_, _, 
					Some(glutin::VirtualKeyCode::Q)) => break 'main,
				_ => ()
			}	
		}
	}
}

//----------------------------------------------------------------
