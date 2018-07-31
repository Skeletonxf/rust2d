local ffi = require 'ffi'
local loverust = require 'src.loverust'

local pong = {
  _DESCRIPTION = [[
A wrapper around the pong rust game.
This is a small example.
  ]],
  _URL = 'https://github.com/Skeletonxf/rust2d',
  _LICENSE = 'MPL2'
}

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

local arrays = require 'src.arrays'

-- These must be defined after loading the arrays module so
-- array_t is defined.
ffi.cdef[[
array_t pong_game_get_trail_x(pong_game_t *);
array_t pong_game_get_trail_y(pong_game_t *);
]]

-- These mirror the settings in the Rust code for the game
-- In a larger project the settings should be defined once and
-- read by both languages (possibly in a lua table and read by Rust
-- using a Lua crate)
local GAME_WIDTH = 1500
local GAME_HEIGHT = 1000
local PADDLE_GAP = 50
local PADDLE_HEIGHT = 150

local Pong = {}
Pong.__index = Pong

-- Gets y position of left player
function Pong.leftPlayer(self)
  return loverust.pong_game_get_left_player(self.pong)
end

-- Gets y position of right player
function Pong.rightPlayer(self)
  return loverust.pong_game_get_right_player(self.pong)
end

-- Gets position of pong ball
function Pong.pongBall(self)
  local x = loverust.pong_game_get_pong_ball_x(self.pong)
  local y = loverust.pong_game_get_pong_ball_y(self.pong)
  return x, y
end

-- Gets game score
function Pong.score(self)
  local p1 = loverust.pong_game_get_left_player_score(self.pong)
  local p2 = loverust.pong_game_get_right_player_score(self.pong)
  return p1, p2
end

-- Frees a Pong instance
function Pong.free(self)
  loverust.pong_game_free(self.pong)
  self.pong = nil
end

-- Gets ball trail array
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

-- Updates the game state by one tick
function Pong.update(self, left, right)
  loverust.pong_game_update(self.pong, left.up, left.down, right.up, right.down)
end

-- Produces an AI player's moves for the next game tick
-- to be fed to the update function
function Pong.ai(self)
  local y = loverust.pong_game_get_right_player_ai_move(self.pong)
  return {
    up = self:rightPlayer() < (y - PADDLE_HEIGHT/3),
    down = self:rightPlayer() > (y + PADDLE_HEIGHT/3),
  }
end

-- Creates a Pong instance, wrapping the Rust code
function pong.new()
  local o = {}
  setmetatable(o, Pong)
  o.pong = loverust.pong_game_new()
  return o
end

return pong
