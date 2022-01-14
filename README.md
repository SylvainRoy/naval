# Naval

## This is a Work in Progress

A naval video game in the making...
The game is built on top of Bevy and *will* be great (lots of work before that...).

## Todo

Background color could be: (0, 0.4118, .5804)

## Roadmap

Dummy enemies

- design sprites
- local player controls

Scene

- Architecture:
  - fixed sized pixels vs multi-layered background
- Design sprites or backgrounds

Animations:

- Boat movements generate waves.

Torpedos

Canons on islands

Bases with flags

Multiplayer via network

Limit visibility of the player

Manage screen size

## Scene design

### Solution 1 - Tile based

Tiles types:

- Tile 1 is Sea
- Tile 1 is Ground
- Tile 1 is Mountains

Boats cannot go on 2 and 3.
Cannonballs cannot on 3, they explode.

Pros:

- Simpler

Cons:

- More sprites. Slower?

### Solution 2 - Multi-layered background

- Layer 0 is bottom of the sea
- Layer 1 is see surface
- Layer 2 is ground
- Layer 3 is mountains

Boat cannot go on 2 (then 3), they are stopped (or sink?).
Canonballs cannot go on 3, they explode.

Layer 1 can be generated to fill the gaps left by 2.
Layer 1 should come with some transparency.

Layer 0 is optionnal.

Questions:

- How to manage various screen size?
- How to generate maps on the fly?
- How to generate maps manually?

Pros:

- Could be much better looking.

Cons:

- More difficult to generate.
