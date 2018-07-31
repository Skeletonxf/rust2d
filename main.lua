local pong_game = require 'src.pong'
local pong

local music = nil

-- Load function is ran at love2d startup
function love.load()
  pong = pong_game.new()
  print("The audio track EzaOne - Supernova [Creative Commons] by Argofox is licensed under a Creative Commons License")
  print('EzaOne - Supernova: youtu.be/xZDYu5azS-c')
  music = love.audio.newSource("assets/Supernova.ogg", "stream")
  music:play()
end

-- Ticks the pong game each frame
-- This should use a fixed time step in a proper project.
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

-- Draws the game using the pong instance to access the
-- position of everything
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
