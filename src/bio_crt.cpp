#include<bio_crt.h>

bool        MouseSupport = false;
ansout_impl ansout;

int ansoutbuf::overflow(int ch)
{
	screenEngine.putChar(ch);
	return ch;
}


ostream& operator<<(ostream& os, const gotoxy& gxy)
{
	screenEngine.getCaretX() = gxy.x;
	screenEngine.getCaretY() = gxy.y;
	return os;
}

ostream& operator<<(ostream& os, const textattr& tattr)
{
	screenEngine.getAttribute() = tattr.attr;
	return os;
}

ostream& operator<<(ostream& os, const clrscrn& clrscrn)
{
	screenEngine.clearScreen();
	return os;
}
//////////
SDL_Surface* screen;
Uint8       defaultPaletteData[] = {
	0x00, 0x00, 0x00, // black
	0x00, 0x00, 0xAA, // blue
	0x00, 0xAA, 0x00, // green
	0x00, 0xAA, 0xAA, // cyan
	0xAA, 0x00, 0x00, // red
	0xAA, 0x00, 0xAA, // magenta
	0xAA, 0x55, 0x00, // brown
	0xAA, 0xAA, 0xAA, // lightgray
	0x55, 0x55, 0x55, // darkgray
	0x55, 0x55, 0xFF, // lightblue
	0x55, 0xFF, 0x55, // lightgreen
	0x55, 0xFF, 0xFF, // lightcyan
	0xFF, 0x55, 0x55, // lightred
	0xFF, 0x55, 0xFF, // lightmagenta
	0xFF, 0xFF, 0x55, // yellow
	0xFF, 0xFF, 0xFF, // white
};

Uint8* ScreenEngine::getDefaultPalette()
{
	return defaultPaletteData;
}

Uint32 ScreenEngine::getpixel(SDL_Surface *surface, int x, int y)
{
	int bpp = surface->format->BytesPerPixel;
	/* Here p is the address to the pixel we want to retrieve */
	Uint8 *p = (Uint8 *)surface->pixels + y * surface->pitch + x * bpp;

	switch(bpp) {
		case 1:
			return *p;

		case 2:
			return *(Uint16 *)p;

		case 3:
			if(SDL_BYTEORDER == SDL_BIG_ENDIAN)
				return p[0] << 16 | p[1] << 8 | p[2];
			else
				return p[0] | p[1] << 8 | p[2] << 16;

		case 4:
			return *(Uint32 *)p;

		default:
			return 0;       /* shouldn't happen, but avoids warnings */
	}
}

void ScreenEngine::putpixel(SDL_Surface *surface, int x, int y, Uint32 pixel)
{
	int bpp = surface->format->BytesPerPixel;

	// Here p is the address to the pixel we want to set
	Uint8 *p = (Uint8 *)surface->pixels + y * surface->pitch + x * bpp;

	switch(bpp) {
		case 1:
			*p = pixel;
			break;
		case 2:
			*(Uint16*)p = pixel;
			break;
		case 3:
			if (SDL_BYTEORDER == SDL_BIG_ENDIAN) {
				p[0] = (pixel >> 16) & 0xff;
				p[1] = (pixel >> 8) & 0xff;
				p[2] = pixel & 0xff;
			} else {
				p[0] = pixel & 0xff;
				p[1] = (pixel >> 8) & 0xff;
				p[2] = (pixel >> 16) & 0xff;
			}
			break;
		case 4:
			*(Uint32 *)p = pixel;
			break;
	}
}

/*
     attribute byte table:

     			 foreground      background
             /-------------\ /-------------\
     value:   1   2   4   8  16  32  64 128
     #byte:   0   1   2   3   4   5   6   7
                          ^               ^
                  font selector       blink
                 (512 font mode)  (blinkmode)
*/
void ScreenEngine::printChar(int xPos, int yPos, unsigned char ch, unsigned char attr)
{
	Uint32 drawPosX = xPos * 8;
	Uint32 drawPosY = yPos * 16;
	int ret = SDL_LockSurface(screen);
	if (ret == 0) {
		Uint32 fOffs = 3 * (attr & (extendedFontMode ? 7 : 15));
		Uint32 bOffs = 3 * ((attr & (blinkMode ? 112 : 240)) >> 4);

		Uint32 foreground = SDL_MapRGB(screen->format, palette[fOffs]     * 4,
		                                               palette[fOffs + 1] * 4,
		                                               palette[fOffs + 2] * 4);
		Uint32 background = SDL_MapRGB(screen->format, palette[bOffs] * 4,
		                                               palette[bOffs + 1] * 4,
		                                               palette[bOffs + 2] * 4);

		// handle blink mode
		if (blinkOn && blinkMode && (attr & 128) == 128) {
			foreground = background;
		}

		for (Uint32 y = 0; y < 16; ++y) {
			Uint8 line = *(font_data + ch * 16 + y + (extendedFontMode && (attr & 8) == 8 ? 256 * 16 : 0));
			for (Uint32 x = 0; x < 8; ++x) {
				if ((line & (128 >> x)) != 0) {
					putpixel(screen, x + drawPosX, y + drawPosY, foreground);
				} else {
					putpixel(screen, x + drawPosX, y + drawPosY, background);
				}
			}
		}

		SDL_UnlockSurface(screen);
	}
}

