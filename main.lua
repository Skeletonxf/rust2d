ffi = require 'ffi'

ffi.cdef[[
typedef struct pong_game_S pong_game_t;
pong_game_t * pong_game_new();
void pong_game_free(pong_game_t *);
void pong_game_update(pong_game_t *, bool left_up, bool left_down, bool right_up, bool right_down);
uint32_t pong_game_get_left_player(pong_game_t *);
uint32_t pong_game_get_right_player(pong_game_t *);
uint32_t pong_game_get_pong_ball_x(pong_game_t *);
uint32_t pong_game_get_pong_ball_y(pong_game_t *);
uint32_t pong_game_get_left_player_score(pong_game_t *);
uint32_t pong_game_get_right_player_score(pong_game_t *);
]]

local loverust = ffi.load('./target/release/libloverust.so')

local pong = {}
local Pong = {}
Pong.__index = Pong
function Pong.leftPlayer(self)
  return loverust.pong_game_get_left_player(self.pong)
end
function Pong.rightPlayer(self)
  return loverust.pong_game_get_right_player(self.pong)
end
function Pong.pongBall(self)
  local x = loverust.pong_game_get_pong_ball_x(self.pong)
  local y = loverust.pong_game_get_pong_ball_y(self.pong)
  return x, y
end
function Pong.score(self)
  local x = loverust.pong_game_get_left_player_score(self.pong)
  local y = loverust.pong_game_get_right_player_score(self.pong)
  return x, y
end
function Pong.free(self)
  loverust.pong_game_free(self.pong)
  self.pong = nil
end
function Pong.update(self, left, right)
  loverust.pong_game_update(self.pong, left.up, left.down, right.up, right.down)
end
setmetatable(pong, Pong)

function love.load()
  pong.pong = loverust.pong_game_new()
end

function love.update()
  local bx, by = pong:pongBall()
  pong:update({
    up = love.keyboard.isDown("w"),
    down = love.keyboard.isDown("s"),
  }, {
    up = pong:rightPlayer() < by,
    down = pong:rightPlayer() > by,
  })
end

local font = love.graphics.getFont()

function love.draw()
  local window = {
    w = love.graphics.getWidth(),
    h = love.graphics.getHeight(),
  }
  local paddleHalfHeight = 75/1000 * window.h
  local leftPlayer = {
    x = (50/1500) * window.w,
    y = (1 - (pong:leftPlayer()/1000)) * window.h,
    h = paddleHalfHeight,
  }
  love.graphics.line(leftPlayer.x, leftPlayer.y - leftPlayer.h, leftPlayer.x, leftPlayer.y + leftPlayer.h)
  local rightPlayer = {
    x = (1450/1500) * window.w,
    y = (1 - (pong:rightPlayer()/1000)) * window.h,
    h = paddleHalfHeight,
  }
  love.graphics.line(rightPlayer.x, rightPlayer.y - rightPlayer.h, rightPlayer.x, rightPlayer.y + rightPlayer.h)
  local bx, by = pong:pongBall()
  local pongBall = {
    x = (bx/1500) * window.w,
    y = (1 - (by/1000)) * window.h,
  }
  love.graphics.circle('line', pongBall.x, pongBall.y, 5)
  local p1, p2 = pong:score()
  local score = tostring(p1) .. ' : ' .. tostring(p2)
  love.graphics.print(score, window.w/2, 0, 0, 1, 1, font:getWidth(score)/2)
end
