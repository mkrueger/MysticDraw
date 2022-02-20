#include<ScreenBuffer.hpp>

// only used internally as default 
// buffer format (can be changed online, no need
// to restart Mystic Draw)
#define MAX_LINES    1000
#define MAX_COLS     80

char translate(char ch)
{
	switch(ch) {
		case 'e':
			return 'î';
		case 'E':
			return 'ä';
		case 'I':
			return '­';
		case 'r':
			return 'ç';
		case 'R':
			return 'ž';
		case 'F':
		case 'f':
			return 'Ÿ';
		case 'a':
			return 'à';
		case 'A':
			return '’';
		case 'b':
		case 'B':
			return 'á';
		case 'n':
		case 'N':
			return 'ã';
		case 'u':
			return 150;
		case 'U':
			return 'ï';
		case 'Y':
			return '';
		case 'o':
			return 'í';
		case 'O':
			return 'å';   
		case 'L':
		case 'l':
			return 'œ';
		case 'X':
		case 'x':
			return '‘';
		case 'S':
		case 's':
			return '$';
		case 'C':
		case 'c':
			return '›';
		case 'D':
		case 'd':
			return 'ë';
		case 'y':
			return 'æ';
		case 't':
			return 'â';
		default:
			return ch;
	}
}


// SCREENBLOCK MEMBERS:
ScreenBlock::ScreenBlock(int width, int height)
{
	this->width  = width;
	this->height = height;
	this->lineLength = width * 2;
	data = new unsigned char[width * height * 2];
	
}

unsigned char* ScreenBlock::getData()
{
	return data;
}

unsigned char& ScreenBlock::getCharacter(int row, int column)
{
	return *(data + row * lineLength + column * 2);
}

unsigned char& ScreenBlock::getAttribute(int row, int column)
{
	return *(data + row * lineLength + column * 2 + 1);
}

int ScreenBlock::getHeight()
{
	return height;
}

int ScreenBlock::getWidth()
{
	return width;
}

int ScreenBlock::getLineLength()
{
	return lineLength;
}

void ScreenBlock::flipY()
{
	for (int y = 0; y < height / 2; ++y) {
		for (int x = 0; x < width; ++x) {
			swap(getCharacter(y, x), getCharacter(height - y - 1, x));
			swap(getAttribute(y, x), getAttribute(height - y - 1, x));
		}
	}
}

void ScreenBlock::flipX()
{
	for (int y = 0; y < height; ++y) {
		for (int x = 0; x < width / 2; ++x) {
			swap(getCharacter(y, x), getCharacter(y, width - x - 1));
			swap(getAttribute(y, x), getAttribute(y, width - x - 1));
		}
	}
}

ScreenBlock::~ScreenBlock()
{
	delete data;
}

// SCREENBUFFER MEMBERS:
ScreenBuffer::ScreenBuffer() : ScreenBlock(MAX_COLS, MAX_LINES)
{
	saveSauce = false;
	clear();
	blinkMode        = true;
	extendedCharMode = false;
	font    = 0;
	palette = 0;
}

void ScreenBuffer::insertLine(int lineNumber)
{
	memmove(data + (lineNumber + 1) * lineLength, data + lineNumber * lineLength, (height - lineNumber - 1) * lineLength); 
	clear(0, lineNumber, width - 1, lineNumber);
}

void ScreenBuffer::removeLine(int lineNumber)
{
	memmove(data + lineNumber * lineLength, data + (lineNumber + 1) * lineLength, (height - lineNumber - 1) * lineLength); 
	clear(0, height - 1, width - 1, height - 1);
}

void ScreenBuffer::insertColumn(int columnNumber)
{
	for (int i = 0; i < height; ++i) {
		memmove(data + i * lineLength + (columnNumber + 1) * 2, data + i * lineLength + columnNumber * 2, (width - columnNumber - 1) * 2); 
	}
	clear(columnNumber, 0, columnNumber, height - 1);
}

void ScreenBuffer::removeColumn(int columnNumber)
{
	for (int i = 0; i < height; ++i) {
		memmove(data + i * lineLength + columnNumber * 2, data + i * lineLength + (columnNumber + 1) * 2, (width - columnNumber - 1) * 2); 
	}
	clear(width - 1, 0, width - 1, height - 1);
}

