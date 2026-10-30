// tidal-rules: CPY-7
// tidal-expect: accept
int g;
struct S { int& r = g; };
S a;
S b(a);
