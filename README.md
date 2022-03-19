# MysticDraw

Back in the 90' I made my own ANSI drawing tool which I used in my BBS for drawing stuff. It supported TheDrawFonts and was a bit more "cool" than TD at that time.
I updated it from time to time - but nobody seemed to use it. Never got a single feedback on that.

Doesn't matter - I know why: Because it was not written in rust :). First I used Turbo Pascal, moved to C 1996? (I think), reworked it in C++ (and introduced a lot of bugs, but with the lack of users it didn't matter). Last update was 2003. So ~20 years later it's time for a 2.0 version.

Now I learn rust and as a medium sized UI project - why not ressurrect my 25 year old ANSI editor?

# Features

 * File formats: Ansi, Ascii, Artworx ADF, Avatar, BIN, XBIN, PCBoard, iCE, Tundra Draw
   - Ansi files can be written to be used in modern terminals on linux or macOs
 * Own custom file format: .mdf
 * Layer model & transparent pixels
 * Support for 256 char fonts of various sizes - fonts can be exported and imported.
 * Support for the draw fonts
 * Can open multiple files at once

# TODO

It's shaping up but still not finished… 
 * Tools
    * Brush & Shading need a configuration dialog
    * Outline mode for rectangle/circle/line
    * Pipette tool needs better functionality and look

 * Settings
    * Change F key sets - key sets should be able to be defined for font name.
    
 * 1.6 features missing
    * .TDF font editor (1.6 had this featuree)
    * Effects tool (1.6 had some custom effects)
    * Screen Font editor - no idea what I've thought at that time but that's out of scope. But Mystic Draw had that…
 * Moebius features missing
    * Mirror mode (super cool in my opinion)

# The Draw Fonts

Just open the font path (there is a button in the font tool for it) and copy .tdf files in there. After a restart they'll get picked up.
(Maybe I'll add an import button later but would require unzip, multiselection etc. to be useful)

I'm not shipping any .tdf fonts because I'm not sure about the copyright issues involved with it. However in the 1.6 branch there are some 'historical' .tdf fonts mystic draw was shipped with.
Searching with google I found Ta nice collection of 1094 TheDrawFonts:

http://www.roysac.com/thedrawfonts-tdf.html

Enjoy

If I get enough users I'll may be in the mood to bring back a the draw font creation tool. However the one in the MysticDraw 1.6 should still work.

# Building

* Get Linux - at least on mac the icons are not working. Even some GTK4 icons are missing.
* Get rust+gtk4 environment. There is 0 chance I can hold that up to date
  * Just go to https://gtk-rs.org/gtk4-rs/git/book/installation.html and follow that

* After that everything is really easy: type "cargo run" and it just works.