int ScreenBuffer::getLastNonEmptyLine()
{
	for (int y = getHeight() - 1; y >= 0; --y) {
		for (int x = 0; x < getWidth(); ++x) {
			if ((getCharacter(y, x) != ' ' && getCharacter(y, x) != 0) || (getAttribute(y, x) & 240) != 0) {
				return y;
			}
		}
	}
	return 0;
}
void ScreenBuffer::leftTrim()
{
	leftTrim(0, 0, width - 1, height - 1);
}

void ScreenBuffer::leftTrim(int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		int d = 0;
		while (getCharacter(y, x1) ==' ' && d < x2 - x1) {
			++d;
			memmove(&getCharacter(y, x1), &getCharacter(y, x1 + 1), (x2 - x1) * 2);
			getCharacter(y, x2) = ' ';
			getAttribute(y, x2) = 7;
		}
	}
}

void ScreenBuffer::rightTrim()
{
	rightTrim(0, 0, width - 1, height - 1);
}

void ScreenBuffer::rightTrim(int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		int d = 0;
		while (getCharacter(y, x2) ==' ' && d < x2 - x1) {
			++d;
			memmove(&getCharacter(y, x1 + 1), &getCharacter(y, x1), (x2 - x1) * 2);
			getCharacter(y, x1) = ' ';
			getAttribute(y, x1) = 7;
		}
	}
}

void ScreenBuffer::center()
{
	center(0, 0, width - 1, height - 1);
}
void ScreenBuffer::center(int x1, int y1, int x2, int y2)
{
	leftTrim(x1, y1, x2, y2);
	
	for (int y = y1; y <= y2; ++y) {
		int d = 0;
		for (int x = x2; x >= x1; --x) { 
			if (getCharacter(y, x) == ' ') {
				++d;
			}
		}
		d /= 2;
		while (--d >= 0) {
			memmove(&getCharacter(y, x1 + 1), &getCharacter(y, x1), (x2 - x1) * 2);
			getCharacter(y, x1) = ' ';
			getAttribute(y, x1) = 7;
		}
	}
}

void ScreenBuffer::transformElite()
{
	transformElite(0, 0, width - 1, height - 1);
}
void ScreenBuffer::transformElite(int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getCharacter(y, x) = translate(getCharacter(y, x));
		}
	}
}

void ScreenBuffer::fillCharacter(unsigned char ch)
{
	fillCharacter(ch, 0, 0, width - 1, height - 1);
}
void ScreenBuffer::fillCharacter(unsigned char ch, int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getCharacter(y, x) = ch;
		}
	}
}

void ScreenBuffer::fillAttribute(unsigned char attribute)
{
	fillAttribute(attribute, 0, 0, width - 1, height - 1);
}
void ScreenBuffer::fillAttribute(unsigned char attribute, int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getAttribute(y, x) = attribute;
		}
	}
}

void ScreenBuffer::fillForeColor(unsigned char foreColor)
{
	fillForeColor(foreColor, 0, 0, width - 1, height - 1);
}
void ScreenBuffer::fillForeColor(unsigned char foreColor, int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getAttribute(y, x) = foreColor | (getAttribute(y, x) & 240);
		}
	}
}

void ScreenBuffer::fillBackColor(unsigned char backColor)
{
	fillBackColor(backColor, 0, 0, width - 1, height - 1);
}
void ScreenBuffer::fillBackColor(unsigned char backColor, int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getAttribute(y, x) = backColor | (getAttribute(y, x) & 15);
		}
	}
}

void ScreenBuffer::clear()
{
	clear(0, 0, width - 1, height - 1);
}
void ScreenBuffer::clear(int x1, int y1, int x2, int y2)
{
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			getCharacter(y, x) = ' ';
			getAttribute(y, x) = 7;
		}
	}
}

