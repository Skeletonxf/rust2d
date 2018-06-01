extern crate libc;

use libc::uint32_t;

use arrays;
use arrays::Array;

pub struct PongGameState {
    // x and y coordinates of pong ball
    pong_ball: (u32, u32),
    // y coordinate of left paddle
    left_player: u32,
    // y coordinate of right paddle
    right_player: u32,
    // how many times to apply unit vector in one tick
    speed: u32,
    // unit vector direction of pong ball
    direction: (i8, i8),
    // the score for each player
    score: (u32, u32),
    trail: Vec<(u32, u32)>,
    right_player_ai_target: Option<u32>,
}

const PADDLE_HEIGHT: u32 = 150;
const GAME_WIDTH: u32 = 1500;
const GAME_HEIGHT: u32 = 1000;
const PADDLE_GAP: u32 = 50;
const MAX_TRAIL_LENGTH: usize = 10000;

impl PongGameState {
    fn new() -> PongGameState {
        PongGameState {
            pong_ball: (GAME_WIDTH/2, GAME_HEIGHT/2),
            left_player: GAME_HEIGHT/2,
            right_player: GAME_HEIGHT/2,
            speed: 10,
            direction: (1, -1),
            score: (0, 0),
            trail: vec![],
            right_player_ai_target: None,
        }
    }

    /*
     * Ticks the game state by 1
     * The inputs can probably be more elegant but this is a toy example
     */
    fn update(&mut self, left_up: bool, left_down: bool, right_up: bool, right_down: bool) {
        // move paddles
        for _ in 0..12 {
            if left_up && !left_down {
                if self.left_player < (GAME_HEIGHT-(PADDLE_HEIGHT/2)) {
                    self.left_player += 1;
                }
            }
            if left_down && !left_up {
                if self.left_player > PADDLE_HEIGHT/2 {
                    self.left_player -= 1;
                }
            }
            if right_up && !right_down {
                if self.right_player < (GAME_HEIGHT-(PADDLE_HEIGHT/2)) {
                    self.right_player += 1;
                }
            }
            if right_down && !right_up {
                if self.right_player > PADDLE_HEIGHT/2 {
                    self.right_player -= 1;
                }
            }
        }
        for _ in 0..self.speed {
            // update the ball
            let (mut x, mut y) = self.pong_ball;
            let (mut dx, mut dy) = self.direction;
            if dx > 0 && x < GAME_WIDTH {
                x += 1;
            }
            if dx < 0 && x > 0 {
                x -= 1;
            }
            if dy > 0 && y < GAME_HEIGHT {
                y += 1;
            }
            if dy < 0 && y > 0 {
                y -= 1;
            }
            if x == GAME_WIDTH {
                dx = -1;
                x = GAME_WIDTH - 100;
                y = GAME_HEIGHT/2;
                self.speed = 10;
                self.score = (self.score.0 + 1, self.score.1);
                self.trail = vec![];
            }
            if x == 0 {
                dx = 1;
                x = 100;
                y = GAME_HEIGHT/2;
                self.speed = 10;
                self.score = (self.score.0, self.score.1 + 1);
                self.trail = vec![];
            }
            if y == GAME_HEIGHT {
                dy = -1;
            }
            if y == 0 {
                dy = 1;
            }
            if self.left_player_hit() {
                dx = 1;
                if y > (self.left_player + PADDLE_HEIGHT/6) {
                    dy = 1;
                }
                if y < (self.left_player - PADDLE_HEIGHT/6) {
                    dy = -1;
                }
                self.speed += 1;
            }
            if self.right_player_hit() {
                dx = -1;
                if y > (self.right_player + PADDLE_HEIGHT/6) {
                    dy = 1;
                }
                if y < (self.right_player - PADDLE_HEIGHT/6) {
                    dy = -1;
                }
                self.speed += 1;
            }
            self.pong_ball = (x, y);
            if self.trail.len() > MAX_TRAIL_LENGTH {
                // possibly an Array was not the smartest data type for something
                // which should have O(1) appending to the end
                // and O(1) truncating the start
                // but this still prevents the game incresing in memory usage
                // indefintely if the player moves into a safe spot
                // which is probably worse
                self.trail.remove(0);
            }
            self.trail.push(self.pong_ball);
            self.direction = (dx, dy);
        }
    }

