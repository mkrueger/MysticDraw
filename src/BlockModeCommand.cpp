#include<BlockModeCommand.hpp>

void BlockModeCommand::CopyBlock(char Mode)
{
	bool under = false;
	unsigned char c,d;
	int x,y,ymax=22;
	if (FullScreen) ymax++;
	ymax+=(LINES-25);
	//SaveScreen();
	Undo = true;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CodeWrite("[S]tamp [P]age [U]nder [X]/[Y] Flip [RETURN] [ESC]");
	
	ScreenBlock screenBlock = MysticDrawMain::getInstance().getCurrentBuffer()->getBlock(X1, Y1, X2, Y2);
	
	if (Mode == 2) {
		MysticDrawMain::getInstance().getCurrentBuffer()->clear(X1, Y1, X2, Y2);
	}
	
	SDL_Event event;
	bool done = false;
	do {
		if (FullScreen)  ansout << gotoxy(0, 0); else  ansout << gotoxy(0, 1);  
		for (y=0;y<=ymax;y++) {	 
			for (x=0;x<=79;x++){
				ansout << textattr(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine(), x + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerRow()));
				c = MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine(), x  + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerRow());
				if (c >= ' ') {
					ansout << c;
				} else {
					ansout << ' ';
				}
			}
			if (COLS>80) {
				ansout << endl;
			}
		}
		
		for (y = Y1; y <= Y2 ;y++) {	 
			if (MysticDrawMain::getInstance().getCaret().getY()+(y-Y1)<ymax+1) {
				if (FullScreen) {
					ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY()+(y-Y1));
				} else {
					ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY()+(y-Y1)+1);
				}
				for (x=X1;x<=X2;x++) { 
					if (MysticDrawMain::getInstance().getCaret().getX()+(x-X1)<80)
					{
						ansout << textattr(screenBlock.getAttribute(y - Y1, x - X1));
						c=screenBlock.getCharacter(y - Y1, x - X1);		
						if (under) {
							d=MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(MysticDrawMain::getInstance().getCaret().getLogicalY()+(y-Y1), MysticDrawMain::getInstance().getCaret().getLogicalX()+(x-X1));
							if ((d==32)|(d==0)) {
								if (c >= ' ') {
									ansout << c;
								} else {
									ansout << ' ';
								}
							} else {
								ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX()+(x-X1)+1, MysticDrawMain::getInstance().getCaret().getY()+(y-Y1)+1);
							}
						} else { 
							if (c >= ' ') {
								ansout << c;
							} else {
								ansout << ' ';
							}
						}
					}
				}
			}
			if (COLS>80) {
				ansout << gotoxy(0, MysticDrawMain::getInstance().getCaret().getY()+(y-Y1));
			}
		}
		
		MysticDrawMain::getInstance().drawStatusLine();
		MysticDrawMain::getInstance().updateColorStatus(Attribute);
		if (FullScreen) {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY());
		} else {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 1);
		}
#ifdef HAS_GPM
		if (MouseSupport)   {
			if (mouse_getbutton()==MOUSE_LEFTBUTTON) ch=13;
			if (mouse_getbutton()==MOUSE_RIGHTBUTTON) ch='s';
		}
#endif
		
		screenEngine.showScreen();
		SDL_Delay(10);
		while (SDL_PollEvent(&event)) {
			switch (event.type){
				case SDL_KEYDOWN:
						switch (event.key.keysym.sym) {
							case SDLK_ESCAPE:
								done = true;
								break;
							case SDLK_RETURN:
								MysticDrawMain::getInstance().getCurrentBuffer()->stampBlock(screenBlock, MysticDrawMain::getInstance().getCaret().getLogicalX(), MysticDrawMain::getInstance().getCaret().getLogicalY(), under);
								done = true;
								break;
							default:
								MysticDrawMain::getInstance().getCaret().handleKeyStroke(&event);
								switch (event.key.keysym.unicode) {
									case 'y':
										screenBlock.flipY();
										break;
									case 'x':
										screenBlock.flipX();
										break;
									case 'u':
										under=!under;
										break;
									case 'p':
										// TOOD: SET ACTIVE PAGE
										//if (++ActivePage>2) ActivePage=1;
										break;
									case 's':
										MysticDrawMain::getInstance().getCurrentBuffer()->stampBlock(screenBlock, MysticDrawMain::getInstance().getCaret().getLogicalX(), MysticDrawMain::getInstance().getCaret().getLogicalY(), under);
										break;
								}
								break;
						}
					
					break;
			}
		}
		
		if (MysticDrawMain::getInstance().getCaret().getY()>ymax) {
			MysticDrawMain::getInstance().getCaret().getY()=ymax;
			MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()++;
		}
		MysticDrawMain::getInstance().getCaret().checkCaretPosition();
	} while (!done);
	
