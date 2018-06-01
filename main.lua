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
uint32_t pong_game_get_right_player_ai_move(pong_game_t *);
]]

loverust = ffi.load('./target/release/libloverust.so')

local arrays = require 'src.arrays'

ffi.cdef[[
array_t pong_game_get_trail_x(pong_game_t *);
array_t pong_game_get_trail_y(pong_game_t *);
]]

local GAME_WIDTH = 1500
local GAME_HEIGHT = 1000
local PADDLE_GAP = 50
local PADDLE_HEIGHT = 150

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
  local p1 = loverust.pong_game_get_left_player_score(self.pong)
  local p2 = loverust.pong_game_get_right_player_score(self.pong)
  return p1, p2
end
function Pong.free(self)
  loverust.pong_game_free(self.pong)
  self.pong = nil
end
function Pong.trail(self)
  -- get the two arrays from the pong game
  local trail = {
    x = arrays.new(loverust.pong_game_get_trail_x(self.pong)),
    y = arrays.new(loverust.pong_game_get_trail_y(self.pong)),
  }
  -- copy them into the trail table array portion
  local length = trail.x:length()
  for i = 0, (length - 1) do
    trail[i+1] = {
      x = trail.x:get(i),
      y = trail.y:get(i),
    }
  end
  -- free the array structs
  trail.x:free()
  trail.y:free()
  -- cache the length
  trail.length = #trail
  -- return the table of tables of coordinates
  return trail
end
function Pong.update(self, left, right)
  loverust.pong_game_update(self.pong, left.up, left.down, right.up, right.down)
end
function Pong.ai(self)
  local y = loverust.pong_game_get_right_player_ai_move(self.pong)
  return {
    up = pong:rightPlayer() < (y - PADDLE_HEIGHT/3),
    down = pong:rightPlayer() > (y + PADDLE_HEIGHT/3),
  }
end
setmetatable(pong, Pong)

local music = nil

function love.load()
  pong.pong = loverust.pong_game_new()

  print("The audio track EzaOne - Supernova [Creative Commons] by Argofox is licensed under a Creative Commons License")
  print('EzaOne - Supernova: youtu.be/xZDYu5azS-c')
  music = love.audio.newSource("assets/Supernova.ogg", "stream")
  music:play()
end

function love.update()
  local bx, by = pong:pongBall()
  local _, scoreBefore = pong:score()

  pong:update({
    up = love.keyboard.isDown("w"),
    down = love.keyboard.isDown("s"),
  }, pong:ai())

  -- reset the music on the player losing a point
  local _, scoreAfter =  pong:score()
  if scoreAfter > scoreBefore then
    music:seek(0.1)
  end
end

local font = love.graphics.getFont()

local draw = {}

function love.draw()
  love.graphics.setColor(1, 1, 1, 1)
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
  draw.ball(bx, by, 5, window)
  local p1, p2 = pong:score()
  local score = tostring(p1) .. ' : ' .. tostring(p2)
  love.graphics.print(score, window.w/2, 0, 0, 1, 1, font:getWidth(score)/2)
  local trail = pong:trail()
  for k, b in ipairs(trail) do
    love.graphics.setColor(1, 1, 1, math.max(0.95 - 0.005*(trail.length-k), 0.01))
    draw.ball(b.x, b.y, 3, window)
  end
end

-- converts from pong game coordinates of the pong ball to screen ones
-- and draws at this location
function draw.ball(bx, by, size, window)
  local pongBall = {
    x = (bx/1500) * window.w,
    y = (1 - (by/1000)) * window.h,
  }
  love.graphics.circle('line', pongBall.x, pongBall.y, size)
end
