
void CopyScreen(int p1,int p2)
{
	// !!! TODO !!!
	/*
	int x;
	for (x=0;x<MAX_LINES;x++)  
		memcpy(Screen[p2][x],Screen[p1][x],160);*/
}

void SaveScreen() {
	// !!! TODO !!!
	/*
	CopyScreen(ActivePage,UNDOPage);
	memcpy(&Screen[UNDOPage][0][0],
	&Screen[ActivePage][0][0],MAX_LINES*160);*/
}

void about()
{
	SDL_Event event;
	DrawBox(29,10,61,14);
	ansout << gotoxy(30, 11);
	CoolWrite("coded 1996 by Mike Krueger     ");
	ansout << gotoxy(30, 12);
	CoolWrite("ansis made by Col. Blair^TUSCON");
	ansout << gotoxy(30, 13);
	CoolWrite("Version 1.6 (GPL)              ");
	
	do {
		screenEngine.showScreen();
		SDL_Delay(50);
		
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					return;
			}
		}
	} while (true);
}
