#include<string>
#include<vector>
#include<dirent.h>
#include<errno.h>
#include"sauce.hpp"

unsigned char LoadAnsi[4096] = {
177,42,223,2,32,2,220,8,223,8,223,8,223,8,223,8,223,8,223,8,220,
8,32,8,32,8,32,40,220,42,32,42,32,42,32,42,219,2,223,2,32,
2,220,8,223,8,223,8,223,8,223,8,223,8,254,8,220,8,223,8,223,
8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,
8,223,8,223,8,254,8,223,8,223,8,223,8,223,8,223,8,223,8,223,
8,223,8,220,8,32,8,32,8,32,8,223,8,220,8,220,8,223,8,223,
8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,
8,223,8,223,8,223,8,220,8,32,8,32,40,176,42,177,42,254,42,176,
42,32,8,219,8,32,8,220,2,219,2,219,2,219,2,219,2,220,2,32,
2,219,8,32,8,32,8,32,40,176,42,177,42,32,42,223,2,32,2,219,
8,32,8,220,2,219,2,219,2,219,2,220,2,32,2,254,8,220,2,219,
2,219,2,219,2,219,2,219,2,219,2,219,2,219,2,219,2,219,2,219,
2,219,2,220,2,32,2,219,2,219,2,219,2,219,2,219,2,219,2,219,
2,220,2,32,2,219,8,32,8,32,8,32,8,222,8,219,8,32,8,68,
15,105,7,83,8,80,8,76,8,65,8,89,8,255,8,70,15,105,7,76,
8,69,8,83,8,32,8,219,8,221,8,32,8,32,40,178,42,177,42,176,
42,32,8,219,8,32,8,219,2,219,2,219,2,176,42,177,42,219,2,220,
2,32,2,219,8,32,8,32,40,177,42,176,42,223,2,32,2,219,8,32,
8,220,2,219,2,176,42,177,42,254,2,219,2,220,2,222,2,219,2,219,
2,219,2,219,114,176,42,177,42,176,42,219,114,176,42,177,42,219,114,254,
2,219,114,219,2,221,2,219,2,219,2,219,2,176,42,177,42,176,42,219,
114,219,2,220,2,32,2,219,8,32,8,32,8,222,8,219,7,32,7,87,
15,105,7,84,8,72,8,255,8,84,15,72,8,105,7,83,8,255,8,32,
8,32,8,32,8,222,8,219,7,221,8,32,8,32,40,176,42,178,42,176,
42,32,8,223,8,220,8,222,2,219,2,219,2,177,42,219,114,176,42,219,
2,32,2,219,8,32,8,32,40,176,42,223,2,32,2,219,8,32,8,220,
2,219,2,176,42,177,42,219,2,177,42,219,114,219,2,220,2,223,2,219,
2,219,114,176,42,177,42,219,114,223,2,223,2,219,114,219,114,176,42,219,
114,219,2,223,2,220,2,219,2,176,42,177,42,219,2,177,42,219,114,177,
42,219,114,219,2,220,2,32,2,254,8,32,8,32,8,254,8,32,8,69,
15,88,8,84,8,69,8,78,8,83,8,105,7,79,8,78,8,83,8,32,
8,32,8,32,8,32,8,254,8,32,8,32,8,32,40,176,42,178,42,176,
42,32,42,32,8,254,8,32,8,219,2,219,2,177,42,177,42,254,2,219,
2,32,2,219,8,32,8,32,8,223,2,32,2,219,8,32,8,220,2,219,
2,254,2,219,114,219,2,223,2,219,2,177,42,176,42,219,2,220,2,222,
2,219,2,176,42,219,114,219,114,32,8,32,8,219,114,219,2,177,42,219,
2,221,2,32,2,219,2,219,2,219,114,177,42,219,2,222,2,176,42,177,
42,176,42,219,2,219,2,32,2,219,8,32,8,32,8,254,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,219,8,32,8,32,8,32,40,177,42,177,42,176,
42,32,8,220,8,223,8,222,2,219,2,176,42,177,42,219,114,176,42,219,
2,32,2,254,8,223,8,223,8,223,8,223,8,254,8,32,8,219,2,219,
2,219,114,177,42,219,2,220,2,219,2,177,42,176,42,219,114,219,2,222,
2,219,2,176,42,177,42,219,114,220,2,220,2,219,2,177,42,219,114,219,
2,221,2,219,2,219,2,219,2,219,2,177,42,219,2,32,2,219,2,177,
42,219,114,219,114,219,2,221,2,222,8,221,8,32,8,254,8,32,8,91,
8,120,15,93,8,32,8,42,7,46,7,65,15,78,7,83,8,32,8,32,
8,32,8,32,8,254,8,254,8,32,8,32,8,32,40,176,42,177,42,32,
8,220,8,223,8,32,8,219,2,219,2,177,42,219,114,177,42,219,2,220,
2,219,2,219,2,219,2,219,2,219,2,219,2,220,2,32,2,223,2,219,
2,219,114,176,42,177,42,219,2,177,42,219,114,176,42,219,2,219,2,222,
2,219,2,219,114,177,42,219,114,219,2,219,2,219,114,178,42,177,42,219,
2,221,2,223,2,219,2,219,2,219,114,178,42,219,2,32,2,32,2,177,
42,176,42,219,114,219,2,221,2,222,8,221,8,32,8,250,8,32,8,91,
8,120,15,93,8,32,8,42,7,46,7,65,15,86,7,84,8,32,8,32,
8,32,8,254,8,254,8,254,8,32,8,32,40,254,42,176,42,178,42,220,
8,223,8,32,8,219,2,219,2,176,42,219,114,176,42,176,42,177,42,177,
42,176,42,176,42,177,42,177,42,176,42,219,2,219,114,219,2,220,2,223,
2,219,2,219,114,176,42,177,42,219,114,254,2,219,2,219,2,32,2,222,
2,219,2,176,42,177,42,219,2,32,2,32,2,219,2,177,42,219,114,219,
2,221,2,32,2,219,2,219,2,176,42,178,42,177,42,220,2,219,114,176,
42,254,2,219,2,219,2,32,2,219,8,32,8,32,8,254,8,32,8,91,
8,120,15,93,8,32,8,42,7,46,7,66,15,73,7,78,8,32,8,32,
8,254,8,254,8,254,8,32,8,32,40,176,42,176,42,177,42,178,42,219,
8,32,8,219,2,219,2,254,2,176,42,177,42,219,114,219,114,219,2,219,
2,219,2,219,2,219,2,219,114,32,34,177,42,254,2,219,114,219,2,32,
2,219,2,219,114,219,114,176,42,219,114,219,114,219,2,32,2,32,2,219,
114,219,114,254,2,219,2,219,2,32,2,32,2,219,2,219,2,254,2,176,
42,219,2,32,2,223,2,219,2,219,2,176,42,219,114,254,2,177,42,219,
114,219,2,219,2,32,2,220,8,223,8,32,8,32,8,254,8,32,8,91,
8,120,15,93,8,32,8,42,7,46,7,80,15,67,7,66,8,32,8,254,
8,254,8,223,8,32,8,32,40,177,42,178,42,177,42,178,42,178,42,219,
8,32,8,223,2,219,2,219,2,219,2,219,2,223,2,32,2,220,8,220,
8,220,8,220,8,220,8,32,8,223,2,219,2,219,2,219,2,223,2,254,
8,32,8,223,2,219,2,219,2,219,2,223,2,32,2,254,8,223,2,219,
2,219,114,219,114,219,2,223,2,254,8,223,2,219,2,219,2,219,2,219,
114,219,114,223,2,32,2,219,2,219,2,219,2,219,2,219,2,219,2,219,
2,223,2,32,2,220,8,223,8,32,8,32,8,32,8,219,8,32,8,91,
8,32,8,93,8,32,8,42,7,46,7,88,15,88,7,88,8,32,8,254,
8,32,8,32,8,32,40,177,42,176,42,177,42,219,42,178,42,219,42,220,
2,223,8,220,8,220,8,220,8,220,8,220,8,220,8,223,8,32,8,32,
8,32,8,32,8,32,8,223,8,220,8,220,8,220,8,220,8,220,8,223,
8,254,8,220,8,220,8,220,8,220,8,220,8,254,8,223,8,220,8,220,
8,220,8,220,8,220,8,220,8,223,8,220,8,220,8,220,8,220,8,220,
8,220,8,254,8,254,8,220,8,220,8,220,8,220,8,220,8,220,8,220,
8,220,8,223,8,32,8,32,8,220,8,220,8,223,8,32,8,223,8,220,
8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,223,
8,32,8,32,40,176,42,254,42,178,42,219,42,178,42,219,42,254,42,254,
8,223,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,220,
8,254,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,
8,223,8,223,8,223,8,254,8,220,8,254,8,223,8,223,8,223,8,223,
8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,8,223,
8,223,8,223,8,223,8,223,8,91,8,84,7,105,7,84,7,76,7,69,
7,93,8,223,8,254,8,220,8,254,8,223,8,223,8,223,8,223,8,223,
8,223,8,91,8,80,7,65,7,105,7,78,7,84,7,69,7,82,7,93,
8,223,8,254,8,220,8,254,8,223,8,223,8,223,8,223,8,223,8,91,
8,71,7,82,7,79,7,85,7,80,7,93,8,223,8,254,8,254,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,219,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,7,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,219,8,223,
8,254,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,
8,220,8,220,8,220,8,254,8,223,8,254,8,220,8,220,8,220,8,220,
8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,254,8,220,
8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,
8,220,8,220,8,254,8,223,8,254,8,220,8,220,8,220,8,220,8,220,
8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,220,
8,220,8,254,8,223,8,254,8,220,8,220,8,220,8,220,8,220,8,220,
8,220,8,220,8,220,8,220,8,220,8,220,8,220,8,254,8,254,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,8,32,
7,32,7,32,7,0,7,0,7,0,7,0,7,0,7,0,7,223,8,32,
8,223,8,32,8,223,8,32,8,32,8,32,8,223,8,223,8,0,7,0,
7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,
7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,7,0,
7,0,7,0,7,0,7,0,7,0,7,0,7,0,7};

