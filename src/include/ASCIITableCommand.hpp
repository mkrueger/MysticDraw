#ifndef ASCII_TABLE__HPP
#define ASCII_TABLE__HPP

#include<bio_crt.h>
#include<command.hpp>
#include<MysticDrawMain.hpp>

class ASCIITableCommand : public Command
{
	private:
		unsigned char show_ASCIITable();
	public:
		void run();
};

#endif
