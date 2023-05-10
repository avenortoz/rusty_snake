#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};

use rand::prelude::*;
use std::error::Error;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PixelCanvasError {
    ColorMap,
    Header,
    Footer,
    UnsupportedImageType(u8),
    UnsupportedBpp(u8),
    MismatchedBpp(u8),
    // FIX
    UnsupportedTgaType,
}

#[derive(Clone, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug, Copy)]
pub struct SnakeCell {
    pub pos: Position,
    pub dir: Direction,
}

#[derive(Debug)]
pub struct Snake {
    // head_texture: Vec<u8>,
    // tail_texture: Vec<u8>,
    // cell_texture: Vec<u8>,
    pub cells: Vec<SnakeCell>,
}

#[derive(Debug, Clone)]
pub struct BoardGrid {
    pub thickness: u8,
    pub color: RGBA,
}

#[derive(Debug)]
pub struct BoardBuilder {
    width: u32,
    height: u32,
    unit_size: u32,
    cell_color: RGBA,
    grid: Option<BoardGrid>,
}

#[derive(Debug)]
pub struct Board {
    pub raw_buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub unit_size: u32,
    pub cell_color: RGBA,
    pub grid: Option<BoardGrid>,
}

impl Board {
    pub fn draw(&mut self, position: Position, color: RGBA) {
        match &self.grid {
            Some(grid) => {
                for i in 0..self.unit_size {
                    for j in 0..self.unit_size {
                        let index: usize = (self.pixel_width as usize
                            * (position.y * self.unit_size
                                + grid.thickness as u32 * (position.y + 1)
                                + j) as usize
                            + (position.x * self.unit_size
                                + grid.thickness as u32 * (position.x + 1)
                                + i) as usize)
                            * 4;
                        self.raw_buffer[index] = color.r;
                        self.raw_buffer[index + 1] = color.g;
                        self.raw_buffer[index + 2] = color.b;
                        self.raw_buffer[index + 3] = color.a;
                    }
                }
            }
            None => {
                for i in 0..self.unit_size {
                    for j in 0..self.unit_size {
                        let index: usize = (self.pixel_width as usize
                            * (position.y * self.unit_size + j) as usize
                            + (position.x * self.unit_size + i) as usize)
                            * 4;
                        self.raw_buffer[index] = color.r;
                        self.raw_buffer[index + 1] = color.g;
                        self.raw_buffer[index + 2] = color.b;
                        self.raw_buffer[index + 3] = color.a;
                    }
                }
            }
        }
    }
    pub fn clear(&mut self) {
        // Fill the board
        self.raw_buffer.chunks_exact_mut(4).for_each(|chunk| {
            let (first, _) = chunk.split_at_mut(4);
            first[0] = self.cell_color.r;
            first[1] = self.cell_color.g;
            first[2] = self.cell_color.b;
            first[3] = self.cell_color.a;
        });

        // Draw the grid
        if let Some(grid) = &self.grid {
            (0..self.pixel_width)
                .step_by(self.unit_size as usize + grid.thickness as usize)
                .for_each(|i| {
                    for k in 0..grid.thickness {
                        for j in 0..self.pixel_height {
                            let index: usize = ((self.pixel_width * j + i + k as u32) * 4) as usize;
                            self.raw_buffer[index] = grid.color.r;
                            self.raw_buffer[index + 1] = grid.color.g;
                            self.raw_buffer[index + 2] = grid.color.b;
                            self.raw_buffer[index + 3] = grid.color.a;
                        }
                    }
                });

            (0..self.pixel_height)
                .step_by(self.unit_size as usize + grid.thickness as usize)
                .for_each(|i| {
                    for k in 0..grid.thickness {
                        for j in 0..self.pixel_width {
                            let index: usize =
                                ((self.pixel_width * (i + k as u32) + j) * 4) as usize;
                            self.raw_buffer[index] = grid.color.r;
                            self.raw_buffer[index + 1] = grid.color.g;
                            self.raw_buffer[index + 2] = grid.color.b;
                            self.raw_buffer[index + 3] = grid.color.a;
                        }
                    }
                });
        }
    }
}

impl BoardBuilder {
    pub fn new(width: u32, height: u32, unit_size: u32) -> Result<Self, PixelCanvasError> {
        Ok(BoardBuilder {
            width,
            height,
            unit_size,
            cell_color: RGBA {
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            },
            grid: Option::None,
        })
    }
    pub fn with_default_background_color(
        &mut self,
        color: RGBA,
    ) -> Result<&mut Self, PixelCanvasError> {
        self.cell_color = color;
        Ok(self)
    }

    pub fn with_grid(&mut self, thickness: u8, color: RGBA) -> Result<&mut Self, PixelCanvasError> {
        self.grid = Some(BoardGrid { thickness, color });
        Ok(self)
    }

