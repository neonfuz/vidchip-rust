Vidchip-Rust
============

# About

Rust version of vidchip, a simple graphics drawing interface for
games. Initially it will be a rust port of the current version of
vidchip, but eventually it will extend beyond this
functionality. Right now this is what I'm planning

# Why

Simplifying all the drawing down to a stream of events means we can do
a lot of cool things with said stream. We can run the game on a
different computer, possibly simplifying multiplayer.

# Timeline

1. Clone current c vidchip
 - This is done. Implementing the rest of the stdio packet based
   interface that existed is time wasting, might as well just jump to
   the next step.
2. +Add protobuf support+
 - protocal buffer support was suspended, probably going to use C ffi
   instead (for now, maybe protobuf support later).
3. 

# Architecture

Draw events will be protobuf objects, which can either be generated
locally or remotely, and sent to the video chip. Because the events
are protobuf objects, the game logic can be written in any language.
