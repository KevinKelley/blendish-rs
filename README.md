blendish-rs
===========

a small, themed, UI toolkit for Rust, using NanoVG vector-graphics library

This is a port, and refactoring, of the single-header Blendish/OUI C code
by duangle (link below).  I started by wrapping the static lib, but found I
wanted to 'oxidize' it in the process.

Work in progress.  I have some example code running, but not complete.
Resources aren't loaded yet, for instance, but that will be soon.


Blendish
========
Blendish is a simple, vector-graphics, widget-drawing library built using
NanoVG.  It's an 'immediate-mode' library -- there's no concept of long-lived
UI objects here; you draw your UI on demand, in the render/update cycle.

The companion library 'OUI' will handle some of that: I've started with it too.


Links
=====
- [Blendish](https://bitbucket.org/duangle/blendish)