#ifdef HAS_GPM
	if (MouseSupport)   {
		do {
			mouse_update();
		} while (mouse_getbutton()==MOUSE_LEFTBUTTON||
		mouse_getbutton()==MOUSE_RIGHTBUTTON);     
	}
#endif
}

void BlockModeCommand::run()
{
	int x1,y1,ch;
	int x,y,maxy=22;
	if (FullScreen) maxy++;
	maxy+=LINES-25;
	MysticDrawMain::getInstance().ClearMessageLine();
	x1=MysticDrawMain::getInstance().getCaret().getLogicalX();
	y1=MysticDrawMain::getInstance().getCaret().getLogicalY();
	
	SDL_Event event;
	bool done = false;
	
	do {
		if (FullScreen) {
			ansout << gotoxy(0, 0);
		} else {
			ansout << gotoxy(0, 1);
		}
		
		if (MysticDrawMain::getInstance().getCaret().getLogicalY()>y1){
			Y1=y1;
			Y2=MysticDrawMain::getInstance().getCaret().getLogicalY();
		} else {
			Y2=y1;
			Y1=MysticDrawMain::getInstance().getCaret().getLogicalY();
		}
		
		if (MysticDrawMain::getInstance().getCaret().getLogicalX()>x1) {
			X1=x1;
			X2=MysticDrawMain::getInstance().getCaret().getLogicalX();
		} else {
			X2=x1;
			X1=MysticDrawMain::getInstance().getCaret().getLogicalX();	 
		}
		
		for (y=0;y<=maxy;y++) {
			for (x=0;x<=79;x++) {
				if ((x + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerRow()>=X1)&(x + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerRow()<=X2)&(y+MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()>=Y1)&(y+MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()<=Y2)) {
					ansout << textattr(112);
				} else {
					ansout << textattr(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine(), x));
				}
				
				unsigned char c = MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine(), x + MysticDrawMain::getInstance().getCaret().getUpperLeftCornerRow());
				if (c >= ' ') {
					ansout << c; 
				} else {
					ansout << ' ';
				}
			}
			if (COLS>80) {
				ansout << endl;
			}
		}
		
		MysticDrawMain::getInstance().drawStatusLine();
		MysticDrawMain::getInstance().updateColorStatus(Attribute);
		ansout << gotoxy(0, LINES - 1);
		CodeWrite("[C]opy [M]ove [F]ill [E]rase [D]elete [O]utline [T]ext [ESC]");
		
		if (FullScreen) {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY());
		} else {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 1);
		}
		screenEngine.showScreen();
		SDL_Delay(10);
		while (SDL_PollEvent(&event)) {
			switch (event.type){
				case SDL_KEYDOWN:
						switch (event.key.keysym.sym) {
							case SDLK_ESCAPE:
								done = true;
								break;
							default:
								MysticDrawMain::getInstance().getCaret().handleKeyStroke(&event);
								switch (event.key.keysym.unicode) {
									case 'm':
										CopyBlock(2);
										done = true;
										break;
									case 'c':
										CopyBlock(1);
										done = true;
										break;
									case 'e': // ERASE BLOCK 
										MysticDrawMain::getInstance().getCurrentBuffer()->clear(X1, Y1, X2, Y2);
										done = true;
										break;
									case 'd': // DELETE BLOCK 
										MysticDrawMain::getInstance().getCurrentBuffer()->clear(X1, Y1, X2, Y2);
										for (y=Y1;y<=Y2;y++) {
											for (x = X2; x < 80; x++) {
												MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x - (X2 - X1)) = MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x);
												MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x - (X2 - X1)) = MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x);
											}
											for (x = 79 - (X2 - X1); x <= 79; ++x) {
												MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x) = ' ';
												MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x) = 7;
											}
										}
										done = true;
										break;
									case 'o': // OUTLINE
										for (y = Y1; y <= Y2; ++y) {
											MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, X1) = CharSet[ActiveCharset][5];
											MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, X1) = Attribute;
											MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, X2) = CharSet[ActiveCharset][5];
											MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, X2) = Attribute;
										}
										for (x = X1; x <= X2; x++) {
											MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y1, x) = CharSet[ActiveCharset][4];
											MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(Y1, x) = Attribute;
											MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y2, x) = CharSet[ActiveCharset][4];
											MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(Y2, x) = Attribute;
										}
										MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y1, X1) = CharSet[ActiveCharset][0];
										MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y1, X2) = CharSet[ActiveCharset][1];
										MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y2, X1) = CharSet[ActiveCharset][2];
										MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(Y2, X2) = CharSet[ActiveCharset][3];
										done = true;
										break;
									case 't': // TEXT
										MysticDrawMain::getInstance().ClearMessageLine();
										ansout << gotoxy(0, LINES - 1);
										switch (chooser(0, 1, "Left", "Center", "Right", "Elite", "eFfect", "Abort", 0)) {
											case 1:
												MysticDrawMain::getInstance().getCurrentBuffer()->leftTrim(X1, Y1, X2, Y2);
												done = true;
												break;
											case 2:
												MysticDrawMain::getInstance().getCurrentBuffer()->center(X1, Y1, X2, Y2);
												done = true;
												break;
											case 3:
												MysticDrawMain::getInstance().getCurrentBuffer()->rightTrim(X1, Y1, X2, Y2);
												done = true;
												break;
											case 4:
												MysticDrawMain::getInstance().getCurrentBuffer()->transformElite(X1, Y1, X2, Y2);
												done = true;
												break;
											case 5:
												MysticDrawMain::getInstance().getCurrentBuffer()->drawEffect(X1, Y1, X2, Y2, effect.Effekt, effect.getColorTable());
												done = true;
												break;
										}
										break;
									case 'f': // FiLL 
										MysticDrawMain::getInstance().ClearMessageLine();
										ansout << gotoxy(0, LINES - 1);
										switch (chooser(0, 1, "Character", "Attribute", "Fore", "Back" , "Abort", 0)) {
											case 1:
												ch = MysticDrawMain::getInstance().readCharacter();
												MysticDrawMain::getInstance().getCurrentBuffer()->fillCharacter(ch, X1, Y1, X2, Y2);
												done = true;
												break;
											case 2:
												MysticDrawMain::getInstance().getCurrentBuffer()->fillAttribute(Attribute, X1, Y1, X2, Y2);
												done = true;
												break;
											case 3:
												MysticDrawMain::getInstance().getCurrentBuffer()->fillForeColor(Attribute & 15, X1, Y1, X2, Y2);
												done = true;
												break;
											case 4:
												MysticDrawMain::getInstance().getCurrentBuffer()->fillBackColor(Attribute & 240, X1, Y1, X2, Y2);
												done = true;
												break;
										}
										break;
								}
								break;
						}
					
					break;
				default:
					break;
			}
		}
		if (MysticDrawMain::getInstance().getCaret().getY()>maxy) {
			MysticDrawMain::getInstance().getCaret().getY()=maxy;
			MysticDrawMain::getInstance().getCaret().getUpperLeftCornerLine()++;
		}
		MysticDrawMain::getInstance().getCaret().checkCaretPosition();
	} while (!done);
}
