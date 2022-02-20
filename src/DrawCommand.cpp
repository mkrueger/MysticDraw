#include<DrawCommand.hpp>

void DrawCommand::selectDrawMode()
{
	int a=0;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Select drawmode :");
	drawMode = 0xFF00;
	a = chooser(19, 1, "Character", "aTribute", "Fore", "Back", "Abort", 0);
	switch(a) { 
		case 1:
			a = MysticDrawMain::getInstance().readCharacter();
			drawMode = 0x0100 | (a & 255);
			break;
		case 2:
			drawMode = 0x0200 | Attribute;
			break;
		case 3:
			drawMode = 0x0300 | Attribute & 15;
			break;
		case 4:
			drawMode = 0x0400 | Attribute & 240;
			break;
	}
}

void DrawCommand::run()
{
	int maxy = 22;
	if (FullScreen) {
		maxy++;
	}
	maxy += LINES-25;
	
	selectDrawMode();
	
	if (drawMode == 0xFF00) {
		return;
	}
	
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Drawmode, use cursorkeys, press <ESC> to quit");
	SDL_Event event;
	bool done = false;
	do {
		if (FullScreen) {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY());
		} else {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 1);
		}
		
		screenEngine.showScreen();
		SDL_Delay(50);
		while (SDL_PollEvent(&event)) {
			switch (event.type){
				case SDL_QUIT:
					done = true;
					break;
				case SDL_KEYDOWN:
						switch (event.key.keysym.sym) {
							case SDLK_ESCAPE:
								done = true;
								break;
							default:
								MysticDrawMain::getInstance().getCaret().handleKeyStroke(&event);
								break;
						}
					
					break;
				default:
					break;
			}
		}
		if (MysticDrawMain::getInstance().getCaret().getY() > maxy) {
			MysticDrawMain::getInstance().getCaret().getY() = maxy;
			MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()++;
		}
		MysticDrawMain::getInstance().getCaret().checkCaretPosition();
		int yPos = MysticDrawMain::getInstance().getCaret().getLogicalY();
		int xPos = MysticDrawMain::getInstance().getCaret().getLogicalX();
		switch(drawMode & 0xFF00) {
			case 0x0100:
				MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(yPos, xPos) = drawMode & 0xFF; 
				break;
			case 0x0200:
				MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(yPos, xPos) = drawMode & 0xFF; 
				break;
			case 0x0300:
				MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(yPos, xPos) = drawMode & 0xFF | MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(yPos, xPos) & 240; 
				break;
			case 0x0400:
				MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(yPos, xPos) = drawMode & 0xFF | MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(yPos, xPos) & 15; 
				break;
		}
		MysticDrawMain::getInstance().drawStatusLine();
		if (FullScreen) {
			MysticDrawMain::getInstance().drawScreen(1, 24);
		} else {
			MysticDrawMain::getInstance().drawScreen(1, 23);
		}
		MysticDrawMain::getInstance().updateColorStatus(Attribute);
	} while(!done);
}
int sgn(int a)
{
	if (a > 0) {
		return 1;
	}
	if (a < 0) {
		return -1;
	}
	return 0;
}

void DrawLineCommand::run()
{
	int a = 0, b = 0, c = 0, d = 0,maxy=22;
	if (FullScreen) maxy++;
	maxy+=LINES-25;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Draw line, use cursorkeys, press <ESC> to quit");
	SDL_Event event;
	bool done = false;
	int oldCaretX = MysticDrawMain::getInstance().getCaret().getLogicalX();
	int oldCaretY = MysticDrawMain::getInstance().getCaret().getLogicalY();
	do {
		if (FullScreen) {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY());
		} else {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 1);
		}
		
		oldCaretX = MysticDrawMain::getInstance().getCaret().getLogicalX();
		oldCaretY = MysticDrawMain::getInstance().getCaret().getLogicalY();
		do {
			screenEngine.showScreen();
			SDL_Delay(50);
			while (SDL_PollEvent(&event)) {
				switch (event.type) {
					case SDL_QUIT:
						done = true;
						break;
					case SDL_KEYDOWN:
						switch (event.key.keysym.sym) {
							case SDLK_ESCAPE:
								done = true;
								break;
							default:
								MysticDrawMain::getInstance().getCaret().handleKeyStroke(&event);
								break;
						}
					
						break;
					default:
						break;
				}
			}
			b = sgn(MysticDrawMain::getInstance().getCaret().getLogicalX() - oldCaretX);
			a = sgn(MysticDrawMain::getInstance().getCaret().getLogicalY() - oldCaretY);
		} while (b == 0 && a == 0);
		if (MysticDrawMain::getInstance().getCaret().getY() > maxy) {
			MysticDrawMain::getInstance().getCaret().getY() = maxy;
			MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()++;
		}
		
		MysticDrawMain::getInstance().getCaret().checkCaretPosition();
		
		switch(a) {
			case 1:
			case -1:
				MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(MysticDrawMain::getInstance().getCaret().getLogicalY(), MysticDrawMain::getInstance().getCaret().getLogicalX()) = CharSet[ActiveCharset][5];
				MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(MysticDrawMain::getInstance().getCaret().getLogicalY(), MysticDrawMain::getInstance().getCaret().getLogicalX()) = Attribute;
				break;
		}
		switch(b) {
			case 1:
			case -1:
				MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(MysticDrawMain::getInstance().getCaret().getLogicalY(), MysticDrawMain::getInstance().getCaret().getLogicalX()) = CharSet[ActiveCharset][4];
				MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(MysticDrawMain::getInstance().getCaret().getLogicalY(), MysticDrawMain::getInstance().getCaret().getLogicalX()) = Attribute;
				break;
		}
		if ((c==1)&(b==-1))
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][3];
		if ((c==1)&(b==1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][2];
		if ((c==-1)&(b==-1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][1];
		if ((c==-1)&(b==1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][0];
		if ((a==1)&(d==-1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][0];
		if ((a==1)&(d==1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][1];
		if ((a==-1)&(d==-1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][2];
		if ((a==-1)&(d==1)) 
			MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(oldCaretY, oldCaretX) = CharSet[ActiveCharset][3];
		c = a;
		d = b;
		a=0;
		b=0;
		MysticDrawMain::getInstance().drawStatusLine();
		if (FullScreen) {
			MysticDrawMain::getInstance().drawScreen(1, 24);
		} else {
			MysticDrawMain::getInstance().drawScreen(1, 23);
		}
		MysticDrawMain::getInstance().updateColorStatus(Attribute);
		
		
	} while(!done);
}

