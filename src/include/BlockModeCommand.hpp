#ifndef BLOCKMODECOMMAND__HHP
#define BLOCKMODECOMMAND__HHP

#include<MysticDrawMain.hpp>
#include<algorithm>

class BlockModeCommand : public Command
{
	private:
		int X1,Y1,X2,Y2;
		void CopyBlock(char Mode);
	public:
		void run();
};
#endif