void ScreenBuffer::drawEffect(int drawEffect, unsigned char* colorTable)
{
	this->drawEffect(0, 0, width - 1, height - 1, drawEffect, colorTable);
}
void ScreenBuffer::drawEffect(int x1, int y1, int x2, int y2, int drawEffect, unsigned char* colorTable)
{
	int Left = x1, Right = x2;
	unsigned char col = '\0';
	
	for (int y = y1; y <= y2; ++y) {
		int c = 0;
		Left=x1;
		for (int x = x1; x <= x2; ++x) { 
			if (getCharacter(y, x) != ' ') { 
				Left=x;
				break;
			}
		}
		for (int x = x2; x >= x1; --x) { 
			if (getCharacter(y, x) != ' ') { 
				Right=x;
				break;
			}
		}
		for (int x = x1; x <= x2; ++x) {
			unsigned char a = getCharacter(y, x);
			c++;
			switch(drawEffect) {
				case 1:
					col = colorTable[1];
					if (isdigit(a)) {
						col = colorTable[2];
					}
					if (isupper(a)) {
						col = colorTable[3];
					}
					if (islower('a')) {
						col = colorTable[4];
					}
					if (a >= 128) {
						col = colorTable[5];
					}
					break;
				case 2:
					if (a <= ' ') {
						c=0;
					}
					switch(c) {
						case 1:
							col = colorTable[1];
							break;
						case 2:
							col = colorTable[2];
							break;
						case 3:
							col = colorTable[3];
							break;
						case 4:
							col = colorTable[4];
							break;
						default:
							col = colorTable[5];
					}
					break;
				case 3:
					col = colorTable[5];
					for (int i = 0; i < 4; ++i) {  
						if (x==Left+i||x==Right-i) {
							col = colorTable[1+i];
						}
					}
					break;
				case 4:
					if (x % 2 == 0) { 
						col = colorTable[1];
					} else {
						col = colorTable[2];
					}
					break;
				
			}
			getAttribute(y, x) = col;
		}
	} 
}


// BLOCK FUNCTIONS
ScreenBlock ScreenBuffer::getBlock(int x1, int y1, int x2, int y2)
{
	ScreenBlock newBlock(x2 - x1 + 1, y2 - y1 + 1);
	for (int y = y1; y <= y2; ++y) {
		for (int x = x1; x <= x2; ++x) {
			newBlock.getCharacter(y - y1, x - x1) = getCharacter(y, x);
			newBlock.getAttribute(y - y1, x - x1) = getAttribute(y, x);
		}
	}
	return newBlock;
}

void ScreenBuffer::stampBlock(ScreenBlock& block, int x1, int y1, bool stampUnder)
{
	int y2 = min(y1 + block.getHeight(), height);
	int x2 = min(x1 + block.getWidth(),  width);
	
	for (int y = y1; y < y2; ++y) {
		for (int x = x1; x < x2; ++x) {
			unsigned char ch = getCharacter(y, x);
			if (!stampUnder || ch == ' ' || ch == 0) {
				getCharacter(y, x) = block.getCharacter(y - y1, x - x1);
				getAttribute(y, x) = block.getAttribute(y - y1, x - x1);
			}
		}
	}
}

// LOAD FUNCTIONS
int  cursorX, cursorY, txtAttr;
bool avt_color, avt_rep, avt_command;
bool pcb_color, pcb_code;
bool ans_esc, ans_code;
int avatarState = 0;

unsigned char c, cc;
unsigned char d, pcb_col = 0;
unsigned char e, ans_attr = 0;

int savedX, savedY;
//stack<pair<int, int> > savedPositions;
char ansicode[20];
char copytmp[20];
char code[20];

char *copy(char *str,int aa,int bb)
{
	sprintf(copytmp,"  "); 
	for (int x = aa; x <= bb; ++x) {
		copytmp[x - aa] = str[x];
	}  
	return copytmp;
}

