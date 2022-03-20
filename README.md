# MysticDraw

Back in the 90' I made my own ANSI drawing tool which I used in my BBS for drawing stuff. It supported TheDrawFonts and was a bit more "cool" than TD at that time.
I updated it from time to time - but nobody seemed to use it. Never got a single feedback on that.

Doesn't matter - I know why: Because it was not written in rust :). First I used Turbo Pascal, moved to C 1996? (I think), reworked it in C++ (and introduced a lot of bugs, but with the lack of users it didn't matter). Last update was 2003. So ~20 years later it's time for a 2.0 version.

Now I learn rust and as a medium sized UI project - why not ressurrect my 25 year old ANSI editor?

![Screenshot](/assets/images/screenshot.png)

# How to get

https://github.com/mkrueger/MysticDraw/releases/tag/%23alpha

Just use the AppImage - it's a pre release. Need some break and user input. I can really need some help - I'm not an ansi artist.

# Features

 * File formats: Ansi, Ascii, Artworx ADF, Avatar, BIN, XBIN, PCBoard, iCE, Tundra Draw
   - Ansi files can be written to be used in modern terminals on linux or macOs
 * Own custom file format: .mdf
 * Layer model & transparent pixels - like modern graphic programs
 * Support for 256 char fonts of various sizes - fonts can be exported and imported.
 * Support for the draw fonts
 * Can open multiple files at once
 * Is able to create extended mode .XBIN files (afaik the only ANSI editor capable of that)

# Status

I would say it's alhpa software and ready for testing. However there are not many people around still drawing ANSI art and even fewer Linux users do…
So I suppose Mystic Draw 2.0 will have the same fate than it's predecessors…

I would really like to get feedback.

Some still awkward things:

+ The file settings can be opened by clicking on the line & column marker. I still need a good UI solution where to put that thing in.

# TODO

 * Tools
    * Brush & Shading need a configuration dialog
    * Outline mode for rectangle/circle/line
 * Settings
    * Change Function key sets not fully implemented
 * Performance. ANSI View and Minimap needs a rework
    - Minimap should render in background using multiple CPUs
    - ANSI View should just render the visible content not all. ATM it renders everything and scrolling is done by GTK.
      Scrolling needs to be customized and only the visible part should be rendered. However it feels "fast enoug" so it's something for a 2.1
    
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

![Screenshot](/assets/images/screenshot_font.png)

# Building

* Get Linux - at least on mac the icons are not working. Even some GTK4 icons are missing.
* Get rust+gtk4 environment. There is 0 chance I can hold that up to date
  * Just go to https://gtk-rs.org/gtk4-rs/git/book/installation.html and follow that

* After that everything is really easy: type "cargo run" and it just works.
