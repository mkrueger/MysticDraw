
void ClearScreen()
{
	int x;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Clear Screen :");
	x=chooser(15, 1, "Yes", "No", 0);
	Undo=false;
	if (x==1) {
		MysticDrawMain::getInstance().getCurrentBuffer()->clear();
	}
	SaveScreen();
}

void global() {
	int x,ch;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Global :");
	x = chooser(10, 1, "Fill", "Copy", "Text", "Abort", 0);
	switch (x) {
		case 1:
			MysticDrawMain::getInstance().ClearMessageLine();
			ansout << gotoxy(0, LINES - 1);
			CoolWrite("Fill :");
			x=chooser(8, 1, "Character", "aTribute", "Fore", "Back", "Abort", 0);
			switch (x) {
				case 1:
					ch = MysticDrawMain::getInstance().readCharacter();
					MysticDrawMain::getInstance().getCurrentBuffer()->fillCharacter(ch);
					break;
				case 2:
					MysticDrawMain::getInstance().getCurrentBuffer()->fillAttribute(Attribute);
					break;
				case 3:
					MysticDrawMain::getInstance().getCurrentBuffer()->fillForeColor(Attribute & 15);
					
					break;
				case 4:
					MysticDrawMain::getInstance().getCurrentBuffer()->fillBackColor(Attribute & 240);
					break;
			}
			break;
		case 2:
			MysticDrawMain::getInstance().ClearMessageLine();
			ansout << gotoxy(0, LINES - 1);
			CoolWrite("Copy to page :");
			switch(chooser(15, 1, "1", "2", "Abort", 0)){
				case 1:
					// TODO !!!
					//CopyScreen(ActivePage,1);
					break;
				case 2:
					// TODO !!!
					//CopyScreen(ActivePage,2);
					break;
			};      
			break;
		case 3:
			MysticDrawMain::getInstance().ClearMessageLine();
			ansout << gotoxy(0, LINES - 1);
			CoolWrite("Text :");
			switch (chooser(8, 1, "Left", "Right", "Center", "Elite", "eFfect", "Abort", 0)){
				case 1:
					MysticDrawMain::getInstance().getCurrentBuffer()->leftTrim();
					break;
				case 2:
					MysticDrawMain::getInstance().getCurrentBuffer()->rightTrim();
					break;
				case 3:
					MysticDrawMain::getInstance().getCurrentBuffer()->center();
					break;
				case 4: 
					MysticDrawMain::getInstance().getCurrentBuffer()->transformElite();
					break; 
				case 5:
					MysticDrawMain::getInstance().getCurrentBuffer()->drawEffect(effect.Effekt, effect.getColorTable());
					break;
			}      
	}   
}

void SetPage()
{
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Set Page :");
	int i = chooser(12, MysticDrawMain::getInstance().getCurrentBufferNumber() + 1, "1", "2", 0);
	if (i >= 1 && i <= 2) {
		MysticDrawMain::getInstance().getCurrentBufferNumber() = i -  1;
	}
}

void UndoLast() {
	if (Undo) {
		MysticDrawMain::getInstance().ClearMessageLine();
		ansout << gotoxy(0, LINES - 1);
		CoolWrite("Undo :");
		// TODO !!!
		//if (chooser(7, ActivePage, "Yes", "No", 0) == 1)  {
		//	CopyScreen(UNDOPage,ActivePage);
		//}
	}
}