unsigned char ScreenBuffer::display_ansi(char ch)
{
	int b, c, retValue;
	
	if (ans_esc && ch == '[') {
		ans_esc  = false;
		ans_code = true;
		return 0;
	}
	ans_esc = false;
	
	if (ans_code) {
		ans_code = false;
		switch (ch) {
			case 'm': 
				ans_attr = txtAttr;
				for (unsigned int x = 0; x < strlen(ansicode); ++x) {
					if (ansicode[x] >= '0' && ansicode[x] <= '9') {
						b = 0;
						code[b++] = ansicode[x];
						if (ansicode[x + 1] >= '0' && ansicode[x + 1] <= '9') {
							code[b++] = ansicode[++x];
						}
						code[b++] = 0;
						
						switch(strtol(code, NULL, 0)) {
							case 0 : 
								ans_attr = 7;
								break;
							case 1 :
								ans_attr = (ans_attr&247)+8;
								break;
							case 5 :
								ans_attr = (ans_attr&127)+128;
								break;
							case 30:
								ans_attr = ans_attr&248;
								break;
							case 34:
								ans_attr = (ans_attr&248)+1;
								break;
							case 32:
								ans_attr = (ans_attr&248)+2;
								break;
							case 36:
								ans_attr = (ans_attr&248)+3;
								break;
							case 31:
								ans_attr = (ans_attr&248)+4;
								break;
							case 35:
								ans_attr = (ans_attr&248)+5;
								break;
							case 33:
								ans_attr = (ans_attr&248)+6;
								break;
							case 37:
								ans_attr = (ans_attr&248)+7;
								break;       
							case 40:
								ans_attr = ans_attr&143;
								break;
							case 44:
								ans_attr = (ans_attr&143)+16; 
								break;
							case 42:
								ans_attr = (ans_attr&143)+32;
								break;
							case 46:
								ans_attr = (ans_attr&143)+48;
								break;
							case 41:
								ans_attr = (ans_attr&143)+64;
								break;
							case 45:
								ans_attr = (ans_attr&143)+80;
								break;
							case 43:
								ans_attr = (ans_attr&143)+(6<<4);
								break;
							case 47:
								ans_attr = (ans_attr&143)+(7<<4);
								break;
						}
						x++;
					}
				}
				txtAttr = ans_attr;
				return 0;
			case 'H':
			case 'f':
				retValue = sscanf(ansicode, "%d;%d", &b, &c);
				if (retValue >= 1) {
					cursorY = (b - 1) + max(cursorY - 24, 0);
					if (retValue >= 2) {
						cursorX = max(min(c - 1, width - 1), 0);
					} else {
						cursorX = 0;
					}
				}
				return 0;
			case 'C':
				retValue = sscanf(ansicode, "%d", &b);
				if (retValue == 1) {
					cursorX = min(cursorX + b, width - 1);
				} else {
					cursorX = min(cursorX + 1, width - 1);
				}
				return 0;
			case 'D':
				retValue = sscanf(ansicode, "%d", &b);
				if (retValue == 1) {
					cursorX = max(cursorX - b, 0);
				} else {
					cursorX = max(cursorX - 1, 0);
				}
				return 0;
			case 'A':
				retValue = sscanf(ansicode, "%d", &b);
				if (retValue == 1) {
					cursorY -= b;
				} else {
					--cursorY;
				}
				cursorY = max(cursorY, 0);
				return 0;
			case 'B':
				retValue = sscanf(ansicode, "%d", &b);
				if (retValue == 1) {
					cursorY += b;
				} else {
					++cursorY;
				}
				cursorY = min(cursorY, height);
				return 0;
			case 'S':
			case 's':
				savedX = cursorX;
				savedY = cursorY;
				//savedPositions.push(pair<int, int>(cursorX, cursorY));
				return 0;
			case 'U':
			case 'u':
				cursorX = savedX;
				cursorY = savedY;
				//cursorX = savedPositions.top().first;
				//cursorY = savedPositions.top().second;
				//savedPositions.pop();
				return 0;
			case 'J':
				cursorX = 0;
				cursorY = 0;
				return 0;
			default:
				sprintf(ansicode, "%s%c", ansicode, ch);
				ans_code = true;
				break;
		}
		return 0;
	}
	switch (ch) {
		case '': 
			ans_code = false;
			ans_esc  = true;
			ansicode[0] = 0;
			return 0;
		default:
			return ch;
	}
	return 0;
}

unsigned char display_PCBoard(char ch)
{
	static char color[24];
	if (pcb_color) {   
		color[0] = 0;
		d++;
		if (d<3) {
			switch (d) {
				case 1:
					sprintf(color, "0x%c", ch);
					pcb_col = strtol(color, NULL, 0) * 16;
					break;
				case 2:
					sprintf(color, "0x0%c", ch);
					pcb_col += strtol(color, NULL, 0);
					break;
			}
			return 0;
		}
		txtAttr = pcb_col;
		pcb_color = false;
		pcb_code  = false;
		return ch;
	}
	
	if (pcb_code) {
		switch(ch){
			case '@':
				pcb_code = false;
				break;
			case 'X':
				pcb_color = true;
				break;
		}      
		return 0;
	} 
	switch (ch) {
		case '@': 
			pcb_code = true;
			break;
		default:
			return ch;
	}
	return 0;
}

