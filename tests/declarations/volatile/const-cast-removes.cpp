// tidal-rules: VOL-3
// tidal-expect: accept
volatile int v;
int* p = const_cast<int*>(&v);
