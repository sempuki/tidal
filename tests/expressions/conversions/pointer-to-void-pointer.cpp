// tidal-rules: CONV-9
// tidal-expect: accept
int x;
void* v = &x;
const void* cv = &x;
const int c = 0;
const void* cv2 = &c;
