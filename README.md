# MysticDraw

Back in the 90' I made my own ANSI drawing tool which I used in my BBS for drawing stuff. It supported TheDrawFonts and was a bit more "cool" than TD at that time.
I updated it from time to time - but nobody seemed to use it. Never got a single feedback on that.

Doesn't matter - I know why: Because it was not written in rust :). First I used Turbo Pascal, moved to C 1996? (I think), reworked it in C++ (and introduced a lot of bugs, but with the lack of users it didn't matter).

Now I learn rust and as a medium sized UI project - why not ressurrect my 25 year old ANSI editor?

# Features

 * File formats: Ansi, Ascii, Artworx ADF, Avatar, BIN, XBIN, PCBoard, iCE, Tundra Draw
 * Own custom file format: .mdf
 * Layer model & transparent pixels
 * Support for 256 char fonts of various sizes - fonts can be exported and imported.
 * Support for the draw fonts
 * Can open multiple files at once

# TODO

It's still in it's infant stages. I wouldn't call that 2.0. But I can start to listen to input if you're keen enough to try it.

 * ANSI & Minimap controls need a rework, they're only placeholders. Minimap crashes if file is too large.
 * Extended char set support (512 chars - everything is ready for it, would help to have some .xbin
   files that contain an extended char set - I couldn't find any)
 * Color Picker that doesn't suck
 * Switch for iCE/Blink (atm it's only in iCE Mode)
 * TDF font tool needs UI - however it basically works

 * Tools
    * Brush & Shading need a configuration dialog
    * Outline mode for rectangle/circle/line
    * Pipette tool needs better functionality and look

 * Settings
    * Change F key sets - key sets should be able to be defined for font name.
    * Set outline format for tdf outline fonts
    
 * 1.6 features missing
    * .TDF font editor (1.6 had this featuree)
    * Effects tool (1.6 had some custom effects)
 * Moebius features missing
    * Mirror mode (super cool in my opinion)

# Building

Todo. Short:

* get rust
* get a gtk4 rust environment running
* type "cargo run" and it just works.