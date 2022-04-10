use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use js_sys::Uint32Array;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
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
    reward_cell: Option<usize>,
    state: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, snake_size: usize) -> World {
        let snake = Snake::new(snake_idx, snake_size);
        let size = width * width;

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            state: None,
            points: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.state = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.state
    }

    pub fn game_status_text(&self) -> String {
        match self.state {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No status"),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn points(&self) -> usize {
        self.points
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
        let next_cell = self.gen_next_snake_cell(
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

        if let Some(GameStatus::Played) = self.state {
            for i in (1..self.snake.body.len()).rev() {
                self.snake.body[i] = self.snake.body[i - 1];
            }
    
            if let Some(next_cell) = self.next_cell {
                self.snake.body[0] = next_cell;
                self.next_cell = None;
            } else {
                self.snake.body[0] = self.gen_next_snake_cell(
                    self.snake_head_idx(), &self.snake.direction
                );
            }

            if self.snake.body[1..self.snake.body.len()].contains(
                &self.snake.body[0]
            ) {
                self.state = Some(GameStatus::Lost);
            }
    
            if self.reward_cell == Some(self.snake_head_idx()) {
                if self.snake.body.len() < self.size {
                    self.points += 1;
                    self.reward_cell = World::gen_reward_cell(
                        self.size, &self.snake.body
                    );
                } else {
                    self.reward_cell = None;
                    self.state = Some(GameStatus::Won);
                }
    
                self.snake.body.push(SnakeCell(self.snake.body[1].0));
            }    
        }
    }

    fn gen_next_snake_cell(
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

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;

        loop {
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(16, 2, 3)
    }
}