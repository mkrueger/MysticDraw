#include<ASCIITableCommand.hpp>

unsigned char ASCIITableCommand::show_ASCIITable()
{
	int x=32;
	SDL_Event event;
	bool done = false;
	DrawBox(29,4,46,20);
	do {
		ansout << gotoxy(30, 5) << textattr(7) << setfill('0');
		ansout << "hex :" << setw(2) << hex << x << "  dec:" << dec << setw(3) << x;
		for (int y = 32; y <= 255; y++) {
			ansout << gotoxy(30+y%16, 6+(y-32)/16);
			if (y==x) {
				ansout << textattr(27);
			} else {
				ansout << textattr(3);
			}
			// filter out non printable characters (btw. why is 155 not printable?)
			if (y != 127 && y != 155) {
				ansout << (char)y;
			} else {
				ansout << '.';
			}
		}
		
#ifdef HAS_GPM
		if (MouseSupport==TRUE)  {
			Gpm_DrawPointer(mouse_x+1,mouse_y+1,1);
			if (mouse_button==MOUSE_RIGHTBUTTON) 
				return 0;
			if (mouse_x>29&&mouse_x<46&&
			mouse_y>5&&mouse_y<20) {
				x=(mouse_x-30)+(mouse_y-4)*16;
				if (mouse_button==MOUSE_LEFTBUTTON) 
					return x;
			}
		}
#endif
		screenEngine.showScreen();
		SDL_Delay(50);
		
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							done = true;
							break;
						case SDLK_RETURN:
							return x;
						case SDLK_LEFT:
							if (x>32) {
								x--;
							} else {
								x = 255;
							}
							break;
						case SDLK_RIGHT:
							if (x<255) {
								x++;
							} else {
								x = 32;
							}
							break;
						case SDLK_DOWN:	 
							if (x <= 239) {
								x += 16;
							}
							break;
						case SDLK_UP:
							if (x>=48) {
								x -= 16;
							}
							break;
						default:
							break;
					}
			}
		}
	} while (!done);
	
	return 0;
}

void ASCIITableCommand::run()
{
	MysticDrawMain::getInstance().typeCharacter(show_ASCIITable());
}
