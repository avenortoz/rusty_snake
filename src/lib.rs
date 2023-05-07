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

#[derive(Clone, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct Snake {
    head_texture: Vec<u8>,
    tail_texture: Vec<u8>,
    cell_texture: Vec<u8>,
    cells: Vec<Position>,
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
    pub fn draw(&mut self, position: Position, texture: Vec<u8>){

    }
    pub fn draw_board(&mut self){
        // Fill the board
        let mut raw_buffer: Vec<u8> =
            vec![0; (self.pixel_width * self.pixel_height * 4) as usize];
        raw_buffer.chunks_exact_mut(4).for_each(|chunk| {
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
                            let index: usize =
                                ((self.pixel_width * j + i + k as u32) * 4) as usize;
                            raw_buffer[index] = grid.color.r;
                            raw_buffer[index + 1] = grid.color.g;
                            raw_buffer[index + 2] = grid.color.b;
                            raw_buffer[index + 3] = grid.color.a;
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
                            raw_buffer[index] = grid.color.r;
                            raw_buffer[index + 1] = grid.color.g;
                            raw_buffer[index + 2] = grid.color.b;
                            raw_buffer[index + 3] = grid.color.a;
                        }
                    }
                });
            }
            self.raw_buffer = raw_buffer;
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

enum Direction {
    N,
    E,
    S,
    W,
}

struct Joint {
    direction: Direction,
    position: Position,
}

struct Game {
    board: Board,
    snake: Snake,
    food: Vec<Position>,
    joints: Vec<Joint>,
}