unsigned char display_avatar(unsigned char ch) 
{
	if (avt_rep) {
		switch (avatarState) {
			case 1:
				c=ch;
				avatarState = 2;
				return 0;
			case 2:
				cc=ch;
				avatarState = 3;
				return 0;
			case 3:
				cc--;
				if (cc > 0) {
					return c;
				}
				break;
		}      
		avt_rep = false;
	}
	avatarState = 0;
	
	if (avt_color) {
		txtAttr     = ch;
		avt_command = false;
		avt_color   = false;
		return 0;
	}
	
	if (avt_command) {
		switch (ch) {
			case 1:
				avt_color = true;
				break;
			default:
				avt_command = false;
		}
		return 0;
	}
	switch (ch) {
		case '': 
			cursorX=0;
			cursorY=0;
			break;
		case '':
			avt_rep     = true;
			avatarState = 1;
			break;
		case '':
			avt_command = true;
			break;
		default:
			return ch;
	}
	return 0;
}

typedef struct XB_Header
{
   unsigned char   id[4];
   unsigned char   eofChar;
   unsigned short  width;
   unsigned short  height;
   unsigned char   fontsize;
   unsigned char   flags;
   
   XB_Header()
   {
	   id[0] = 'X';
	   id[1] = 'B';
	   id[2] = 'I';
	   id[3] = 'N';
	   eofChar  = 28;
	   fontsize = 16;
	   flags    = 0;
	   width = height = 0;
   }
   // FLAGS:
   // 7        6        5        4          3          2         1     0
   // Unused   Unused   Unused   512Chars   NonBlink   Compress  Font  Palette
};

ostream& operator<<(ostream& os, XB_Header& header)
{
	os << "[XB_Header:eofChar=" << header.eofChar << ", width=" << header.width << ", height=" << header.height << 
	      ", fontsize=" << (short)header.fontsize << ", flags=" << (short)header.flags << "]"; 
	return os;
}

