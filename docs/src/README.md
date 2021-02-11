# rs-pong

rs-pong is an experimental and educational implementation of pong.
This implementation uses interpolation rather than psudo physics.

## Full Design Breakdown

- Screen (1)
- Game Field (1)
- Ball Tracker (1)
  - A mechanism that attempts to keep up with the position of the ball.
  - Mechanism is artificially restricted to make it not keep up sometimes.
  - Mechanism is used as an input.
- Ball (1)
  - Has a velocity
  - Can acquire velocity from a paddle.
  - Can bounce off of a wall.
- Paddles (2)
  - Can be moved perpendicular to their forward face.
  - Can be controlled through an input.
    - Mouse input
    - Keyboard input
    - Ball Tracker input
  - Can interact with the Ball.
  - Will imbue velocity to the ball if the paddle has velocity when the Ball is reflected.
- Wall (4)
  - Two types of wall:
    - Termination
      - Termination wall is owned by a player.
      - When a termination wall and a ball interact, the owning placer is awarded a point.
    - Reflector
      - Reflector wall is not owned by a player.
      - When a reflector wall and a ball interact, the ball's velocity is mirrored across the forward of the wall.
  - Wall has a position, a length, and an orientation.
- Ball Mover
  - A mechanism that determines how the ball is moved about the field.
  - Moves the ball with interpolation, not velocity.
  - If the end of an interpolation is reached, the type of wall at the end is used to trigger the state.
  - Interpolation may be interrupted by a paddle.
- Replay Engine
  - A mechanism that keeps track of every important tick of the game.
  - Ball vector changes.
  - Ball vector magnitude changes.
  - Ball terminations.
