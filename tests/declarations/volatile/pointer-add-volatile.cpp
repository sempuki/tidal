// tidal-rules: VOL-3
// tidal-expect: accept
int i;
volatile int* p = &i;
const volatile int* q = p;