    // returns target y for the right player as a basic AI
    fn right_player_ai(&mut self) -> u32 {
        let (x, y) = self.pong_ball;
        let (dx, dy) = self.direction;
        if dx > 0 {
            // cache this prediction so it doesn't need to be re calculated each frame
            match self.right_player_ai_target {
                Some(y) => return y,
                None => {
                    let mut x = x;
                    let mut y = y;
                    let mut dy = dy;
                    // model the ball physics up to where the paddle needs to be
                    while x < (GAME_WIDTH - PADDLE_GAP) {
                        x += 1;
                        if dy > 0 && y < GAME_HEIGHT {
                            y += 1;
                        }
                        if dy < 0 && y > 0 {
                            y -= 1;
                        }
                        if y == GAME_HEIGHT {
                            dy = -1;
                        }
                        if y == 0 {
                            dy = 1;
                        }
                    }
                    self.right_player_ai_target = Some(y);
                    return y
                }
            }
        } else {
            self.right_player_ai_target = None;
            return GAME_HEIGHT/2
        }
    }

    fn left_player_hit(&self) -> bool {
        let (x, y) = self.pong_ball;
        let paddle = self.left_player;
        if x == PADDLE_GAP {
            return ((paddle + PADDLE_HEIGHT/2) >= y) && ((paddle - PADDLE_HEIGHT/2) <= y)
        }
        false
    }

    fn right_player_hit(&self) -> bool {
        let (x, y) = self.pong_ball;
        let paddle = self.right_player;
        if x == (GAME_WIDTH - PADDLE_GAP) {
            return ((paddle + PADDLE_HEIGHT/2) >= y) && ((paddle - PADDLE_HEIGHT/2) <= y)
        }
        false
    }

    fn left_player(&self) -> u32 {
        self.left_player
    }

    fn right_player(&self) -> u32 {
        self.right_player
    }

    fn pong_ball(&self) -> (u32, u32) {
        self.pong_ball
    }

    fn score(&self) -> (u32, u32) {
        self.score
    }

    fn pong_trail_x(&self) -> Array {
        arrays::array_from_vector(self.trail.iter().map(|&c| c.0).collect())
    }

    fn pong_trail_y(&self) -> Array {
        arrays::array_from_vector(self.trail.iter().map(|&c| c.1).collect())
    }
}

#[no_mangle]
pub extern fn pong_game_new() -> *mut PongGameState {
    Box::into_raw(Box::new(PongGameState::new()))
}

#[no_mangle]
pub extern fn pong_game_free(pointer: *mut PongGameState) {
    if pointer.is_null() {
        eprintln!("Expected pointer to PongGameState to not be null");
        return
    }
    unsafe {
        // take back to allow it to be freed when exiting this block
        Box::from_raw(pointer);
    }
}

#[no_mangle]
pub extern fn pong_game_update(pointer: *mut PongGameState,
    left_up: bool, left_down: bool, right_up: bool, right_down: bool) {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.update(left_up, left_down, right_up, right_down);
}

#[no_mangle]
pub extern fn pong_game_get_trail_x(pointer: *mut PongGameState) -> Array {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.pong_trail_x()
}

#[no_mangle]
pub extern fn pong_game_get_trail_y(pointer: *mut PongGameState) -> Array {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.pong_trail_y()
}

#[no_mangle]
pub extern fn pong_game_get_left_player(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.left_player()
}

#[no_mangle]
pub extern fn pong_game_get_right_player(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.right_player()
}

#[no_mangle]
pub extern fn pong_game_get_pong_ball_x(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.pong_ball().0
}

#[no_mangle]
pub extern fn pong_game_get_pong_ball_y(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.pong_ball().1
}

#[no_mangle]
pub extern fn pong_game_get_left_player_score(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.score().0
}

#[no_mangle]
pub extern fn pong_game_get_right_player_score(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.score().1
}

#[no_mangle]
pub extern fn pong_game_get_right_player_ai_move(pointer: *mut PongGameState) -> uint32_t {
    let pong_game = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    pong_game.right_player_ai()
}
