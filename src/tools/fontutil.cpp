/* same license as Mystic Draw*/
#include<unistd.h>
#include<stdio.h>
#include<stdlib.h>
#include"include/fonts.h"
#include<ncurses.h>
#include<string.h>
#include<bio_crt.h>
#include<errno.h>
#include<ctype.h>

void ClearWindow(void) {
   int x,y;
   for (y=5;y<=17;y++) {
      move(y,30);
      for (x=30;x<=60;x++) writechar(32);
   }
};

void ShowCharacter(int num) {
   unsigned char c;
   int a,b;
   ReadFonts(num);
   TextColor(7);
   ClearWindow();
   if (TDFont.Chartable[num-32]==0xFFFF) 
     return;
   if (MaxX>60) MaxX=60;
   if (MaxY>24) MaxY=24;
   for (a=1;a<=MaxY;a++)
     for (b=1;b<=MaxX;b++) {
	switch (TDFont.FontType) {
	 case 0:
	 case 1:
	   move(a+4,b+29);
	   if (Chars[b][a]>=32)
	     writechar(Chars[b][a]); else writechar(32);
	   break;
	 case 2:
	   move(a+4,b+29);
	   TextColor(Chars[b*2-1][a]);
	   if (Chars[b*2][a]>=32)
	     writechar(Chars[b*2][a]); else writechar(32);
	   break;
	}
     }
};

void ShowFont(int number){
   int a=0,b,x,ch;
   clear();
   Openfont(number);
   do {
      for (x=33;x<127;x++) {
	 move((x-33)/47,20+(x-33)%47);
	 if (x==(a+33)) TextColor(15+16); else TextColor(8);
	 if (TDFont.Chartable[x-32]!=0xFFFF)
	   if (x==(a+33)) TextColor(14+16); else TextColor(14);
	 writechar(x);
      }
      ShowCharacter(a+33);
      doupdate();
      do 
      ch=getch(); while (ch==ERR);
      switch(ch){
       case KEY_UP:
	 if ((a-47)>=0) a-=47;
	 break;
       case KEY_DOWN:
	 if (a<47) a+=47;
	 break;
       case KEY_LEFT:
	 a--;
	 if (a<0) a=126-33;
	 break;
       case KEY_RIGHT:
	 a++;
	 if ((a+33)>126) a=0;
	 break;
      }           
   } while (toupper(ch)!='Q');
   clear();
   ch=0;
}

