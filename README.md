blendish-rs
===========

a small, themed, UI toolkit for Rust, using NanoVG vector-graphics library

This is a port, and refactoring, of the single-header Blendish/OUI C code
by duangle (link below).  I started by wrapping the static lib, but found I
wanted to 'oxidize' it in the process.

Work in progress.  Blendish port is done, and demo runs, there might be
some glitches.  Needs some refactoring to be more appropriate for Rust.


Blendish
========
Blendish is a simple, vector-graphics, widget-drawing library built using
NanoVG.  It's an 'immediate-mode' library -- there's no concept of long-lived
UI objects here; you draw your UI on demand, in the render/update cycle.

The companion library 'OUI' will handle some of that: I've started with it too.


Links
=====
- [Blendish](https://bitbucket.org/duangle/blendish)
