unsigned char ActiveMenue=1,MaxItem;
char * MenueItem[20],Length;
int MouseX,MouseY;

int Menues(int x, int y)
{	
	int a,b,c,d;
	DrawBox(x,y,x+Length+1,y+MaxItem+1);
	b=1;
	c=255;
	SDL_Event event;
	bool done = false;
	do {
		if (c!=b) { 
			for (a=1;a<=MaxItem;a++) {
				ansout << gotoxy(x + 1, y + a);
				if (b==a) ansout << textattr(32+15); else ansout << textattr(10);
				for (d=1;d<=2;d++) {
					ansout << MenueItem[a][d];
				}
				if (b==a) ansout << textattr(32+15); else ansout << textattr(2);
				for (d=3;d<Length-6;d++) {
					ansout << MenueItem[a][d];
				}
				if (b==a) ansout << textattr(32+8); else ansout << textattr(7);
				for (d=Length-6;d<=Length;d++) {
					ansout << MenueItem[a][d];
				}
			}
		}
		c=b;
#ifdef HAS_GPM
		if (MouseSupport==TRUE) 
			Gpm_DrawPointer(mouse_x+1,mouse_y+1,1);
#endif
		if (FullScreen) {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 0);
		} else {
			ansout << gotoxy(MysticDrawMain::getInstance().getCaret().getX(), MysticDrawMain::getInstance().getCaret().getY() + 1);
		}

#ifdef HAS_GPM
		if (MouseSupport==TRUE)  {	 
			mouse_update();
			MouseX=mouse_getx();
			MouseY=mouse_gety();
			if (MouseX>=x&&MouseX<=x+Length+1&&MouseY>y&&MouseY<y+MaxItem+1) {
				b=MouseY-y;
				if (mouse_getbutton()==MOUSE_LEFTBUTTON) ch=13;
			}
		}
#endif
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
							done = true;
							break;
						case SDLK_RETURN:
							return b;
						case SDLK_UP:
							b--;
							break;
						case SDLK_DOWN:
							b++;
							break;
						case SDLK_LEFT:
							return 253;
						case SDLK_RIGHT:
							return 254;
						default:
							break;
					}
				}
			}
		if (b<1) b=1;
		if (b>MaxItem) b=MaxItem;
		
		#ifdef HAS_GPM
		if (MouseSupport==TRUE)  {
			if (MouseX>79) 
				MouseX=79;
			if (MouseY>25) 
				MouseY=25;
			if (mouse_getbutton()==MOUSE_RIGHTBUTTON) ch=27;
			if (mouse_getbutton()==MOUSE_LEFTBUTTON&&MouseY==1) {
				if (MouseX>=2&&MouseX<=6){
					ActiveMenue=1;
					return 252;
				}
				if (MouseX>=13&&MouseX<=17){
					ActiveMenue=2;
					return 252;
				}
				if (MouseX>=24&&MouseX<=30){
					ActiveMenue=3;
					return 252;
				}      
				if (MouseX>=37&&MouseX<=42){
					ActiveMenue=4;
					return 252;
				}      
				if (MouseX>=49&&MouseX<=53){
					ActiveMenue=5;
					return 252;
				}      
				if (MouseX>=62&&MouseX<=68){
					ActiveMenue=6;
					return 252;
				}      
				if (MouseX>=73&&MouseX<=76){
					ActiveMenue=7;
					return 252;
				}
			}
		}
		#endif
	} while (!done);
	#ifdef HAS_GPM
	if (MouseSupport==TRUE)  
	do {
		mouse_update();
	} while (mouse_getbutton()==MOUSE_RIGHTBUTTON);
	#endif
	return 255;
}

