#include"sauce.hpp"

unsigned char oldColor=7;
char *AvatarColor(int col){
	char *tmp;
	tmp=(char *)malloc(4);
	if (col==oldColor) return NULL;
	sprintf(tmp,"%c",col);
	oldColor=col;
	return tmp;
}
char *strupr(char *str){
	unsigned int x;
	for (x=0;x<=strlen(str);x++)
		str[x]=toupper(str[x]);
	return str;
}

char *PCBoardColor(int col){
	char *a;
	if (col==oldColor) return NULL;
	a=(char *)malloc(4);
	sprintf(a,"@X%02x",col);
	a=strupr(a);
	oldColor=col;   
	return a;
}

char *AnsiColor(unsigned char col){
	char *a;
	if (col==oldColor) return NULL;
	a=(char *)malloc(30);
	*a = 0;
	if ((oldColor>=128)&&(col<128)) {
		sprintf(a,"[0m");
		oldColor=7;
		if (col==7) return a;
	}
	sprintf(a,"%s[",a);
	if (((col&8)!=8)&((oldColor&8)==8)) { 
		sprintf(a,"%s0;",a);
		oldColor=oldColor&15;
	}
	if (((col&8)==8)&((oldColor&8)!=8)) sprintf(a,"%s1;",a);   
	
	if ((col&128)==128) {
		sprintf(a,"%s5",a);
		if ((col&15)!=(oldColor&15)||(col&112)!=(oldColor&112))
			sprintf(a,"%s;",a);
	}
	
	if ((col&15)!=(oldColor&15))
	switch (col&7) {
		case 0:
		sprintf(a,"%s30",a);
		break;
		case 1:
		sprintf(a,"%s34",a);
		break;
		case 2:
		sprintf(a,"%s32",a);
		break;
		case 3:
		sprintf(a,"%s36",a);
		break;
		case 4:
		sprintf(a,"%s31",a);
		break;
		case 5:
		sprintf(a,"%s35",a);
		break;
		case 6:
		sprintf(a,"%s33",a);
		break;
		case 7:
		sprintf(a,"%s37",a);
		break;
	}   
	if (((col&15))!=(oldColor&15)&&(col&112)!=(oldColor&112)) 
		sprintf(a,"%s;",a);
	if ((col&112)!=(oldColor&112))
	switch ((col&112)>>4) {
		case 0:
		sprintf(a,"%s40",a);
		break;
		case 1:
		sprintf(a,"%s44",a);
		break;
		case 2:
		sprintf(a,"%s42",a);
		break;
		case 3:
		sprintf(a,"%s46",a);
		break;
		case 4:
		sprintf(a,"%s41",a);
		break;
		case 5:
		sprintf(a,"%s45",a);
		break;
		case 6:
		sprintf(a,"%s43",a);
		break;
		case 7:
		sprintf(a,"%s47",a);
		break;
	}   
	sprintf(a,"%sm",a); 
	oldColor=col;
	return a;
}

int SelectSaveMode()
{
	MysticDrawMain::getInstance().ClearMessageLine();
	return chooser(16, 1, "Clearscreen", "Home", "None", 0);
}

char *EnterName(char *b) {
	char *a;
	char *ext;
	MysticDrawMain::getInstance().ClearMessageLine();
	ansout << gotoxy(0, LINES - 1);
	CoolWrite("Enter Filename :");
	a="";
	a=inputfield(a,60,16,LINES-1);
	if (strlen(a)==0) a=strdup("NONAME");
	ext=strchr(a,'.');
	if (ext==NULL) a=strcat(a,b);
	return a;
}

int CharCount(int d,int e,int a,int chr) {
	int b,c=0;
	for (b=d;b<=e;b++)
		if ((MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(a, b)==chr)&(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(a, b)==MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(a, d))) c++; else break;
	return c;
}

int Numberofchars(int a) {
	int b,c=0;
	for (b=0;b<=79;b++) 
		if ( ( (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(a, b) != ' ') & (MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(a, b)!=0) 
			)|
	((MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(a, b)&112)!=0) ) c=b;
	return c;
}

