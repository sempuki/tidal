// tidal-rules: CV-7
// tidal-expect: accept
int x;
int* p = &x;
const int* q = p;
int** pp = &p;
const int* const* qq = pp;
volatile int* v = p;
