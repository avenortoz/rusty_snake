pub mod game;

#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn init_shaders(display: &glium::Display) -> Result<glium::Program, &'static str> {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
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
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
    Ok(program)
}
pub fn run(mut game: Game) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("rusty_snake")
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 600.0,
            height: 600.0,
        });

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    implement_vertex!(Vertex, position, tex_coords);

    const TARGET_FPS: u64 = 2;

    let program = init_shaders(&display).unwrap();
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();

        if let InputHandleType::Break = handle_input(event, control_flow, &mut game) {
            return;
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

        let shape = get_shape(&display, &game);
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
            tex: sampler
        };
        let params = glium::DrawParameters {
            ..Default::default()
        };
        target
            .draw(&vertex_buffer, indices, &program, &uniforms, &params)
            .unwrap();
        target.finish().unwrap();
    });
}

use crate::game::*;
fn get_shape(display: &glium::Display, game: &Game) -> Vec<Vertex> {
    let mut vertex1 = Vertex {
        position: [-0.9, -0.9],
        tex_coords: [0.0, 0.0],
    };
    let mut vertex2 = Vertex {
        position: [-0.9, 0.9],
        tex_coords: [0.0, 1.0],
    };
    let mut vertex3 = Vertex {
        position: [0.9, -0.9],
        tex_coords: [1.0, 0.0],
    };
    let mut vertex4 = Vertex {
        position: [0.9, 0.9],
        tex_coords: [1.0, 1.0],
    };
    let dimensions = display.gl_window().window().inner_size();
    let (window_real_width, window_real_height) = (dimensions.width, dimensions.height);
    let window_aspect_ratio = window_real_width as f32 / window_real_height as f32;

    let (board_width, board_height) = (game.board.pixel_width, game.board.pixel_height);
    let board_aspect_ratio = board_width as f32 / board_height as f32;

    if window_aspect_ratio < 1.0 && board_aspect_ratio >= 1.0
        || board_aspect_ratio >= window_aspect_ratio
    {
        let new_height =
            window_real_width as f32 / (board_aspect_ratio * window_real_height as f32);
        vertex1.position = [-1.0, -new_height];
        vertex2.position = [-1.0, new_height];
        vertex3.position = [1.0, -new_height];
        vertex4.position = [1.0, new_height];
    } else if window_aspect_ratio >= 1.0 && board_aspect_ratio < 1.0
        || window_aspect_ratio > board_aspect_ratio
    {
        let new_width = window_real_height as f32 * board_aspect_ratio / window_real_width as f32;
        vertex1.position = [-new_width, -1.0];
        vertex2.position = [-new_width, 1.0];
        vertex3.position = [new_width, -1.0];
        vertex4.position = [new_width, 1.0];
    }

    vec![vertex2, vertex3, vertex4, vertex2, vertex1, vertex3]
}

enum InputHandleType {
    Proceed,
    Break,
}

fn handle_input(
    event: glutin::event::Event<()>,
    control_flow: &mut glutin::event_loop::ControlFlow,
    game: &mut Game,
) -> InputHandleType {
    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return InputHandleType::Break;
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if input.state == glutin::event::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::Q => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                return InputHandleType::Break;
                            }
                            glutin::event::VirtualKeyCode::H => {
                                game.add_joint(Direction::W);
                                return InputHandleType::Break;
                            }
                            glutin::event::VirtualKeyCode::J => {
                                game.add_joint(Direction::N);
                                return InputHandleType::Break;
                            }
                            glutin::event::VirtualKeyCode::K => {
                                game.add_joint(Direction::S);
                                return InputHandleType::Break;
                            }
                            glutin::event::VirtualKeyCode::L => {
                                game.add_joint(Direction::E);
                                return InputHandleType::Break;
                            }
                            _ => {}
                        }
                    }
                } else {
                    return InputHandleType::Break;
                }
            }
            _ => return InputHandleType::Break,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return InputHandleType::Break,
        },
        _ => return InputHandleType::Break,
    }
    InputHandleType::Proceed
}
