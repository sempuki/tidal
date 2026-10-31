// tidal-rules: VOL-5
// tidal-expect: accept
// tidal-flags: -Wall -Werror
volatile int v;
void f() { v += 1; v |= 2; v = v + 1; }
