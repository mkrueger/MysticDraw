#ifndef DRAWCOMMAND_HPP
#define DRAWCOMMAND_HPP

#include<command.hpp>
#include<bio_crt.h>
#include<MysticDrawMain.hpp>

class DrawCommand : Command
{
	private:
		unsigned int drawMode;
		void selectDrawMode();		
	public: 
		void run();
};

class DrawLineCommand : Command
{
	public: 
		void run();
};


#endif
