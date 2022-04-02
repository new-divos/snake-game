use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use js_sys::Uint32Array;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy, PartialEq)]
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, snake_size: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, snake_size),
            next_cell: None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        if !self.snake.body.is_empty() {
            self.snake.body[0].0
        }
        else {
            usize::MAX
        }
    }

    pub fn snake_cells(&self) -> Uint32Array {
        Uint32Array::from(
            self.snake.body.iter()
                .map(|item| item.0 as u32)
                .collect::<Vec<u32>>()
                .as_ref()
        )
    }

    pub fn change_shake_dir(&mut self, direction: Direction) {
        let next_cell = self.next_snake_cell(
            self.snake_head_idx(), &direction
        );

        if self.snake.body[1] == next_cell {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn step(&mut self) {
        if self.snake.body.is_empty() {
            return
        }

        for i in (1..self.snake.body.len()).rev() {
            self.snake.body[i] = self.snake.body[i - 1];
        }

        if let Some(next_cell) = self.next_cell {
            self.snake.body[0] = next_cell;
            self.next_cell = None;
        } else {
            self.snake.body[0] = self.next_snake_cell(
                self.snake_head_idx(), &self.snake.direction
            );
        }
    }

    fn next_snake_cell(
        &self, snake_idx: usize, direction: &Direction
    ) -> SnakeCell {
        let row = snake_idx / self.width;

        match direction {
            Direction::Up => {
                let treshold = snake_idx - row * self.width;
                if snake_idx == treshold {
                    SnakeCell(self.size - self.width + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },

            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },

            Direction::Down => {
                let treshold = snake_idx + (self.width - row) * self.width;
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },

            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + self.width - 1)
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(16, 2, 3)
    }
}