/*
 * Mystic Draw 1.6 : A nice ANSI Drawing tool
 * Copyright(C) 1996-2003 by Mike Krueger (mike@icsharpcode.net)
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

#include<MysticDrawMain.hpp>

using namespace std;

unsigned char tabs[80];
unsigned char CursorPos[80], cpos=1;

unsigned char Attribute=7;
bool Undo=false, FontTyped=false;
unsigned char ActiveCharset = 1;
bool FullScreen = false;

char* CharSet[12]={
     "",
     "⁄ø¿Ÿƒ≥√¥¡¬",
     "…ª»ºÕ∫Ãπ À",
     "’∏‘æÕ≥∆µœ—",
     "≈Œÿ◊üÈõúôÔ",
     "∞±≤€ﬂ‹›ﬁ˛˙",
     "˜˘≠®ëíûÄá",
     "ìîï¢ßñÅó£ö",
     "ÆØÚÛ©™˝ˆ´¨",
     "„ÒÙıÍù‰¯˚¸",
     "‡·‚ÂÊÁÎÏÌÓ",
     "àâäÇêåãç°¶"
};

char EliteTable[256];
teffekt effect;

#include<miscfunctions.h>
#include<options.h>
#include<save.h>
#include<load.h>
#include"menue.h"

void Caret::checkCaretPosition()
{
	if (caretY < 0) {
		caretY = 0;
		--upperLeftCornerLine;
	}
	if (caretX < 0) {
		caretX = 0;
		--upperLeftCornerRow;
	}
	
	if (FullScreen) {       
		if (caretY > LINES - 1) {
			caretY = LINES - 1;
			++upperLeftCornerLine;
		}
		if (upperLeftCornerLine + LINES > MysticDrawMain::getInstance().getCurrentBuffer()->getHeight()) {
			upperLeftCornerLine = MysticDrawMain::getInstance().getCurrentBuffer()->getHeight() - LINES;
		}
	} else {
		if (caretY > LINES - 2) {
			caretY = LINES - 2;
			++upperLeftCornerLine;
		}
		if (upperLeftCornerLine + LINES - 1 > MysticDrawMain::getInstance().getCurrentBuffer()->getHeight()) {
			upperLeftCornerLine = MysticDrawMain::getInstance().getCurrentBuffer()->getHeight() - LINES + 1;
		}
	}
	
	if (caretX >= COLS) {
		upperLeftCornerRow += caretX - COLS  + 1;
		caretX = COLS - 1;
	}
	
	if (upperLeftCornerRow + 80 > MysticDrawMain::getInstance().getCurrentBuffer()->getWidth()) {
		upperLeftCornerRow = MysticDrawMain::getInstance().getCurrentBuffer()->getWidth() - 80;
	}
	
	upperLeftCornerLine = max(upperLeftCornerLine, 0);
	upperLeftCornerRow  = max(upperLeftCornerRow,  0);
	
	caretX = max(caretX, 0);
	caretX = min(caretX, 79);
}


MysticDrawMain MysticDrawMain::mysticDrawMain;

MysticDrawMain& MysticDrawMain::getInstance()
{
	return mysticDrawMain;
}

void MysticDrawMain::drawStatusLine()
{
	if (FullScreen) {
		return;
	}
	ansout << textattr(2) << gotoxy(0, 0);
	for (int i = 0; i < 80; ++i) {
		ansout << ' ';
	}
	ansout << gotoxy(0, 0);
	ansout << '(' << setw(3) << setfill('0') << caret.getLogicalX() + 1;
	ansout << ", "<< setw(4) << setfill('0') << caret.getLogicalY() + 1 << ')';
	
	ansout << gotoxy(18, 0) << textattr(8);
	if (caret.eliteMode()) { 
		ansout << 'e';
	} else {
		ansout << ' ';
	}
	ansout << gotoxy(20, 0) << textattr(14);
	if (caret.insertMode()) {
		ansout << "Ins";
	} else {
		ansout << "   ";
	}
	ansout << gotoxy(24, 0) << textattr(2);
	ansout << "Page:" << currentBuffer << textattr(9);
	ansout << gotoxy(32, 0);
	ansout << "Set: " << (int)ActiveCharset;
	ansout << gotoxy(39, 0);
	for (int i = 1; i <= 10; ++i) {
		unsigned char b = CharSet[ActiveCharset][i - 1];
		ansout << textattr(64) << ' ' << i << "=" << textattr(64 + 15) << b;
	}
	ansout << textattr(7) << ' ';
}

void MysticDrawMain::updateColorStatus(unsigned char col)
{
	if (FullScreen) {
		return;
	}
	ansout << gotoxy(11, 0) << textattr(col) << " Color ";
}


/*
 * Version        4 Byte
 * Color          1 Byte
 * TABPos        80 Byte
 * Outline        1 Byte
 * EffectStruct   1 teffekt
*/
void MysticDrawMain::loadconfig()
{
	FILE* fp = fopen(getConfigurationFileName().c_str(), "rb");
	if (fp != NULL) {      
		int ver = 0;
		fread(&ver,4,1,fp);
		if (ver >= 1) { 	 
			fread(&Attribute,1,1,fp);
			for (int i = 1; i <= 80; ++i) {
				fread(&tabs[i],1,1,fp);
			}
			fread(&FontLibrary::getInstance().currentOutline,1,1,fp);
			fread(&effect,sizeof(effect),1,fp);
		}
		if (ver == 2) {
			fread(&caret.insertMode(),1,1,fp);
			fread(&caret.eliteMode(),1,1,fp);
			fread(&FullScreen,1,1,fp);
		}
		fclose(fp);
	} else {
		cout << "Error opening configuration file check that " << getConfigurationFileName() << " exists" << endl;
		configrationLoaded = false;
	}
}

