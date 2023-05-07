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

    let mut game = Game::new().unwrap();

    implement_vertex!(Vertex, position, tex_coords);
    let vertex1 = Vertex {
        position: [-0.9, -0.9],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [-0.9, 0.9],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.9, -0.9],
        tex_coords: [1.0, 0.0],
    };
    let vertex4 = Vertex {
        position: [0.9, 0.9],
        tex_coords: [1.0, 1.0],
    };
    let shape = vec![vertex2, vertex3, vertex4, vertex2, vertex1, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    const TARGET_FPS: u64 = 2;

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
                            let mut top = &mut game.snake.cells[0];
                            match key {
                                glutin::event::VirtualKeyCode::Q => {
                                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                                    return;
                                }
                                glutin::event::VirtualKeyCode::H => {
                                    if (!game.joint_flag) {
                                        match top.dir {
                                            Direction::E => {}
                                            _ => {
                                                top.dir = Direction::W;
                                                game.joints.push(Joint {
                                                    position: top.pos,
                                                    direction: top.dir,
                                                });
                                                game.joint_flag = true;
                                            }
                                        }
                                    }
                                    return;
                                }
                                glutin::event::VirtualKeyCode::J => {
                                    if (!game.joint_flag) {
                                        match top.dir {
                                            Direction::S => {}
                                            _ => {
                                                top.dir = Direction::N;
                                                game.joints.push(Joint {
                                                    position: top.pos,
                                                    direction: top.dir,
                                                });
                                                game.joint_flag = true;
                                            }
                                        }
                                    }
                                    return;
                                }
                                glutin::event::VirtualKeyCode::K => {
                                    if (!game.joint_flag) {
                                        match top.dir {
                                            Direction::N => {}
                                            _ => {
                                                top.dir = Direction::S;
                                                game.joints.push(Joint {
                                                    position: top.pos,
                                                    direction: top.dir,
                                                });
                                                game.joint_flag = true;
                                            }
                                        }
                                    }
                                    return;
                                }
                                glutin::event::VirtualKeyCode::L => {
                                    if (!game.joint_flag) {
                                        match top.dir {
                                            Direction::W => {}
                                            _ => {
                                                top.dir = Direction::E;
                                                game.joints.push(Joint {
                                                    position: top.pos,
                                                    direction: top.dir,
                                                });
                                                game.joint_flag = true;
                                            }
                                        }
                                    }
                                    return;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        return;
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
        game.update();
        game.draw();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &game.board.raw_buffer,
            (game.board.pixel_width, game.board.pixel_height),
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
