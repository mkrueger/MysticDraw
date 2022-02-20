#include<SelectEffectModeCommand.hpp>

void SelectEffectModeCommand::draweffekt(int effekt,char *blabla,int highlite) 
{
	int y=0;
	char col = '\0';
	for (unsigned int x=0;x<strlen(blabla);x++) {
		y++;
		switch(effekt) {
			case 1:
				col=effect.colorTable[1][1];
				if (blabla[x]>='0'&&blabla[x]<='9') col=effect.colorTable[1][2];
				if (blabla[x]>='A'&&blabla[x]<='Z') col=effect.colorTable[1][3];
				if (blabla[x]>='a'&&blabla[x]<='z') col=effect.colorTable[1][4];
				if (blabla[x]<0) col=effect.colorTable[1][5];
				break;
			case 2:
				if (blabla[x]==' ') y=0;
				switch(y) {
					case 1:
						col=effect.colorTable[2][1];
						break;
					case 2:
						col=effect.colorTable[2][2];
						break;
					case 3:
						col=effect.colorTable[2][3];
						break;
					case 4:
						col=effect.colorTable[2][4];
						break;
					default:
						col=effect.colorTable[2][5];
				}
				break;
			case 3:
				col=effect.colorTable[3][5];
				for (unsigned int i=0;i<4;i++)  
					if (x==0+i||x==strlen(blabla)-i-1) col=effect.colorTable[3][1+i];
				break;
			case 4:
				if (x%2==0) 
					col=effect.colorTable[4][1]; else
				col=effect.colorTable[4][2];	 
				break;
		}
		ansout << textattr(col);
		if (highlite==1) ansout << textattr(col+16);
		ansout << blabla[x];
	}
}

void SelectEffectModeCommand::changecolor(int Effekt)
{
	int x=1,y,colnum=5;
	DrawBox(39,9,52,15);
	for (y=1;y<=5;y++) {
		ansout << gotoxy(40, 9 + y) << "Color " << y << ":";
		if (y>colnum) {
			ansout << " --";
		}
	}   
	SDL_Event event;
	bool done = false;
	do {
		ansout << gotoxy(11, 10+Effekt);
		draweffekt(Effekt,"ABCabc123!@# Sample",0);
		for (y=1;y<=colnum;y++) {
			ansout << gotoxy(48, 9 + y) << textattr(15);
			if (y==x) ansout << ">"; else ansout << " ";
			ansout << textattr(effect.colorTable[Effekt][y]);
			ansout << "##";
			ansout << textattr(15);
			if (y==x) ansout << "<"; else ansout << " ";
		}
		screenEngine.showScreen();
		SDL_Delay(50);
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
						case SDLK_RETURN:
							done = true;
							break;
						case SDLK_LEFT:
							effect.colorTable[Effekt][x]--;
							if (effect.colorTable[Effekt][x]>=128) effect.colorTable[Effekt][x]=15;
							break;
						case SDLK_RIGHT:
							effect.colorTable[Effekt][x]++;
							if (effect.colorTable[Effekt][x]>15) effect.colorTable[Effekt][x]=0;
							break;
						case SDLK_UP:
							x--;
							if (x==0) x=colnum;
							break;	 
						case SDLK_DOWN:
							x++;
							if (x>colnum) x=1;
							break;
						default:
							break;
					}
				default:
					break;
			}
		}
	} while (!done);
}

void SelectEffectModeCommand::run()
{
	int x = 0, y = effect.Effekt;
	if (y<1) {
		y = 1;
	}
	DrawBox(10,10,30,15);
	ansout << gotoxy(11, 14) << textattr(7);
	ansout << "c - change color   ";
	SDL_Event event;
	bool done = false;
	do {
		for (x=1;x<=3;x++) {	 
			ansout << gotoxy(11, 10 + x);
			if (y == x) {
				draweffekt(x,"ABCabc123!@# Sample",1);
			} else { 
				draweffekt(x,"ABCabc123!@# Sample",0);
			}
		}
		screenEngine.showScreen();
		SDL_Delay(10);
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							done = true;
							break;
						case SDLK_RETURN:
							effect.Effekt=y;
							done = true;
							break;
						case SDLK_UP:
							y--;
							if (y==0) y=3;
							break;
						case SDLK_DOWN:
							y++;
							if (y==4) y=1;
							break;
						default:
							switch (event.key.keysym.unicode) {
								case 'C':
								case 'c':
									cout << "CHANGECOLOR" << endl;
									changecolor(y);
									break;
							}
							break;
							
					}
					break;
			}
		}
	} while (!done);
}