void MysticDrawMain::saveconfig()
{
	int ver=2;
	FILE* fp = fopen(getConfigurationFileName().c_str(), "wb");
	if (fp != NULL) {
		fwrite(&ver,4,1,fp);
		fwrite(&Attribute,1,1,fp);
		for (int i=1;i<=80;i++) {
			fwrite(&tabs[i],1,1,fp);
		}
		fwrite(&FontLibrary::getInstance().currentOutline,1,1,fp);
		fwrite(&effect,sizeof(effect),1,fp);
		fwrite(&caret.insertMode(),1,1,fp);
		fwrite(&caret.eliteMode(),1,1,fp);
		fwrite(&FullScreen,1,1,fp);
		fclose(fp);
	} else {
		cout << "Error while writing to the configuration file " << getConfigurationFileName() << endl;
		cout << "Check that the path exists and that you've write permssion to it" << endl;
	}
}

void MysticDrawMain::startMysticDraw(int argnum, char* args[])
{
	this->args = args;
	int a,b;
	loadconfig();

#if WIN32
	char* homeDir = "";
	char* relFontDir  = "allfont.fnt";
#else
	char* homeDir = getenv("HOME");
	char* relFontDir  = "/.mdraw/allfont.fnt";
#endif	

	char* fontLibrary = (char*)malloc(strlen(homeDir) + strlen(relFontDir) + 1);
	sprintf(fontLibrary, "%s%s", homeDir, relFontDir);
	cout << fontLibrary << endl;
	if (!FontLibrary::getInstance().readFontLibrary(fontLibrary)) {
		cout << "Error loading font library see INSTALL" << endl; 
	}
	free(fontLibrary);
	
	if (argnum > 1) {
		cout << "Loading file " << args[1] << endl;
		getCurrentBuffer()->load(args[1]);
	}
	init_bio_crt();
	SDL_EnableUNICODE(1);
	
#ifdef HAS_GPM
	if (mouse_init()!=0) { 
		MouseSupport = false;
	}
#endif
	screenEngine.LoadFont(args);
	
	HelpCommand           helpCommand;
	ASCIITableCommand     asciiTableCommand;
	TabulatorSetupCommand tabulatorSetupCommand;
	SelectFontCommand     selectFontCommand;
	SelectOutlineCommand  selectOutlineCommand;
	SelectSauceCommand    selectSauceCommand;
	DrawCommand           drawCommand;
	DrawLineCommand       drawLineCommand;
	SelectEffectModeCommand   selectEffectCommand;
	SelectColorCommand selectColorCommand;
	BlockModeCommand     blockModeCommand;
	FontEditorCommand    fontEditorCommand;
	PaletteEditorCommand paletteEditorCommand;
	ViewModeCommand      viewModeCommand;
	SDL_Event event;
	
	done = false;
	do {
		drawStatusLine();
		if (FullScreen) {
			drawScreen(1, 25);
		} else {
			drawScreen(1, 24);
		}
		updateColorStatus(Attribute);
		if (FullScreen) {
			ansout << gotoxy(caret.getX(), caret.getY() + 0);
			screenEngine.getCaretY() = caret.getY();
		} else {
			ansout << gotoxy(caret.getX(), caret.getY() + 1);
			screenEngine.getCaretY() = caret.getY() + 1;
		}
		/* *** */
#ifdef HAS_GPM
		if (MouseSupport)  {
			if (mouse_getbutton()==MOUSE_RIGHTBUTTON) a=27;
			if (mouse_getbutton()==MOUSE_LEFTBUTTON) blockModeCommand.run();
		}
#endif
		screenEngine.getCaretX() = caret.getX();
		
		screenEngine.showScreen();
		SDL_Delay(10);
		while (SDL_PollEvent(&event)){
			switch (event.type){
				case SDL_QUIT:
					done = true;
					break;
				case SDL_KEYDOWN:
					caret.handleKeyStroke(&event);
					if (event.key.keysym.mod & KMOD_CTRL) {
						switch (event.key.keysym.sym) {
							case 's':
								selectSauceCommand.run();
								break;
							case 'f':
								fontEditorCommand.run();
								break;
							case 'p':
								paletteEditorCommand.run();
								break;
							default:
								break;
						}
					
					} else if (event.key.keysym.mod & KMOD_ALT) {
						switch (event.key.keysym.sym) {
							case 'z':
								Attribute ^= 128;
								break;
							case 'k':
								asciiTableCommand.run();
								break;
							case '1':
								getCurrentBuffer()->removeColumn(caret.getLogicalX());
								break;
							case '2':
								getCurrentBuffer()->insertColumn(caret.getLogicalX());
								break;
							case 'v':
								viewModeCommand.run();
								break;
							case 'r': /* ALT+R UNDO*/
								UndoLast();
								break;
							case 'g':
								global();
								break;
							case 'u': /* ALT+U PiCKUP COLOR*/
								Attribute = getCurrentBuffer()->getAttribute(caret.getLogicalY(), caret.getLogicalX());
								break;
							case 'w':
								selectOutlineCommand.run();
								break;
							case 'i':/* ALT+I InsLine*/
								getCurrentBuffer()->insertLine(caret.getLogicalY());
								break;
							case 'y': /* ALT+Y - DelLine */
								getCurrentBuffer()->removeLine(caret.getLogicalY());
								break;
							case 'e': /* ALT+E - Elite*/
								caret.eliteMode() = !caret.eliteMode();
								break;
							case 'x': /* ALT+X - Exit */
								exitMysticDraw();
								break;
							case 'a': /* ALT+A - Color */
								selectColorCommand.run();
								break;
							case 'n': /* ALT+N - Font Mode */
								caret.fontMode() = !caret.fontMode();
								Undo=0;
								SaveScreen();
								break;
							case 's':
								save();
								break;
							case 'c': /* ALT+C - ClearScreen*/
								ClearScreen();
								break;
							case 'l':
								load();
								break;
							case 'p': /* ALT+P - SetPage */
								SetPage();
								break;
							case 't':
								tabulatorSetupCommand.run();
								break;
							case 'h': /* alt+h help */
								helpCommand.run();
								break;
							case '-': /* alt+- draw mode */
								drawCommand.run();
								break;
							case 9: /* ALT+TAB */
								// TODO
								//caret.getLogicalX() = getPrevTab(caret.getLogicalX());
								break;
							case 'b':
								blockModeCommand.run();
								break;
							case 'd': /* ALT+D Draw Line Mode */
								drawLineCommand.run();
								break;
							case 'm':
								selectEffectCommand.run();
								break; 
							case 'f': /* ALT+F - Select Font */
								selectFontCommand.run();
								break;
							default:
								break;
						}
					} else {
						switch (event.key.keysym.sym) {
							case SDLK_F1:
								typeCharacter(CharSet[ActiveCharset][0]);
								break;
							case SDLK_F2:
								typeCharacter(CharSet[ActiveCharset][1]);
								break;
							case SDLK_F3:
								typeCharacter(CharSet[ActiveCharset][2]);
								break;
							case SDLK_F4:
								typeCharacter(CharSet[ActiveCharset][3]);
								break;
							case SDLK_F5:
								typeCharacter(CharSet[ActiveCharset][4]);
								break;
							case SDLK_F6:
								typeCharacter(CharSet[ActiveCharset][5]);
								break;
							case SDLK_F7:
								typeCharacter(CharSet[ActiveCharset][6]);
								break;
							case SDLK_F8:
								typeCharacter(CharSet[ActiveCharset][7]);
								break;
							case SDLK_F9:
								typeCharacter(CharSet[ActiveCharset][8]);
								break;
							case SDLK_F10:
								typeCharacter(CharSet[ActiveCharset][9]);
								break;
							case SDLK_PRINT:
								FullScreen = !FullScreen;
								break;
							case SDLK_ESCAPE:
								menuemode();
								break;
							case SDLK_TAB:
								// TODO:
								//caret.getLogicalX() = getNextTab(caret.getLogicalX();
								break;
							case SDLK_RETURN:
								caret.getX()                  = 0;
								caret.getUpperLeftCornerRow() = 0;
								
								if (caret.fontMode() && FontTyped)  {
									caret.getY() += FontLibrary::getInstance().maxY;
									Undo=false;
									SaveScreen();
									FontTyped=false;
								} else {
									caret.getY()++;
								}
								cpos=0;
								break;
							case SDLK_DELETE:
								for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
									getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
									getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
								}
								getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
								getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
								break;
							case SDLK_INSERT:
								caret.insertMode() = !caret.insertMode();
								break;
							case SDLK_BACKSPACE:
								if (caret.getLogicalX()>0){
									if (caret.fontMode() && FontTyped && cpos > 0)  {
										caret.getX() -= CursorPos[cpos] - 1;
										for (a=0;a<=CursorPos[cpos];a++)
										for (b=0;b<=FontLibrary::getInstance().maxY;b++) {
											getCurrentBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a);
											getCurrentBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a);
										}
										cpos--;
									} else {	
										cpos=0;
										caret.getX()--;
										if (caret.insertMode()) {
											for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
												getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
												getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
											}
											getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
											getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
										} else  {
											getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
											getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
										} 
									}
								}
								break;
							default:
								a = event.key.keysym.unicode;
								if (caret.fontMode() && a > 32 && a < 127) {
									renderFontCharacter(a);
								} else  {
									if (caret.fontMode() && FontTyped) {
										cpos++;
										CursorPos[cpos]=2;
									}
									if (caret.eliteMode()) {
										typeCharacter(translate(a)); 
									} else {
										typeCharacter(a);
									}
								}
						}
					
					}
			}
		}
		caret.checkCaretPosition();
	} while (!done);
}