void ScreenEngine::LoadFont(char* args[])
{
#if WIN32
	char* homeDir     = "";
	char* relFontDir  = "font.fnt";
#else
	char* homeDir     = getenv("HOME");
	char* relFontDir  = "/.mdraw/font.fnt";
#endif
	char* fontFile = (char*)malloc(strlen(homeDir) + strlen(relFontDir) + 1);
	sprintf(fontFile, "%s%s", homeDir, relFontDir);

	font_data = default_font_data = new Uint8[4096];

	FILE* fp = fopen(fontFile, "rb");
	fread(font_data, 4096, 1, fp);
	fclose(fp);
	free(fontFile);
}

ScreenEngine::ScreenEngine()
{
	width  = 80;
	height = 25;
	lineLength = width * 2;

	palette = getDefaultPalette();
	screen_data = new Uint8[height * lineLength];

	for (Uint32 y = 0; y < height; ++y) {
		for (Uint32 x = 0; x < width; ++x) {
			getCharacterXY(x, y) = ' ';
			getAttributeXY(x, y) = 7;
		}
	}


	extendedFontMode = false;
	blinkMode        = false;
	blinkOn          = false;


	curAttr = 0x7;
	caretX  = 0;
	caretY  = 0;
}

Uint32& ScreenEngine::getCaretX()
{
	return caretX;
}

Uint32& ScreenEngine::getCaretY()
{
	return caretY;
}

Uint8& ScreenEngine::getAttribute()
{
	return curAttr;
}

bool& ScreenEngine::getExtendedFontMode()
{
	return extendedFontMode;
}

bool& ScreenEngine::getBlinkMode()
{
	return blinkMode;
}

Uint8& ScreenEngine::getCharacterXY(int column, int row)
{
	return *(screen_data + row * lineLength + column * 2);
}

Uint8& ScreenEngine::getAttributeXY(int column, int row)
{
	return *(screen_data + row * lineLength + column * 2 + 1);
}

void ScreenEngine::putChar(unsigned char ch)
{
	getCharacterXY(caretX, caretY) = ch;
	getAttributeXY(caretX, caretY) = curAttr;
	++caretX;
	if (caretX >= width) {
		caretX = 0;
		caretY = min(caretY + 1, height - 1);
	}
}

void ScreenEngine::xorCaret()
{
	if (!caretVisible) {
		return;
	}

	Uint32 drawPosX = caretX * 8;
	Uint32 drawPosY = caretY * 16;

	int ret = SDL_LockSurface(screen);
	if (ret == 0) {
		Uint32 fOffs = 3 * (getAttributeXY(caretX, caretY) & (extendedFontMode ? 7 : 15));
		Uint32 foreground = SDL_MapRGB(screen->format, (int)palette[fOffs],
		                                               (int)palette[fOffs + 1],
		                                               (int)palette[fOffs + 2]);

		for (Uint32 y = 14; y < 16; ++y) {
			for (Uint32 x = 0; x < 8; ++x) {
				putpixel(screen, x + drawPosX, y + drawPosY, foreground);
			}
		}
		//getpixel(screen, x + drawPosX, y + drawPosY) ^
		SDL_UnlockSurface(screen);
	}
}


void ScreenEngine::update()
{
	if (blinkMode) {
		Uint32 now = SDL_GetTicks();
		if (now - next_blink > 300) {
			blinkOn = !blinkOn;
			next_blink = now;
		}
	}

	Uint32 now = SDL_GetTicks();
	if (now - caretBlink > 500) {
		caretVisible = !caretVisible;
		caretBlink = now;
	}
}

