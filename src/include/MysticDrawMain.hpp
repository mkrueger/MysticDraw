#ifndef MYSTICDRAWMAIN__HPP
#define MYSTICDRAWMAIN__HPP

#include<ctype.h>
#include<string.h>
#include<stdlib.h>
#include <sys/types.h> 
#include<fstream>
#include<iostream>
#include<algorithm>
#include<unistd.h>
#include<string>
#include<iomanip>
#ifdef HAS_SVGALIB
#include<vga.h>
#endif

#include<bio_crt.h>

#include<ScreenBuffer.hpp>
#include<command.hpp>
#include<HelpCommand.hpp>
#include<ASCIITableCommand.hpp>
#include<TabulatorCommand.hpp>
#include<SelectFontCommand.hpp>
#include<SelectSauceCommand.hpp>
#include<SelectEffectModeCommand.hpp>
#include<SelectColorCommand.hpp>
#include<FontEditorCommand.hpp>
#include<PaletteEditorCommand.hpp>
#include<BlockModeCommand.hpp>
#include<DrawCommand.hpp>
#include<ViewModeCommand.hpp>
#include<sauce.hpp>
#include<fonts.hpp>

#define UNDOPage 0
#define COPYPage 3

extern char* CharSet[12];
extern unsigned char tabs[80];
extern unsigned char CursorPos[80];
extern unsigned char cpos;
extern unsigned char Attribute;
extern bool Undo, FontTyped;
extern unsigned char ActiveCharset;
extern bool FullScreen;


struct teffekt  {
   int Effekt;
   unsigned char colorTable[5][10];
   unsigned char* getColorTable()
   {
	   return colorTable[Effekt];
   }
};

extern teffekt effect;

using namespace std;
class Caret
{
	private:
		int caretX;
		int caretY;
		int upperLeftCornerLine;
		int upperLeftCornerRow;
		
		bool isInInsertMode;
		bool isInEliteMode;
		bool isInFontMode;
	public:
		Caret()
		{
			caretX = 0;
			caretY = 0;
			upperLeftCornerLine = 0;
			upperLeftCornerRow  = 0;
			isInInsertMode = false;
			isInEliteMode = false;
		}
		
		int& getX()
		{
			return caretX;
		}
		
		int& getY()
		{
			return caretY;
		}
		
		int& getUpperLeftCornerLine()
		{
			return upperLeftCornerLine;
		}
		
		int& getUpperLeftCornerRow()
		{
			return upperLeftCornerRow;
		}
		
		int getLogicalY()
		{
			return caretY + upperLeftCornerLine;
		}
		
		int getLogicalX()
		{
			return caretX + upperLeftCornerRow;
		}
		
		bool& insertMode()
		{
			// guard against 'stange' insertMode values (like insertMode == 6 which switches between 6/7)
			if (isInInsertMode) {
				isInInsertMode = true;
			}
				
			return isInInsertMode;
		}
		
		bool& eliteMode()
		{
			// guard against 'stange' eliteMode values (like eliteMode == 6 which switches between 6/7)
			if (isInEliteMode) {
				isInEliteMode = true;
			}
			return isInEliteMode;
		}
		
		bool& fontMode()
		{
			// guard against 'stange' eliteMode values (like eliteMode == 6 which switches between 6/7)
			if (isInFontMode) {
				isInFontMode = true;
			}
			return isInFontMode;
		}
		
		void checkCaretPosition();
		
		bool handleKeyStroke(SDL_Event* event)
		{
#ifdef HAS_GPM
			if (MouseSupport) {      
				mouse_update();
				caretX += mouse_deltax;
				caretY += mouse_deltay;
				mouse_deltax=0;
				mouse_deltay=0;
			}
#endif
			switch (event->key.keysym.sym) {   
				case SDLK_DOWN:
					caretY++;
					checkCaretPosition();
					return true;
				case SDLK_UP:
					caretY--;
					checkCaretPosition();
					return true;
				case SDLK_LEFT:
					caretX--;
					checkCaretPosition();
					return true;
				case SDLK_RIGHT:
					caretX++;
					checkCaretPosition();
					return true;
				case SDLK_PAGEDOWN:
					upperLeftCornerLine += LINES-1;
					if (FullScreen) {
						caretY = LINES-1;
					} else {
						caretY=LINES-2;
					}
					checkCaretPosition();
					return true;
				case SDLK_PAGEUP:
					upperLeftCornerLine -= LINES-1;
					caretY=0;
					checkCaretPosition();
					return true;
				case SDLK_HOME:
					caretX = 0;
					checkCaretPosition();
					return true;
				case SDLK_END:
					caretX = 79;
					checkCaretPosition();
					return true;
				default:
					break;
			}
			if (event->key.keysym.mod & KMOD_SHIFT) {
				switch (event->key.keysym.sym) {   
					case SDLK_F1:
						ActiveCharset=1;
						return true;
					case SDLK_F2:
						ActiveCharset=2;
						return true;
					case SDLK_F3:
						ActiveCharset=3;
						return true;
					case SDLK_F4:
						ActiveCharset=4;
						return true;
					case SDLK_F5:
						ActiveCharset=5;
						return true;
					case SDLK_F6:
						ActiveCharset=6;
						return true;
					case SDLK_F7:
						ActiveCharset=7;
						return true;
					case SDLK_F8:
						ActiveCharset=8;
						return true;
					case SDLK_F9:
						ActiveCharset=9;
						return true;
					case SDLK_F10:
						ActiveCharset=10;
						return true;
					default:
						break;
				}
			}
			return false;
		}
};

class MysticDrawMain
{
	private:
		static MysticDrawMain mysticDrawMain;
		int    currentBuffer;
		
		bool   done;
		ScreenBuffer** screen;
		
		Caret caret;
		bool configrationLoaded;
		
		const string getConfigurationFileName()
		{
#if WIN32
			return "mdraw.config";
#else
			return string(getenv("HOME")) + "/.mdraw/mdraw.config";
#endif		
		}
		
		void loadconfig();
		void saveconfig();
		
		void renderFontCharacter(char c);
	public:
		char** args;

		static MysticDrawMain& getInstance();
		
		MysticDrawMain()
		{
			const int maxScreens = 4;
			screen = new ScreenBuffer*[maxScreens];
			for (int i = 0; i < maxScreens; ++i) {
				screen[i] = new ScreenBuffer();
			}
			
			currentBuffer = 0;
		}
		
		ScreenBuffer* getCurrentBuffer()
		{
			return screen[currentBuffer];
		}
		
		ScreenBuffer* getUndoBuffer()
		{
			return screen[3];
		}
		
		int& getCurrentBufferNumber()
		{
			return currentBuffer;
		}
		
		Caret& getCaret()
		{
			return caret;
		}
		
		void startMysticDraw(int argnum, char* args[]);
		void drawStatusLine();
		void updateColorStatus(unsigned char color);
		
		void ClearMessageLine();
		void drawScreen(int startLine, int endLine);
		void typeCharacter(unsigned char ch);
		char readCharacter();
		
		void exitMysticDraw();
		
		~MysticDrawMain()
		{
			delete [] screen;
		}
};


#endif
