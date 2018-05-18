extern crate libc;

use libc::uint32_t;

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
}

impl PongGameState {
    fn new() -> PongGameState {
        PongGameState {
            pong_ball: (500, 500),
            left_player: 500,
            right_player: 500,
            speed: 10,
            direction: (1, -1),
            score: (0, 0),
        }
    }

    /*
     * Ticks the game state by 1
     * The inputs can probably be more elegant but this is a toy example
     */
    fn update(&mut self, left_up: bool, left_down: bool, right_up: bool, right_down: bool) {
        // move paddles
        // each paddle has a height of 150 units (out of 1000)
        // the game is 1500 units wide
        // each paddle is 50 units from the side
        for _ in 0..12 {
            if left_up && !left_down {
                if self.left_player < (1000-75) {
                    self.left_player += 1;
                }
            }
            if left_down && !left_up {
                if self.left_player > 75 {
                    self.left_player -= 1;
                }
            }
            if right_up && !right_down {
                if self.right_player < (1000-75) {
                    self.right_player += 1;
                }
            }
            if right_down && !right_up {
                if self.right_player > 75 {
                    self.right_player -= 1;
                }
            }
        }
        for _ in 0..self.speed {
            // update the ball
            let (mut x, mut y) = self.pong_ball;
            let (mut dx, mut dy) = self.direction;
            if dx > 0 && x < 1500 {
                x += 1;
            }
            if dx < 0 && x > 0 {
                x -= 1;
            }
            if dy > 0 && y < 1000 {
                y += 1;
            }
            if dy < 0 && y > 0 {
                y -= 1;
            }
            if x == 1500 {
                dx = -1;
                x = 1400;
                y = 500;
                self.speed = 10;
                self.score = (self.score.0 + 1, self.score.1)
            }
            if x == 0 {
                dx = 1;
                x = 100;
                y = 500;
                self.speed = 10;
                self.score = (self.score.0, self.score.1 + 1)
            }
            if y == 1000 {
                dy = -1;
            }
            if y == 0 {
                dy = 1;
            }
            if self.left_player_hit() {
                dx = 1;
                if y > (self.left_player + 50) {
                    dy = 1;
                }
                if y < (self.left_player - 50) {
                    dy = -1;
                }
                self.speed += 1;
            }
            if self.right_player_hit() {
                dx = -1;
                if y > (self.right_player + 50) {
                    dy = 1;
                }
                if y < (self.right_player - 50) {
                    dy = -1;
                }
                self.speed += 1;
            }
            self.pong_ball = (x, y);
            self.direction = (dx, dy);
        }
    }

    fn left_player_hit(&self) -> bool {
        let (x, y) = self.pong_ball;
        if x == 50 {
            return ((self.left_player + 75) >= y) && ((self.left_player - 75) <= y)
        }
        false
    }

    fn right_player_hit(&self) -> bool {
        let (x, y) = self.pong_ball;
        if x == 1450 {
            return ((self.right_player + 75) >= y) && ((self.right_player - 75) <= y)
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
