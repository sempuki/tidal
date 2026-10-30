// tidal-rules: CTOR-3
// tidal-expect: accept
int g;
struct S { int& r = g; };
S s;
