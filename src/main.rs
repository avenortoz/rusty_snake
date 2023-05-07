#[macro_use]
extern crate glium;
#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

use rusty_snake::*;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("rusty_snake")
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 600.0,
            height: 600.0,
        });
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut board: Board = BoardBuilder::new(20, 20, 16)
        .unwrap()
        .with_grid(
            5,
            RGBA {
                r: 143,
                g: 190,
                b: 103,
                a: 255,
            },
        )
        .unwrap()
        .with_default_background_color(RGBA {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        })
        .unwrap()
        .build()
        .unwrap();
    board.draw_board();

    implement_vertex!(Vertex, position, tex_coords);
    let vertex1 = Vertex {
        position: [-0.9, -0.9],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [-0.9, 0.9],
        // tex_coords: [0.0, 300.0 / bb.pixel_height as f32],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.9, -0.9],
        // tex_coords: [300.0 / bb.pixel_width as f32, 0.0],
        tex_coords: [1.0, 0.0],
    };
    let vertex4 = Vertex {
        position: [0.9, 0.9],
        // tex_coords: [300.0 / bb.pixel_width as f32, 300.0 / bb.pixel_height as f32],
        tex_coords: [1.0, 1.0],
    };
    let shape = vec![vertex2, vertex3, vertex4, vertex2, vertex1, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    const TARGET_FPS: u64 = 60;

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;
    let mut delta: f32 = 0.02;
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                glutin::event::VirtualKeyCode::C => delta = -delta,
                                glutin::event::VirtualKeyCode::R => t = 0.0,
                                glutin::event::VirtualKeyCode::Q => {
                                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                                    return;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let elapsed_time = std::time::Instant::now()
            .duration_since(start_time)
            .as_millis() as u64;

        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0,
        };
        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(new_inst);

        // t += delta;

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &board.raw_buffer,
            (board.pixel_width, board.pixel_height),
        );
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

        let mut target = display.draw();
        target.clear_color(143.0 / 255.0, 190.0 / 255.0, 103.0 / 255.0, 1.0);


        let sampler = glium::uniforms::Sampler::new(&texture)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: sampler
        };
        let params = glium::DrawParameters {
            // polygon_mode: glium::draw_parameters::PolygonMode::Line,
            ..Default::default()
        };
        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
        target.finish().unwrap();
    });
}