int main() {
   int x,y,ch,b,c;
   FILE *fp,*fp2,*fp3;
   char a[255],fnts[2000][16];
   char * Name;
   unsigned char tmp;
   int size[2000];
   char FontFile[255];
   sprintf(FontFile,"%s%s",getenv("HOME"),"/.mdraw/allfont.fnt");
   
   init_bio_crt();
   fp=fopen(FontFile,"rb");
   if (fp==NULL) {
      CreateFontFile();
      fp=fopen(FontFile,"rb");      
   }   
   fread(&Header.sign,1,10,fp);
   fread(&Header.NumberofFonts,2,1,fp);
   fclose(fp);
   for (y=0;y<Header.NumberofFonts;y++) {
      Openfont(y);
      sprintf(fnts[y],"%s",FontRec.FontName);
      size[y]=FontRec.Length;
   }
   refresh();
   b=1;c=0;
   do {
      TextColor(15);
      move(4,40);
      write("[E] Export font");
      move(5,40);
      write("[I] Import font");
      move(6,40);
      write("[Q] Quit");
      for (y=0;y<=22;y++) 
	if (y+c<Header.NumberofFonts)
	{
	   move(y,0);
	   if (y==b-1) TextColor(27); else TextColor(11);
	   for (x=1;x<=fnts[y+c][0];x++) writechar(fnts[y+c][x]);
	   if (y==b-1) TextColor(24); else TextColor(8);
	   for (x=x;x<=16;x++) writechar(250);
	   TextColor(10);
	   write("%02d",(size[y+c]/1024));
	   TextColor(2);write("kb");
	}
      do
      ch=getch(); while (ch==ERR);
      switch (ch){
       case 'e':
       case 'E':
	 Openfont(c+b-1);
	 fp=fopen(FontFile,"rb");
	 for (x=1;x<=fnts[b+c-1][0];x++) a[x-1]=fnts[b+c-1][x];
	 a[x-1]=0;
	 sprintf(a,"%s%s",a,".tdf");
	 fp2=fopen(a,"wb");
	 fseek(fp,FontRec.FilePos,SEEK_SET);
	 for (x=0;x<FontRec.Length;x++)
	   fputc(fgetc(fp),fp2);
	 fclose(fp2);
	 fclose(fp);
	 break;	 
       case 'i':
       case 'I':
	 move(9,25);
	 write("Enter Name:");
	 Name=strdup(inputfield("",30,25,10));
	 fp=fopen(Name,"rb");
	 fread(&TDFont.Sign,1,19,fp);
	 fclose(fp);	 
	 TDFont.Sign[19]=0;
	 if (strcmp(TDFont.Sign,"TheDraw FONTS file")!=0) break;
	 fp=fopen(FontFile,"rb");
	 b=0;
	 while (!feof(fp)) {
	    tmp=fgetc(fp);
	    b++;
	 }
	 fseek(fp,0,SEEK_SET);
	 fp2=fopen("tmp.fnt","wb");
	 fp3=fopen(Name,"rb");
	 if (errno!=0) {
	    fclose(fp);
	    fclose(fp2);
	    fclose(fp3);
	    clear();
	    break;
	 }	 
	 fread(&Header.sign,1,10,fp);
	 fwrite(&Header.sign,1,10,fp2);
	 fread(&Header.NumberofFonts,2,1,fp);
	 Header.NumberofFonts++;
	 fwrite(&Header.NumberofFonts,2,1,fp2);	 
	 for (y=1;y<Header.NumberofFonts;y++)
	   {
	      for (x=0;x<=16;x++) FontRec.FontName[x]=fgetc(fp);
	      for (x=0;x<=16;x++) fputc(FontRec.FontName[x],fp2);
	      fread(&FontRec.FilePos,4,1,fp);
	      fread(&FontRec.Length,4,1,fp);
	      FontRec.FilePos+=FontRecordSize;
	      fwrite(&FontRec.FilePos,4,1,fp2); 
	      fwrite(&FontRec.Length,4,1,fp2);
	   }
	 for (x=0;x<=19;x++) TDFont.Sign[x]=fgetc(fp3);
	 for (x=0;x<=3;x++) TDFont.a[x]=fgetc(fp3);
	 for (x=0;x<=16;x++) TDFont.Name[x]=fgetc(fp3);
	 fread(&TDFont.FontType,1,1,fp3);
	 fread(&TDFont.Spaces,1,1,fp3);
	 fread(&TDFont.Nul,2,1,fp3);
	 for (x=1;x<=94;x++) fread(&TDFont.Chartable[x],2,1,fp3);
	 for (x=1;x<=22;x++) TDFont.b[x]=fgetc(fp3);
	 for (x=0;x<=16;x++) FontRec.FontName[x]=TDFont.Name[x];
       	 FontRec.FilePos=b+FontRecordSize;
	 for (x=0;x<=16;x++) fputc(FontRec.FontName[x],fp2);
	 fwrite(&FontRec.FilePos,4,1,fp2); 
	 fseek(fp3,0,SEEK_END);
	 FontRec.Length=ftell(fp3);
	 fwrite(&FontRec.Length,4,1,fp2);
	 x=0;
	 while (!feof(fp)) fputc(fgetc(fp),fp2);
	 fseek(fp3,0,SEEK_SET);
	 while (!feof(fp3)) fputc(fgetc(fp3),fp2);
	 fclose(fp);
	 fclose(fp2);
	 fclose(fp3);
	 remove(FontFile);
	 rename("tmp.fnt",FontFile);
	 clear();	 
	 break;
       case 10:
       case 13:
	 ShowFont(c+b-1);
	 break;
       case KEY_UP:
	 b--;
	 if (b<1) {
	    b=1;
	    c--;
	    if (c<0) c=0;
	 }	 
	 break;
       case KEY_DOWN:
	 b++;
	 if (b>Header.NumberofFonts) b=Header.NumberofFonts;
	 if (b>23) {
	    b=23;
	    c++;
	    if (c+b>=Header.NumberofFonts) c=Header.NumberofFonts-b-1;
	 }	 
	 break;
      }      
      
   } while(toupper(ch)!='Q');
   exit_bio_crt();
	return 0;
}
