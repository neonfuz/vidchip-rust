Vidchip-Rust
============

# Intro

Rust version of vidchip, a simple graphics drawing interface for
games. Initially it will be a rust port of the current version of
vidchip, but eventually it will extend beyond this
functionality. Right now this is what I'm planning

# Timeline

1. Clone current c vidchip
 - This is done. Implementing the rest of the stdio packet based
   interface that existed is time wasting, might as well just jump to
   the next step.
2. Add protobuf support

# Architecture

Draw events will be protobuf objects, which can either be generated
locally or remotely, and sent to the video chip. Because the events
are protobuf objects, the game logic can be written in any language.
