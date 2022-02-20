CC=g++
STRIP=strip
LIBS=-lSDL -lm
#LIBS=-lncurses -lm -lgpm -lvga
OPTIONS=-I include -Wall -pedantic -I include

#Full features:
#DEFINE=-DHAS_GPM -DHAS_SVGALIB
DEFINE=


objects = ASCIITableCommand.o DrawCommand.o mdraw.o SelectColorCommand.o SelectSauceCommand.o \
          bio_crt.o fonts.o sauce.o SelectEffectModeCommand.o TabulatorCommand.o \
          BlockModeCommand.o HelpCommand.o ScreenBuffer.o SelectFontCommand.o FontEditorCommand.o \
	  PaletteEditorCommand.o ViewModeCommand.o

all:	$(objects) buildfont
	$(CC) $(DEFINE) $(OPTIONS) -o mdraw $(objects) $(LIBS)
ASCIITableCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o ASCIITableCommand.o ASCIITableCommand.cpp
DrawCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o DrawCommand.o DrawCommand.cpp
SelectColorCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o SelectColorCommand.o SelectColorCommand.cpp
SelectSauceCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o SelectSauceCommand.o SelectSauceCommand.cpp
bio_crt.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o bio_crt.o bio_crt.cpp
fonts.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o fonts.o fonts.cpp
sauce.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o sauce.o sauce.cpp
SelectEffectModeCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o SelectEffectModeCommand.o SelectEffectModeCommand.cpp
TabulatorCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o TabulatorCommand.o TabulatorCommand.cpp
BlockModeCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o BlockModeCommand.o BlockModeCommand.cpp
HelpCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o HelpCommand.o HelpCommand.cpp
ScreenBuffer.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o ScreenBuffer.o ScreenBuffer.cpp
SelectFontCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o SelectFontCommand.o SelectFontCommand.cpp
mdraw.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o mdraw.o mdraw.cpp
FontEditorCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o FontEditorCommand.o FontEditorCommand.cpp
PaletteEditorCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o PaletteEditorCommand.o PaletteEditorCommand.cpp
ViewModeCommand.o:
	${CC} ${DEFINE} ${OPTIONS} -c -o ViewModeCommand.o ViewModeCommand.cpp

buildfont:
	$(CC) $(DEFINE) $(OPTIONS) -o buildfont tools/buildfont.cpp fonts.o $(LIBS)

clean:
	rm -f buildfont fontutil mdraw
	rm -f *.o
