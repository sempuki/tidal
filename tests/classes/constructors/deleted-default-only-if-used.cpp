// tidal-rules: CTOR-3
// tidal-expect: accept
struct S { int& r; };
int g;
S s{g};
