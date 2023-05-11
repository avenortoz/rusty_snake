use rand::prelude::*;
use std::error::Error;
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PixelCanvasError {
    ColorMap,
    Header,
    Footer,
    UnsupportedImageType(u8),
    UnsupportedBpp(u8),
    MismatchedBpp(u8),
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
                        Self::set_pixel_color(&mut self.raw_buffer, index, &color);
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
                        Self::set_pixel_color(&mut self.raw_buffer, index, &color);
                    }
                }
            }
        }
    }
    pub fn clear(&mut self) {
        // Fill the board
        self.raw_buffer.chunks_exact_mut(4).for_each(|chunk| {
            let (first, _) = chunk.split_at_mut(4);
            Self::set_pixel_color(first, 0, &self.cell_color);
        });

        // Draw the grid
        if let Some(grid) = &self.grid {
            (0..self.pixel_width)
                .step_by(self.unit_size as usize + grid.thickness as usize)
                .for_each(|i| {
                    for k in 0..grid.thickness {
                        for j in 0..self.pixel_height {
                            let index: usize = ((self.pixel_width * j + i + k as u32) * 4) as usize;
                            Self::set_pixel_color(&mut self.raw_buffer, index, &grid.color);
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
                            Self::set_pixel_color(&mut self.raw_buffer, index, &grid.color);
                        }
                    }
                });
        }
    }

    pub fn set_pixel_color(raw_buffer: &mut [u8], index: usize, color: &RGBA) {
        raw_buffer[index] = color.r;
        raw_buffer[index + 1] = color.g;
        raw_buffer[index + 2] = color.b;
        raw_buffer[index + 3] = color.a;
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
            Board::set_pixel_color(first, 0, &self.cell_color);
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

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    N = 0x1,
    E = 0x2,
    S = -0x1,
    W = -0x2,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Self::N => Direction::S,
            Self::E => Direction::W,
            Self::S => Direction::N,
            Self::W => Direction::E,
        }
    }
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
    pub joint_flag: bool,
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
            joints: Vec::new(),
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

        self.joint_flag = false;
    }

    pub fn draw(&mut self) {
        self.board.clear();
        for cell in self.snake.cells.iter() {
            self.board.draw(
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
    pub fn add_joint(&mut self, direction: Direction) {
        let mut top = &mut self.snake.cells[0];
        let prohibited = top.dir.reverse();

        if !self.joint_flag && direction != prohibited {
            top.dir = direction;
            let joint = Joint {
                position: top.pos,
                direction: top.dir,
            };
            self.joints.push(joint);
            self.joint_flag = true;
        }
    }
}
