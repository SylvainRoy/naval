# Naval

## This is a Work in Progress

A naval video game in the making...
The game is built on top of Bevy and *will* be great (lots of work before that...).

## Roadmap

Speed should stay unchanged with no action form user. Possible value should go from -50% to +100%

Collisions should only be possible every X secs.

Canons on islands

Dummy enemies:

- design sprites
- local player controls

Animations

- Boat movements generate waves.

Bases with flags

Multiplayer via network

Limit visibility of the player

Better manage screen size

## Study on command line

### Single-player mode

    > naval 

### Multi-player mode

    > naval-server --ip <IP> --port <port>
    > naval --ip <IP> --port <port>

## Client-Server

30 FPS --> 33 ms between calls.

Events:

- reset
- set-map{tiles:[...], canons:[...]}
- canon{origin:(x, y), direction:a, energy:e}
- torpedo{origin:(x, y), direction: a}
- player{position:(x, y), direction:a, speed:s, life:l}
