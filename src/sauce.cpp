#include<sauce.hpp>

void Sauce::AppendSauce(FILE *fp)
{
	char a = 26;
	fwrite(&a,1,1,fp);
	fputc('S', fp);
	fputc('A', fp);
	fputc('U', fp);
	fputc('C', fp);
	fputc('E', fp);
	
	Version[0]='0';
	Version[1]='0';
	fwrite(&Version,1,2,fp);
	fwrite(&Title,1,35,fp);
	fwrite(&Author,1,20,fp);
	fwrite(&Group,1,20,fp);
	fwrite(&Date,1,8,fp);
	fwrite(&FileSize,4,1,fp);
	fwrite(&DataType,1,1,fp);
	fwrite(&FileType,1,1,fp);
	fwrite(&TInfo1,2,1,fp);
	fwrite(&TInfo2,2,1,fp);
	fwrite(&TInfo3,2,1,fp);
	fwrite(&TInfo4,2,1,fp);
	fwrite(&Comments,1,1,fp);
	fwrite(&Flags,1,1,fp);
	fwrite(&Filler,1,22,fp);
}

bool Sauce::ReadSauce(FILE *fp)
{
	if (fp == NULL) {
		return false;
	}
	
	fseek(fp, -128, SEEK_END);
	if (fread(&id, 1, 5,fp) <= 0) {
		return false;
	}
	
	fread(&Version, 1, 2,fp);
	fread(&Title,   1, 35,fp);
	fread(&Author,  1, 20,fp);
	fread(&Group,   1, 20,fp);
	fread(&Date,    1, 8,fp);
	Title[35]=0;
	Group[20]=0;
	Author[20]=0;
	fread(&FileSize, 1, 4,fp);
	fread(&DataType, 1, 1,fp);
	fread(&FileType, 1, 1,fp);
	fread(&TInfo1, 1, 2,fp);
	fread(&TInfo2, 1, 2,fp);
	fread(&TInfo3, 1, 2,fp);
	fread(&TInfo4, 1, 2,fp);
	fread(&Comments, 1, 1,fp);
	fread(&Flags, 1, 1,fp);
	fread(&Filler, 1, 22,fp);
	
	bool sauceOk = id[0]=='S' && id[1]=='A' && id[2]=='U' && id[3]=='C' && id[4]=='E';
	
	if (!sauceOk) {
		memset(Title, 0, sizeof(Title));
		memset(Group, 0, sizeof(Group));
		memset(Author, 0, sizeof(Author));
	}
	return sauceOk;
}

