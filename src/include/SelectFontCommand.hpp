#ifndef SELECTFONTCOMMAND__HPP
#define SELECTFONTCOMMAND__HPP

#include<math.h>
#include<bio_crt.h>
#include<command.hpp>
#include<fonts.hpp>
#include<iostream>

class SelectOutlineCommand : public Command
{
		private:
			void setunderground(int Underg,int b);
			
		public:
			void run();
};

class SelectFontCommand : public Command
{
	private:
		void clearWindow();
		void showPreviewCharacter(int fontNumber);
		
	public:
		void run();
};

#endif
