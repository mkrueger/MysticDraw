#ifndef FONTS_HPP
#define FONTS_HPP

#include<vector>
#include<iostream>

using namespace std;

class TheDrawFont
{
	public:
		char skipBytes[4]; // I don't know what these are good for
		char name[17];
		
		unsigned char fontType;
		unsigned char spaces;
		unsigned short chartable[94];
		unsigned short fontDataSize;
		unsigned char* fontData;
		
		TheDrawFont();
		bool writeFontData(FILE* fp);
		bool readFontData(FILE* fp);
		bool loadFile(char* fileName);
		~TheDrawFont();
};

class FontLibrary
{
	private:
			static FontLibrary library;
	
			unsigned char transformOutline(unsigned char ch);
	public:
			static FontLibrary& getInstance();
			
			unsigned int activeFont;
			vector<TheDrawFont*> fonts;
			
			FontLibrary()
			{
				activeFont = 0;
			}
			
			int currentOutline;
			
			unsigned char charTable[120][80 * 2];
			int maxX;
			int maxY;
			
			bool writeFontLibrary(char* fileName);
			bool GetFontChar(unsigned char c);
			bool readFontLibrary(char* fileName);
			
			TheDrawFont* getCurrentFont()
			{
				return fonts[activeFont];
			}
			
			~FontLibrary();
};

extern const char* outlineCharSet[];

#endif
