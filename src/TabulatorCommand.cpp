#include<TabulatorCommand.hpp>

int getNextTab(int caretPosition)
{
	for (int i = caretPosition + 1; i < 80; ++i) {
		if (tabs[i]) {
			return i;
		}
	}
	return 79;
}

int getPrevTab(int caretPosition)
{
	for (int i = caretPosition; i >= 0; --i) {
		if (tabs[i]) {
			return i;
		}
	}
	return 0;
}

void TabulatorSetupCommand::run()
{
	int x = 0, ax;
	char * str="";
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CodeWrite("[<]/[>] Move Cursor [S]et/Clear [R]eset [E]rase [I]ncrement [ESC]");
	bool done = false;
	SDL_Event event;
	
	do {
		ansout << gotoxy(0, LINES -3) << textattr(15) << '(' << setfill('0') << setw(2) << x + 1 << ')';
		ansout << gotoxy(0, LINES - 2);
		for (int i = 0; i < 80; ++i) {
			if (tabs[i]) {
				ansout << textattr(7) << (char)254;
			} else {
				ansout << textattr(8) << (char)250;
			}
		}	
		ansout << gotoxy(x, LINES - 2);
		screenEngine.showScreen();
		SDL_Delay(50);
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							done = true;
							break;
						case 'I':
						case 'i':
							ansout << gotoxy(9, LINES - 3);
							CoolWrite("Increment (1-79) :");
							str="";
							str=inputfield(str,2,27,LINES-3);
							ax=strtol(str,NULL,0);
							for (int y=x;y<=80;y++) {
								if (((y-x)%ax)==0) tabs[y]=true;
							}
							ansout << gotoxy(9, LINES - 3);
							ansout << "                   ";	 
							break;
						case 'R':
						case 'r':
							for (int i = 0; i < 80; ++i) {
								tabs[i] = (i % 10 == 0); 
							}
							break;
						case 'E':
						case 'e':
							for (int i =0; i < 80; ++i) {
								tabs[i] = false;
							}
							break;
						case 'S': 
						case 's':
							tabs[x] = !tabs[x];
							break;
						case SDLK_TAB:
							x=getNextTab(x);
							break;
						/* SHIFT TAB
						case :
							ch=getch();
							if (ch==9) x=getPrevTab(x); else ch=27;
							break;*/
						case SDLK_LEFT:
							if (x > 0) {
								x--;
							}
							break;
						case SDLK_RIGHT:
							if (x < 79) {
								x++;
							}
							break;
						default:
							break;
					}
			}
		}
	} while (!done);  
}