void ScreenEngine::showScreen()
{
	update();
	for (Uint32 y = 0; y < height; ++y) {
		for (Uint32 x = 0; x < width; ++x) {
			printChar(x, y, getCharacterXY(x, y), getAttributeXY(x, y));
		}
	}
	xorCaret();
	SDL_UpdateRect(screen, 0, 0, 0, 0);
}

ScreenEngine::~ScreenEngine()
{
	delete default_font_data;
	delete screen_data;
	// don't need to delete palette
}
ScreenEngine screenEngine;

//////////
void init_bio_crt()
{
	if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER) == -1) {
		cerr << "Can't init SDL: " << SDL_GetError() << endl;
        exit(1);
    }
    atexit(SDL_Quit);
    screen = SDL_SetVideoMode(640, 400, 32, SDL_HWSURFACE | SDL_FULLSCREEN);

	if (screen == NULL) {
        cerr << "Can't set video mode: " << SDL_GetError() << endl;
        exit(1);
    }
	SDL_EnableKeyRepeat(250, 30);
	SDL_ShowCursor(SDL_DISABLE);
}

char *inputfield(char *Str, unsigned int length, int x1, int y)
{
	static char nul[100];
	unsigned int x;
	unsigned int pos=0;
	ansout << gotoxy(x1, y) << textattr(8);
	for (x=1;x<=length;x++) ansout << (char)250;
	ansout << gotoxy(x1, y) << textattr(7);
	sprintf(nul,"%s",Str);
	ansout << gotoxy(x1, y) << nul;

	bool done = false;
	SDL_Event event;
	do {
		ansout << gotoxy(x1 + pos, y);
		screenEngine.showScreen();
		SDL_Delay(50);
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							return Str;
						case SDLK_RETURN:
							done = true;
							break;
						case SDLK_LEFT:
							if (pos>0) pos--;
							break;
						case SDLK_RIGHT:
							if (pos<strlen(nul)) pos++;
							break;
						case SDLK_HOME:
							pos=0;
							break;
						case SDLK_END:
							pos=strlen(nul);
							break;
						case SDLK_DELETE:
							if (pos!=strlen(nul)) {
								memcpy(&nul[pos],&nul[pos+1],200-pos);
								ansout  << gotoxy(x1, y) << nul;
								ansout << textattr(8) << (char)250 << textattr(7) << gotoxy(x1 + pos, y);
							}
							break;
						case SDLK_BACKSPACE:
							if (pos>0) {
								memcpy(&nul[pos-1],&nul[pos],200-pos);
								pos--;
								ansout  << gotoxy(x1, y) << nul;
								ansout << textattr(8) << (char)250 << textattr(7) << gotoxy(x1 + pos, y);
							}
							break;
						default:
							unsigned char ch = event.key.keysym.unicode;
							if ((pos<length)&(ch>=32)&(ch<=127)) {
								if (pos==strlen(nul)) {
									sprintf(nul,"%s%c",nul,(ch&255));
								} else {
									nul[pos]=ch;
								}
								ansout << gotoxy(x1, y) << nul;
								pos++;
								ansout << textattr(7) << gotoxy(x1 + pos, y);
							}
							break;
					}
			}
		}
	} while (!done);
	return nul;
}
void CoolWrite(char * a) {
	int x=0,i=0;
	while (a[x]!=0) {
		i++;
		if (a[x]==32) i=0;
		switch (i) {
			case 1:
				ansout << textattr(8);
				break;
			case 3:
			case 2:
				ansout << textattr(3);
				break;
			default:
				ansout << textattr(11);
				break;
		}
		ansout << (char)a[x++];
	}
}
void CodeWrite(char * a) {
	int x=0;
	ansout << textattr(15);
	while (a[x]!=0) {
		if (a[x]==']') ansout << textattr(15);
		ansout << a[x];
		if (a[x]=='[') ansout << textattr(7);
		x++;
	}
}

void DrawBox (int x1,int y1,int x2,int y2) /* Draws a nice looking box ... */
{
	int x,y;
	for (x=x1+2;x<=x2-2;x++) {
		ansout << gotoxy(x, y1) << textattr(10) << (char)223;
		ansout << gotoxy(x, y2) << textattr(2) << (char)220;
	}
	for (y=y1;y<=y2;y++) {
		if ((y==y1+1)||(y==y2-1)) ansout << textattr(10); else
		if ((y==y1+2)||(y==y2-2)) ansout << textattr(7); else ansout << textattr(8);
		ansout << gotoxy(x1, y) << (char)221;
		ansout << gotoxy(x2, y) << (char)222;
	}
	ansout << textattr(2);
	ansout << gotoxy(x1, y1) << (char)220;
	ansout << gotoxy(x2, y1) << (char)220;
	ansout << gotoxy(x1, y2) << (char)223;
	ansout << gotoxy(x2, y2) << (char)223;
	ansout << textattr(10);
	ansout << gotoxy(x1 + 1, y1) << (char)254;
	ansout << gotoxy(x2 - 1, y1) << (char)254;
	ansout << gotoxy(x1 + 1, y2) << (char)254;
	ansout << gotoxy(x2 - 1, y2) << (char)254;
}

