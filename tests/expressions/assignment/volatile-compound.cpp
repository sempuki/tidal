// tidal-rules: ASN-7
// tidal-expect: accept
volatile int v = 0;
void f() { v += 1; v |= 2; }