    pub fn build(&mut self) -> Result<Board, PixelCanvasError> {
        let mut total_pixel_width = self.width * self.unit_size;
        if let Some(grid) = &self.grid {
            total_pixel_width += (self.width + 1) * grid.thickness as u32;
        }

        let mut total_pixel_height = self.height * self.unit_size;
        if let Some(grid) = &self.grid {
            total_pixel_height += (self.height + 1) * grid.thickness as u32;
        }

        // Fill the board
        let mut raw_buffer: Vec<u8> =
            vec![0; (total_pixel_width * total_pixel_height * 4) as usize];
        raw_buffer.chunks_exact_mut(4).for_each(|chunk| {
            let (first, _) = chunk.split_at_mut(4);
            first[0] = self.cell_color.r;
            first[1] = self.cell_color.g;
            first[2] = self.cell_color.b;
            first[3] = self.cell_color.a;
        });

        Ok(Board {
            raw_buffer,
            width: self.width,
            height: self.height,
            pixel_width: total_pixel_width,
            pixel_height: total_pixel_height,
            unit_size: self.unit_size,
            cell_color: self.cell_color.clone(),
            grid: self.grid.clone(),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub struct Joint {
    pub direction: Direction,
    pub position: Position,
}

pub struct Game {
    pub board: Board,
    pub snake: Snake,
    pub food: Vec<Position>,
    pub joints: Vec<Joint>,
    food_couter_mod: i32,
    food_couter: i32,
    pub joint_flag: bool, // food_seed: i32,
                          // pub head: Position,
                          // pub direction: Direction,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let board: Board = BoardBuilder::new(20, 20, 16)
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

        let snake = Snake {
            cells: vec![
                SnakeCell {
                    pos: Position { x: 3, y: 0 },
                    dir: Direction::E,
                },
                SnakeCell {
                    pos: Position { x: 2, y: 0 },
                    dir: Direction::E,
                },
                SnakeCell {
                    pos: Position { x: 1, y: 0 },
                    dir: Direction::E,
                },
            ],
        };

        Ok(Game {
            board,
            snake,
            food: Vec::new(),
            food_couter_mod: 40,
            food_couter: 0,
            joints: Vec::new(), // food_seed: 0, // joins: Vec<Joint>::new()
            joint_flag: false,
        })
    }

    pub fn update(&mut self) {
        if self.food_couter == 0 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.board.width);
            let y = rng.gen_range(0..self.board.height);
            self.food.push(Position { x, y });
        }
        self.food_couter = (self.food_couter + 1) % self.food_couter_mod;

        // Update directions
        for cell in self.snake.cells.iter_mut() {
            for joint in self.joints.iter() {
                if joint.position == cell.pos {
                    cell.dir = joint.direction;
                }
            }
        }

        if let Some(last_cell) = self.snake.cells.last() {
            if !self.joints.is_empty() && last_cell.pos == self.joints[0].position {
                self.joints.remove(0);
            }
        }
        let last = self.snake.cells[self.snake.cells.len() - 1];

        for cell in self.snake.cells.iter_mut() {
            match cell.dir {
                Direction::S => {
                    if cell.pos.y == 0 {
                        cell.pos.y = self.board.height - 1;
                    } else {
                        cell.pos.y -= 1;
                    }
                }
                Direction::N => {
                    if cell.pos.y == self.board.height - 1 {
                        cell.pos.y = 0;
                    } else {
                        cell.pos.y += 1;
                    }
                }
                Direction::E => {
                    if cell.pos.x == self.board.width - 1 {
                        cell.pos.x = 0;
                    } else {
                        cell.pos.x += 1;
                    }
                }
                Direction::W => {
                    if cell.pos.x == 0 {
                        cell.pos.x = self.board.width - 1;
                    } else {
                        cell.pos.x -= 1;
                    }
                }
            }
        }

        let &mut top = &mut self.snake.cells[0];
        for i in 0..self.food.len() {
            if top.pos == self.food[i] {
                println!("Eaten: {:?}", self.food[i]);
                self.food.remove(i);
                self.snake.cells.push(last);
                break;
            }
        }
        let head = self.snake.cells[0].pos;
        for i in 1..self.snake.cells.len() {
            if head == self.snake.cells[i].pos {
                println!("Game Over");
                panic!();
            }
        }

        // Check if snake bited itself

        self.joint_flag = false;
    }

    pub fn draw(&mut self) {
        self.board.clear();
        for cell in self.snake.cells.iter() {
            self.board.draw(
                // self.snake.cells[0].pos,
                cell.pos,
                RGBA {
                    r: 100,
                    g: 100,
                    b: 0,
                    a: 255,
                },
            );
        }

        for food_cell in self.food.iter() {
            self.board.draw(
                *food_cell,
                RGBA {
                    r: 255,
                    g: 100,
                    b: 100,
                    a: 255,
                },
            )
        }
    }
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

        if let InputHandleType::BREAK = handle_input(event, control_flow, &mut game) {
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
    let (window_real_width, window_real_height) =
        (dimensions.width as u32, dimensions.height as u32);
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
    PROCEED,
    BREAK,
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
                return InputHandleType::BREAK;
            }
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if input.state == glutin::event::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        let mut top = &mut game.snake.cells[0];
                        match key {
                            glutin::event::VirtualKeyCode::Q => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                return InputHandleType::BREAK;
                            }
                            glutin::event::VirtualKeyCode::H => {
                                if !game.joint_flag {
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
                                return InputHandleType::BREAK;
                            }
                            glutin::event::VirtualKeyCode::J => {
                                if !game.joint_flag {
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
                                return InputHandleType::BREAK;
                            }
                            glutin::event::VirtualKeyCode::K => {
                                if !game.joint_flag {
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
                                return InputHandleType::BREAK;
                            }
                            glutin::event::VirtualKeyCode::L => {
                                if !game.joint_flag {
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
                                return InputHandleType::BREAK;
                            }
                            _ => {}
                        }
                    }
                } else {
                    return InputHandleType::BREAK;
                }
            }
            _ => return InputHandleType::BREAK,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return InputHandleType::BREAK,
        },
        _ => return InputHandleType::BREAK,
    }
    InputHandleType::PROCEED
}
