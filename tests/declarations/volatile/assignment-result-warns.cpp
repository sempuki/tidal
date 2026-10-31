// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-flags: -Wall -Werror
volatile int v;
int f() { int a = (v = 1); return a; }
