# engine
### a spaceship game engine in rust

#### goals
- API should have three main components: 1. configure front-end (set view size) 2. stage game changes, 3. commit game changes and return a new game state
- engine should be front-end agnostic
- game state should return all necessary information to paint the screen

#### game rules
- ships are composed of squares (like pixels), making them modular and able to be destroyed piece by piece
- ship acceleration is determined by weight (# of pixels) and engine capacity (# of engines)
- any chunk of ship attached to an engine is independently pilotable
- destroyed chunks of ships remain until shot at or flown through
- ship projectiles also move with acceleration/velocity (missiles, bullets)
