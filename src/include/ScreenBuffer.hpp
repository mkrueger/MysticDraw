#ifndef SCREENBUFFER_HPP
#define SCREENBUFFER_HPP

#include<sauce.hpp>
#include<stack>
#include<algorithm>
#include<cctype>
#include<iostream>

using namespace std;

enum ScreenFileFormat {
	Ansi,
	Ascii,
	Avatar,
	Binary,
	XBinary,
	C,
	PCBoard
};

class ScreenBlock
{
	protected:
		unsigned char* data;
		int width;
		int height;
		int lineLength;
	public:
		ScreenBlock(int width, int height);
		
		unsigned char* getData();
		unsigned char& getCharacter(int row, int column);
		unsigned char& getAttribute(int row, int column);
		
		int getHeight();
		int getWidth();
		int getLineLength();
		
		void flipY();
		void flipX();
		
		~ScreenBlock();
};

class ScreenBuffer : public ScreenBlock
{
	private:
		Sauce sauce;
		bool  saveSauce;
		
		bool blinkMode;
		bool extendedCharMode;
		unsigned char* font;
		unsigned char* palette;
		
		unsigned char display_ansi(char ch);
		
	public:
		
		Sauce&  getSauce()
		{
			return sauce;
		}
		
		bool& doSaveSauce()
		{
			return saveSauce;
		}
		
		bool& getBlinkMode()
		{
			return blinkMode;
		}
		
		bool& getExtendedCharMode()
		{
			return extendedCharMode;
		}
		
		unsigned char*& getFont()
		{
			return font;
		}
		
		unsigned char*& getPalette()
		{
			return palette;
		}
		
		ScreenBuffer();
		~ScreenBuffer()
		{
			if (font) {
				delete font;
				font = 0;
			}
			if (palette) {
				delete palette;
				palette = 0;
			}
		}
		
		void insertLine(int lineNumber);
		void removeLine(int lineNumber);
		
		void insertColumn(int columnNumber);
		void removeColumn(int columnNumber);
		
		int getLastNonEmptyLine();
		
		void clear();
		void clear(int x1, int y1, int x2, int y2);
		
		void leftTrim();
		void leftTrim(int x1, int y1, int x2, int y2);
		
		void rightTrim();
		void rightTrim(int x1, int y1, int x2, int y2);
		
		void center();
		void center(int x1, int y1, int x2, int y2);
		
		void fillCharacter(unsigned char ch);
		void fillCharacter(unsigned char ch, int x1, int y1, int x2, int y2);
		
		void fillAttribute(unsigned char attribute);
		void fillAttribute(unsigned char attribute, int x1, int y1, int x2, int y2);
		
		void fillForeColor(unsigned char foreColor);
		void fillForeColor(unsigned char foreColor, int x1, int y1, int x2, int y2);
		
		void fillBackColor(unsigned char backColor);
		void fillBackColor(unsigned char backColor, int x1, int y1, int x2, int y2);
		
		void transformElite();
		void transformElite(int x1, int y1, int x2, int y2);
		
		void drawEffect(int drawEffect, unsigned char* colorTable);
		void drawEffect(int x1, int y1, int x2, int y2, int drawEffect, unsigned char* colorTable);
		
		ScreenBlock getBlock(int x1, int y1, int x2, int y2);
		void stampBlock(ScreenBlock& block, int x1, int y1, bool stampUnder);
		
		bool load(char* fileName);
		bool save(char* fileName, ScreenFileFormat format);
};

extern char translate(char ch);
#endif