void MysticDrawMain::ClearMessageLine()
{
	ansout << gotoxy(0, LINES - 1) << textattr(7);
	for (int i = 0; i < 80; ++i) {
		ansout << ' ';
	}
}

void MysticDrawMain::renderFontCharacter(char c)
{
	FontLibrary* fl= &FontLibrary::getInstance();
	
	if (!fl->GetFontChar(c)) {
		return;
	}
	
	if (getCaret().getLogicalX() + fl->maxX + fl->getCurrentFont()->spaces > getCurrentBuffer()->getWidth() |
	    fl->maxX == 0 |
		fl->maxY + getCaret().getLogicalY() > getCurrentBuffer()->getHeight())  {
		return;
	}
	
	FontTyped= true;
	
	if (fl->maxY + getCaret().getY() > LINES - 1) {
		getCaret().getUpperLeftCornerLine() += fl->maxY + getCaret().getY() - LINES + 2;
		getCaret().getY()                   -= fl->maxY + getCaret().getY() - LINES + 2;
	}
	
	for (int y = 0; y < fl->maxY; ++y) {
		for (int x = 0; x < fl->maxX; ++x) {
			switch (fl->getCurrentFont()->fontType) {
				case 0:
				case 1:
					getCurrentBuffer()->getCharacter(getCaret().getLogicalY() + y, getCaret().getLogicalX() + x) = fl->charTable[y][x];
					getCurrentBuffer()->getAttribute(getCaret().getLogicalY() + y, getCaret().getLogicalX() + x) = Attribute;
					break;
				case 2:
					getCurrentBuffer()->getCharacter(getCaret().getLogicalY() + y, getCaret().getLogicalX() + x) = fl->charTable[y][x * 2];
					getCurrentBuffer()->getAttribute(getCaret().getLogicalY() + y, getCaret().getLogicalX() + x) = fl->charTable[y][x * 2 + 1];
					break;
			}
		}
	}
	
	cpos++;
	CursorPos[cpos]    = fl->maxX + fl->getCurrentFont()->spaces;
	getCaret().getX() += fl->maxX + fl->getCurrentFont()->spaces - 1;
}

