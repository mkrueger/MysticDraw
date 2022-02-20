#ifndef BIO_CRT__H
#define BIO_CRT__H

#include<stdarg.h>

#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cctype>
#include<vector>
#include<iostream>
#include<algorithm>

#include<SDL/SDL.h>

#define LINES 25
#define COLS  80

using namespace std;

#define MOUSE_LEFTBUTTON  4
#define MOUSE_MIDBUTTON   2
#define MOUSE_RIGHTBUTTON 1

class ansoutbuf : public std::streambuf
{
	protected:
		virtual int overflow(int ch);
};

class ansout_impl : public std::ostream
{
	public:
		ansout_impl() : std::ostream(new ansoutbuf)
		{}
};

// own stream manipulators
struct gotoxy
{
	int x, y;
	
	gotoxy(int x, int y) : x(x), y(y)
	{}
};

struct textattr
{
	int attr;
	
	textattr(int attr) : attr(attr)
	{}
};

struct clrscrn
{
};

// overloaded operators
extern ostream& operator<<(ostream& os, const gotoxy& gxy);
extern ostream& operator<<(ostream& os, const textattr& tattr);
extern ostream& operator<<(ostream& os, const clrscrn& clrscrn);


// underlying screen engine
class ScreenEngine
{
	private:
		Uint8* default_font_data;
		Uint8* font_data;
		Uint8* screen_data;
		Uint8* palette;
		
		Uint32 width;
		Uint32 height;
		Uint32 lineLength; // width * 2
		
		bool extendedFontMode;
		
		// blink
		Uint32 next_blink;
		bool blinkOn;
		bool blinkMode;
		
		// caret
		Uint32 caretX, caretY;
		unsigned char curAttr;
		Uint32 caretBlink; 
		bool caretVisible;
		
		Uint32 getpixel(SDL_Surface *surface, int x, int y);
		void putpixel(SDL_Surface *surface, int x, int y, Uint32 pixel);
		
		void printChar(int xPos, int yPos, unsigned char ch, unsigned char attr);
	public:
		ScreenEngine();
		Uint32& getCaretX();
		Uint32& getCaretY();
		Uint8& getAttribute();
		
		bool& getExtendedFontMode();
		bool& getBlinkMode();
		
		Uint8* getFont()
		{
			return font_data;
		}
		
		Uint8* getDefaultFont()
		{
			return default_font_data;
		}
		Uint8* getDefaultPalette();
		
		Uint8* getPalette()
		{
			return palette;
		}
		
		void setFont(Uint8* font)
		{
			font_data = font; 
		}
		
		void setPalette(Uint8* pal)
		{
			palette = pal; 
		}
		void LoadFont(char* args[]);
		
		Uint8& getCharacterXY(int column, int row);
		Uint8& getAttributeXY(int column, int row);
		
		void putChar(unsigned char ch);
		void setStandardPalette();
		
		
		void clearScreen()
		{
			fill_n(screen_data, height * lineLength, 0);
		}
		
		void xorCaret();
		void update();
		
		void showScreen();
		
		~ScreenEngine();
};

extern ScreenEngine screenEngine;
extern ansout_impl  ansout;
extern bool         MouseSupport;
extern SDL_Surface* screen;

extern void init_bio_crt();
extern void exit_bio_crt();

extern char* inputfield(char* str, unsigned int length, int x1, int y);
extern void CoolWrite(char* a);
extern void CodeWrite(char* a);
extern void DrawBox (int x1, int y1, int x2, int y2); /* Draws a nice looking box ... */
extern int chooser(int col, int first, ...);

#ifdef HAS_GPM
	#include<gpm.h>
	extern int mouse_x;
	extern int mouse_y;
	extern int mouse_button;
	extern int mouse_deltax;
	extern int mouse_deltay;
	extern char mouse_activ;
	
	extern int mouse_getx();
	extern int mouse_gety();
	extern int mouse_getbutton();
	
	extern int mouse_handler(Gpm_Event *event,void *data);    
	extern void showmouse();
	extern void hidemouse();
	extern void mouse_update();
	
	extern int mouse_init();
	extern void mouse_close();
#endif

#endif