extern int chooser(int col, int first, ...)
{
	vector<char*> optionList;
	// write optionList to the vector
	va_list ap;
	va_start(ap, first);
	while (true) {
		char* ptr = va_arg(ap, char*);
		if (ptr == 0) {
			break;
		}
		optionList.push_back(ptr);
	}
	va_end(ap);

	// retrieve keyboard shortcuts
	vector<pair<int, char> > keyboardShortcuts;

	for (unsigned int i = 0; i < optionList.size(); ++i) {
		char ch  = '\0';
		int  pos = 0;
		char* ptr = optionList[i];
		while (*ptr != 0) {
			if (isupper(*ptr)) {
				ch = *ptr;
				break;
			}
			++ptr;
			++pos;
		}
		if (*ptr == 0) {
			pos = -1;
		}

		keyboardShortcuts.push_back(pair<int, char>(pos, ch));
	}
	unsigned int selectedItem = first - 1;

	SDL_Event event;
	bool done = false;
	do {
		ansout << gotoxy(col, LINES - 1);
		for (unsigned int i = 0; i < optionList.size(); ++i) {
			char* ptr = optionList[i];
			int index = 0;
			while (*ptr != 0) {
				if (index == keyboardShortcuts[i].first) {
					ansout << textattr(i == selectedItem ? 15 + 16 : 7);
				} else {
					ansout << textattr(i == selectedItem ? 0x1B    : 8);
				}
				ansout << (char)(i == selectedItem ? toupper(*ptr) : tolower(*ptr));
				++index;
				++ptr;
			}
			ansout << textattr(7) << "  ";
		}
		screenEngine.showScreen();
		SDL_Delay(50);

		while (SDL_PollEvent(&event)) {
			switch (event.type){
				case SDL_QUIT:
					exit(0);
					break;
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							return -1;
						case SDLK_LEFT:
							if (selectedItem == 0) {
								selectedItem = optionList.size() - 1;
							} else {
								--selectedItem;
							}
							break;
						case SDLK_RIGHT:
							++selectedItem;
							if (selectedItem >= optionList.size()) {
								selectedItem = 0;
							}
							break;
						case SDLK_RETURN:
							done = true;
							break;
						default:
							for (unsigned int i = 0; i <= optionList.size(); ++i) {
								if (toupper(event.key.keysym.sym) == keyboardShortcuts[i].second) {
									return i + 1;
								}
							}

							break;
					}
			}
		}
	} while (!done);
	return selectedItem + 1;
}

#ifdef HAS_GPM
int mouse_x;
int mouse_y;
int mouse_button;
int mouse_deltax;
int mouse_deltay;
char mouse_activ=0;

int mouse_getx()  {
	return mouse_x;
}

int mouse_gety()  {
	return mouse_y;
}

int mouse_getbutton()  {
	return mouse_button;
}

int mouse_handler(Gpm_Event *event,void *data) {
	/*
	do  {
		mouse_x=event->x;
		mouse_y=event->y;
		mouse_button=event->buttons;
	}
	while((event->type&GPM_DRAG) && Gpm_Repeat(200));
	*/
	return 0;
}

void showmouse() {
	mouse_activ=1;
}
void hidemouse() {
	mouse_activ=0;
}

void mouse_update()  {
	Gpm_Event event;
	if (Gpm_Repeat(0)==0)  {
		Gpm_GetEvent(&event);
		mouse_deltax=event.dx;
		mouse_deltay=event.dy;
		mouse_x=event.x;
		mouse_y=event.y;
		mouse_button=event.buttons;
		if (mouse_activ==1)  {
			GPM_DRAWPOINTER(&event);
		}
	};
}


int mouse_init() {
	int status=0;
	Gpm_Connect conn;
	conn.eventMask=~0;
	conn.defaultMask=0;
	conn.maxMod=~0;
	conn.minMod=0;
	if (Gpm_Open(&conn,0)==-1) {
		status=-1;
	}
	gpm_handler=mouse_handler;
	return status;
}

void mouse_close() {
	Gpm_Close();
}
#endif
