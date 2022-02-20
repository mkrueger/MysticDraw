#include<ViewModeCommand.hpp>

void ViewModeCommand::putpixel(SDL_Surface *surface, int x, int y, Uint32 pixel)
{
	int bpp = surface->format->BytesPerPixel;
	
	// Here p is the address to the pixel we want to set
	Uint8 *p = (Uint8 *)surface->pixels + y * surface->pitch + x * bpp;
	
	switch(bpp) {
		case 1:
			*p = pixel;
			break;
		case 2:
			*(Uint16*)p = pixel;
			break;
		case 3:
			if (SDL_BYTEORDER == SDL_BIG_ENDIAN) {
				p[0] = (pixel >> 16) & 0xff;
				p[1] = (pixel >> 8) & 0xff;
				p[2] = pixel & 0xff;
			} else {
				p[0] = pixel & 0xff;
				p[1] = (pixel >> 8) & 0xff;
				p[2] = (pixel >> 16) & 0xff;
			}
			break;
		case 4:
			*(Uint32 *)p = pixel;
			break;
	}
}

void ViewModeCommand::run()
{
	bool done = false;
	SDL_Event event;
	SDL_FillRect(screen, NULL, SDL_MapRGB(screen->format, 0, 0, 0));
	
	int width  = MysticDrawMain::getInstance().getCurrentBuffer()->getWidth();
	int height = MysticDrawMain::getInstance().getCurrentBuffer()->getHeight();
	Uint8* palette = screenEngine.getPalette();
	do {
		SDL_Delay(200);
		int ret = SDL_LockSurface(screen);
		if (ret == 0) {
			for (int y = 0; y < screen->h / 2 - 1; ++y) {
				for (int x = 0; x < width; ++x) {
					if (y < height) {
					
						int back = (MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x) & 240) >> 4;
						int fore = MysticDrawMain::getInstance().getCurrentBuffer()->getAttribute(y, x) & 15;
						
						Uint32 pixel1;
						Uint32 pixel2;
						
						switch (MysticDrawMain::getInstance().getCurrentBuffer()->getCharacter(y, x)) {
							case 0:
							case ' ':
							case 255:
								pixel1 = pixel2 = SDL_MapRGB(screen->format, *(palette + back * 3) * 4, *(palette + back * 3 + 1) * 4, *(palette + back * 3 + 2) * 4);
								break;
							case 196:
							case 223:
							case '-':
								pixel1 = SDL_MapRGB(screen->format, *(palette + fore * 3) * 4, *(palette + fore * 3 + 1) * 4, *(palette + fore * 3 + 2) * 4);
								pixel2 = SDL_MapRGB(screen->format, *(palette + back * 3) * 4, *(palette + back * 3 + 1) * 4, *(palette + back * 3 + 2) * 4);
								break;
							case 178:
								pixel1 = pixel2 = SDL_MapRGB(screen->format, (int)((float)*(palette + fore * 3)     * 0.75 + (float)*(palette + back * 3) * 0.25) * 4, 
								                                             (int)((float)*(palette + fore * 3 + 1) * 0.75 + (float)*(palette + back * 3 + 1) * 0.25) * 4,
																			 (int)((float)*(palette + fore * 3 + 2) * 0.75 + (float)*(palette + back * 3 + 2) * 0.25) * 4);
								break;
							case 222:
							case 221:
							case 177:
								pixel1 = pixel2 = SDL_MapRGB(screen->format, (int)((float)*(palette + fore * 3)     * 0.5 + (float)*(palette + back * 3) * 0.5) * 4, 
								                                             (int)((float)*(palette + fore * 3 + 1) * 0.5 + (float)*(palette + back * 3 + 1) * 0.5) * 4,
																			 (int)((float)*(palette + fore * 3 + 2) * 0.5 + (float)*(palette + back * 3 + 2) * 0.5) * 4);
								break;
							case 176:
								pixel1 = pixel2 = SDL_MapRGB(screen->format, (int)((float)*(palette + fore * 3)     * 0.25 + (float)*(palette + back * 3) * 0.75) * 4, 
								                                             (int)((float)*(palette + fore * 3 + 1) * 0.25 + (float)*(palette + back * 3 + 1) * 0.75) * 4,
																			 (int)((float)*(palette + fore * 3 + 2) * 0.25 + (float)*(palette + back * 3 + 2) * 0.75) * 4);
								break;
							default:
								pixel1 = pixel2 = SDL_MapRGB(screen->format, *(palette + fore * 3) * 4, *(palette + fore * 3 + 1) * 4, *(palette + fore * 3 + 2) * 4);
								break;
						}
						putpixel(screen, x, y * 2,     pixel1);
						putpixel(screen, x, y * 2 + 1, pixel2);
					} else {
						putpixel(screen, x, y * 2,     0);
						putpixel(screen, x, y * 2 + 1, 0);
					}
				}
			}
			SDL_UnlockSurface(screen);
			SDL_UpdateRect(screen, 0, 0, 0, 0);
		}
		while (SDL_PollEvent(&event)) {
			switch (event.type) {
				case SDL_KEYDOWN:
					switch (event.key.keysym.sym) {
						case SDLK_ESCAPE:
							done = true;
							break;
						default:
							break;
					}
					break;
			}
		}
	} while (!done);
}