bool ScreenBuffer::load(char *fileName)
{
	FILE *fp = fopen(fileName, "rb");
	if (fp == NULL) {
		return false;
	}
	
	clear();
	
	cursorX=0;
	cursorY=0;
	txtAttr=7;
	
	int ch = strlen(fileName) - 3;
	
	// read plain binary
	if (ch >= 0 && toupper(fileName[ch]) == 'B' && toupper(fileName[ch + 1]) == 'I' && toupper(fileName[ch + 2]) == 'N') {
		do {
			for (int x = 0; x < 80; ++x) {
				getCharacter(cursorY, x) = fgetc(fp);
				getAttribute(cursorY, x) = fgetc(fp);
			}
			cursorY++;
		} while (!feof(fp));
		return true;
	}
	
	// read XBIN
	if (ch >= 0 && toupper(fileName[ch + 1]) == 'X' && toupper(fileName[ch + 2]) == 'B') {
		XB_Header header;
		fread(&header.id, 4, 1, fp);
		fread(&header.eofChar, 1, 1, fp);
		fread(&header.width, 2, 1, fp);
		fread(&header.height, 2, 1, fp);
		fread(&header.fontsize, 1, 1, fp);
		fread(&header.flags, 1, 1, fp);
		bool customPalette  = (header.flags &  1) == 1;
		bool customFont     = (header.flags &  2) == 2;
		bool compress       = (header.flags &  4) == 4;
		blinkMode           = (header.flags &  8) != 8;
		extendedCharMode    = (header.flags & 16) == 16;
		
		if (customPalette) {
			cout << "customPalette" << endl;
			palette = new unsigned char[3 * 16];
			fread(palette, 3 * 16, 1, fp);
		}
		
		if (customFont) {
			unsigned int fontSize = header.fontsize * (extendedCharMode ? 512 : 256);
			cout << "customFont" << fontSize << endl;
			font = new unsigned char[fontSize];
			fread(font, fontSize, 1, fp);
		}
		
		// TODO: resize buffer, if neccessary
		if (compress) {
			int x = 0;
			int y = 0;
			unsigned char ch, attr; 
			bool done = false;
			while (!done) {
				unsigned char repCounter = fgetc(fp);
				int repAttr = (repCounter & (64 | 128)) >> 6;
				int repNum  = (repCounter & 63) + 1;
				switch (repAttr) {
					case 0: // no compression
						for (int i = 0; i < repNum; ++i) {
							getCharacter(y, x) = fgetc(fp);
							getAttribute(y, x) = fgetc(fp);
							++x;
							// TODO : error handling, if x > width
						}
						break;
					case 1: // character compression
						ch = fgetc(fp);
						for (int i = 0; i < repNum; ++i) {
							getCharacter(y, x) = ch;
							getAttribute(y, x) = fgetc(fp);
							++x;
							// TODO : error handling, if x > width
						}
						break;
					case 2: // attribute compression
						attr = fgetc(fp);
						for (int i = 0; i < repNum; ++i) {
							getCharacter(y, x) = fgetc(fp);
							getAttribute(y, x) = attr;
							++x;
							// TODO : error handling, if x > width
						}
						break;
					case 3: // character & attribute compression
						ch   = fgetc(fp);
						attr = fgetc(fp);
						for (int i = 0; i < repNum; ++i) {
							getCharacter(y, x) = ch;
							getAttribute(y, x) = attr;
							++x;
							// TODO : error handling, if x > width
						}
						break;
				}
				if (x >= header.width) {
					x = 0;
					++y;
					if (y >= header.height) {
						done = true;
					}
				}
			}
		} else {
			for (int y = 0; y < header.height; ++y) {
				for (int x = 0; x < header.width; ++x) {
					getCharacter(y, x) = fgetc(fp);
					getAttribute(y, x) = fgetc(fp);
				}
			}
		}
		goto save;
	}
		
	do {
		ch = fgetc(fp);	
		
		if (ch == 26 && !avt_command && !avt_rep) {
			goto save;
		}
		do {
			ch = display_PCBoard(display_ansi(display_avatar(ch)));
			switch (ch) {
				case 0:
					break;
				case 10:
					cursorX = 0;
					cursorY++;
					break;
				case 13:
					cursorX = 0;
					break;
				default:
					getCharacter(cursorY, cursorX) = (unsigned char)ch;
					getAttribute(cursorY, cursorX) = txtAttr;
					cursorX++;
					// try to lookahead. If the nex character is a return we don't
					// do an 'artificial' return ... I think this behaviour is not 100% correct
					// (try to think that there follows an escape sequence which is not showed and
					// then the final return is done)
					// but this behaviour seems to get good display results ... but needs overworking
					ch = fgetc(fp);
					ungetc(ch, fp);
					if (cursorX > 79 && ch != 13 && ch != 10) {
						cursorX = 0;
						cursorY++;
					}
			}
		} while (avt_rep && avatarState == 3);
	} while(!feof(fp));
	
	save:
	saveSauce = sauce.ReadSauce(fp);
	
	fclose(fp);
	return true;
}

// SAVE ROUTINES
bool ScreenBuffer::save(char* fileName, ScreenFileFormat format)
{
	XB_Header header;
	FILE* fp = fopen(fileName, "wb");
	switch (format) {
		case XBinary:
			if (palette != 0) {
				header.flags |= 1;
			}
			if (font != 0) {
				header.flags |= 2;
			}
			// compress : todo 
			// header.flags |= 4;
			if (!blinkMode) {
				header.flags |= 8;
			}
			if (extendedCharMode) {
				header.flags |= 16;
			}
			header.width  = width;
			header.height = getLastNonEmptyLine() + 1;
			
			fwrite(&header.id, 4, 1, fp);
			fwrite(&header.eofChar, 1, 1, fp);
			fwrite(&header.width, 2, 1, fp);
			fwrite(&header.height, 2, 1, fp);
			fwrite(&header.fontsize, 1, 1, fp);
			fwrite(&header.flags, 1, 1, fp);
			if (palette != 0) {
				fwrite(palette, 3 * 16, 1, fp);
			}
			if (font != 0) {
				unsigned int fontSize = header.fontsize * (extendedCharMode ? 512 : 256);
				fwrite(font, fontSize, 1, fp);
			}
			
			// write uncompressed
			for (int y = 0; y < header.height; ++y) {
				for (int x = 0; x < header.width; ++x) {
					fputc(getCharacter(y, x), fp);
					fputc(getAttribute(y, x), fp);
				}
			}
			
			fclose(fp);
			break;
		default:
			cerr << "error unimplemented file format" << format;
			return false;
	}
	return true;
}
