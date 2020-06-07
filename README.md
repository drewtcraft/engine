# engine
### a spaceship game engine in rust

#### game rules
- ships are composed of squares (like pixels), making them modular and able to be destroyed piece by piece
- ship acceleration is determined by weight (# of pixels) and engine capacity (# of engines)
- ships have velocity and do not slow down 
- any chunk of ship attached to an engine is independently pilotable
- destroyed chunks of ships remain until shot at or flown through
- ship projectiles also move with acceleration/velocity (missiles, bullets)
