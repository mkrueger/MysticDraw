#ifndef EFFECTMODECOMMAND__HPP
#define EFFECTMODECOMMAND__HPP

#include<command.hpp>
#include<MysticDrawMain.hpp>

class SelectEffectModeCommand : public Command
{
	private:
		void draweffekt(int effekt,char *blabla,int highlite);
		void changecolor(int Effekt);
	public:
		void run();
};

#endif
