#ifndef TABULATORCOMMAND__HPP
#define TABULATORCOMMAND__HPP

#include<MysticDrawMain.hpp>
#include<command.hpp>

class TabulatorSetupCommand : public Command
{
	public:
		void run();
};

extern int getPrevTab(int caretPosition);
extern int getNextTab(int caretPosition);

#endif