int menue()
{		  
	int x,a,b;
	b=0;
	ActiveMenue=1;
	#ifdef HAS_GPM
	if (MouseSupport==TRUE)  
	do {
		mouse_update();
	} while (mouse_getbutton()==MOUSE_RIGHTBUTTON);   
	#endif
	do {	 
		a=0;
		if (ActiveMenue!=b) {
			if (FullScreen) {
				MysticDrawMain::getInstance().drawScreen(1, 25);
			} else {
				MysticDrawMain::getInstance().drawScreen(1, 24);
			}
			ansout << gotoxy(0, 0)<< textattr(8);
			ansout << (char)223;
			ansout << textattr(7);
			ansout << ' ';
			ansout << textattr(8);
			for (x=1;x<79;x++) {
				ansout << (char)220;
			}
			ansout << textattr(7);
			ansout << gotoxy(79, 0);
			ansout << (char)223;
			ansout << gotoxy(0, 1);
			ansout << textattr(2);
			ansout << (char)223;
			if (ActiveMenue==1) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << " FILES      ";
			if (ActiveMenue==2) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "FONTS      ";
			if (ActiveMenue==3) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "OPTIONS      ";
			if (ActiveMenue==4) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "SCREEN      ";
			if (ActiveMenue==5) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "MISC.        ";
			if (ActiveMenue==6) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "TOGGLES    ";
			if (ActiveMenue==7) ansout << textattr(8+32); else ansout << textattr(15+32);
			ansout << "HELP";
			ansout << textattr(2);
			ansout << (char)219 << (char)220 << (char)223;
		}
		b=ActiveMenue;
		switch (ActiveMenue) {
			case 1:
			MenueItem[1]=" LOAD       ALT+L  ";
			MenueItem[2]=" SAVE       ALT+S  ";
			MenueItem[3]=" QUIT       ALT+X  ";
			Length=18;
			MaxItem=3;
			a=Menues(0,2);
			break;
			case 2:
			MenueItem[1]=" SELECT FONT    ALT+F  ";
			MenueItem[2]=" FONT MODE      ALT+N  ";
			MenueItem[3]=" OUTLINE TYPE   ALT+W  ";
			Length=22;
			MaxItem=3;
			a=Menues(12,2);
			break;
			case 3:
			MenueItem[1]=" SAUCE SETUP   CTRL+S ";
			MenueItem[2]=" SET PAGE      ALT+P  ";
			MenueItem[3]=" TAB SETUP     ALT+T  ";
			MenueItem[4]=" GLOBAL        ALT+G  ";
			MenueItem[5]=" SET EFFECT    ALT+M  ";
			Length=20;
			MaxItem=5;
			a=Menues(25,2);
			break;
			case 4:
			MenueItem[1]=" CLEAR PAGE    ALT+C  ";
			MenueItem[2]=" INSERT LINE   ALT+I  ";
			MenueItem[3]=" DELTE LINE    ALT+Y  ";
			MenueItem[4]=" INSERT COLUMN ALT+1  ";
			MenueItem[5]=" DELTE COLUMN  ALT+2  ";
			MenueItem[6]=" UNDO/RESTORE  ALT+R  ";
			Length=21;
			MaxItem=6;
			a=Menues(38,2);
			break;
			case 5:
			MenueItem[1]=" SET COLORS    ALT+A  ";
			MenueItem[2]=" PICK UP COLOR ALT+U  ";
			MenueItem[3]=" ASCII TABLE   ALT+K  ";
			Length=21;
			MaxItem=3;
			a=Menues(50,2);
			break;
			case 6:
			MenueItem[1]=" LINE DRAW        ALT+D  ";
			MenueItem[2]=" DRAW MODE        ALT+-  ";
			MenueItem[3]=" INSERT MODE      INS    ";
			MenueItem[4]=" VIEW IN 320x200  ALT+V  ";
			MenueItem[5]=" ELITE MODE       ALT+E  ";
			Length=24;
			MaxItem=5;
			a=Menues(53,2);
			break;
			case 7:
			MenueItem[1]=" HELP         ALT+H  ";
			MenueItem[2]=" ABOUT               ";
			Length=20;
			MaxItem=2;
			a=Menues(56,2);
			break;
		};
		switch(a) {			    
			case 253:
			ActiveMenue--;
			break;
			case 254:
			ActiveMenue++;
			break;
			case 255:
			return 0;
			break;
		};
		if (ActiveMenue<1) ActiveMenue=7;
		if (ActiveMenue>7) ActiveMenue=1; 
	} while (a>200);
	#ifdef HAS_GPM
	if (MouseSupport==TRUE)  
	do {
		mouse_update();
	} while (mouse_getbutton()==MOUSE_RIGHTBUTTON);   
	#endif
	return a+(ActiveMenue<<8);
}

void menuemode()
{
	unsigned int a=0;
	a=menue();
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
	
	switch((a&0xFF00)>>8) {		       
		case 1:
		switch(a&0xFF) {			    
			case 1:
			load();
			break;
			case 2:
			save();
			break;
			case 3:
			MysticDrawMain::getInstance().exitMysticDraw();
			break;
		}
		break;      
		case 2:
		switch(a&0xFF) {			    
			case 1:
			selectFontCommand.run();
			break;       
			case 2:
			MysticDrawMain::getInstance().getCaret().fontMode() = !MysticDrawMain::getInstance().getCaret().fontMode(); 
			Undo = false;
			SaveScreen();
			break;       
			case 3: 
			selectOutlineCommand.run();
			break;
		}
		break;
		case 3:
		switch(a&0xFF) {			    
			case 1:
			selectSauceCommand.run();
			break;
			case 2:
			SetPage();
			break;
			case 3:
			tabulatorSetupCommand.run();
			break;
			case 4:
			global();
			break;
			case 5:
			selectEffectCommand.run();
			break;
		}
		break;
		case 4:
		switch(a&0xFF) {			    
			case 1:
			ClearScreen();
			break;
			case 2:
			MysticDrawMain::getInstance().getCurrentBuffer()->insertLine(MysticDrawMain::getInstance().getCaret().getLogicalY());
			break;
			case 3:
			MysticDrawMain::getInstance().getCurrentBuffer()->removeLine(MysticDrawMain::getInstance().getCaret().getLogicalY());
			break;
			case 4:
			MysticDrawMain::getInstance().getCurrentBuffer()->insertColumn(MysticDrawMain::getInstance().getCaret().getX());
			break;
			case 5:
			MysticDrawMain::getInstance().getCurrentBuffer()->removeColumn(MysticDrawMain::getInstance().getCaret().getX());
			break;
			case 6:
			UndoLast();
			break;
		}
		break;
		case 5:
		switch(a&0xFF) {			    
			case 1:
			selectColorCommand.run();
			break;
			case 2:
			Attribute = MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(MysticDrawMain::getInstance().getCaret().getLogicalY(), MysticDrawMain::getInstance().getCaret().getX());
			break;
			case 3:
			asciiTableCommand.run();
			break;
		}
		break;
		case 6:
		switch(a&0xFF) {			    
			case 1:
			drawLineCommand.run();
			break;
			case 2:
			drawCommand.run();
			break;
			case 3:
			MysticDrawMain::getInstance().getCaret().insertMode() = !MysticDrawMain::getInstance().getCaret().insertMode();
			break;
			#ifdef HAS_SVGALIB
			case 4:
			viewmode();
			break;
			#endif
			case 5:
			MysticDrawMain::getInstance().getCaret().eliteMode() = !MysticDrawMain::getInstance().getCaret().eliteMode();
			break;
		}
		case 7:
		switch(a&0xFF) {
			case 1:
			helpCommand.run();
			break;
			case 2:
			about();
			break;
		}
		break;
	}
}
