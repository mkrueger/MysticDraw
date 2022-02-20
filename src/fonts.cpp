#include <fonts.hpp>
#include <cstring>

// TheDrawFont members
TheDrawFont::TheDrawFont()
{
	fontData = 0;
}

bool TheDrawFont::writeFontData(FILE* fp)
{
	// write signature
	fputc(19, fp); // size of signature (19 >IS< correct)
	char* sign = "TheDraw FONTS file";
	for (int i = 0; i < 19; ++i) {
		fputc(sign[i], fp);
	}
	
	// write skipBytes
	for (int i = 0; i < 4; ++i) {
		fputc(skipBytes[i], fp);
	}
	
	for (int i = 0; i <= 16; ++i) {
		fputc(name[i], fp);
	}
	
	fwrite(&fontType, 1, 1, fp);
	fwrite(&spaces, 1, 1, fp);
	fwrite(&fontDataSize, 2, 1, fp);
	
	for (int i = 0; i < 94; ++i) {
		fwrite(&chartable[i], 2, 1, fp);
	}
	
	// write the character data
	fwrite(fontData, fontDataSize, 1, fp);
	return true;
}

bool TheDrawFont::readFontData(FILE* fp)
{
	// read the font signature
	fgetc(fp); // skip size byte (always == 19)
	char sign[19];
	for (int i = 0; i < 19; ++i) {
		sign[i] = fgetc(fp);
	}
	sign[18] = 0;
	
	if (strcmp(sign, "TheDraw FONTS file") != 0) {
		cout << "wrong file signature" << endl;
		return false;
	}
	
	for (int i = 0; i < 4; ++i) {
		skipBytes[i] = fgetc(fp);
	}
	
	for (int i = 0; i <= 16; ++i) {
		name[i] = fgetc(fp);
	}
				
	fread(&fontType, 1, 1, fp);
	fread(&spaces,   1, 1, fp);
	fread(&fontDataSize, 2, 1, fp);
	
	for (int i = 0; i < 94; ++i) {
		fread(&chartable[i], 2, 1, fp);
	}
	
	// read the character data
	fontData = new unsigned char[fontDataSize];
	fread(fontData, fontDataSize, 1, fp);
	return true;
}

bool TheDrawFont::loadFile(char* fileName)
{
	FILE* fp = fopen(fileName, "rb");
	
	if (fp == NULL) {
		return false;
	}
	
	bool success = readFontData(fp);
	
	fclose(fp);
	
	return success;
}

TheDrawFont::~TheDrawFont()
{
	if (fontData) {
		delete fontData;
	}
}

// FontLibrary members

FontLibrary FontLibrary::library;

unsigned char FontLibrary::transformOutline(unsigned char ch)
{
	currentOutline = min(max(currentOutline, 0), 18);
	if (ch - 64 > 0 && ch - 64 <= 17) {
		return outlineCharSet[currentOutline][ch - 65];
	}
	return ' ';
}

bool FontLibrary::writeFontLibrary(char* fileName)
{
	FILE* fp = fopen(fileName, "wb");
	if (fp != NULL) {
		int numberOfFonts = fonts.size();
		fwrite(&numberOfFonts, sizeof(int), 1, fp);

		for (unsigned int i = 0; i < fonts.size(); ++i) {
			fonts[i]->writeFontData(fp);
		}
		
		fclose(fp);
		return true;
	} else {
		return false;
	}
}

bool FontLibrary::GetFontChar(unsigned char c)
{
	if (activeFont >= fonts.size()) {
		return false;
	}
	TheDrawFont* tdf = fonts[activeFont];
	if (tdf == 0) {
		return false;
	}
	
	unsigned short charOffset = tdf->chartable[c - ' ' - 1];
	
	if (charOffset == 0xFFFF) {
		maxX = maxY = 0;
		return false;
	}
	
	maxX = tdf->fontData[charOffset++];
	maxY = tdf->fontData[charOffset++];
	
	int x = 0; 
	int y = 0;
	
	memset(charTable, 0, sizeof(charTable));
	
	unsigned char ch;
	switch (tdf->fontType) {
		case 0: // Outline-Font
			do {
				ch = tdf->fontData[charOffset];
				if (ch == 13) {
					x = 0;
					++y;
				} else if (x < 80 && y < 120)  {
					charTable[y][x] = transformOutline(ch);
					++x;
				}
				++charOffset;
			} while (tdf->fontData[charOffset] != 0);
			break;
		case 1: // Block-Font
			do {
				ch = tdf->fontData[charOffset];
				if (ch == 13) {
					x = 0;
					++y;
				} else if (x < 80 && y < 120)  {
					charTable[y][x] = ch;
					++x;
				}
				++charOffset;
			} while (tdf->fontData[charOffset] != 0);
			break;
		case 2: // Color-Font 
			do {
				ch = tdf->fontData[charOffset];
				unsigned char color = tdf->fontData[charOffset + 1];
				if (ch == 13) {
					x = 0;
					++y;
					++charOffset; // Caution ch == 13 is only saved in 1 char (= has no color attribute)
				} else if (x < 80 && y < 120) {
					charTable[y][x * 2]     = ch;
					charTable[y][x * 2 + 1] = color;
					++x;
					charOffset += 2;
				}
				
			} while (tdf->fontData[charOffset] != 0);
			break;
	}
	
	return true;
}

bool FontLibrary::readFontLibrary(char* fileName)
{
	FILE* fp = fopen(fileName, "rb");
	if (fp != NULL) {
		int numberOfFonts;
		fread(&numberOfFonts, sizeof(int), 1, fp);
		cout << "read " << numberOfFonts << endl;
		for (int i = 0; i < numberOfFonts; ++i) {
			TheDrawFont* tdf = new TheDrawFont();
			if (!tdf->readFontData(fp)) {
				cout << "error reading font data" << endl;
				// don't clear font library because some fonts are better
				// than NO fonts ... but this may be reconsidered
				return false;
			}
			fonts.push_back(tdf);
		}
		return true;
	}
	return false;
}

FontLibrary& FontLibrary::getInstance()
{
	return library;
}

FontLibrary::~FontLibrary()
{
	for (unsigned int i = 0; i < fonts.size(); ++i) {
		delete fonts[i];
	}
	fonts.clear();
}

// Global 

const char* outlineCharSet[20]={
   "�ĳ�ڿڿ���ٴ�   ",
   "�ĳ�ոڿԾ�ٵ�   ",
   "�ͳ�ڿո��Ծ��   ",
   "�ͳ�ոոԾԾ��   ",
   "�ĺ�ֿڷ���ٶ�   ",
   "�ĺ�ɸڷԼ�ٹ�   ",
   "�ͺ�ֿջ��Ⱦ��   ",
   "�ͺ�ɸջԼȾ��   ",
   "�ĳ�ڷֿ������   ",
   "�ĳ�ջֿȾ����   ",
   "�ͳ�ڷɸ��Լ��   ",
   "�ͳ�ջɸȾԼ��   ",
   "�ĺ�ַַӽӽ��   ",
   "�ĺ�ɻַȼӽ��   ",
   "�ͺ�ַɻӽȼ��   ",
   "�ͺ�ɻɻȼȼ��   ",
   "��������������   ",
   "��������������   ",
   "��������������   "
};
