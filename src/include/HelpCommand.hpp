#ifndef HELP__HPP
#define HELP__HPP
#include<bio_crt.h>
#include<command.hpp>

class HelpCommand : public Command
{
	public:
		void run();
};

#endif
