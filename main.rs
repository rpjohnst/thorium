#![feature(globs)]

extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::Context;
use std::{mem, ptr, str};

static VS_SRC: &'static str = r"
#version 150
in vec2 position;
void main() {
	gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &'static str = r"
#version 150
out vec4 out_color;
void main() {
	out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

static VERTEX_DATA: [GLfloat, ..6] = [
	0.0, 0.5,
	0.5, -0.5,
	-0.5, -0.5
];

fn main() {
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::ContextVersion(3, 1));

	let (window, events) = glfw.create_window(
		800, 600, "Hello World", glfw::Windowed
	).expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	gl::load_with(|s| glfw.get_proc_address(s));

	let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
	let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
	let program = link_program(vs, fs);

	let mut vao = 0;
	let mut vbo = 0;

	unsafe {
		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);

		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			mem::transmute(&VERTEX_DATA[0]),
			gl::STATIC_DRAW
		);

		gl::UseProgram(program);
		"out_color".with_c_str(
			|ptr| gl::BindFragDataLocation(program, 0, ptr)
		);
		let pos_attr = "position".with_c_str(
			|ptr| gl::GetAttribLocation(program, ptr)
		);
		gl::EnableVertexAttribArray(pos_attr as GLuint);
		gl::VertexAttribPointer(
			pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean,
			0, ptr::null()
		);
	}

	gl::ClearColor(0.3, 0.3, 0.3, 1.0);

	while !window.should_close() {
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			match event {
				glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
					window.set_should_close(true)
				}
				_ => {}
			}
		}

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::DrawArrays(gl::TRIANGLES, 0, 3);

		window.swap_buffers();
	}

	gl::DeleteProgram(program);
	gl::DeleteShader(fs);
	gl::DeleteShader(vs);
	unsafe {
		gl::DeleteBuffers(1, &vbo);
		gl::DeleteBuffers(1, &vao);
	}
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
	let shader = gl::CreateShader(ty);
	unsafe {
		src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
		gl::CompileShader(shader);

		let mut status = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

		if status != (gl::TRUE as GLint) {
			let mut len = 0;
			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

			let mut buf = Vec::from_elem(len as uint - 1, 0u8);
			gl::GetShaderInfoLog(
				shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar
			);
			fail!("{}", str::from_utf8(buf.as_slice())
				.expect("ShaderInfoLog is not valid UTF-8"));
		}
	}
	shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
	let program = gl::CreateProgram();
	unsafe {
		gl::AttachShader(program, vs);
		gl::AttachShader(program, fs);
		gl::LinkProgram(program);

		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
		
		if status != (gl::TRUE as GLint) {
			let mut len = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

			let mut buf = Vec::from_elem(len as uint - 1, 0u8);
			gl::GetProgramInfoLog(
				program, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar
			);
			fail!("{}", str::from_utf8(buf.as_slice())
				.expect("ProgramInfoLog is not valid UTF-8"));
		}
	}
	program
}