void MysticDrawMain::drawScreen(int startLine, int endLine)
{
	unsigned char oldColor = 7;
	cout << gotoxy(0, startLine);
	if (FullScreen) {
		cout << gotoxy(0, 0);
	} else {
		cout << gotoxy(0, startLine);
	}
	
	cout << textattr(oldColor);
	
	screenEngine.getExtendedFontMode() = getCurrentBuffer()->getExtendedCharMode();
	screenEngine.getBlinkMode()        = getCurrentBuffer()->getBlinkMode();
	if (getCurrentBuffer()->getFont() != 0) {
		screenEngine.setFont(getCurrentBuffer()->getFont());
	} else {
		screenEngine.setFont(screenEngine.getDefaultFont());
	}
	
	if (getCurrentBuffer()->getPalette() != 0) {
		screenEngine.setPalette(getCurrentBuffer()->getPalette());
	} else {
		screenEngine.setPalette(screenEngine.getDefaultPalette());
	}
	
	for (int y = startLine - 1; y <= endLine - startLine + (LINES - 25); ++y) {
		for (int x = 0; x <= 79; ++x) {
			unsigned char newColor = getCurrentBuffer()->getAttribute(y + getCaret().getUpperLeftCornerLine(), x + getCaret().getUpperLeftCornerRow()); 
			if (newColor != oldColor) {
				cout << textattr(newColor);
				oldColor = newColor;
			}
			unsigned char ch = getCurrentBuffer()->getCharacter(y + getCaret().getUpperLeftCornerLine(), x + getCaret().getUpperLeftCornerRow());
			if (ch >= ' ') {
				ansout << ch;
			} else { 
				ansout << ' ';
			}
		}
		if (COLS > 80) {
			ansout << endl;
		}
	}
}

