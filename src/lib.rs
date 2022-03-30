use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            snake: Snake::new(snake_idx),
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

    pub fn update(&mut self) {
        if self.snake.body.is_empty() {
            return
        }

        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);

        let (row, col) = match self.snake.direction {
            Direction::Up => {
                ((row - 1) % self.width, col)
            },

            Direction::Right => {
                (row, (snake_idx + 1) % self.width)
            },

            Direction::Down => {
                ((row + 1) % self.width, col)
            },

            Direction::Left => {
                (row, (snake_idx - 1) % self.width)
            },
        };

        self.set_snake_head(self.cell_to_index(row, col));
    }

    #[inline]
    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    #[inline]
    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    #[inline]
    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(16, 2)
    }
}