struct FileDescriptor
{
   string name;
   bool   isDirectory;

   // sauce information
   string title;
   string artist;
   string group;
};

void load()
{
	vector<FileDescriptor> files;
	int readnew=1;
	FILE *fp;
	struct dirent *direntp;
	Sauce sauce;
	DIR *dirp;
  const char* file_name;
	ansout << gotoxy(0, 0);
	for (int x = 0; x <= 1950; x++) {
		if (LoadAnsi[x<<1]==0) {
			LoadAnsi[x<<1]= ' ';
		}
		ansout << textattr(LoadAnsi[(x<<1)+1]) << LoadAnsi[x<<1];
		if (COLS>80 && (x + 1) % 80 == 0) {
			ansout << endl;
		}
	}
	ansout << setfill(' ');
	SDL_Event event;
	bool done = false;
	unsigned int z=0;
	unsigned int y=0;
	do {
		if (readnew==1) {
			z = y = 0;
			readnew = 0;
			files.clear();
			dirp=opendir(".");
			direntp=readdir(dirp);
			if (dirp!=NULL){
				for (;;){
					direntp=readdir(dirp);
					if (direntp==NULL) break;
					FileDescriptor newDescriptor;
					newDescriptor.name = string(direntp->d_name);
					errno=0;
          if (strcmp(direntp->d_name, ".") == 0)
            continue; 
					fp = fopen(direntp->d_name, "rb");
					bool sauceRead = sauce.ReadSauce(fp);
					if (errno==EISDIR || fp == NULL) {
						newDescriptor.isDirectory = true;
						files.push_back(newDescriptor);
					} else {
						newDescriptor.isDirectory = false;
						if (sauceRead) {
							newDescriptor.title  = string((const char*)sauce.Title);
							newDescriptor.artist = string((const char*)sauce.Author);
							newDescriptor.group  = string((const char*)sauce.Group);
						}
						fclose(fp);
						files.push_back(newDescriptor);
					}
				}
			}
			closedir(dirp);
		}
		for (unsigned int x = 0; x <= 8; x++) {
			if (x + z < files.size()) {
				ansout << gotoxy(2, 13 + x);
				ansout << textattr(15 + (y == x ? 16 : 0));
				if (files[x + z].isDirectory) {
					ansout << textattr(7 + (y == x ? 16 : 0));
				}
				ansout << setw(12) << files[x + z].name;
				ansout << gotoxy(14, 13 + x) << ' ';
				if (!files[x + z].isDirectory) {
					ansout << textattr(7 + (y == x ? 16 : 0));
					ansout << (char)250;
					ansout << textattr(15 + (y == x ? 16 : 0));
					ansout << " " << setw(27) << files[x + z].title;
					ansout << textattr(7 + (y == x ? 16 : 0));
					ansout << (char)250;
					ansout << textattr(14 + (y == x ? 16 : 0));
					ansout << " " << setw(17) << files[x + z].artist;
					ansout << textattr(7 + (y == x ? 16 : 0));
					ansout << (char)250;
					ansout << textattr(15 + (y == x ? 16 : 0));
					ansout << setw(15) << files[x + z].group;
				} else {
					ansout << textattr(8 + (y == x ? 16 : 0));
					ansout << " <DiRECTORY>                                                    ";
				}
			} else {
				ansout << gotoxy(2, 13 + x) << textattr(7) << "                                                                             ";
			}
		}
		screenEngine.showScreen();
		while (SDL_PollEvent(&event)) {
			switch (event.type){
					case SDL_KEYDOWN:
						switch (event.key.keysym.sym) {
							case SDLK_ESCAPE:
							done = true;
							break;
						case SDLK_RETURN: {
                file_name = files[y+z].name.c_str();
                printf("file name: %s %d\n", file_name, strcmp("..", file_name));
  							if (files[y+z].isDirectory || strcmp("..", file_name) == 0) {
                  printf("chdir '%s'", file_name);
  								chdir(file_name);
  								readnew=1;
  							} else {
                  printf("try to open file: %s\n", file_name);
  								MysticDrawMain::getInstance().getCurrentBuffer()->load((char*)file_name);
  								done = true;
  							}
              }
							break;
						case SDLK_PAGEUP:
							y=0;
							if (z < 8) {
								z = 0;
							} else {
								z -= 8;
							}
							break;
						case SDLK_PAGEDOWN:
							y=8;
							z+=8;
							if (z + 9 > files.size()) z = files.size() - 9;
							if (y + 1 > files.size()) {
								y = files.size() - 1;
								z = 0;
							}
							break;
						case SDLK_UP:
							if  (y >= 1) {
								y--;
							} else if (z > 0) {
								z--;
							}
							break;
						case SDLK_DOWN:
							y++;
							if (y>8) {
								y=8;
								if (z + 9 < files.size()) z++;
							}
							if (y + 1 > files.size())  y = files.size() - 1;
							break;
						default:
							break;
					}
				}
			}
	} while (!done);
}