void MysticDrawMain::typeCharacter(unsigned char ch)
{
	if (ch < ' ') {
		return;
	}
	
	if (getCaret().insertMode()) {
		for (int x = getCurrentBuffer()->getWidth() - 1; x >= getCaret().getLogicalX(); x--) {
			getCurrentBuffer()->getCharacter(getCaret().getLogicalY(), x) = getCurrentBuffer()->getCharacter(getCaret().getLogicalY(), x - 1);
			getCurrentBuffer()->getAttribute(getCaret().getLogicalY(), x) = getCurrentBuffer()->getAttribute(getCaret().getLogicalY(), x - 1);
		}
	}
	
	getCurrentBuffer()->getCharacter(getCaret().getLogicalY(), getCaret().getLogicalX()) = ch;
	getCurrentBuffer()->getAttribute(getCaret().getLogicalY(), getCaret().getLogicalX()) = Attribute;
	
	++getCaret().getX();
}

char MysticDrawMain::readCharacter()
{
	int ch = 512;
	SDL_Event event;
	MysticDrawMain::getInstance().ClearMessageLine();
	cout << gotoxy(0, LINES - 1);
	
	CoolWrite("Enter Character :");
	do {
		screenEngine.showScreen();
		SDL_Delay(50);
		
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					ch = event.key.keysym.unicode;
					// TODO : Fx keys
					break;
			}
		}
	} while (ch > 255);
	return ch;
}


void MysticDrawMain::exitMysticDraw()
{
	MysticDrawMain::getInstance().ClearMessageLine();
	cout << gotoxy(0, LINES - 1);
	CoolWrite("Sure ? ");
	switch(chooser(8, 2, "Yes", "No", 0)){
		case 1:
			// saveconfig();
#ifdef HAS_GPM
			if (MouseSupport) {
				mouse_close();
			}
#endif
			done = true;
	}   
}

int main(int argnum,char *args[]) 
{
	MysticDrawMain::getInstance().startMysticDraw(argnum, args);
	cout << "Thank you for using Mystic Draw" << endl;
   return 0;
}
