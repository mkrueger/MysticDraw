#ifndef SAUCE__HPP
#define SAUCE__HPP

#include<SDL/SDL.h>
#include<cstdio>
#include<string.h>

class Sauce
{
	public:
		Uint8  id[6];
		Uint8  Version[2];
		Uint8  Title[36];
		Uint8  Author[21];
		Uint8  Group[21];
		Uint8  Date[8];
		Uint32 FileSize;
		Uint8 DataType;
		Uint8 FileType;
		Uint16 TInfo1;
		Uint16 TInfo2;
		Uint16 TInfo3;
		Uint16 TInfo4;
		Uint16 Comments;
		Uint16 Flags;
		Uint16 Filler[22];
		
		void AppendSauce(FILE *fp);
		bool ReadSauce(FILE *fp);
};

#endif