void save() {
	char *Name,*s;
	FILE *fp;
	int x,y,z,chnum;
	oldColor=0;
	ansout << gotoxy(0, LINES - 1);
	x = chooser(0, 1, "aNsi", "aVatar", "Pcboard", "aScii", "Binary", "C", "XBin", "Abort", 0);
	int lastLine = MysticDrawMain::getInstance().getCurrentBuffer()->getLastNonEmptyLine();
	switch(x) {
		case 1: /*ANSI Save*/
			Name=EnterName(".ans");
			fp=fopen(Name,"wb");
			switch(SelectSaveMode()) {
				case 1:
				fprintf(fp,"[2J");
				break;
				case 2:
				fprintf(fp,"[1;1H");
				break;
			}
			for (y=0;y<=lastLine;y++) {
				/*	if (y>0) if (Numberofchars(y-1)>=79) fprintf(fp,"[A");*/
				chnum=Numberofchars(y);
				for (x=0;x<=chnum;x++) {
					z=CharCount(x,chnum,y,' '); 
					if ((z>2)&((MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x)&112)==0)) {
						fprintf(fp,"[%dC",z);
						x+=z-1;
					} else {
						if (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)==0) MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)=32;
						s=AnsiColor(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x));
						if (s!=NULL) fprintf(fp,"%s",s);
						fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x), fp);
					}
				}	 
				fputc(13,fp);
				fputc(10,fp);
			}
			fprintf(fp,"[0m");
			if (MysticDrawMain::getInstance().getCurrentBuffer()->doSaveSauce()) {
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().DataType = 1;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().FileType = 1;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo1   = 80;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo2   = lastLine;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().AppendSauce(fp);
			}
			fclose(fp);
			break;      
		case 2: /*AVATAR Save*/
			Name=EnterName(".avt");      
			fp=fopen(Name,"wb");
			switch(SelectSaveMode()) {
				case 1:
					fputc('',fp);
					break;
				case 2:
					fprintf(fp,"");
					break;
			}
			for (y=0;y<=lastLine;y++) {
				chnum=Numberofchars(y);
				for (x=0;x<=chnum;x++) {
					z=CharCount(x,chnum,y,MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x));	    
					s=AvatarColor(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x));
					if (s!=NULL) fprintf(fp,"%s",s);
					if ((z>2)&((MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x)&112)==0)) {
						fprintf(fp,"%c%c",MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),z);
						x+=z-1;
					} else {
						if (MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x)==0) fputc(0,fp);
						if (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)==0) MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)=32;
						fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),fp);
					}	    
				}
				fputc(13, fp);
				fputc(10, fp);
			}
			fprintf(fp,"");
			if (MysticDrawMain::getInstance().getCurrentBuffer()->doSaveSauce()) {
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().DataType = 1;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().FileType = 5;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().AppendSauce(fp);
			}
			fclose(fp);
			break;
		case 3:
			Name=EnterName(".pcb");      
			fp=fopen(Name,"wb");
			switch(SelectSaveMode()) {
				case 1:
				fprintf(fp,"@CLS@");
				break;
				case 2:
				fprintf(fp,"@HOME@");
				break;
			}
			for (y=0;y<=lastLine;y++) {
				for (x=0;x<=Numberofchars(y);x++) {	    
					if (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)==0) MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)=32;
					s=PCBoardColor(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x));
					if (s!=NULL) fprintf(fp,"%s",s);
					fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),fp);
				}
				fputc(13, fp);
				fputc(10, fp);
			}
			if (MysticDrawMain::getInstance().getCurrentBuffer()->doSaveSauce()) {
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().DataType = 1;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().FileType = 4;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo1   = 80;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo2   = lastLine;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().AppendSauce(fp);
			}
			fclose(fp);
			break;
		case 4: 
			Name=EnterName(".asc");
			fp=fopen(Name,"wb");
			for (y=0;y<=lastLine;y++) {
				for (x=0;x<=Numberofchars(y);x++) {
					if (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)==0) MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)=32;
					fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),fp);
				}	 
				fputc(13, fp);
				fputc(10, fp);
			}
			if (MysticDrawMain::getInstance().getCurrentBuffer()->doSaveSauce()) {
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().DataType = 1;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().FileType = 0;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo1   = 80;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().TInfo2   = lastLine;
				MysticDrawMain::getInstance().getCurrentBuffer()->getSauce().AppendSauce(fp);
			}
			fclose(fp);
			break;
		case 5:
			Name=EnterName(".bin");
			fp=fopen(Name,"wb");
			for (y=0;y<=lastLine;y++) {
				for (x=0;x<MysticDrawMain::getInstance().getCurrentBuffer()->getWidth();x++) {
					fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),fp);
					fputc(MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x),fp);
				}
			}
			fclose(fp);
			break;
		case 6:
			Name=EnterName(" ");
			fp=fopen(Name,"wb");
			fprintf(fp,"unsigned char %s[%d]={\n",Name,(lastLine+1)*160);
			for (y=0;y<=lastLine;y++) {
				for (x=0;x<MysticDrawMain::getInstance().getCurrentBuffer()->getWidth();x++) {
					fprintf(fp,"%d,%d", MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x),
					MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x));
					if (x + 1 <MysticDrawMain::getInstance().getCurrentBuffer()->getWidth()) fputc(',',fp);
				}
				if (y<lastLine) fputc(',',fp); else fprintf(fp,"};");
				fputc(13, fp);
				fputc(10, fp);
			}
			fclose(fp);
			break;
		case 7:
			Name=EnterName(" ");
			MysticDrawMain::getInstance().getCurrentBuffer()->save(Name, XBinary);
			break;
	}   
}
